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
    description='Daily ingestion of NSE bhavcopy data into Hudi',
    schedule='@daily',  # Run daily
    catchup=False,
)

ingest_task = BashOperator(
    task_id='ingest_bhavcopy',
    bash_command='cd /media/sandeep/DataDrive/galactus && export PYTHONPATH=/media/sandeep/DataDrive/galactus && /media/sandeep/DataDrive/galactus/.venv/bin/spark-submit --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 --master local[*] /media/sandeep/DataDrive/galactus/jobs/ingest_bhavcopy_daily.py {{ ds }} /media/sandeep/DataDrive/galactus/spark-warehouse',
    dag=dag,
)