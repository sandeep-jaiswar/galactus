# Using Airflow in Your System

Apache Airflow is now integrated into your data processing pipeline.

## Accessing Airflow
- **Web UI:** http://localhost:8080
- **Username:** admin
- **Password:** DuM7v7utTSdmn3Ye

## Your DAGs
- **bhavcopy_ingestion:** Daily ingestion of NSE bhavcopy data into Hudi.
  - Runs the existing `ingest_bhavcopy.py` script via Spark.
  - Scheduled to run daily at midnight.
  - Uses local Spark for processing.

## Managing DAGs
- In the web UI, go to DAGs page.
- Click on `bhavcopy_ingestion` to view details.
- Use the toggle to pause/unpause.
- Click "Trigger DAG" to run manually.
- View task logs under Graph > Task Instance > Logs.

## Adding New DAGs
- Place Python files defining DAGs in `/media/sandeep/DataDrive/datastore/airflow/dags/`
- Airflow will automatically detect and load them.
- Example: Create a DAG for other jobs like data processing or reporting.

## CLI Commands
- List DAGs: `airflow dags list`
- Trigger DAG: `airflow dags trigger <dag_id>`
- Check status: `airflow dags state <dag_id> <execution_date>`

## Integration Benefits
- Schedule and monitor your existing jobs.
- Handle dependencies between tasks.
- Retry failed jobs automatically.
- Visualize workflows and data lineage.