#!/usr/bin/env python3
"""
Daily Bhavcopy Data Ingestion Script with ClickHouse Sync

This script downloads the NSE bhavcopy data for a given date and ingests it into a Hudi table
using Apache Spark. It performs data cleaning, adds necessary columns, and writes to Hudi.
After successful Hudi write, it syncs the data to ClickHouse for real-time analytics.

Usage:
    spark-submit ingest_bhavcopy_daily_sync.py <YYYY-MM-DD> <hudi_base_path>

Arguments:
    YYYY-MM-DD: The session date for which to download bhavcopy data
    hudi_base_path: The base path for Hudi tables

Example:
    spark-submit ingest_bhavcopy_daily_sync.py 2023-12-25 /path/to/hudi/base
"""

# Standard library imports
import logging
import os
import sys
import subprocess
from datetime import datetime
from pathlib import Path
from typing import Optional

# Third-party imports
from pyspark.sql import SparkSession
from pyspark.sql.functions import col, concat_ws

# Local imports
from conf.hudi import hudi_write_options
from utils.nse_download import download_bhavcopy

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

def validate_date(date_str: str) -> bool:
    """Validate date string format YYYY-MM-DD"""
    try:
        datetime.strptime(date_str, '%Y-%m-%d')
        return True
    except ValueError:
        return False

def create_spark_session(app_name: str = "bhavcopy-ingest-daily") -> SparkSession:
    """Create Spark session with necessary configurations"""
    logger.info("Creating Spark session...")

    spark = SparkSession.builder \
        .appName(app_name) \
        .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer") \
        .config("spark.sql.hive.convertMetastoreParquet", "false") \
        .config("spark.hadoop.hive.metastore.schema.verification", "false") \
        .config("spark.sql.legacy.timeParserPolicy", "LEGACY") \
        .getOrCreate()

    logger.info("Spark session created successfully")
    return spark

def process_bhavcopy_data(spark: SparkSession, csv_path: str, trade_date: str) -> Optional[object]:
    """Process CSV data and return DataFrame"""
    logger.info(f"Processing CSV file: {csv_path}")

    try:
        # Read CSV with proper schema
        df = spark.read \
            .option("header", "true") \
            .option("inferSchema", "true") \
            .csv(csv_path)

        # Trim column names first
        df = df.select([col(c).alias(c.strip()) for c in df.columns])

        # Rename columns to snake_case
        column_mapping = {
            'SYMBOL': 'symbol',
            'SERIES': 'series',
            'OPEN_PRICE': 'open_price',
            'HIGH_PRICE': 'high_price',
            'LOW_PRICE': 'low_price',
            'CLOSE_PRICE': 'close_price',
            'LAST_PRICE': 'last_price',
            'PREV_CLOSE': 'prev_close',
            'TTL_TRD_QNTY': 'total_traded_qty',
            'TURNOVER_LACS': 'turnover_lacs',
            'NO_OF_TRADES': 'no_of_trades',
            'DELIV_QTY': 'deliv_qty',
            'DELIV_PER': 'deliv_per'
        }

        for old_col, new_col in column_mapping.items():
            if old_col in df.columns:
                df = df.withColumnRenamed(old_col, new_col)

        # Add trade_date column from DATE1 with proper date parsing
        from pyspark.sql.functions import to_date
        df = df.withColumn("trade_date", to_date(col("DATE1"), "dd-MMM-yyyy"))

        # Create record key for Hudi
        df = df.withColumn("record_key", concat_ws("_", col("symbol"), col("series"), col("trade_date")))

        logger.info(f"Processed {df.count()} records")
        return df

    except Exception as e:
        logger.error(f"Error processing CSV: {e}")
        return None

def write_to_hudi(df, hudi_base_path: str, table_name: str = "sec_bhavdata"):
    """Write DataFrame to Hudi table"""
    logger.info(f"Writing to Hudi table: {table_name}")

    try:
        hudi_path = f"{hudi_base_path}/{table_name}"

        # Get Hudi write options
        write_options = hudi_write_options(
            table_name=table_name,
            record_key="record_key",
            precombine_key="trade_date",
            partition_key="trade_date"
        )

        # Write to Hudi
        df.write \
            .format("hudi") \
            .options(**write_options) \
            .mode("append") \
            .save(hudi_path)

        logger.info("Successfully wrote data to Hudi")
        return True

    except Exception as e:
        logger.error(f"Error writing to Hudi: {e}")
        return False

def sync_to_clickhouse(table_name: str, hudi_base_path: str):
    """Sync data from Hudi to ClickHouse"""
    logger.info("Starting ClickHouse sync...")

    try:
        # Run the sync script
        cmd = [
            sys.executable,
            "/media/sandeep/DataDrive/galactus/utils/hudi_clickhouse_sync.py",
            "--table", table_name,
            "--hudi-path", hudi_base_path
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, cwd="/media/sandeep/DataDrive/galactus")

        if result.returncode == 0:
            logger.info("ClickHouse sync completed successfully")
            return True
        else:
            logger.error(f"ClickHouse sync failed: {result.stderr}")
            return False

    except Exception as e:
        logger.error(f"Error during ClickHouse sync: {e}")
        return False

def main():
    """Main function"""
    if len(sys.argv) != 3:
        logger.error("Usage: spark-submit ingest_bhavcopy_daily_sync.py <YYYY-MM-DD> <hudi_base_path>")
        sys.exit(1)

    date_str = sys.argv[1]
    hudi_base_path = sys.argv[2]

    # Validate inputs
    if not validate_date(date_str):
        logger.error(f"Invalid date format: {date_str}. Expected YYYY-MM-DD")
        sys.exit(1)

    logger.info(f"Starting bhavcopy daily ingestion for date: {date_str}")

    # Create Spark session
    spark = create_spark_session()

    try:
        # Download bhavcopy
        logger.info(f"Downloading bhavcopy for date: {date_str}")
        csv_path = download_bhavcopy(date_str)

        if not csv_path or not os.path.exists(csv_path):
            logger.error(f"Failed to download bhavcopy for {date_str}")
            sys.exit(1)

        # Process data
        df = process_bhavcopy_data(spark, csv_path, date_str)
        if df is None:
            logger.error("Failed to process bhavcopy data")
            sys.exit(1)

        # Write to Hudi
        table_name = "sec_bhavdata"
        if not write_to_hudi(df, hudi_base_path, table_name):
            logger.error("Failed to write to Hudi")
            sys.exit(1)

        # Sync to ClickHouse
        if not sync_to_clickhouse(table_name, hudi_base_path):
            logger.warning("ClickHouse sync failed, but Hudi write was successful")

        logger.info("Bhavcopy ingestion completed successfully")

    except Exception as e:
        logger.error(f"Ingestion failed: {e}")
        sys.exit(1)
    finally:
        spark.stop()

if __name__ == "__main__":
    main()