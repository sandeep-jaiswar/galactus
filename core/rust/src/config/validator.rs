//! Configuration Validator
//!
//! Validates configuration values to ensure they meet system requirements.

use super::{ConfigError, types::*};

/// Validation error
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation error in {}: {}", self.field, self.message)
    }
}

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate entire configuration
    pub fn validate(&self, config: &GalactusConfig) -> Result<(), ConfigError> {
        self.validate_data_ingestion(&config.data_ingestion)?;
        self.validate_intent_engine(&config.intent_engine)?;
        self.validate_persistence(&config.persistence)?;
        self.validate_streaming(&config.streaming)?;
        self.validate_api(&config.api)?;
        Ok(())
    }

    /// Validate data ingestion configuration
    fn validate_data_ingestion(&self, config: &DataIngestionConfig) -> Result<(), ConfigError> {
        if config.batch_size == 0 {
            return Err(ConfigError::InvalidValue(
                "data_ingestion.batch_size".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        if config.batch_size > 10000 {
            return Err(ConfigError::InvalidValue(
                "data_ingestion.batch_size".to_string(),
                "Must be less than or equal to 10000".to_string(),
            ));
        }

        if config.min_completeness < 0.0 || config.min_completeness > 1.0 {
            return Err(ConfigError::InvalidValue(
                "data_ingestion.min_completeness".to_string(),
                "Must be between 0.0 and 1.0".to_string(),
            ));
        }

        if config.approved_sources.is_empty() {
            return Err(ConfigError::InvalidValue(
                "data_ingestion.approved_sources".to_string(),
                "Must have at least one approved source".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate intent engine configuration
    fn validate_intent_engine(&self, config: &IntentEngineConfig) -> Result<(), ConfigError> {
        if config.min_confidence < 0.0 || config.min_confidence > 1.0 {
            return Err(ConfigError::InvalidValue(
                "intent_engine.min_confidence".to_string(),
                "Must be between 0.0 and 1.0".to_string(),
            ));
        }

        if config.num_alternatives > 10 {
            return Err(ConfigError::InvalidValue(
                "intent_engine.num_alternatives".to_string(),
                "Must be less than or equal to 10".to_string(),
            ));
        }

        // Validate aggregation method
        let valid_methods = vec!["weighted_average", "median", "mode"];
        if !valid_methods.contains(&config.aggregation_method.as_str()) {
            return Err(ConfigError::InvalidValue(
                "intent_engine.aggregation_method".to_string(),
                format!("Must be one of: {:?}", valid_methods),
            ));
        }

        Ok(())
    }

    /// Validate persistence configuration
    fn validate_persistence(&self, config: &PersistenceConfig) -> Result<(), ConfigError> {
        if config.storage_path.is_empty() {
            return Err(ConfigError::InvalidValue(
                "persistence.storage_path".to_string(),
                "Cannot be empty".to_string(),
            ));
        }

        if config.max_memory_items == 0 {
            return Err(ConfigError::InvalidValue(
                "persistence.max_memory_items".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        if config.flush_interval_seconds == 0 {
            return Err(ConfigError::InvalidValue(
                "persistence.flush_interval_seconds".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        if config.retention_days == 0 {
            return Err(ConfigError::InvalidValue(
                "persistence.retention_days".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate streaming configuration
    fn validate_streaming(&self, config: &StreamingConfig) -> Result<(), ConfigError> {
        if config.nse_websocket_url.is_empty() {
            return Err(ConfigError::InvalidValue(
                "streaming.nse_websocket_url".to_string(),
                "Cannot be empty".to_string(),
            ));
        }

        if config.bse_websocket_url.is_empty() {
            return Err(ConfigError::InvalidValue(
                "streaming.bse_websocket_url".to_string(),
                "Cannot be empty".to_string(),
            ));
        }

        if !config.nse_websocket_url.starts_with("ws://") && !config.nse_websocket_url.starts_with("wss://") {
            return Err(ConfigError::InvalidValue(
                "streaming.nse_websocket_url".to_string(),
                "Must start with ws:// or wss://".to_string(),
            ));
        }

        if !config.bse_websocket_url.starts_with("ws://") && !config.bse_websocket_url.starts_with("wss://") {
            return Err(ConfigError::InvalidValue(
                "streaming.bse_websocket_url".to_string(),
                "Must start with ws:// or wss://".to_string(),
            ));
        }

        if config.max_reconnect_attempts == 0 {
            return Err(ConfigError::InvalidValue(
                "streaming.max_reconnect_attempts".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate API configuration
    fn validate_api(&self, config: &ApiConfig) -> Result<(), ConfigError> {
        if config.host.is_empty() {
            return Err(ConfigError::InvalidValue(
                "api.host".to_string(),
                "Cannot be empty".to_string(),
            ));
        }

        if config.port == 0 {
            return Err(ConfigError::InvalidValue(
                "api.port".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        if config.grpc_port == 0 {
            return Err(ConfigError::InvalidValue(
                "api.grpc_port".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        if config.port == config.grpc_port {
            return Err(ConfigError::InvalidValue(
                "api.port".to_string(),
                "HTTP and gRPC ports must be different".to_string(),
            ));
        }

        if config.timeout_seconds == 0 {
            return Err(ConfigError::InvalidValue(
                "api.timeout_seconds".to_string(),
                "Must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        let validator = ConfigValidator::new();
        let config = GalactusConfig::default();
        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_invalid_batch_size() {
        let validator = ConfigValidator::new();
        let mut config = GalactusConfig::default();
        config.data_ingestion.batch_size = 0;
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_invalid_min_confidence() {
        let validator = ConfigValidator::new();
        let mut config = GalactusConfig::default();
        config.intent_engine.min_confidence = 1.5;
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_invalid_websocket_url() {
        let validator = ConfigValidator::new();
        let mut config = GalactusConfig::default();
        config.streaming.nse_websocket_url = "http://invalid.com".to_string();
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_same_ports() {
        let validator = ConfigValidator::new();
        let mut config = GalactusConfig::default();
        config.api.port = 8080;
        config.api.grpc_port = 8080;
        assert!(validator.validate(&config).is_err());
    }
}
