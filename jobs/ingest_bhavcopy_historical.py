#!/usr/bin/env python3
"""
Historical Bhavcopy Data Ingestion Script

This script downloads NSE bhavcopy data for a date range and ingests it into a Hudi table
using Apache Spark. It performs data cleaning, adds necessary columns, and writes to Hudi
in append mode for each date in the specified range.

Usage:
    spark-submit ingest_bhavcopy_historical.py <start_date YYYY-MM-DD> <end_date YYYY-MM-DD> <hudi_base_path>

Arguments:
    start_date: Start date for data ingestion (YYYY-MM-DD)
    end_date: End date for data ingestion (YYYY-MM-DD)
    hudi_base_path: The base path for Hudi tables

Example:
    spark-submit ingest_bhavcopy_historical.py 2023-01-01 2023-12-31 /path/to/hudi/base
"""

# Standard library imports
import logging
import sys
from datetime import datetime, timedelta
from pathlib import Path
from typing import Optional

# Third-party imports
from pyspark.sql import SparkSession
from pyspark.sql.functions import col, concat_ws

# Add parent directory to path for local imports
sys.path.insert(0, str(Path(__file__).parent.parent))

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
        datetime.strptime(date_str, "%Y-%m-%d")
        return True
    except ValueError:
        return False


def validate_inputs() -> tuple[datetime, datetime, str]:
    """Validate command line arguments and return parsed dates and hudi_base_path"""
    if len(sys.argv) != 4:
        logger.error("Usage: spark-submit ingest_bhavcopy_historical.py <start_date YYYY-MM-DD> <end_date YYYY-MM-DD> <hudi_base_path>")
        sys.exit(1)

    start_date_str = sys.argv[1]
    end_date_str = sys.argv[2]
    hudi_base_path = sys.argv[3]

    if not validate_date(start_date_str):
        logger.error(f"Invalid start date format: {start_date_str}. Expected YYYY-MM-DD")
        sys.exit(1)

    if not validate_date(end_date_str):
        logger.error(f"Invalid end date format: {end_date_str}. Expected YYYY-MM-DD")
        sys.exit(1)

    if not hudi_base_path:
        logger.error("Hudi base path cannot be empty")
        sys.exit(1)

    start_date = datetime.strptime(start_date_str, "%Y-%m-%d")
    end_date = datetime.strptime(end_date_str, "%Y-%m-%d")

    if start_date > end_date:
        logger.error("Start date cannot be after end date")
        sys.exit(1)

    return start_date, end_date, hudi_base_path


def create_spark_session(start_date: str, end_date: str) -> 'SparkSession':
    """Create and configure Spark session"""
    logger.info("Creating Spark session...")
    spark = (
        SparkSession.builder
        .appName(f"bhavcopy-ingest-historical-{start_date}-{end_date}")
        .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer")
        .config("spark.sql.adaptive.enabled", "true")
        .config("spark.sql.adaptive.coalescePartitions.enabled", "true")
        .getOrCreate()
    )
    logger.info("Spark session created successfully")
    return spark


def process_bhavcopy_data(spark: 'SparkSession', csv_path: str) -> Optional['DataFrame']:
    """Process the downloaded CSV file into a cleaned DataFrame"""
    logger.info(f"Processing CSV file: {csv_path}")

    if not Path(csv_path).exists():
        logger.error(f"CSV file does not exist: {csv_path}")
        return None

    try:
        # Read CSV with options
        df = (
            spark.read
            .option("header", "true")
            .option("inferSchema", "true")
            .option("ignoreLeadingWhiteSpace", "true")
            .option("ignoreTrailingWhiteSpace", "true")
            .csv(csv_path)
        )

        row_count = df.count()
        logger.info(f"Read {row_count} rows from CSV")

        if row_count == 0:
            logger.warning("No data found in CSV file")
            return None

        # Trim column names
        df = df.select([col(c).alias(c.strip()) for c in df.columns])

        # Add required columns for Hudi
        df = df.withColumn("record_key", concat_ws("-", col("SYMBOL"), col("DATE1")))
        df = df.withColumn("trade_date", col("DATE1"))

        logger.info("Data processing completed")
        return df

    except Exception as e:
        logger.error(f"Error processing CSV file: {e}")
        return None


def write_to_hudi(df: 'DataFrame', hudi_base_path: str, session_date: str) -> bool:
    """Write DataFrame to Hudi table"""
    try:
        hudi_options = hudi_write_options(
            table_name="sec_bhavdata",
            record_key="record_key",
            precombine_key="trade_date",
            partition_key="trade_date"
        )

        table_path = f"{hudi_base_path}/sec_bhavdata"
        logger.info(f"Writing to Hudi table at: {table_path}")

        df.write.format("hudi").options(**hudi_options).mode("append").save(table_path)

        logger.info("Successfully wrote data to Hudi")
        return True

    except Exception as e:
        logger.error(f"Error writing to Hudi: {e}")
        return False


def process_date(spark: 'SparkSession', session_date: str, hudi_base_path: str) -> bool:
    """Process data for a single date"""
    logger.info(f"Processing date: {session_date}")

    try:
        # Download bhavcopy
        csv_path = download_bhavcopy(session_date)
        logger.info(f"Downloaded bhavcopy to: {csv_path}")

        # Process data
        df = process_bhavcopy_data(spark, csv_path)
        if df is None:
            logger.error(f"Failed to process bhavcopy data for {session_date}")
            return False

        # Write to Hudi
        if not write_to_hudi(df, hudi_base_path, session_date):
            logger.error(f"Failed to write data to Hudi for {session_date}")
            return False

        logger.info(f"Successfully ingested data for {session_date}")
        return True

    except Exception as e:
        logger.error(f"Error processing {session_date}: {e}")
        return False


def main():
    """Main execution function"""
    logger.info("Starting bhavcopy historical ingestion")

    # Validate inputs
    start_date, end_date, hudi_base_path = validate_inputs()

    spark = None
    success_count = 0
    total_dates = 0

    try:
        # Create Spark session
        spark = create_spark_session(start_date.strftime("%Y-%m-%d"), end_date.strftime("%Y-%m-%d"))

        # Process each date in the range
        current_date = start_date
        while current_date <= end_date:
            session_date = current_date.strftime("%Y-%m-%d")
            total_dates += 1

            if process_date(spark, session_date, hudi_base_path):
                success_count += 1

            current_date += timedelta(days=1)

        logger.info(f"Historical ingestion completed. Processed {success_count}/{total_dates} dates successfully")

    except Exception as e:
        logger.error(f"Historical ingestion failed: {e}")
        sys.exit(1)
    finally:
        if spark:
            logger.info("Stopping Spark session")
            spark.stop()


if __name__ == "__main__":
    main()


