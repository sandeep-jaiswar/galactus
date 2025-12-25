from pyspark.sql import SparkSession

spark = SparkSession.builder.appName("check-hudi-status").getOrCreate()

try:
    df = spark.read.format("hudi").load("/tmp/hudi_data/sec_bhavdata")
    total_records = df.count()
    unique_dates = df.select("trade_date").distinct().count()
    
    print(f"Total records: {total_records}")
    print(f"Unique trade dates: {unique_dates}")
    
    # Show sample dates
    print("Sample trade dates:")
    df.select("trade_date").distinct().orderBy("trade_date").show(10, truncate=False)
    
finally:
    spark.stop()
