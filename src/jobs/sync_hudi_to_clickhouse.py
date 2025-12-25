import sys
import os
from datetime import datetime

if len(sys.argv) != 3:
    print("Usage: spark-submit sync_hudi_to_clickhouse.py <session_date> <hudi_base_path>")
    sys.exit(1)

SESSION_DATE = sys.argv[1]
HUDI_BASE_PATH = sys.argv[2]

# Convert YYYY-MM-DD to DD-MMM-YYYY format for filtering
def convert_date_format(date_str):
    try:
        dt = datetime.strptime(date_str, "%Y-%m-%d")
        return dt.strftime("%d-%b-%Y")
    except ValueError:
        # If already in DD-MMM-YYYY format, return as is
        return date_str

FILTER_DATE = convert_date_format(SESSION_DATE)

from pyspark.sql import SparkSession
from pyspark.sql.functions import col

# Initialize Spark session
spark = (
    SparkSession.builder
    .appName(f"hudi-to-clickhouse-sync-{SESSION_DATE}")
    .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer")
    .getOrCreate()
)

try:
    # Read from Hudi table (stored as Parquet)
    hudi_df = spark.read.parquet(f"{HUDI_BASE_PATH}/sec_bhavdata")

    # Print schema for debugging
    print("Parquet schema:")
    hudi_df.printSchema()
    print(f"Total records: {hudi_df.count()}")

    # Select only business columns and rename to lowercase to match ClickHouse schema
    # Exclude Hudi metadata columns (_hoodie_*) and map column names and types
    from pyspark.sql.functions import col, lower
    
    # Map Parquet columns to ClickHouse schema
    column_mapping = {
        "SYMBOL": "symbol",
        "SERIES": "series", 
        "DATE1": "date1",
        "PREV_CLOSE": "prev_close",
        "OPEN_PRICE": "open_price",
        "HIGH_PRICE": "high_price",
        "LOW_PRICE": "low_price",
        "LAST_PRICE": "last_price",
        "CLOSE_PRICE": "close_price",
        "AVG_PRICE": "avg_price",
        "TTL_TRD_QNTY": "ttl_trd_qnty",  # Will be cast to long
        "TURNOVER_LACS": "turnover_lacs",
        "NO_OF_TRADES": "no_of_trds",  # Will be cast to long
        "DELIV_QTY": "deliv_qty",  # Will be cast to long
        "DELIV_PER": "deliv_per",
        "record_key": "record_key",
        "trade_date": "trade_date"
    }
    
    # Select and rename columns, applying type conversions where needed
    selected_cols = []
    for parquet_col, ch_col in column_mapping.items():
        if parquet_col in ["TTL_TRD_QNTY", "NO_OF_TRADES", "DELIV_QTY"]:
            # Convert string/integer to long for ClickHouse UInt64
            selected_cols.append(col(parquet_col).cast("long").alias(ch_col))
        elif parquet_col == "DELIV_QTY":
            # DELIV_QTY is string in Parquet, needs to be converted to long
            selected_cols.append(col(parquet_col).cast("long").alias(ch_col))
        else:
            selected_cols.append(col(parquet_col).alias(ch_col))
    
    # Apply column selection and renaming
    hudi_df = hudi_df.select(*selected_cols)

    # Filter for the specific date if needed (optional)
    if SESSION_DATE:
        hudi_df = hudi_df.filter(col("trade_date") == FILTER_DATE)

    print(f"Transformed schema:")
    hudi_df.printSchema()
    print(f"Records to sync: {hudi_df.count()}")

    # Write to ClickHouse
    clickhouse_url = "jdbc:clickhouse://clickhouse:8123/nse"

    properties = {
        "user": "default",
        "password": "clickhouse",
        "driver": "com.clickhouse.jdbc.ClickHouseDriver"
    }

    # Write data to ClickHouse
    hudi_df.write \
        .mode("append") \
        .format("jdbc") \
        .option("url", clickhouse_url) \
        .option("dbtable", "sec_bhavdata") \
        .option("user", "default") \
        .option("password", "clickhouse") \
        .option("driver", "com.clickhouse.jdbc.ClickHouseDriver") \
        .option("batchsize", "10000") \
        .save()

    print(f"Successfully synced {hudi_df.count()} records to ClickHouse for date {SESSION_DATE}")

except Exception as e:
    print(f"Error syncing data to ClickHouse: {e}")
    raise

finally:
    spark.stop()