import sys

if len(sys.argv) != 4:
    print("Usage: spark-submit ingest_bhavcopy.py <start_date YYYY-MM-DD> <end_date YYYY-MM-DD> <hudi_base_path>")
    sys.exit(1)   
START_DATE = sys.argv[1]
END_DATE = sys.argv[2]
HUDI_BASE_PATH = sys.argv[3]


from pyspark.sql import SparkSession
from datetime import datetime, timedelta

# Parse dates
start_date = datetime.strptime(START_DATE, "%Y-%m-%d")
end_date = datetime.strptime(END_DATE, "%Y-%m-%d")

spark = (
    SparkSession.builder
    .appName(f"bhavcopy-ingest-{START_DATE}-{END_DATE}")
    .config("spark.serializer", "org.apache.spark.serializer.KryoSerializer")
    .getOrCreate()
)

from utils.nse_download import download_bhavcopy
from pyspark.sql.functions import col, concat_ws
from conf.hudi import hudi_write_options

# Loop through each date from start to end inclusive
current_date = start_date
while current_date <= end_date:
    session_date = current_date.strftime("%Y-%m-%d")
    print(f"Processing date: {session_date}")
    
    try:
        csv_path = download_bhavcopy(session_date)
        df = (
            spark.read
            .option("header", "true")
            .option("inferSchema", "true")
            .option("ignoreLeadingWhiteSpace", "true")
            .option("ignoreTrailingWhiteSpace", "true")
            .csv(csv_path)
        )
        
        # Trim column names to remove any leading/trailing spaces
        df = df.select([col(c).alias(c.strip()) for c in df.columns])
        
        # Add required columns for Hudi
        df = df.withColumn("record_key", concat_ws("-", col("SYMBOL"), col("DATE1")))
        df = df.withColumn("trade_date", col("DATE1"))
        
        hudi_options = hudi_write_options(
            table_name="sec_bhavdata",
            record_key="record_key",
            precombine_key="trade_date",
            partition_key="trade_date"
        )
        df.write.format("hudi").options(**hudi_options).mode("append").save(
            f"{HUDI_BASE_PATH}/sec_bhavdata"
        )
        print(f"Successfully ingested data for {session_date}")
    except Exception as e:
        print(f"Error processing {session_date}: {e}")
    
    current_date += timedelta(days=1)

spark.stop()


