"""
Unit tests for logging utilities
"""

import logging
from pathlib import Path
import tempfile
import pytest


def test_setup_logger():
    """Test logger setup"""
    from utils.logging_utils import setup_logger
    
    logger = setup_logger("test_logger", log_level="INFO")
    
    assert logger is not None
    assert isinstance(logger, logging.Logger)
    assert logger.name == "test_logger"
    assert logger.level == logging.INFO


def test_setup_logger_with_file():
    """Test logger setup with file handler"""
    from utils.logging_utils import setup_logger
    
    with tempfile.TemporaryDirectory() as tmpdir:
        log_file = Path(tmpdir) / "test.log"
        logger = setup_logger("test_logger_file", log_file=log_file)
        
        # Log a message
        logger.info("Test message")
        
        # Check that file was created and contains message
        assert log_file.exists()
        content = log_file.read_text()
        assert "Test message" in content


def test_setup_logger_levels():
    """Test different log levels"""
    from utils.logging_utils import setup_logger
    
    for level in ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]:
        logger = setup_logger(f"test_{level}", log_level=level)
        assert logger.level == getattr(logging, level)


def test_log_job_start():
    """Test job start logging"""
    from utils.logging_utils import setup_logger, log_job_start
    
    with tempfile.TemporaryDirectory() as tmpdir:
        log_file = Path(tmpdir) / "job.log"
        logger = setup_logger("test_job", log_file=log_file)
        
        log_job_start(logger, "test_job", date="2024-01-15", table="test_table")
        
        content = log_file.read_text()
        assert "Starting job: test_job" in content
        assert "date: 2024-01-15" in content
        assert "table: test_table" in content


def test_log_job_end():
    """Test job end logging"""
    from utils.logging_utils import setup_logger, log_job_end
    
    with tempfile.TemporaryDirectory() as tmpdir:
        log_file = Path(tmpdir) / "job.log"
        logger = setup_logger("test_job", log_file=log_file)
        
        # Test successful job
        log_job_end(logger, "test_job", success=True, records=1000)
        content = log_file.read_text()
        assert "COMPLETED SUCCESSFULLY" in content
        assert "records: 1000" in content
        
        # Test failed job
        log_job_end(logger, "test_job", success=False, error="test error")
        content = log_file.read_text()
        assert "FAILED" in content


def test_log_data_lineage():
    """Test data lineage logging"""
    from utils.logging_utils import setup_logger, log_data_lineage
    
    with tempfile.TemporaryDirectory() as tmpdir:
        log_file = Path(tmpdir) / "lineage.log"
        logger = setup_logger("test_lineage", log_file=log_file)
        
        log_data_lineage(
            logger,
            dataset_name="test_dataset",
            date_range="2024-01-15",
            input_records=1000,
            output_records=950,
            commit_id="abc123"
        )
        
        content = log_file.read_text()
        assert "Data Lineage" in content
        assert "test_dataset" in content
        assert "2024-01-15" in content
        assert "Input Records: 1000" in content
        assert "Output Records: 950" in content
        assert "Rejected Records: 50" in content
        assert "abc123" in content


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
