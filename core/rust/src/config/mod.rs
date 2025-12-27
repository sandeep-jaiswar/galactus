//! Configuration Management System
//!
//! Provides environment-based configuration with validation and hot-reload capabilities.
//!
//! # Design Principles
//! - Environment variables override defaults
//! - All config values are validated on load
//! - Hot-reload support for non-critical settings
//! - Type-safe configuration access
//! - No secrets in config files (use environment variables)

pub mod loader;
pub mod validator;
pub mod types;

pub use loader::{ConfigLoader, ConfigSource};
pub use validator::{ConfigValidator, ValidationError};
pub use types::{
    GalactusConfig, DataIngestionConfig, IntentEngineConfig,
    PersistenceConfig, StreamingConfig, ApiConfig,
};

use std::sync::{Arc, RwLock};
use std::path::PathBuf;

/// Configuration manager with hot-reload support
pub struct ConfigManager {
    config: Arc<RwLock<GalactusConfig>>,
    loader: ConfigLoader,
    validator: ConfigValidator,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> Result<Self, ConfigError> {
        let loader = ConfigLoader::new();
        let validator = ConfigValidator::new();
        let config = loader.load()?;
        validator.validate(&config)?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            loader,
            validator,
        })
    }

    /// Create from a specific configuration file
    pub fn from_file(path: PathBuf) -> Result<Self, ConfigError> {
        let loader = ConfigLoader::from_file(path);
        let validator = ConfigValidator::new();
        let config = loader.load()?;
        validator.validate(&config)?;

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            loader,
            validator,
        })
    }

    /// Get current configuration (read-only)
    pub fn get(&self) -> GalactusConfig {
        self.config.read().unwrap().clone()
    }

    /// Reload configuration from source
    pub fn reload(&self) -> Result<(), ConfigError> {
        let new_config = self.loader.load()?;
        self.validator.validate(&new_config)?;

        let mut config = self.config.write().unwrap();
        *config = new_config;

        Ok(())
    }

    /// Get a shared reference to the configuration
    pub fn get_shared(&self) -> Arc<RwLock<GalactusConfig>> {
        Arc::clone(&self.config)
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default ConfigManager")
    }
}

/// Configuration errors
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Failed to load configuration
    LoadError(String),
    /// Configuration validation failed
    ValidationError(String),
    /// Missing required configuration
    MissingRequired(String),
    /// Invalid configuration value
    InvalidValue(String, String),
    /// Environment variable not found
    EnvVarNotFound(String),
    /// File not found
    FileNotFound(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::LoadError(msg) => write!(f, "Configuration load error: {}", msg),
            ConfigError::ValidationError(msg) => write!(f, "Configuration validation error: {}", msg),
            ConfigError::MissingRequired(field) => write!(f, "Missing required configuration: {}", field),
            ConfigError::InvalidValue(field, reason) => write!(f, "Invalid value for {}: {}", field, reason),
            ConfigError::EnvVarNotFound(var) => write!(f, "Environment variable not found: {}", var),
            ConfigError::FileNotFound(path) => write!(f, "Configuration file not found: {}", path),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager_creation() {
        let manager = ConfigManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_config_get() {
        let manager = ConfigManager::new().unwrap();
        let config = manager.get();
        assert!(config.data_ingestion.batch_size > 0);
    }
}
