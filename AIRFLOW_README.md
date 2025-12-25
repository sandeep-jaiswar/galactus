# Apache Airflow Setup

Apache Airflow has been set up locally on your system.

## Access
- Web UI: http://localhost:8080
- Username: admin
- Password: DuM7v7utTSdmn3Ye (stored in airflow/simple_auth_manager_passwords.json.generated)

## Installation Details
- Virtual Environment: /media/sandeep/DataDrive/galactus/.venv
- Airflow Home: /media/sandeep/DataDrive/galactus/airflow
- Database: SQLite (airflow.db)
- DAGs Folder: /media/sandeep/DataDrive/galactus/airflow/dags

## Running Airflow
- Manual Start: ./start_airflow.sh
- System Service: sudo systemctl start airflow
- Service Status: sudo systemctl status airflow
- Service Stop: sudo systemctl stop airflow

## Adding DAGs
Place your DAG Python files in the `airflow/dags` directory. They will be automatically picked up by the scheduler.

## Notes
- Airflow is running in standalone mode for development.
- The service is enabled to start on boot.