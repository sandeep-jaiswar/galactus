#!/bin/bash
# Airflow startup script

export AIRFLOW_HOME=/media/sandeep/DataDrive/datastore/airflow
export PATH=/media/sandeep/DataDrive/datastore/.venv/bin:$PATH

# Start Airflow standalone
/media/sandeep/DataDrive/datastore/.venv/bin/airflow standalone