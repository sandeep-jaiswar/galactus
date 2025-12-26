"""
Unit tests for configuration module
"""

import os
import tempfile
from pathlib import Path
import pytest


def test_config_import():
    """Test that config module can be imported"""
    from conf.config import GalactusConfig, config
    
    assert config is not None
    assert isinstance(config, GalactusConfig)


def test_config_paths():
    """Test that config creates necessary paths"""
    from conf.config import config
    
    # Test that key paths exist
    assert config.PROJECT_ROOT.exists()
    assert config.BRONZE_PATH.exists()
    assert config.SILVER_PATH.exists()
    assert config.GOLD_PATH.exists()
    assert config.LOG_DIR.exists()


def test_config_get_bronze_path():
    """Test Bronze layer path generation"""
    from conf.config import config
    
    # Test without date
    path = config.get_bronze_path("test_dataset")
    assert "bronze/test_dataset" in str(path)
    
    # Test with date
    path_dated = config.get_bronze_path("test_dataset", "2024-01-15")
    assert "bronze/test_dataset/date=2024-01-15" in str(path_dated)


def test_config_get_silver_path():
    """Test Silver layer path generation"""
    from conf.config import config
    
    path = config.get_silver_path("test_table")
    assert "silver/test_table" in str(path)


def test_config_get_gold_path():
    """Test Gold layer path generation"""
    from conf.config import config
    
    path = config.get_gold_path("test_derived")
    assert "gold/test_derived" in str(path)


def test_config_validation():
    """Test config validation"""
    from conf.config import GalactusConfig
    
    # Valid config should pass
    config = GalactusConfig()
    assert config.validate() is True


def test_config_validation_failures():
    """Test that config validation catches invalid values"""
    from conf.config import GalactusConfig
    
    # Test invalid port
    config = GalactusConfig()
    config.CLICKHOUSE_PORT = 999999
    
    with pytest.raises(ValueError):
        config.validate()
    
    # Test invalid timeout
    config2 = GalactusConfig()
    config2.NSE_DOWNLOAD_TIMEOUT = -5
    
    with pytest.raises(ValueError):
        config2.validate()
    
    # Test invalid log level
    config3 = GalactusConfig()
    config3.LOG_LEVEL = "INVALID"
    
    with pytest.raises(ValueError):
        config3.validate()


def test_config_environment_override():
    """Test that environment variables override defaults"""
    # Set environment variable
    test_log_level = "DEBUG"
    os.environ['GALACTUS_LOG_LEVEL'] = test_log_level
    
    # Import config (will read from environment)
    from conf.config import GalactusConfig
    config = GalactusConfig()
    
    assert config.LOG_LEVEL == test_log_level
    
    # Cleanup
    del os.environ['GALACTUS_LOG_LEVEL']


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
