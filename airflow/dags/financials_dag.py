"""Airflow DAG to run the financial reports ingestion job (scaffold).

This mirrors the `bhavcopy_dag.py` scheduling pattern and uses a BashOperator
to call the spark-submit wrapper with the new `jobs/ingest_financial_reports.py`.
"""
from airflow import DAG
from airflow.operators.bash import BashOperator
from datetime import datetime, timedelta

default_args = {
    "owner": "galactus",
    "depends_on_past": False,
    "start_date": datetime(2025, 1, 1),
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
}

with DAG("financials_ingest", schedule_interval="@daily", default_args=default_args, catchup=False) as dag:
    run_ingest = BashOperator(
        task_id="run_ingest_financial_reports",
        bash_command="cd /media/sandeep/DataDrive/galactus && \"/opt/hudi/spark-submit-hudi.sh\" jobs/ingest_financial_reports.py --date {{ ds }} --hudi-path /tmp/hudi_data/company_financial_reports",
    )

    run_ingest
