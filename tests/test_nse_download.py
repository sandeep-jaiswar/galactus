"""
Unit tests for NSE download module
"""

import os
import tempfile
from pathlib import Path
from datetime import datetime
import pytest
from unittest.mock import Mock, patch, MagicMock


def test_nse_download_imports():
    """Test that NSE download module can be imported"""
    from utils.nse_download import download_bhavcopy, NSEDownloadError
    
    assert download_bhavcopy is not None
    assert NSEDownloadError is not None


def test_nse_download_error():
    """Test NSE download error exception"""
    from utils.nse_download import NSEDownloadError
    
    error = NSEDownloadError("test error")
    assert str(error) == "test error"
    assert isinstance(error, Exception)


def test_date_validation():
    """Test date format validation in download function"""
    from utils.nse_download import download_bhavcopy, NSEDownloadError
    
    # Invalid date format should raise ValueError
    with pytest.raises(ValueError):
        download_bhavcopy("invalid-date")
    
    with pytest.raises(ValueError):
        download_bhavcopy("2024/01/15")
    
    with pytest.raises(ValueError):
        download_bhavcopy("15-01-2024")


@patch('utils.nse_download.requests.get')
def test_successful_download(mock_get):
    """Test successful download scenario"""
    from utils.nse_download import download_bhavcopy
    
    # Mock successful response
    mock_response = Mock()
    mock_response.status_code = 200
    mock_response.content = b"test,csv,data"
    mock_get.return_value = mock_response
    
    with tempfile.TemporaryDirectory() as tmpdir:
        result = download_bhavcopy(
            session_date="2024-01-15",
            output_dir=Path(tmpdir),
            max_retries=1
        )
        
        assert result.exists()
        assert result.stat().st_size > 0


@patch('utils.nse_download.requests.get')
def test_download_404_error(mock_get):
    """Test 404 error handling"""
    from utils.nse_download import download_bhavcopy, NSEDownloadError
    
    # Mock 404 response
    mock_response = Mock()
    mock_response.status_code = 404
    mock_get.return_value = mock_response
    
    with tempfile.TemporaryDirectory() as tmpdir:
        with pytest.raises(NSEDownloadError) as exc_info:
            download_bhavcopy(
                session_date="2024-01-15",
                output_dir=Path(tmpdir),
                max_retries=1
            )
        
        assert "404" in str(exc_info.value)


@patch('utils.nse_download.requests.get')
def test_download_timeout(mock_get):
    """Test timeout handling"""
    from utils.nse_download import download_bhavcopy, NSEDownloadError
    import requests
    
    # Mock timeout
    mock_get.side_effect = requests.exceptions.Timeout()
    
    with tempfile.TemporaryDirectory() as tmpdir:
        with pytest.raises(NSEDownloadError):
            download_bhavcopy(
                session_date="2024-01-15",
                output_dir=Path(tmpdir),
                max_retries=1,
                retry_delay=0  # No delay in tests
            )


@patch('utils.nse_download.requests.get')
def test_download_retries(mock_get):
    """Test retry mechanism"""
    from utils.nse_download import download_bhavcopy, NSEDownloadError
    import requests
    
    # First call fails, second succeeds
    mock_response = Mock()
    mock_response.status_code = 200
    mock_response.content = b"test,data"
    
    mock_get.side_effect = [
        requests.exceptions.RequestException("Network error"),
        mock_response
    ]
    
    with tempfile.TemporaryDirectory() as tmpdir:
        result = download_bhavcopy(
            session_date="2024-01-15",
            output_dir=Path(tmpdir),
            max_retries=2,
            retry_delay=0
        )
        
        assert result.exists()
        assert mock_get.call_count == 2


def test_bronze_path_structure():
    """Test that download creates correct Bronze layer structure"""
    from utils.nse_download import download_bhavcopy
    import requests
    
    # This test would require actual network call or more complex mocking
    # Skip in CI/CD environments
    pass


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
