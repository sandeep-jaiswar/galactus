from datetime import datetime, timedelta
from airflow import DAG
from airflow.operators.bash import BashOperator
import os

# Get configuration from environment or use defaults
PROJECT_ROOT = os.getenv('GALACTUS_PROJECT_ROOT', '/app')
SPARK_SUBMIT = os.getenv('SPARK_SUBMIT_PATH', 'spark-submit')
HUDI_BASE_PATH = os.getenv('HUDI_BASE_PATH', f'{PROJECT_ROOT}/data/silver')

default_args = {
    'owner': 'airflow',
    'depends_on_past': False,
    'start_date': datetime(2025, 12, 23),
    'email_on_failure': False,
    'email_on_retry': False,
    'retries': 3,
    'retry_delay': timedelta(minutes=5),
}

dag = DAG(
    'bhavcopy_ingestion',
    default_args=default_args,
    description='Daily ingestion of NSE bhavcopy data into Hudi',
    schedule='@daily',  # Run daily
    catchup=False,
    tags=['galactus', 'nse', 'bhavcopy', 'daily'],
)

# Use improved version of the job
ingest_task = BashOperator(
    task_id='ingest_bhavcopy',
    bash_command=(
        f'cd {PROJECT_ROOT} && '
        f'export PYTHONPATH={PROJECT_ROOT} && '
        f'{SPARK_SUBMIT} '
        '--packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 '
        '--master local[*] '
        f'{PROJECT_ROOT}/jobs/ingest_bhavcopy_daily_v2.py '
        '{{ ds }} '  # Airflow execution date in YYYY-MM-DD format
        f'{HUDI_BASE_PATH}'
    ),
    dag=dag,
)

# Optional: Add ClickHouse sync task
sync_clickhouse = BashOperator(
    task_id='sync_to_clickhouse',
    bash_command=(
        f'cd {PROJECT_ROOT} && '
        f'export PYTHONPATH={PROJECT_ROOT} && '
        f'python {PROJECT_ROOT}/utils/hudi_clickhouse_sync.py '
        '--table sec_bhavdata '
        f'--hudi-path {HUDI_BASE_PATH}'
    ),
    dag=dag,
)

# Set task dependencies
ingest_task >> sync_clickhouse