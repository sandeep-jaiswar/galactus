"""
NSE Data Scraper - Bronze Layer

This module downloads data from NSE and stores it in the Bronze layer
EXACTLY as published by NSE - no transformations, no cleaning.

Key Principles:
- Raw data storage only
- Immutable files
- Date-partitioned
- Preserve NSE naming conventions
- Explicit error handling
"""

import logging
import time
from datetime import datetime
from pathlib import Path
from typing import Optional
import hashlib

import requests

# Setup logger
logger = logging.getLogger(__name__)


class NSEDownloadError(Exception):
    """Custom exception for NSE download failures"""
    pass


def download_bhavcopy(
    session_date: str,
    output_dir: Optional[Path] = None,
    max_retries: int = 3,
    retry_delay: int = 5,
    timeout: int = 30
) -> Path:
    """
    Download NSE bhavcopy data and store in Bronze layer
    
    This function downloads raw CSV data from NSE and stores it immutably
    in the Bronze layer with no transformations.
    
    Args:
        session_date: Trading date in YYYY-MM-DD format
        output_dir: Optional custom output directory (defaults to Bronze layer)
        max_retries: Number of retry attempts on failure
        retry_delay: Delay in seconds between retries
        timeout: HTTP request timeout in seconds
    
    Returns:
        Path to the downloaded CSV file in Bronze layer
    
    Raises:
        NSEDownloadError: If download fails after all retries
        ValueError: If date format is invalid
    """
    # Validate date format
    try:
        date_obj = datetime.strptime(session_date, "%Y-%m-%d")
    except ValueError as e:
        raise ValueError(f"Invalid date format '{session_date}'. Expected YYYY-MM-DD") from e
    
    # Format date for NSE URL
    session_date_str = date_obj.strftime("%d%m%Y")
    url = f"https://nsearchives.nseindia.com/products/content/sec_bhavdata_full_{session_date_str}.csv"
    
    # Determine output directory - Bronze layer with date partitioning
    if output_dir is None:
        try:
            from conf.config import config
            output_dir = config.get_bronze_path('bhavcopy', session_date)
        except ImportError:
            # Fallback if config not available
            output_dir = Path(f"/tmp/galactus/bronze/bhavcopy/date={session_date}")
    
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Preserve NSE filename convention in Bronze
    file_path = output_dir / f"sec_bhavdata_full_{session_date_str}.csv"
    
    # NSE requires browser-like headers
    headers = {
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept-Language": "en-US,en;q=0.5",
        "Accept-Encoding": "gzip, deflate",
        "Connection": "keep-alive",
        "Upgrade-Insecure-Requests": "1",
    }
    
    # Attempt download with retries
    last_error = None
    for attempt in range(1, max_retries + 1):
        try:
            logger.info(f"Downloading bhavcopy for {session_date} (attempt {attempt}/{max_retries})")
            logger.debug(f"URL: {url}")
            
            response = requests.get(url, headers=headers, timeout=timeout)
            
            if response.status_code == 200:
                # Write raw content to Bronze layer
                with open(file_path, 'wb') as f:
                    f.write(response.content)
                
                # Verify file was written
                if not file_path.exists() or file_path.stat().st_size == 0:
                    raise NSEDownloadError(f"Downloaded file is empty or not created: {file_path}")
                
                # Calculate and log file hash for auditability
                file_hash = _calculate_file_hash(file_path)
                logger.info(
                    f"Successfully downloaded bhavcopy for {session_date} "
                    f"to {file_path} (size={file_path.stat().st_size} bytes, sha256={file_hash})"
                )
                
                return file_path
            
            elif response.status_code == 404:
                # No data for this date - could be holiday/weekend
                raise NSEDownloadError(
                    f"No bhavcopy data available for {session_date} (HTTP 404). "
                    "This date may be a non-trading day."
                )
            else:
                raise NSEDownloadError(
                    f"Failed to download bhavcopy for {session_date}. "
                    f"HTTP status: {response.status_code}"
                )
        
        except requests.exceptions.Timeout as e:
            last_error = e
            logger.warning(f"Attempt {attempt} failed: Request timeout after {timeout}s: {e}")
        
        except requests.exceptions.RequestException as e:
            last_error = e
            logger.warning(f"Attempt {attempt} failed: Network error: {e}")
        
        except Exception as e:
            last_error = e
            logger.warning(f"Attempt {attempt} failed: {e}")
        
        # Wait before retry (except on last attempt)
        if attempt < max_retries:
            logger.info(f"Retrying in {retry_delay} seconds...")
            time.sleep(retry_delay)
    
    # All retries failed
    if last_error:
        error_msg = (
            f"Failed to download bhavcopy for {session_date} after {max_retries} attempts: {last_error}"
        )
    else:
        error_msg = f"Failed to download bhavcopy for {session_date} after {max_retries} attempts"

    logger.error(error_msg)
    raise NSEDownloadError(error_msg) from last_error


def _calculate_file_hash(file_path: Path) -> str:
    """
    Calculate SHA-256 hash of a file for auditability
    
    Args:
        file_path: Path to the file
    
    Returns:
        Hexadecimal hash string
    """
    sha256_hash = hashlib.sha256()
    with open(file_path, "rb") as f:
        # Read in chunks to handle large files
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()

