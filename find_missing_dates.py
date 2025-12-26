#!/usr/bin/env python3
"""
Find Missing Trading Dates

This script compares trading dates in the Hudi table against expected trading days
and reports any missing dates.
"""

import os
import sys
from datetime import datetime, timedelta
from pathlib import Path

import pandas as pd
from pyspark.sql import SparkSession

# Add parent to path
sys.path.insert(0, str(Path(__file__).parent))

# Try to import config
try:
    from conf.config import config
    default_table_path = str(config.get_silver_path("sec_bhavdata"))
except ImportError:
    default_table_path = "/tmp/hudi_data/sec_bhavdata"

# Get table path from environment or use default
table_path = os.environ.get("HUDI_TABLE_PATH", default_table_path)

print(f"Analyzing Hudi table: {table_path}")

# Create Spark session
spark = SparkSession.builder \
    .appName("Find Missing Dates") \
    .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer") \
    .config("spark.sql.extensions", "org.apache.spark.sql.hudi.HoodieSparkSessionExtension") \
    .getOrCreate()

try:
    # Read the Hudi table
    hudi_df = spark.read.format("hudi").load(table_path)
    existing_dates = hudi_df.select("trade_date").distinct().toPandas()

    # Convert to datetime and sort
    existing_dates['trade_date'] = pd.to_datetime(existing_dates['trade_date'], format='%Y-%m-%d')
    existing_dates = existing_dates.sort_values('trade_date')

    print(f"\n{'='*60}")
    print("Existing Data Summary")
    print('='*60)
    print(f"Existing dates count: {len(existing_dates)}")
    print("\nFirst few existing dates:")
    print(existing_dates.head())
    print("\nLast few existing dates:")
    print(existing_dates.tail())

    # Generate all trading days for 2025 (assuming weekdays)
    start_date = datetime(2025, 1, 1)
    end_date = datetime(2025, 12, 31)
    all_dates = []
    current_date = start_date

    while current_date <= end_date:
        # Skip weekends (Saturday=5, Sunday=6)
        if current_date.weekday() < 5:  # Monday to Friday
            all_dates.append(current_date)
        current_date += timedelta(days=1)

    print(f"\nTotal expected trading days in 2025 (weekdays): {len(all_dates)}")

    # Find missing dates
    existing_date_set = set(existing_dates['trade_date'])
    missing_dates = []

    for date in all_dates:
        if date not in existing_date_set:
            missing_dates.append(date)

    print(f"\n{'='*60}")
    print("Missing Dates Analysis")
    print('='*60)
    print(f"Missing dates count: {len(missing_dates)}")
    
    if missing_dates:
        print("\nMissing dates:")
        for i, date in enumerate(missing_dates[:20]):  # Show first 20
            print(f"  {date.strftime('%Y-%m-%d')}")
        if len(missing_dates) > 20:
            print(f"  ... and {len(missing_dates) - 20} more")
    else:
        print("✓ No missing dates found!")
    
    print('='*60)

finally:
    spark.stop()
