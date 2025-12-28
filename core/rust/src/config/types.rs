//! Configuration Types
//!
//! Defines all configuration structures used throughout the system.

use serde::{Deserialize, Serialize};

/// Main Galactus configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GalactusConfig {
    /// Data ingestion configuration
    pub data_ingestion: DataIngestionConfig,
    /// Intent engine configuration
    pub intent_engine: IntentEngineConfig,
    /// Persistence configuration
    pub persistence: PersistenceConfig,
    /// Streaming configuration
    pub streaming: StreamingConfig,
    /// API configuration
    pub api: ApiConfig,
}

/// Data ingestion configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataIngestionConfig {
    /// Batch size for processing events
    pub batch_size: usize,
    /// Maximum delay in seconds for data to be considered fresh
    pub max_delay_seconds: u64,
    /// Minimum data completeness ratio (0.0 - 1.0)
    pub min_completeness: f64,
    /// Maximum allowed contradictions in data
    pub max_contradictions: usize,
    /// Approved data sources
    pub approved_sources: Vec<String>,
}

impl Default for DataIngestionConfig {
    fn default() -> Self {
        Self {
            batch_size: 100,
            max_delay_seconds: 300, // 5 minutes
            min_completeness: 0.8,
            max_contradictions: 0,
            approved_sources: vec!["NSE".to_string(), "BSE".to_string(), "NSE_FO".to_string()],
        }
    }
}

/// Intent engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentEngineConfig {
    /// Minimum confidence threshold for intent vectors
    pub min_confidence: f64,
    /// Number of alternative interpretations to generate
    pub num_alternatives: usize,
    /// Signal aggregation method
    pub aggregation_method: String,
    /// Enable regime-aware processing
    pub regime_aware: bool,
}

impl Default for IntentEngineConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.6,
            num_alternatives: 3,
            aggregation_method: "weighted_average".to_string(),
            regime_aware: true,
        }
    }
}

/// Persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceConfig {
    /// Storage directory path
    pub storage_path: String,
    /// Maximum number of intent vectors to keep in memory
    pub max_memory_items: usize,
    /// Flush interval in seconds
    pub flush_interval_seconds: u64,
    /// Enable compression
    pub enable_compression: bool,
    /// Retention period in days
    pub retention_days: u64,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            storage_path: "./data/intents".to_string(),
            max_memory_items: 10000,
            flush_interval_seconds: 60,
            enable_compression: true,
            retention_days: 365,
        }
    }
}

/// Streaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    /// WebSocket endpoint for NSE data
    pub nse_websocket_url: String,
    /// WebSocket endpoint for BSE data
    pub bse_websocket_url: String,
    /// Reconnection delay in seconds
    pub reconnect_delay_seconds: u64,
    /// Maximum reconnection attempts
    pub max_reconnect_attempts: usize,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,
    /// Enable automatic reconnection
    pub auto_reconnect: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            nse_websocket_url: "wss://example.com/nse".to_string(),
            bse_websocket_url: "wss://example.com/bse".to_string(),
            reconnect_delay_seconds: 5,
            max_reconnect_attempts: 10,
            heartbeat_interval_seconds: 30,
            auto_reconnect: true,
        }
    }
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// HTTP server host
    pub host: String,
    /// HTTP server port
    pub port: u16,
    /// gRPC server port
    pub grpc_port: u16,
    /// Enable CORS
    pub enable_cors: bool,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            grpc_port: 50051,
            enable_cors: true,
            timeout_seconds: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = GalactusConfig::default();
        assert_eq!(config.data_ingestion.batch_size, 100);
        assert_eq!(config.intent_engine.min_confidence, 0.6);
        assert_eq!(config.persistence.retention_days, 365);
        assert_eq!(config.api.port, 8080);
    }

    #[test]
    fn test_data_ingestion_defaults() {
        let config = DataIngestionConfig::default();
        assert_eq!(config.approved_sources.len(), 3);
        assert!(config.approved_sources.contains(&"NSE".to_string()));
    }
}
