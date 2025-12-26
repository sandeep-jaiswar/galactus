#!/usr/bin/env python3
"""
Check Hudi Table Status

This script provides basic statistics about a Hudi table.
"""

import os
import sys
from pathlib import Path

# Add parent to path
sys.path.insert(0, str(Path(__file__).parent))

from pyspark.sql import SparkSession

# Try to import config
try:
    from conf.config import config
    default_table_path = str(config.get_silver_path("sec_bhavdata"))
except ImportError:
    default_table_path = "/tmp/hudi_data/sec_bhavdata"

# Get table path from environment or use default
table_path = os.environ.get("HUDI_TABLE_PATH", default_table_path)

print(f"Checking Hudi table: {table_path}")

spark = SparkSession.builder \
    .appName("check-hudi-status") \
    .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer") \
    .getOrCreate()

try:
    df = spark.read.format("hudi").load(table_path)
    total_records = df.count()
    unique_dates = df.select("trade_date").distinct().count()
    
    print(f"\n{'='*60}")
    print("Hudi Table Statistics")
    print('='*60)
    print(f"Total records: {total_records:,}")
    print(f"Unique trade dates: {unique_dates}")
    print('='*60)
    
    # Show sample dates
    print("\nSample trade dates (most recent):")
    df.select("trade_date").distinct().orderBy("trade_date", ascending=False).show(10, truncate=False)
    
    # Show schema
    print("\nTable Schema:")
    df.printSchema()
    
finally:
    spark.stop()
