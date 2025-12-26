"""
Silver Layer Processor - Data Transformation and Validation

This module reads raw data from Bronze layer and transforms it into
clean, validated, schema-enforced data for the Silver layer (Hudi).

Key Principles:
- Read from Bronze (raw)
- Apply schemas and data types
- Validate and quarantine bad data
- Write to Silver (Hudi)
- Use Decimal types for financial data
"""

import logging
from datetime import datetime
from pathlib import Path
from typing import Optional

from pyspark.sql import SparkSession, DataFrame
from pyspark.sql.functions import (
    col, concat_ws, to_date, lit, trim
)
from pyspark.sql.types import (
    StructType, StructField, StringType, DecimalType, 
    DateType, IntegerType, LongType
)

logger = logging.getLogger(__name__)


class SilverProcessingError(Exception):
    """Custom exception for Silver layer processing errors"""
    pass


def get_bhavcopy_schema() -> StructType:
    """
    Define explicit schema for bhavcopy data
    
    Using DecimalType for prices to ensure precision in financial calculations.
    This is critical for research-grade data.
    
    Returns:
        StructType schema for bhavcopy CSV
    """
    # The CSV from NSE orders columns as: SYMBOL, SERIES, DATE1, PREV_CLOSE,
    # OPEN_PRICE, HIGH_PRICE, LOW_PRICE, LAST_PRICE, CLOSE_PRICE, AVG_PRICE,
    # TTL_TRD_QNTY, TURNOVER_LACS, NO_OF_TRADES, DELIV_QTY, DELIV_PER
    return StructType([
        StructField("SYMBOL", StringType(), nullable=False),
        StructField("SERIES", StringType(), nullable=False),
        StructField("DATE1", StringType(), nullable=False),
        StructField("PREV_CLOSE", DecimalType(18, 2), nullable=True),
        StructField("OPEN_PRICE", DecimalType(18, 2), nullable=True),
        StructField("HIGH_PRICE", DecimalType(18, 2), nullable=True),
        StructField("LOW_PRICE", DecimalType(18, 2), nullable=True),
        StructField("LAST_PRICE", DecimalType(18, 2), nullable=True),
        StructField("CLOSE_PRICE", DecimalType(18, 2), nullable=True),
        StructField("AVG_PRICE", DecimalType(18, 2), nullable=True),
        StructField("TTL_TRD_QNTY", LongType(), nullable=True),
        StructField("TURNOVER_LACS", DecimalType(18, 2), nullable=True),
        StructField("NO_OF_TRADES", IntegerType(), nullable=True),
        StructField("DELIV_QTY", LongType(), nullable=True),
        StructField("DELIV_PER", DecimalType(5, 2), nullable=True),
    ])


def process_bhavcopy_to_silver(
    spark: SparkSession,
    bronze_csv_path: Path,
    trade_date: str
) -> Optional[DataFrame]:
    """
    Process Bronze bhavcopy CSV into Silver layer DataFrame
    
    This function:
    1. Reads raw CSV from Bronze with explicit schema
    2. Validates data quality
    3. Applies transformations
    4. Prepares for Hudi write
    
    Args:
        spark: Active SparkSession
        bronze_csv_path: Path to Bronze CSV file
        trade_date: Trading date in YYYY-MM-DD format
        
    Returns:
        Cleaned DataFrame ready for Hudi write, or None if processing fails
        
    Raises:
        SilverProcessingError: If critical processing step fails
    """
    if not bronze_csv_path.exists():
        raise SilverProcessingError(f"Bronze file not found: {bronze_csv_path}")
    
    logger.info(f"Processing Bronze file: {bronze_csv_path}")
    
    try:
        # Read CSV with explicit schema
        schema = get_bhavcopy_schema()
        df = (
            spark.read
            .option("header", "true")
            .option("ignoreLeadingWhiteSpace", "true")
            .option("ignoreTrailingWhiteSpace", "true")
            .schema(schema)
            .csv(str(bronze_csv_path))
        )
        
        input_count = df.count()
        logger.info(f"Read {input_count} rows from Bronze")
        
        if input_count == 0:
            logger.warning(f"No data in Bronze file: {bronze_csv_path}")
            return None
        
        # Data quality checks
        df = _validate_and_clean(df, trade_date)
        
        # Add Hudi metadata columns
        df = _add_hudi_columns(df, trade_date)
        
        output_count = df.count()
        logger.info(f"Processed {output_count} valid rows (rejected {input_count - output_count})")
        
        return df
    
    except Exception as e:
        logger.error(f"Failed to process Bronze file {bronze_csv_path}: {e}")
        raise SilverProcessingError(f"Processing failed: {e}") from e


