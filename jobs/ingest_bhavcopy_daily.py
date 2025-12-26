#!/usr/bin/env python3
"""
Daily Bhavcopy Data Ingestion Script

DEPRECATED: This script is kept for backwards compatibility.
For new deployments, use ingest_bhavcopy_daily_v2.py which follows proper architecture.

This script downloads the NSE bhavcopy data for a given date and ingests it into a Hudi table
using Apache Spark. It performs data cleaning, adds necessary columns, and writes to Hudi
in append mode.

Usage:
    spark-submit ingest_bhavcopy_daily.py <YYYY-MM-DD> <hudi_base_path>

Arguments:
    YYYY-MM-DD: The session date for which to download bhavcopy data
    hudi_base_path: The base path for Hudi tables

Example:
    spark-submit ingest_bhavcopy_daily.py 2023-12-25 /path/to/hudi/base
"""

# Standard library imports
import logging
import os
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional

# Third-party imports
from pyspark.sql import SparkSession
from pyspark.sql.functions import col, concat_ws

# Add parent directory to path for local imports
sys.path.insert(0, str(Path(__file__).parent.parent))

# Local imports
from conf.hudi import hudi_write_options

# Import legacy download function for backwards compatibility
try:
    from utils.nse_download import download_bhavcopy
except ImportError:
    # Fallback to inline implementation for compatibility
    import requests
    
    def download_bhavcopy(session_date: str) -> str:
        """Legacy download function"""
        date_obj = datetime.strptime(session_date, "%Y-%m-%d")
        session_date_str = date_obj.strftime("%d%m%Y")
        url = f"https://nsearchives.nseindia.com/products/content/sec_bhavdata_full_{session_date_str}.csv"
        output_dir = f"/tmp/bhavcopy/{session_date}"
        os.makedirs(output_dir, exist_ok=True)

        headers = {
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            "Accept-Language": "en-US,en;q=0.5",
            "Accept-Encoding": "gzip, deflate",
            "Connection": "keep-alive",
            "Upgrade-Insecure-Requests": "1",
        }

        file_path = f"{output_dir}/sec_bhavdata_{session_date}.csv"
        response = requests.get(url, headers=headers, timeout=30)
        if response.status_code == 200:
            with open(file_path, 'wb') as f:
                f.write(response.content)
            return file_path
        else:
            raise Exception(f"Failed to download bhavcopy for {session_date}, status code: {response.status_code}")


# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

# Log deprecation warning
logger.warning("=" * 80)
logger.warning("DEPRECATION WARNING: This script uses the old architecture.")
logger.warning("For new deployments, please use ingest_bhavcopy_daily_v2.py")
logger.warning("=" * 80)


def validate_date(date_str: str) -> bool:
    """Validate date string format YYYY-MM-DD"""
    try:
        datetime.strptime(date_str, "%Y-%m-%d")
        return True
    except ValueError:
        return False


def validate_inputs() -> tuple[str, str]:
    """Validate command line arguments and return session_date and hudi_base_path"""
    if len(sys.argv) != 3:
        logger.error("Usage: spark-submit ingest_bhavcopy_daily.py <YYYY-MM-DD> <hudi_base_path>")
        sys.exit(1)

    session_date = sys.argv[1]
    hudi_base_path = sys.argv[2]

    if not validate_date(session_date):
        logger.error(f"Invalid date format: {session_date}. Expected YYYY-MM-DD")
        sys.exit(1)

    if not hudi_base_path:
        logger.error("Hudi base path cannot be empty")
        sys.exit(1)

    return session_date, hudi_base_path


def create_spark_session(session_date: str) -> 'SparkSession':
    """Create and configure Spark session"""

    logger.info("Creating Spark session...")
    spark = (
        SparkSession.builder
        .appName(f"bhavcopy-ingest-daily-{session_date}")
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

        logger.info(f"Read {df.count()} rows from CSV")

        if df.count() == 0:
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


def main():
    """Main execution function"""
    logger.info("Starting bhavcopy daily ingestion")

    # Validate inputs
    session_date, hudi_base_path = validate_inputs()

    spark = None
    try:
        # Create Spark session
        spark = create_spark_session(session_date)

        # Download bhavcopy
        logger.info(f"Downloading bhavcopy for date: {session_date}")
        csv_path = download_bhavcopy(session_date)
        logger.info(f"Downloaded bhavcopy to: {csv_path}")

        # Process data
        df = process_bhavcopy_data(spark, csv_path)
        if df is None:
            logger.error("Failed to process bhavcopy data")
            sys.exit(1)

        # Write to Hudi
        if not write_to_hudi(df, hudi_base_path, session_date):
            logger.error("Failed to write data to Hudi")
            sys.exit(1)

        logger.info("Bhavcopy ingestion completed successfully")

    except Exception as e:
        logger.error(f"Ingestion failed: {e}")
        sys.exit(1)
    finally:
        if spark:
            logger.info("Stopping Spark session")
            spark.stop()


if __name__ == "__main__":
    main()