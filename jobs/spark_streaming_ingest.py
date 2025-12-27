#!/usr/bin/env python3
"""
Spark Streaming Ingestion Job for Galactus

Consumes NSE data events from Kafka and writes to Hudi data lake layers:
- Bronze: Raw immutable events
- Silver: Cleaned and validated
- Gold: Analytics-ready aggregations

This job runs continuously as a streaming application.
"""

import logging
import sys
from datetime import datetime
from typing import Dict, Any

from pyspark.sql import SparkSession
from pyspark.sql.functions import (
    col, from_json, schema_of_json, explode,
    window, count, sum, avg, max, min
)
from pyspark.sql.types import StructType, StructField, StringType, TimestampType, IntegerType, BinaryType

from conf.config import config
from conf.hudi import hudi_write_options
from utils.logging_utils import setup_logger, log_job_start, log_data_lineage

# Setup logging
logger = setup_logger(__name__)


def create_spark_session() -> SparkSession:
    """Create Spark session with Kafka and Hudi support"""
    return (
        SparkSession.builder
        .appName("Galactus-Spark-Streaming")
        .config("spark.sql.streaming.checkpointLocation", f"{config.DATA_ROOT}/checkpoints")
        .config("spark.jars.packages",
                "org.apache.spark:spark-sql-kafka-0-10_2.12:3.4.0,"
                "org.apache.hudi:hudi-spark3.4-bundle_2.12:0.14.0")
        .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer")
        .config("spark.sql.hive.convertMetastoreParquet", "false")
        .master(config.SPARK_MASTER)
        .getOrCreate()
    )


def get_kafka_options(topic: str) -> Dict[str, str]:
    """Get Kafka source options"""
    return {
        "kafka.bootstrap.servers": config.KAFKA_BOOTSTRAP_SERVERS,
        "subscribe": topic,
        "startingOffsets": "latest",  # For production, use "earliest" for backfill
        "failOnDataLoss": "false"
    }


def get_bronze_schema() -> StructType:
    """Schema for Bronze layer events"""
    return StructType([
        StructField("source", StringType(), True),
        StructField("dataset", StringType(), True),
        StructField("symbol", StringType(), True),
        StructField("event_time", StringType(), True),
        StructField("scrape_time", StringType(), True),
        StructField("payload", StringType(), True),  # JSON string
        StructField("checksum", StringType(), True),
        StructField("version", IntegerType(), True)
    ])


def write_to_hudi_bronze(df, table_name: str):
    """Write streaming DataFrame to Hudi Bronze table"""
    hudi_options = hudi_write_options(
        table_name=table_name,
        record_key="checksum",
        precombine_field="scrape_time",
        partition_field="event_time",  # Partition by date
        operation="upsert",
        table_type="COPY_ON_WRITE"
    )

    return (
        df.writeStream
        .format("hudi")
        .options(**hudi_options)
        .outputMode("append")
        .option("checkpointLocation", f"{config.DATA_ROOT}/checkpoints/bronze_{table_name}")
        .start(config.get_silver_path(table_name).as_posix())
    )


def process_bronze_to_silver(spark: SparkSession, bronze_table: str, silver_table: str):
    """Process Bronze data to Silver layer with cleaning and validation"""

    # Read from Bronze Hudi table
    bronze_df = (
        spark.read
        .format("hudi")
        .load(config.get_silver_path(bronze_table).as_posix())
    )

    # Parse payload and clean data
    # This is a simplified example - actual cleaning logic would be more complex
    silver_df = (
        bronze_df
        .withColumn("parsed_payload", from_json(col("payload"), schema_of_json(col("payload"))))
        .select(
            col("checksum").alias("record_key"),
            col("symbol"),
            col("event_time").alias("trade_date"),
            col("parsed_payload.*"),
            col("version"),
            col("scrape_time")
        )
    )

    # Write to Silver Hudi table
    hudi_options = hudi_write_options(
        table_name=silver_table,
        record_key="record_key",
        precombine_field="version",
        partition_field="trade_date",
        operation="upsert",
        table_type="MERGE_ON_READ"
    )

    (
        silver_df.write
        .format("hudi")
        .options(**hudi_options)
        .mode("append")
        .save(config.get_silver_path(silver_table).as_posix())
    )


def process_silver_to_gold(spark: SparkSession, silver_table: str, gold_table: str):
    """Aggregate Silver data to Gold layer for analytics"""

    # Read from Silver Hudi table
    silver_df = (
        spark.read
        .format("hudi")
        .load(config.get_silver_path(silver_table).as_posix())
    )

    # Example aggregation: Daily OHLCV by symbol
    gold_df = (
        silver_df
        .groupBy("symbol", "trade_date")
        .agg(
            count("*").alias("num_trades"),
            sum("volume").alias("total_volume"),
            avg("close").alias("avg_close"),
            max("high").alias("max_high"),
            min("low").alias("min_low")
        )
        .withColumn("gold_timestamp", col("trade_date"))
    )

    # Write to Gold Hudi table
    hudi_options = hudi_write_options(
        table_name=gold_table,
        record_key="symbol,trade_date",
        precombine_field="gold_timestamp",
        partition_field="trade_date",
        operation="upsert",
        table_type="COPY_ON_WRITE"
    )

    (
        gold_df.write
        .format("hudi")
        .options(**hudi_options)
        .mode("append")
        .save(config.get_silver_path(gold_table).as_posix())
    )


def main():
    """Main streaming job"""
    log_job_start("spark_streaming_ingest")

    spark = create_spark_session()

    try:
        # Kafka topics to consume
        topics = {
            "nse.raw.bhavcopy.cash": "hudi_nse_bronze_bhavcopy",
            "nse.raw.bhavcopy.fo": "hudi_nse_bronze_fo",
            "nse.raw.corporate.actions": "hudi_nse_bronze_corporate_actions"
        }

        # Start streaming queries for Bronze layer
        streaming_queries = []

        for topic, bronze_table in topics.items():
            # Read from Kafka
            kafka_df = (
                spark.readStream
                .format("kafka")
                .options(**get_kafka_options(topic))
                .load()
            )

            # Parse JSON value to structured data
            bronze_df = (
                kafka_df
                .selectExpr("CAST(value AS STRING) as json_value")
                .select(from_json(col("json_value"), get_bronze_schema()).alias("data"))
                .select("data.*")
                .withColumn("event_time", col("event_time").cast("date"))  # For partitioning
            )

            # Write to Bronze Hudi
            query = write_to_hudi_bronze(bronze_df, bronze_table)
            streaming_queries.append(query)

            logger.info(f"Started streaming ingestion for {topic} -> {bronze_table}")

        # Wait for termination or run batch processing
        for query in streaming_queries:
            query.awaitTermination()

    except Exception as e:
        logger.error(f"Streaming job failed: {e}")
        raise
    finally:
        spark.stop()


if __name__ == "__main__":
    main()