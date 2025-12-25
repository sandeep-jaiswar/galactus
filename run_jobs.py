#!/usr/bin/env python3
"""
Sequential Job Runner for Galactus Data Ingestion

This script runs both daily and historical bhavcopy ingestion jobs sequentially.
Make sure to activate the spark310 conda environment before running this script.

Usage:
    conda activate spark310
    python run_jobs.py [daily_date] [historical_start_date] [historical_end_date] [hudi_base_path]

Arguments:
    daily_date: Date for daily ingestion (YYYY-MM-DD) - defaults to today
    historical_start_date: Start date for historical ingestion (YYYY-MM-DD)
    historical_end_date: End date for historical ingestion (YYYY-MM-DD)
    hudi_base_path: Base path for Hudi tables - defaults to /tmp/hudi_data

Example:
    python run_jobs.py 2025-12-25 2025-12-20 2025-12-24 /path/to/hudi
"""

import sys
import subprocess
import os
from datetime import datetime, timedelta

def run_command(cmd, description):
    """Run a shell command and return success status"""
    print(f"\n{'='*60}")
    print(f"Running: {description}")
    print(f"Command: {' '.join(cmd)}")
    print('='*60)

    try:
        result = subprocess.run(cmd, check=True, capture_output=True, text=True, cwd="/media/sandeep/DataDrive/galactus")
        print("✓ Command completed successfully")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Command failed with exit code {e.returncode}")
        # Print last few lines of stderr for debugging
        stderr_lines = e.stderr.strip().split('\n')
        for line in stderr_lines[-10:]:  # Show last 10 lines
            if line.strip():
                print(f"  {line}")
        return False

def main():
    # Set default arguments
    today = datetime.now().strftime("%Y-%m-%d")
    yesterday = (datetime.now() - timedelta(days=1)).strftime("%Y-%m-%d")
    week_ago = (datetime.now() - timedelta(days=7)).strftime("%Y-%m-%d")

    # Parse command line arguments
    daily_date = sys.argv[1] if len(sys.argv) > 1 else today
    hist_start = sys.argv[2] if len(sys.argv) > 2 else week_ago
    hist_end = sys.argv[3] if len(sys.argv) > 3 else yesterday
    hudi_path = sys.argv[4] if len(sys.argv) > 4 else "/tmp/hudi_data"

    print("Galactus Data Ingestion Job Runner")
    print(f"Daily Date: {daily_date}")
    print(f"Historical Range: {hist_start} to {hist_end}")
    print(f"Hudi Base Path: {hudi_path}")

    # Set PYTHONPATH for the scripts
    env = os.environ.copy()
    env['PYTHONPATH'] = '/media/sandeep/DataDrive/galactus'

    # Run daily ingestion
    daily_cmd = [
        '/opt/hudi/spark-submit-hudi.sh',
        'jobs/ingest_bhavcopy_daily.py',
        daily_date,
        hudi_path
    ]

    if not run_command(daily_cmd, f"Daily Bhavcopy Ingestion for {daily_date}"):
        print("Daily ingestion failed, but continuing with historical ingestion...")

    # Run historical ingestion
    hist_cmd = [
        '/opt/hudi/spark-submit-hudi.sh',
        'jobs/ingest_bhavcopy_historical.py',
        hist_start,
        hist_end,
        hudi_path
    ]

    success = run_command(hist_cmd, f"Historical Bhavcopy Ingestion from {hist_start} to {hist_end}")

    print(f"\n{'='*60}")
    if success:
        print("✓ All jobs completed successfully!")
    else:
        print("✗ Some jobs failed. Check the output above for details.")
    print('='*60)

if __name__ == "__main__":
    main()