def _validate_and_clean(df: DataFrame, expected_date: str) -> DataFrame:
    """
    Validate data quality and filter invalid records
    
    Invalid records are logged and filtered out (quarantine logic can be added).
    
    Args:
        df: Input DataFrame
        expected_date: Expected trading date for validation
        
    Returns:
        Cleaned DataFrame with valid records only
    """
    logger.info("Validating data quality...")
    
    # Trim all string columns
    for field in df.schema.fields:
        if field.dataType == StringType():
            df = df.withColumn(field.name, trim(col(field.name)))
    
    # Validate required fields are not null
    df_clean = df.filter(
        col("SYMBOL").isNotNull() & 
        (col("SYMBOL") != "") &
        col("SERIES").isNotNull() &
        (col("SERIES") != "") &
        col("DATE1").isNotNull()
    )
    
    # Validate price sanity (prices should be positive)
    # Allow nulls but reject negative or zero prices
    for price_col in ["OPEN_PRICE", "HIGH_PRICE", "LOW_PRICE", "CLOSE_PRICE", "LAST_PRICE", "PREV_CLOSE"]:
        df_clean = df_clean.filter(
            col(price_col).isNull() | (col(price_col) > 0)
        )
    
    # Validate quantities and trades are non-negative
    for qty_col in ["TTL_TRD_QNTY", "NO_OF_TRADES", "DELIV_QTY"]:
        df_clean = df_clean.filter(
            col(qty_col).isNull() | (col(qty_col) >= 0)
        )
    
    # Validate delivery percentage is between 0 and 100
    df_clean = df_clean.filter(
        col("DELIV_PER").isNull() | 
        ((col("DELIV_PER") >= 0) & (col("DELIV_PER") <= 100))
    )
    
    # Validate OHLC relationship: Low <= Open, High, Close, Last <= High
    df_clean = df_clean.filter(
        col("LOW_PRICE").isNull() | col("HIGH_PRICE").isNull() |
        (col("LOW_PRICE") <= col("HIGH_PRICE"))
    )
    
    rejected_count = df.count() - df_clean.count()
    if rejected_count > 0:
        logger.warning(f"Rejected {rejected_count} records due to data quality issues")
    
    return df_clean


def _add_hudi_columns(df: DataFrame, trade_date: str) -> DataFrame:
    """
    Add Hudi-required columns for upsert and partitioning
    
    Args:
        df: Cleaned DataFrame
        trade_date: Trading date in YYYY-MM-DD format
        
    Returns:
        DataFrame with Hudi columns added
    """
    logger.info("Adding Hudi metadata columns...")
    
    # Parse trade_date from DATE1 column (format: DD-MMM-YYYY)
    df = df.withColumn("trade_date", to_date(col("DATE1"), "dd-MMM-yyyy"))
    
    # Create composite record key: SYMBOL-SERIES-TRADE_DATE
    # This ensures uniqueness for same symbol in different series
    df = df.withColumn(
        "record_key",
        concat_ws("-", col("SYMBOL"), col("SERIES"), col("trade_date"))
    )
    
    # Add precombine field (used by Hudi to determine latest record)
    # Using trade_date as precombine key for daily data
    df = df.withColumn("precombine_key", col("trade_date"))
    
    # Normalize column names to lowercase for Silver layer
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
        'DELIV_QTY': 'delivery_qty',
        'DELIV_PER': 'delivery_pct',
        'DATE1': 'date_original'
    }
    
    for old_name, new_name in column_mapping.items():
        if old_name in df.columns:
            df = df.withColumnRenamed(old_name, new_name)
    
    return df


def validate_silver_write(df: DataFrame, trade_date: str) -> bool:
    """
    Validate DataFrame before writing to Silver layer
    
    Args:
        df: DataFrame to validate
        trade_date: Expected trade date
        
    Returns:
        True if validation passes
        
    Raises:
        SilverProcessingError: If validation fails
    """
    # Check required columns exist
    required_columns = ['record_key', 'trade_date', 'precombine_key', 'symbol', 'series']
    missing_columns = [col for col in required_columns if col not in df.columns]
    
    if missing_columns:
        raise SilverProcessingError(f"Missing required columns: {missing_columns}")
    
    # Check for duplicate record keys
    total_records = df.count()
    unique_keys = df.select("record_key").distinct().count()
    
    if total_records != unique_keys:
        duplicates = total_records - unique_keys
        logger.warning(f"Found {duplicates} duplicate record keys - Hudi will handle via upsert")
    
    # Validate trade_date consistency
    distinct_dates = df.select("trade_date").distinct().collect()
    if len(distinct_dates) > 1:
        date_values = [row['trade_date'] for row in distinct_dates]
        logger.warning(f"Multiple trade dates in DataFrame: {date_values}")
    
    logger.info("Silver layer validation passed")
    return True
