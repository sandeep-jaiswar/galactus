"""
Centralized Logging Configuration for Galactus

Provides structured logging with context, audit trails, and proper levels.
"""

import logging
import sys
from pathlib import Path
from typing import Optional


def setup_logger(
    name: str,
    log_level: Optional[str] = None,
    log_file: Optional[Path] = None,
    console: bool = True
) -> logging.Logger:
    """
    Setup a logger with consistent formatting and handlers
    
    Args:
        name: Logger name (typically __name__ of the module)
        log_level: Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
        log_file: Optional path to log file
        console: Whether to log to console (default: True)
    
    Returns:
        Configured logger instance
    """
    # Create logger
    logger = logging.getLogger(name)
    
    # Set level
    if log_level is None:
        try:
            from conf.config import config
            log_level = config.LOG_LEVEL
        except ImportError:
            log_level = 'INFO'
    
    logger.setLevel(getattr(logging, log_level.upper()))
    
    # Clear existing handlers to avoid duplicates
    logger.handlers.clear()
    
    # Create formatter with context
    formatter = logging.Formatter(
        fmt='%(asctime)s - %(name)s - %(levelname)s - [%(filename)s:%(lineno)d] - %(message)s',
        datefmt='%Y-%m-%d %H:%M:%S'
    )
    
    # Console handler
    if console:
        console_handler = logging.StreamHandler(sys.stdout)
        console_handler.setFormatter(formatter)
        logger.addHandler(console_handler)
    
    # File handler
    if log_file:
        log_file.parent.mkdir(parents=True, exist_ok=True)
        file_handler = logging.FileHandler(log_file)
        file_handler.setFormatter(formatter)
        logger.addHandler(file_handler)
    
    return logger


def log_job_start(logger: logging.Logger, job_name: str, **kwargs):
    """
    Log the start of a job with context
    
    Args:
        logger: Logger instance
        job_name: Name of the job
        **kwargs: Additional context to log
    """
    logger.info("=" * 80)
    logger.info(f"Starting job: {job_name}")
    for key, value in kwargs.items():
        logger.info(f"  {key}: {value}")
    logger.info("=" * 80)


def log_job_end(logger: logging.Logger, job_name: str, success: bool, **kwargs):
    """
    Log the end of a job with results
    
    Args:
        logger: Logger instance
        job_name: Name of the job
        success: Whether job succeeded
        **kwargs: Additional results to log
    """
    logger.info("=" * 80)
    status = "COMPLETED SUCCESSFULLY" if success else "FAILED"
    logger.info(f"Job {job_name}: {status}")
    for key, value in kwargs.items():
        logger.info(f"  {key}: {value}")
    logger.info("=" * 80)


def log_data_lineage(
    logger: logging.Logger,
    dataset_name: str,
    date_range: str,
    input_records: int,
    output_records: int,
    commit_id: Optional[str] = None
):
    """
    Log data lineage information for auditability
    
    Args:
        logger: Logger instance
        dataset_name: Name of the dataset
        date_range: Date or date range processed
        input_records: Number of input records
        output_records: Number of output records
        commit_id: Optional Hudi commit ID
    """
    logger.info(f"Data Lineage - Dataset: {dataset_name}")
    logger.info(f"  Date Range: {date_range}")
    logger.info(f"  Input Records: {input_records}")
    logger.info(f"  Output Records: {output_records}")
    logger.info(f"  Rejected Records: {input_records - output_records}")
    if commit_id:
        logger.info(f"  Hudi Commit ID: {commit_id}")
