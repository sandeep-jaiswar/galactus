# API Documentation

## NSE Data Download API

### `download_bhavcopy(session_date: str) -> str`

Downloads NSE bhavcopy data for a specific date.

**Parameters:**
- `session_date`: Date in YYYY-MM-DD format

**Returns:**
- Local file path to downloaded CSV

**Raises:**
- `Exception`: If download fails

**Example:**
```python
from utils.nse_download import download_bhavcopy

csv_path = download_bhavcopy("2025-12-25")
print(f"Downloaded to: {csv_path}")
```

## Hudi Configuration API

### `hudi_write_options(table_name, record_key, precombine_key, partition_key) -> dict`

Generates Hudi write configuration for table operations.

**Parameters:**
- `table_name`: Name of the Hudi table
- `record_key`: Column name for record key
- `precombine_key`: Column name for precombine key
- `partition_key`: Column name for partition key

**Returns:**
- Dictionary with Hudi configuration options

**Example:**
```python
from config.hudi import hudi_write_options

options = hudi_write_options(
    table_name="sec_bhavdata",
    record_key="record_key",
    precombine_key="trade_date",
    partition_key="trade_date"
)

df.write.format("hudi").options(**options).mode("append").save("/path/to/table")
```

## Data Processing Functions

### Record Key Generation
```python
# In Spark job
df = df.withColumn("record_key", concat_ws("-", col("SYMBOL"), col("DATE1")))
```

### Trade Date Extraction
```python
# In Spark job
df = df.withColumn("trade_date", col("DATE1"))
```

### Column Name Cleaning
```python
# Remove leading/trailing spaces from column names
df = df.select([col(c).alias(c.strip()) for c in df.columns])
```

## Airflow DAG Configuration

### DAG Parameters
```python
default_args = {
    'owner': 'airflow',
    'depends_on_past': False,
    'start_date': datetime(2025, 12, 23),
    'email_on_failure': False,
    'email_on_retry': False,
    'retries': 1,
    'retry_delay': timedelta(minutes=5),
}

dag = DAG(
    'bhavcopy_ingestion',
    default_args=default_args,
    description='Daily ingestion of NSE bhavcopy data into Hudi',
    schedule='@daily',
    catchup=False,
)
```

### Task Definition
```python
ingest_task = BashOperator(
    task_id='ingest_bhavcopy',
    bash_command='cd /opt/airflow && export PYTHONPATH=/opt/airflow/src:/opt/airflow/config && spark-submit --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 --master spark://spark-master-service:7077 /opt/airflow/src/jobs/ingest_bhavcopy_daily.py {{ ds }} /opt/airflow/data/spark-warehouse',
    dag=dag,
)
```

## Spark Job Interface

### Command Line Arguments
```bash
spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  --master spark://spark-master:7077 \
  /path/to/job.py \
  <session_date> \
  <hudi_base_path>
```

### Job Parameters
- `session_date`: Date in YYYY-MM-DD format
- `hudi_base_path`: Base path for Hudi tables

## Data Schema Definitions

### Input CSV Schema
```python
schema = StructType([
    StructField("SYMBOL", StringType(), True),
    StructField("SERIES", StringType(), True),
    StructField("DATE1", StringType(), True),
    StructField("PREV_CLOSE", DoubleType(), True),
    StructField("OPEN_PRICE", DoubleType(), True),
    StructField("HIGH_PRICE", DoubleType(), True),
    StructField("LOW_PRICE", DoubleType(), True),
    StructField("LAST_PRICE", DoubleType(), True),
    StructField("CLOSE_PRICE", DoubleType(), True),
    StructField("AVG_PRICE", DoubleType(), True),
    StructField("TTL_TRD_QNTY", LongType(), True),
    StructField("TURNOVER_LACS", DoubleType(), True),
    StructField("NO_OF_TRDS", LongType(), True),
    StructField("DELIV_QTY", LongType(), True),
    StructField("DELIV_PER", DoubleType(), True),
])
```

### Output Hudi Schema
```python
# Same as input plus:
StructField("record_key", StringType(), False),  # SYMBOL-DATE1
StructField("trade_date", StringType(), False),  # DATE1
```

## Error Handling

### Network Errors
```python
try:
    response = requests.get(url, headers=headers, timeout=30)
    response.raise_for_status()
except requests.exceptions.RequestException as e:
    raise Exception(f"Failed to download bhavcopy: {e}")
```

### Spark Job Failures
```python
try:
    # Spark operations
    df.write.format("hudi").options(**options).mode("append").save(path)
except Exception as e:
    logger.error(f"Hudi write failed: {e}")
    raise
```

### Data Validation
```python
if df.count() == 0:
    raise ValueError("No data found in CSV")

if df.filter(col("SYMBOL").isNull()).count() > 0:
    raise ValueError("Missing SYMBOL values")
```

## Configuration Files

### airflow.cfg
```ini
[core]
dags_folder = /opt/airflow/dags
load_examples = False
executor = LocalExecutor

[database]
sql_alchemy_conn = postgresql+psycopg2://airflow:airflow@postgres/airflow
```

### docker-compose.yml
```yaml
version: '3.8'
services:
  postgres:
    image: postgres:13
    environment:
      POSTGRES_USER: airflow
      POSTGRES_PASSWORD: airflow
      POSTGRES_DB: airflow
    ports:
      - "5433:5432"
```

## Environment Variables

### Airflow
- `AIRFLOW__CORE__EXECUTOR`: Executor type
- `AIRFLOW__DATABASE__SQL_ALCHEMY_CONN`: Database connection
- `AIRFLOW__CORE__FERNET_KEY`: Encryption key

### Spark
- `SPARK_MODE`: master/worker
- `SPARK_MASTER_URL`: Master URL for workers

### Application
- `PYTHONPATH`: Python module search path
- `HUDI_BASE_PATH`: Base path for Hudi tables