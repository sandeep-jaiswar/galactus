#!/usr/bin/env python3
"""
Daily Bhavcopy Data Ingestion - Improved Version

This script orchestrates the daily NSE bhavcopy ingestion following proper architecture:
1. Scrape raw data to Bronze layer (immutable)
2. Transform and validate to Silver layer (Hudi)
3. Follow all architectural principles

Usage:
    spark-submit \
        --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
        ingest_bhavcopy_daily_v2.py <YYYY-MM-DD> [hudi_base_path]

Arguments:
    YYYY-MM-DD: Trading date to process
    hudi_base_path: Optional Hudi base path (defaults to config)
"""

import sys
from datetime import datetime
from pathlib import Path
from typing import Optional

# Add parent directory to path for local imports
sys.path.insert(0, str(Path(__file__).parent.parent))

from pyspark.sql import SparkSession

from conf.config import config
from conf.hudi import hudi_write_options
from utils.nse_download import download_bhavcopy, NSEDownloadError
from utils.silver_processor import (
    process_bhavcopy_to_silver, 
    validate_silver_write,
    SilverProcessingError
)
from utils.logging_utils import (
    setup_logger, 
    log_job_start, 
    log_job_end,
    log_data_lineage
)


# Setup logger
logger = setup_logger(
    __name__,
    log_file=config.LOG_DIR / f"daily_ingestion_{datetime.now().strftime('%Y%m%d')}.log"
)


def validate_inputs() -> tuple[str, Path]:
    """
    Validate command line arguments
    
    Returns:
        Tuple of (trade_date, hudi_base_path)
    """
    if len(sys.argv) < 2:
        logger.error("Usage: spark-submit ingest_bhavcopy_daily_v2.py <YYYY-MM-DD> [hudi_base_path]")
        sys.exit(1)
    
    trade_date = sys.argv[1]
    
    # Validate date format
    try:
        datetime.strptime(trade_date, "%Y-%m-%d")
    except ValueError:
        logger.error(f"Invalid date format: {trade_date}. Expected YYYY-MM-DD")
        sys.exit(1)
    
    # Get Hudi base path from args or config
    if len(sys.argv) >= 3:
        hudi_base_path = Path(sys.argv[2])
    else:
        hudi_base_path = config.HUDI_BASE_PATH
    
    logger.info(f"Using Hudi base path: {hudi_base_path}")
    
    return trade_date, hudi_base_path


def create_spark_session(trade_date: str) -> SparkSession:
    """
    Create and configure Spark session
    
    Args:
        trade_date: Trading date for app name
        
    Returns:
        Configured SparkSession
    """
    logger.info("Creating Spark session...")
    
    spark = (
        SparkSession.builder
        .appName(f"galactus-bhavcopy-daily-{trade_date}")
        .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer")
        .config("spark.sql.adaptive.enabled", "true")
        .config("spark.sql.adaptive.coalescePartitions.enabled", "true")
        # Hudi configurations
        .config("spark.sql.catalog.spark_catalog", "org.apache.spark.sql.hudi.catalog.HoodieCatalog")
        .config("spark.sql.extensions", "org.apache.spark.sql.hudi.HoodieSparkSessionExtension")
        .getOrCreate()
    )
    
    logger.info(f"Spark session created: {spark.version}")
    logger.info(f"Spark master: {spark.sparkContext.master}")
    
    return spark


def write_to_hudi(
    spark: SparkSession,
    df,
    table_name: str,
    hudi_base_path: Path,
    trade_date: str
) -> bool:
    """
    Write DataFrame to Hudi Silver layer
    
    Args:
        spark: Active SparkSession
        df: DataFrame to write
        table_name: Name of the Hudi table
        hudi_base_path: Base path for Hudi tables
        trade_date: Trading date for logging
        
    Returns:
        True if successful, False otherwise
    """
    try:
        # Validate before write
        validate_silver_write(df, trade_date)
        
        # Get Hudi write options
        write_options = hudi_write_options(
            table_name=table_name,
            record_key="record_key",
            precombine_key="precombine_key",
            partition_key="trade_date"
        )
        
        table_path = hudi_base_path / table_name
        logger.info(f"Writing to Hudi table: {table_path}")
        
        # Write to Hudi
        df.write.format("hudi").options(**write_options).mode("append").save(str(table_path))
        
        logger.info(f"Successfully wrote data to Hudi table: {table_name}")
        
        # Log commit information
        commits = spark.read.format("hudi").load(str(table_path)).select("_hoodie_commit_time").distinct().collect()
        if commits:
            latest_commit = max(row['_hoodie_commit_time'] for row in commits)
            logger.info(f"Latest Hudi commit: {latest_commit}")
        
        return True
        
    except Exception as e:
        logger.error(f"Failed to write to Hudi: {e}", exc_info=True)
        return False


def main():
    """Main execution function"""
    # Validate inputs
    trade_date, hudi_base_path = validate_inputs()
    
    log_job_start(
        logger,
        "Daily Bhavcopy Ingestion",
        trade_date=trade_date,
        hudi_base_path=str(hudi_base_path)
    )
    
    spark = None
    success = False
    input_records = 0
    output_records = 0
    
    try:
        # Step 1: Download to Bronze layer
        logger.info("=" * 80)
        logger.info("STEP 1: Downloading to Bronze Layer")
        logger.info("=" * 80)
        
        try:
            bronze_csv_path = download_bhavcopy(
                session_date=trade_date,
                max_retries=config.NSE_RETRY_ATTEMPTS,
                retry_delay=config.NSE_RETRY_DELAY,
                timeout=config.NSE_DOWNLOAD_TIMEOUT
            )
            logger.info(f"Bronze layer file: {bronze_csv_path}")
        except NSEDownloadError as e:
            logger.error(f"Failed to download from NSE: {e}")
            sys.exit(1)
        
        # Step 2: Create Spark session
        spark = create_spark_session(trade_date)
        
        # Step 3: Transform to Silver layer
        logger.info("=" * 80)
        logger.info("STEP 2: Processing to Silver Layer")
        logger.info("=" * 80)
        
        try:
            df_silver = process_bhavcopy_to_silver(
                spark=spark,
                bronze_csv_path=bronze_csv_path,
                trade_date=trade_date
            )
            
            if df_silver is None:
                logger.error("Silver processing returned no data")
                sys.exit(1)
            
            input_records = df_silver.count()
            output_records = input_records  # After validation
            
        except SilverProcessingError as e:
            logger.error(f"Silver processing failed: {e}")
            sys.exit(1)
        
        # Step 4: Write to Hudi
        logger.info("=" * 80)
        logger.info("STEP 3: Writing to Hudi")
        logger.info("=" * 80)
        
        table_name = "sec_bhavdata"
        if not write_to_hudi(spark, df_silver, table_name, hudi_base_path, trade_date):
            logger.error("Failed to write to Hudi")
            sys.exit(1)
        
        # Log data lineage
        log_data_lineage(
            logger,
            dataset_name=table_name,
            date_range=trade_date,
            input_records=input_records,
            output_records=output_records
        )
        
        success = True
        
    except Exception as e:
        logger.error(f"Job failed with unexpected error: {e}", exc_info=True)
        sys.exit(1)
        
    finally:
        if spark:
            logger.info("Stopping Spark session")
            spark.stop()
        
        log_job_end(
            logger,
            "Daily Bhavcopy Ingestion",
            success=success,
            records_processed=output_records
        )


if __name__ == "__main__":
    main()
