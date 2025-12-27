//! API Layer Module
//!
//! External interfaces for Galactus intent inference.
//! Provides gRPC and HTTP APIs for external consumers.
//!
//! # Architecture
//!
//! The API layer consists of:
//! 1. **gRPC Service**: High-performance RPC interface
//! 2. **HTTP Service**: RESTful interface for web clients
//! 3. **Serialization**: Safe intent vector serialization
//! 4. **Access Control**: Authentication and authorization
//! 5. **Language Safety**: Type-safe interfaces across languages
//!
//! # Safety & Security
//!
//! - **Type Safety**: Compile-time guarantees across language boundaries
//! - **Access Control**: Configurable authentication and rate limiting
//! - **Input Validation**: Comprehensive request validation
//! - **Error Handling**: Safe error propagation to clients
//! - **Resource Limits**: Configurable timeouts and size limits
//!
//! # API Endpoints
//!
//! ## gRPC Service
//! - `GetIntent`: Single intent vector computation
//! - `BatchIntent`: Multiple intent vectors in batch
//! - `StreamIntent`: Real-time intent stream
//! - `HealthCheck`: Service health monitoring
//!
//! ## HTTP Service
//! - `POST /api/v1/intent`: Single intent computation
//! - `POST /api/v1/intent/batch`: Batch intent computation
//! - `GET /api/v1/health`: Health check
//! - `GET /api/v1/metrics`: Service metrics

pub mod grpc;
pub mod http;
pub mod types;
pub mod health;
pub mod metrics;

// Re-export main types for convenience
pub use grpc::IntentService as GrpcIntentService;
pub use http::IntentApi as HttpIntentApi;
pub use health::{HealthChecker, HealthCheckResponse};
pub use metrics::MetricsCollector;

// Core types for API communication
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::intent::{IntentResult, SignalInput};
use crate::features::{FeatureInputs, MarketDataPoint, OptionChain, FuturesData};

/// Request for intent computation
#[derive(Debug, Clone)]
pub struct IntentRequest {
    /// Market data for feature computation
    pub market_data: HashMap<String, MarketDataPoint>,

    /// Option chain data for feature computation
    pub options_data: HashMap<String, OptionChain>,

    /// Futures data for feature computation
    pub futures_data: HashMap<String, FuturesData>,

    /// Additional context data
    pub context: HashMap<String, String>,

    /// Request metadata
    pub metadata: HashMap<String, String>,

    /// Client identifier
    pub client_id: String,

    /// Request timestamp
    pub timestamp: i64,
}

/// Response containing intent result
#[derive(Debug, Clone)]
pub struct IntentResponse {
    /// Intent computation result
    pub result: IntentResult,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Processing timestamp
    pub timestamp: i64,
}

/// Request for batch intent computation
#[derive(Debug, Clone)]
pub struct BatchIntentRequest {
    /// Multiple intent requests
    pub requests: Vec<IntentRequest>,

    /// Batch metadata
    pub metadata: HashMap<String, String>,

    /// Client identifier
    pub client_id: String,

    /// Request timestamp
    pub timestamp: i64,
}

/// Response for batch intent computation
#[derive(Debug, Clone)]
pub struct BatchIntentResponse {
    /// Individual intent responses
    pub responses: Vec<IntentResponse>,

    /// Batch processing summary
    pub summary: BatchSummary,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Processing timestamp
    pub timestamp: i64,
}

/// Summary of batch processing
#[derive(Debug, Clone)]
pub struct BatchSummary {
    /// Total requests processed
    pub total_requests: usize,

    /// Successful computations
    pub successful: usize,

    /// Failed computations
    pub failed: usize,

    /// Total processing time
    pub total_processing_time_ns: u128,

    /// Average processing time per request
    pub avg_processing_time_ns: u64,
}

/// API configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Maximum signals per request
    pub max_signals_per_request: usize,

    /// Maximum batch size
    pub max_batch_size: usize,

    /// Request timeout (milliseconds)
    pub request_timeout_ms: u64,

    /// Maximum request size (bytes)
    pub max_request_size_bytes: usize,

    /// Enable authentication
    pub enable_auth: bool,

    /// Rate limiting (requests per second per client)
    pub rate_limit_rps: u32,

    /// Enable detailed logging
    pub enable_logging: bool,

    /// CORS settings for HTTP API
    pub cors_allowed_origins: Vec<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            max_signals_per_request: 10,
            max_batch_size: 100,
            request_timeout_ms: 5000, // 5 seconds
            max_request_size_bytes: 1024 * 1024, // 1MB
            enable_auth: true,
            rate_limit_rps: 100,
            enable_logging: false,
            cors_allowed_origins: vec!["*".to_string()],
        }
    }
}

/// API errors
#[derive(Debug, Clone, PartialEq)]
pub enum ApiError {
    /// Invalid request format
    InvalidRequest(String),

    /// Authentication failed
    AuthenticationFailed(String),

    /// Authorization failed
    AuthorizationFailed(String),

    /// Rate limit exceeded
    RateLimitExceeded(String),

    /// Request too large
    RequestTooLarge(String),

    /// Processing failed
    ProcessingError(String),

    /// Internal server error
    InternalError(String),

    /// Service unavailable
    ServiceUnavailable(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            ApiError::AuthorizationFailed(msg) => write!(f, "Authorization failed: {}", msg),
            ApiError::RateLimitExceeded(msg) => write!(f, "Rate limit exceeded: {}", msg),
            ApiError::RequestTooLarge(msg) => write!(f, "Request too large: {}", msg),
            ApiError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ApiError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<crate::intent::IntentError> for ApiError {
    fn from(error: crate::intent::IntentError) -> Self {
        ApiError::ProcessingError(error.to_string())
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize)]
pub struct HealthCheck {
    /// Service status
    pub status: HealthStatus,

    /// Service version
    pub version: String,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// Additional health metrics
    pub metrics: HashMap<String, String>,
}

/// Service health status
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,

    /// Service is degraded but functional
    Degraded,

    /// Service is unhealthy
    Unhealthy,
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_defaults() {
        let config = ApiConfig::default();
        assert_eq!(config.max_signals_per_request, 10);
        assert_eq!(config.max_batch_size, 100);
        assert_eq!(config.request_timeout_ms, 5000);
        assert!(config.enable_auth);
    }

    #[test]
    fn test_intent_request_creation() {
        let request = IntentRequest {
            signals: vec![],
            metadata: HashMap::new(),
            client_id: "test_client".to_string(),
            timestamp: 1234567890,
        };

        assert_eq!(request.client_id, "test_client");
        assert_eq!(request.timestamp, 1234567890);
    }

    #[test]
    fn test_health_status_display() {
        assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
        assert_eq!(HealthStatus::Degraded.to_string(), "degraded");
        assert_eq!(HealthStatus::Unhealthy.to_string(), "unhealthy");
    }
}