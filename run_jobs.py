#!/usr/bin/env python3
"""
Sequential Job Runner for Galactus Data Ingestion

This script runs both daily and historical bhavcopy ingestion jobs sequentially.

Usage:
    python run_jobs.py [daily_date] [historical_start_date] [historical_end_date] [hudi_base_path]

Arguments:
    daily_date: Date for daily ingestion (YYYY-MM-DD) - defaults to today
    historical_start_date: Start date for historical ingestion (YYYY-MM-DD)
    historical_end_date: End date for historical ingestion (YYYY-MM-DD)
    hudi_base_path: Base path for Hudi tables - defaults to config

Example:
    python run_jobs.py 2025-12-25 2025-12-20 2025-12-24 /path/to/hudi
"""

import sys
import subprocess
import os
from datetime import datetime, timedelta
from pathlib import Path

# Add parent directory to path for configuration
sys.path.insert(0, str(Path(__file__).parent))

try:
    from conf.config import config
    USE_CONFIG = True
except ImportError:
    USE_CONFIG = False
    print("Warning: Could not import config module. Using defaults.")


def get_project_root():
    """Get project root directory"""
    if USE_CONFIG:
        return str(config.PROJECT_ROOT)
    return os.getenv('GALACTUS_PROJECT_ROOT', os.path.dirname(os.path.abspath(__file__)))


def get_hudi_path(override=None):
    """Get Hudi base path"""
    if override:
        return override
    if USE_CONFIG:
        return str(config.HUDI_BASE_PATH)
    return os.getenv('HUDI_BASE_PATH', '/tmp/hudi_data')


def get_spark_submit():
    """Get spark-submit command"""
    return os.getenv('SPARK_SUBMIT_PATH', 'spark-submit')


def run_command(cmd, description):
    """Run a shell command and return success status"""
    print(f"\n{'='*60}")
    print(f"Running: {description}")
    print(f"Command: {' '.join(cmd)}")
    print('='*60)

    try:
        project_root = get_project_root()
        env = os.environ.copy()
        env['PYTHONPATH'] = project_root
        
        result = subprocess.run(
            cmd, 
            check=True, 
            capture_output=True, 
            text=True, 
            cwd=project_root,
            env=env
        )
        print("✓ Command completed successfully")
        if result.stdout:
            print("Output:", result.stdout[-500:])  # Last 500 chars
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
    hudi_path = get_hudi_path(sys.argv[4] if len(sys.argv) > 4 else None)

    project_root = get_project_root()
    spark_submit = get_spark_submit()

    print("Galactus Data Ingestion Job Runner")
    print(f"Project Root: {project_root}")
    print(f"Daily Date: {daily_date}")
    print(f"Historical Range: {hist_start} to {hist_end}")
    print(f"Hudi Base Path: {hudi_path}")

    # Run daily ingestion (using improved version)
    daily_cmd = [
        spark_submit,
        '--packages', 'org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0',
        '--master', 'local[*]',
        f'{project_root}/jobs/ingest_bhavcopy_daily_v2.py',
        daily_date,
        hudi_path
    ]

    if not run_command(daily_cmd, f"Daily Bhavcopy Ingestion for {daily_date}"):
        print("Daily ingestion failed, but continuing with historical ingestion...")

    # Run historical ingestion
    hist_cmd = [
        spark_submit,
        '--packages', 'org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0',
        '--master', 'local[*]',
        f'{project_root}/jobs/ingest_bhavcopy_historical.py',
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