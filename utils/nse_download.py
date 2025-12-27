"""
NSE Data Scraper - Event Publisher

This module downloads data from NSE and publishes raw events to Kafka
EXACTLY as published by NSE - no transformations, no cleaning.

Key Principles:
- Raw data events only
- Immutable payloads
- Event-driven architecture
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

from utils.kafka_utils import producer

# Setup logger
logger = logging.getLogger(__name__)


class NSEDownloadError(Exception):
    """Custom exception for NSE download failures"""
    pass


def download_bhavcopy(
    session_date: str,
    max_retries: int = 1,
    retry_delay: int = 5,
    timeout: int = 30
) -> Optional[str]:
    """
    Download NSE bhavcopy data and publish to Kafka
    
    This function downloads raw CSV data from NSE, publishes it as an event
    to Kafka, and returns the content for immediate processing.
    
    Args:
        session_date: Trading date in YYYY-MM-DD format
        max_retries: Number of retry attempts on failure
        retry_delay: Delay in seconds between retries
        timeout: HTTP request timeout in seconds
    
    Returns:
        CSV content as string if successful, None if no data available (404)
    
    Raises:
        NSEDownloadError: If download fails due to network/other errors
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
                # Calculate checksum of raw content
                content = response.content
                checksum = hashlib.sha256(content).hexdigest()
                
                # Publish raw event to Kafka
                event_payload = {
                    "filename": f"sec_bhavdata_full_{session_date_str}.csv",
                    "content": content.decode('utf-8', errors='replace'),  # Decode for JSON serialization
                    "size_bytes": len(content),
                    "checksum": checksum
                }
                
                success = producer.publish_nse_event(
                    topic="nse.raw.bhavcopy.cash",
                    dataset="bhavcopy",
                    payload=event_payload,
                    event_time=date_obj
                )
                
                if success:
                    logger.info(
                        f"Successfully published bhavcopy event for {session_date} "
                        f"(size={len(content)} bytes, sha256={checksum})"
                    )
                    return content.decode('utf-8', errors='replace')
                else:
                    raise NSEDownloadError(f"Failed to publish bhavcopy event to Kafka for {session_date}")
            
            elif response.status_code == 404:
                # No data for this date - could be holiday/weekend
                logger.info(
                    f"No bhavcopy data available for {session_date} (HTTP 404). "
                    "This date may be a non-trading day. Skipping."
                )
                return None
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

