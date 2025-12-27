//! Configuration Loader
//!
//! Loads configuration from multiple sources with priority:
//! 1. Environment variables (highest priority)
//! 2. Configuration file
//! 3. Default values (lowest priority)

use super::{ConfigError, types::*};
use std::env;
use std::path::PathBuf;

/// Configuration source
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Load from environment variables and defaults
    Environment,
    /// Load from a specific file
    File(PathBuf),
}

/// Configuration loader
pub struct ConfigLoader {
    source: ConfigSource,
}

impl ConfigLoader {
    /// Create a new loader from environment variables
    pub fn new() -> Self {
        Self {
            source: ConfigSource::Environment,
        }
    }

    /// Create a new loader from a specific file
    pub fn from_file(path: PathBuf) -> Self {
        Self {
            source: ConfigSource::File(path),
        }
    }

    /// Load configuration from the configured source
    pub fn load(&self) -> Result<GalactusConfig, ConfigError> {
        // Start with defaults
        let mut config = GalactusConfig::default();

        // Apply environment variables
        self.apply_env_overrides(&mut config)?;

        Ok(config)
    }

    /// Apply environment variable overrides to configuration
    fn apply_env_overrides(&self, config: &mut GalactusConfig) -> Result<(), ConfigError> {
        // Data ingestion overrides
        if let Ok(val) = env::var("GALACTUS_BATCH_SIZE") {
            config.data_ingestion.batch_size = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_BATCH_SIZE".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_MAX_DELAY_SECONDS") {
            config.data_ingestion.max_delay_seconds = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_MAX_DELAY_SECONDS".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_MIN_COMPLETENESS") {
            config.data_ingestion.min_completeness = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_MIN_COMPLETENESS".to_string(), "Invalid number".to_string()))?;
        }

        // Intent engine overrides
        if let Ok(val) = env::var("GALACTUS_MIN_CONFIDENCE") {
            config.intent_engine.min_confidence = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_MIN_CONFIDENCE".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_NUM_ALTERNATIVES") {
            config.intent_engine.num_alternatives = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_NUM_ALTERNATIVES".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_REGIME_AWARE") {
            config.intent_engine.regime_aware = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_REGIME_AWARE".to_string(), "Invalid boolean".to_string()))?;
        }

        // Persistence overrides
        if let Ok(val) = env::var("GALACTUS_STORAGE_PATH") {
            config.persistence.storage_path = val;
        }

        if let Ok(val) = env::var("GALACTUS_MAX_MEMORY_ITEMS") {
            config.persistence.max_memory_items = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_MAX_MEMORY_ITEMS".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_FLUSH_INTERVAL_SECONDS") {
            config.persistence.flush_interval_seconds = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_FLUSH_INTERVAL_SECONDS".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_ENABLE_COMPRESSION") {
            config.persistence.enable_compression = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_ENABLE_COMPRESSION".to_string(), "Invalid boolean".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_RETENTION_DAYS") {
            config.persistence.retention_days = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_RETENTION_DAYS".to_string(), "Invalid number".to_string()))?;
        }

        // Streaming overrides
        if let Ok(val) = env::var("GALACTUS_NSE_WEBSOCKET_URL") {
            config.streaming.nse_websocket_url = val;
        }

        if let Ok(val) = env::var("GALACTUS_BSE_WEBSOCKET_URL") {
            config.streaming.bse_websocket_url = val;
        }

        if let Ok(val) = env::var("GALACTUS_RECONNECT_DELAY_SECONDS") {
            config.streaming.reconnect_delay_seconds = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_RECONNECT_DELAY_SECONDS".to_string(), "Invalid number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_MAX_RECONNECT_ATTEMPTS") {
            config.streaming.max_reconnect_attempts = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_MAX_RECONNECT_ATTEMPTS".to_string(), "Invalid number".to_string()))?;
        }

        // API overrides
        if let Ok(val) = env::var("GALACTUS_API_HOST") {
            config.api.host = val;
        }

        if let Ok(val) = env::var("GALACTUS_API_PORT") {
            config.api.port = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_API_PORT".to_string(), "Invalid port number".to_string()))?;
        }

        if let Ok(val) = env::var("GALACTUS_GRPC_PORT") {
            config.api.grpc_port = val.parse()
                .map_err(|_| ConfigError::InvalidValue("GALACTUS_GRPC_PORT".to_string(), "Invalid port number".to_string()))?;
        }

        Ok(())
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_defaults() {
        let loader = ConfigLoader::new();
        let config = loader.load().unwrap();
        assert_eq!(config.data_ingestion.batch_size, 100);
    }

    #[test]
    fn test_env_override() {
        env::set_var("GALACTUS_BATCH_SIZE", "200");
        let loader = ConfigLoader::new();
        let config = loader.load().unwrap();
        assert_eq!(config.data_ingestion.batch_size, 200);
        env::remove_var("GALACTUS_BATCH_SIZE");
    }

    #[test]
    fn test_invalid_env_value() {
        env::set_var("GALACTUS_BATCH_SIZE", "invalid");
        let loader = ConfigLoader::new();
        let result = loader.load();
        assert!(result.is_err());
        env::remove_var("GALACTUS_BATCH_SIZE");
    }
}
