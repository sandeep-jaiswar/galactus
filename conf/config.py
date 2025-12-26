"""
Centralized Configuration Management for Galactus

This module provides environment-based configuration for all components.
No hardcoded paths - everything is configurable via environment variables.
"""

import os
from pathlib import Path
from typing import Optional


class GalactusConfig:
    """Central configuration for Galactus platform"""
    
    def __init__(self):
        # Project root
        self.PROJECT_ROOT = Path(os.getenv('GALACTUS_PROJECT_ROOT', Path(__file__).parent.parent))
        
        # Data paths - following Bronze/Silver/Gold architecture
        self.DATA_ROOT = Path(os.getenv('GALACTUS_DATA_ROOT', self.PROJECT_ROOT / 'data'))
        self.BRONZE_PATH = self.DATA_ROOT / 'bronze'
        self.SILVER_PATH = Path(os.getenv('GALACTUS_SILVER_PATH', self.DATA_ROOT / 'silver'))
        self.GOLD_PATH = self.DATA_ROOT / 'gold'
        
        # Hudi configuration
        self.HUDI_BASE_PATH = Path(os.getenv('HUDI_BASE_PATH', self.SILVER_PATH))
        
        # Temporary storage for downloads
        self.TEMP_DIR = Path(os.getenv('GALACTUS_TEMP_DIR', '/tmp/galactus'))
        
        # Logging
        self.LOG_DIR = Path(os.getenv('GALACTUS_LOG_DIR', self.PROJECT_ROOT / 'logs'))
        self.LOG_LEVEL = os.getenv('GALACTUS_LOG_LEVEL', 'INFO')
        
        # ClickHouse configuration
        self.CLICKHOUSE_HOST = os.getenv('CLICKHOUSE_HOST', 'localhost')
        self.CLICKHOUSE_PORT = int(os.getenv('CLICKHOUSE_PORT', '8123'))
        self.CLICKHOUSE_DATABASE = os.getenv('CLICKHOUSE_DATABASE', 'galactus')
        self.CLICKHOUSE_USER = os.getenv('CLICKHOUSE_USER', 'default')
        self.CLICKHOUSE_PASSWORD = os.getenv('CLICKHOUSE_PASSWORD', '')
        
        # Hive Metastore configuration
        self.HIVE_METASTORE_URI = os.getenv('HIVE_METASTORE_URI', 'thrift://localhost:9083')
        
        # Spark configuration
        self.SPARK_MASTER = os.getenv('SPARK_MASTER', 'local[*]')
        self.SPARK_DRIVER_MEMORY = os.getenv('SPARK_DRIVER_MEMORY', '4g')
        self.SPARK_EXECUTOR_MEMORY = os.getenv('SPARK_EXECUTOR_MEMORY', '4g')
        
        # NSE configuration
        self.NSE_DOWNLOAD_TIMEOUT = int(os.getenv('NSE_DOWNLOAD_TIMEOUT', '30'))
        self.NSE_RETRY_ATTEMPTS = int(os.getenv('NSE_RETRY_ATTEMPTS', '3'))
        self.NSE_RETRY_DELAY = int(os.getenv('NSE_RETRY_DELAY', '5'))
        
        # Ensure directories exist
        self._create_directories()
    
    def _create_directories(self):
        """Create necessary directories if they don't exist"""
        for path in [
            self.BRONZE_PATH, 
            self.SILVER_PATH, 
            self.GOLD_PATH,
            self.TEMP_DIR,
            self.LOG_DIR
        ]:
            path.mkdir(parents=True, exist_ok=True)
    
    def get_bronze_path(self, dataset: str, date: Optional[str] = None) -> Path:
        """
        Get Bronze layer path for a dataset
        
        Args:
            dataset: Name of the dataset (e.g., 'bhavcopy', 'announcements')
            date: Optional date string in YYYY-MM-DD format for date partitioning
            
        Returns:
            Path to the Bronze storage location
        """
        base_path = self.BRONZE_PATH / dataset
        if date:
            base_path = base_path / f"date={date}"
        return base_path
    
    def get_silver_path(self, table_name: str) -> Path:
        """
        Get Silver layer (Hudi) path for a table
        
        Args:
            table_name: Name of the Hudi table
            
        Returns:
            Path to the Hudi table
        """
        return self.HUDI_BASE_PATH / table_name
    
    def get_gold_path(self, dataset: str) -> Path:
        """
        Get Gold layer path for derived datasets
        
        Args:
            dataset: Name of the derived dataset
            
        Returns:
            Path to the Gold storage location
        """
        return self.GOLD_PATH / dataset
    
    def validate(self):
        """Validate configuration values"""
        errors = []
        
        # Validate numeric values
        if self.CLICKHOUSE_PORT < 1 or self.CLICKHOUSE_PORT > 65535:
            errors.append(f"Invalid CLICKHOUSE_PORT: {self.CLICKHOUSE_PORT}")
        
        if self.NSE_DOWNLOAD_TIMEOUT < 1:
            errors.append(f"Invalid NSE_DOWNLOAD_TIMEOUT: {self.NSE_DOWNLOAD_TIMEOUT}")
        
        if self.NSE_RETRY_ATTEMPTS < 0:
            errors.append(f"Invalid NSE_RETRY_ATTEMPTS: {self.NSE_RETRY_ATTEMPTS}")
        
        # Validate log level
        valid_log_levels = ['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']
        if self.LOG_LEVEL not in valid_log_levels:
            errors.append(f"Invalid LOG_LEVEL: {self.LOG_LEVEL}. Must be one of {valid_log_levels}")
        
        if errors:
            raise ValueError(f"Configuration validation failed:\n" + "\n".join(errors))
        
        return True


# Global configuration instance
config = GalactusConfig()
