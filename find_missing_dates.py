from datetime import datetime, timedelta
import pandas as pd
import os

# Read the existing trade dates from the Hudi table
from pyspark.sql import SparkSession

spark = SparkSession.builder \
    .appName("Find Missing Dates") \
    .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer") \
    .config("spark.sql.extensions", "org.apache.spark.sql.hudi.HoodieSparkSessionExtension") \
    .getOrCreate()

# Read the Hudi table from configurable path
table_path = os.environ.get("HUDI_TABLE_PATH", "/tmp/hudi_data")
hudi_df = spark.read.format("hudi").load(table_path)
existing_dates = hudi_df.select("trade_date").distinct().toPandas()

# Convert to datetime and sort - use consistent format with hudi_clickhouse_sync.py
existing_dates['trade_date'] = pd.to_datetime(existing_dates['trade_date'], format='%Y-%m-%d')
existing_dates = existing_dates.sort_values('trade_date')

print(f"Existing dates count: {len(existing_dates)}")
print("First few existing dates:")
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

print(f"\nTotal trading days in 2025: {len(all_dates)}")

# Find missing dates
existing_date_set = set(existing_dates['trade_date'])
missing_dates = []

for date in all_dates:
    if date not in existing_date_set:
        missing_dates.append(date)

print(f"\nMissing dates count: {len(missing_dates)}")
if missing_dates:
    print("Missing dates:")
    for date in missing_dates[:20]:  # Show first 20
        print(date.strftime('%Y-%m-%d'))
    if len(missing_dates) > 20:
        print(f"... and {len(missing_dates) - 20} more")
else:
    print("No missing dates found!")

spark.stop()
