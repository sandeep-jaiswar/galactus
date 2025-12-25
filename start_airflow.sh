#!/bin/bash
# Airflow startup script

export AIRFLOW_HOME=/media/sandeep/DataDrive/galactus/airflow
export PATH=/media/sandeep/DataDrive/galactus/.venv/bin:$PATH

# Start Airflow standalone
/media/sandeep/DataDrive/galactus/.venv/bin/airflow standalone