from datetime import datetime, timedelta
from airflow import DAG
from airflow.operators.bash import BashOperator

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
    description='Daily ingestion of NSE bhavcopy data into Hudi and ClickHouse',
    schedule='@daily',  # Run daily
    catchup=False,
)

ingest_task = BashOperator(
    task_id='ingest_bhavcopy',
    bash_command='docker exec datastore-spark-1 bash -c "export PYTHONPATH=/opt/spark/src:/opt/spark/config && spark-submit --packages org.apache.hudi:hudi-spark3.5-bundle_2.12:0.15.0 --master spark://spark:7077 /opt/spark/src/jobs/ingest_bhavcopy_daily.py {{ ds }} /opt/spark/data/spark-warehouse"',
    dag=dag,
)

sync_to_clickhouse = BashOperator(
    task_id='sync_to_clickhouse',
    bash_command='docker exec datastore-spark-1 bash -c "export PYTHONPATH=/opt/spark/src:/opt/spark/config && spark-submit --packages org.apache.hudi:hudi-spark3.5-bundle_2.12:0.15.0 --master spark://spark:7077 /opt/spark/src/jobs/sync_hudi_to_clickhouse.py {{ ds }} /opt/spark/data/spark-warehouse"',
    dag=dag,
)

# Set up task dependencies
ingest_task >> sync_to_clickhouse