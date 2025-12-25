import sys

if len(sys.argv) != 3:
    print("Usage: spark-submit ingest_bhavcopy_daily.py <YYYY-MM-DD> <hudi_base_path>")
    sys.exit(1)   
SESSION_DATE = sys.argv[1]
HUDI_BASE_PATH = sys.argv[2]


from pyspark.sql import SparkSession
spark = (
    SparkSession.builder
    .appName(f"bhavcopy-ingest-daily-{SESSION_DATE}")
    .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer")
    .getOrCreate()
)


from utils.nse_download import download_bhavcopy
csv_path = download_bhavcopy(SESSION_DATE)
df = (
    spark.read
    .option("header", "true")
    .option("inferSchema", "true")
    .option("ignoreLeadingWhiteSpace", "true")
    .option("ignoreTrailingWhiteSpace", "true")
    .csv(csv_path)
)

# Trim column names to remove any leading/trailing spaces
from pyspark.sql.functions import col
df = df.select([col(c).alias(c.strip()) for c in df.columns])

# Add required columns for Hudi
from pyspark.sql.functions import concat_ws
df = df.withColumn("record_key", concat_ws("-", col("SYMBOL"), col("DATE1")))
df = df.withColumn("trade_date", col("DATE1"))

from config.hudi import hudi_write_options
hudi_options = hudi_write_options(
    table_name="sec_bhavdata",
    record_key="record_key",
    precombine_key="trade_date",
    partition_key="trade_date"
)
df.write.format("hudi").options(**hudi_options).mode("append").save(
    f"{HUDI_BASE_PATH}/sec_bhavdata"
)

spark.stop()