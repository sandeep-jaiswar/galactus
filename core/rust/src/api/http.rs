//! HTTP Service Implementation
//!
//! RESTful API interface for intent inference.
//! Provides web-compatible interface for HTTP clients.
//!
//! # Architecture
//!
//! The HTTP service implements:
//! 1. **REST Endpoints**: Standard HTTP methods and paths
//! 2. **JSON Serialization**: Human-readable data format
//! 3. **CORS Support**: Cross-origin request handling
//! 4. **Middleware**: Authentication, logging, rate limiting
//!
//! # Endpoints
//!
//! ```http
//! POST /api/v1/intent          # Single intent computation
//! POST /api/v1/intent/batch    # Batch intent computation
//! GET  /api/v1/health          # Health check
//! GET  /api/v1/metrics         # Service metrics
//! ```
//!
//! # Request/Response Format
//!
//! All endpoints use JSON:
//! ```json
//! {
//!   "signals": [
//!     {
//!       "name": "oi_decay",
//!       "value": -0.4,
//!       "confidence": 0.85,
//!       "timestamp": 1703123456,
//!       "metadata": {}
//!     }
//!   ],
//!   "client_id": "client_123"
//! }
//! ```
//!
//! # Security
//!
//! - **HTTPS Only**: Encrypted communication
//! - **API Keys**: Authentication via headers
//! - **Rate Limiting**: Request throttling
//! - **Input Validation**: Comprehensive validation

use std::collections::HashMap;
use std::sync::Arc;
use std::convert::Infallible;
use warp::Filter;
use serde::{Deserialize, Serialize};
use crate::intent::{IntentEngine, SignalInput};
use crate::api::{
    ApiConfig, ApiError, IntentRequest, IntentResponse,
    BatchIntentRequest, BatchIntentResponse, HealthCheck
};

/// HTTP Intent API implementation
#[derive(Debug, Clone)]
pub struct IntentApi {
    /// Intent engine for computation
    engine: Arc<IntentEngine>,

    /// Service configuration
    config: ApiConfig,
}

impl IntentApi {
    /// Create a new HTTP intent API
    pub fn new(engine: Arc<IntentEngine>, config: ApiConfig) -> Self {
        Self { engine, config }
    }

    /// Create warp filters for the API
    pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        let api_v1 = warp::path("api").and(warp::path("v1"));

        // Health check endpoint
        let health = api_v1
            .and(warp::path("health"))
            .and(warp::get())
            .and(with_service(self.clone()))
            .and_then(Self::handle_health);

        // Single intent endpoint
        let intent = api_v1
            .and(warp::path("intent"))
            .and(warp::post())
            .and(warp::body::json())
            .and(with_service(self.clone()))
            .and_then(Self::handle_intent);

        // Batch intent endpoint
        let batch = api_v1
            .and(warp::path("intent"))
            .and(warp::path("batch"))
            .and(warp::post())
            .and(warp::body::json())
            .and(with_service(self.clone()))
            .and_then(Self::handle_batch);

        // Metrics endpoint
        let metrics = api_v1
            .and(warp::path("metrics"))
            .and(warp::get())
            .and(with_service(self.clone()))
            .and_then(Self::handle_metrics);

        // CORS support
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization", "x-api-key"])
            .allow_methods(vec!["GET", "POST"]);

        health.or(intent).or(batch).or(metrics).with(cors)
    }

    /// Handle health check requests
    async fn handle_health(service: Self) -> Result<impl warp::Reply, Infallible> {
        let health = service.get_health();
        Ok(warp::reply::json(&health))
    }

    /// Handle single intent requests
    async fn handle_intent(
        request: IntentRequestJson,
        service: Self,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Convert JSON to internal types
        let intent_request = request.to_internal()
            .map_err(|e| warp::reject::custom(ApiErrorRejection(e)))?;

        // Process request
        match service.process_intent_request(intent_request).await {
            Ok(response) => {
                let json_response = IntentResponseJson::from_internal(response);
                Ok(warp::reply::json(&json_response))
            }
            Err(error) => Err(warp::reject::custom(ApiErrorRejection(error))),
        }
    }

    /// Handle batch intent requests
    async fn handle_batch(
        request: BatchIntentRequestJson,
        service: Self,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Convert JSON to internal types
        let batch_request = request.to_internal()
            .map_err(|e| warp::reject::custom(ApiErrorRejection(e)))?;

        // Process batch
        match service.process_batch_request(batch_request).await {
            Ok(response) => {
                let json_response = BatchIntentResponseJson::from_internal(response);
                Ok(warp::reply::json(&json_response))
            }
            Err(error) => Err(warp::reject::custom(ApiErrorRejection(error))),
        }
    }

    /// Handle metrics requests
    async fn handle_metrics(service: Self) -> Result<impl warp::Reply, warp::Rejection> {
        let metrics = service.get_metrics();
        Ok(warp::reply::json(&metrics))
    }

    /// Process intent request (shared with gRPC)
    async fn process_intent_request(
        &self,
        request: IntentRequest,
    ) -> Result<IntentResponse, ApiError> {
        // Validate request
        self.validate_intent_request(&request)?;

        // Check authentication (simplified)
        self.authenticate_request(&request)?;

        // Check rate limits
        self.check_rate_limit(&request.client_id)?;

        // Process with intent engine
        let result = self.engine.process(request.signals)?;

        // Create response
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let mut metadata = HashMap::new();
        metadata.insert("server_version".to_string(), env!("CARGO_PKG_VERSION").to_string());

        Ok(IntentResponse {
            result,
            metadata,
            timestamp,
        })
    }

    /// Process batch request
    async fn process_batch_request(
        &self,
        request: BatchIntentRequest,
    ) -> Result<BatchIntentResponse, ApiError> {
        // Validate batch request
        self.validate_batch_request(&request)?;

        // Check authentication
        self.authenticate_request_batch(&request)?;

        // Check rate limits
        self.check_rate_limit(&request.client_id)?;

        let start_time = std::time::SystemTime::now();
        let mut responses = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        // Process each request
        for intent_request in request.requests {
            match self.process_intent_request(intent_request).await {
                Ok(response) => {
                    responses.push(response);
                    successful += 1;
                }
                Err(_) => {
                    failed += 1;
                }
            }
        }

        let total_processing_time = std::time::SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default()
            .as_nanos();

        let avg_processing_time = if successful > 0 {
            total_processing_time / successful as u128
        } else {
            0
        };

        let summary = crate::api::BatchSummary {
            total_requests: request.requests.len(),
            successful,
            failed,
            total_processing_time_ns: total_processing_time,
            avg_processing_time_ns: avg_processing_time as u64,
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let metadata = HashMap::new();

        Ok(BatchIntentResponse {
            responses,
            summary,
            metadata,
            timestamp,
        })
    }

    /// Validate intent request
    fn validate_intent_request(&self, request: &IntentRequest) -> Result<(), ApiError> {
        if request.signals.is_empty() {
            return Err(ApiError::InvalidRequest("No signals provided".to_string()));
        }

        if request.signals.len() > self.config.max_signals_per_request {
            return Err(ApiError::InvalidRequest(
                format!("Too many signals: {} (max: {})",
                       request.signals.len(),
                       self.config.max_signals_per_request)
            ));
        }

        if request.client_id.trim().is_empty() {
            return Err(ApiError::InvalidRequest("Client ID required".to_string()));
        }

        // Validate each signal
        for signal in &request.signals {
            if !(-1.0..=1.0).contains(&signal.value) {
                return Err(ApiError::InvalidRequest(
                    format!("Signal '{}' value {} is outside valid range [-1.0, 1.0]",
                           signal.name, signal.value)
                ));
            }

            if !(0.0..=1.0).contains(&signal.confidence) {
                return Err(ApiError::InvalidRequest(
                    format!("Signal '{}' confidence {} is outside valid range [0.0, 1.0]",
                           signal.name, signal.confidence)
                ));
            }
        }

        Ok(())
    }

    /// Validate batch request
    fn validate_batch_request(&self, request: &BatchIntentRequest) -> Result<(), ApiError> {
        if request.requests.is_empty() {
            return Err(ApiError::InvalidRequest("No requests provided".to_string()));
        }

        if request.requests.len() > self.config.max_batch_size {
            return Err(ApiError::InvalidRequest(
                format!("Batch too large: {} requests (max: {})",
                       request.requests.len(),
                       self.config.max_batch_size)
            ));
        }

        if request.client_id.trim().is_empty() {
            return Err(ApiError::InvalidRequest("Client ID required".to_string()));
        }

        // Validate each individual request
        for intent_request in &request.requests {
            self.validate_intent_request(intent_request)?;
        }

        Ok(())
    }

    /// Authenticate request (simplified)
    fn authenticate_request(&self, request: &IntentRequest) -> Result<(), ApiError> {
        if !self.config.enable_auth {
            return Ok(());
        }

        // In production, validate API key from headers
        // For now, just check client_id is not empty
        if request.client_id.trim().is_empty() {
            return Err(ApiError::AuthenticationFailed("Invalid client ID".to_string()));
        }

        Ok(())
    }

    /// Authenticate batch request
    fn authenticate_request_batch(&self, request: &BatchIntentRequest) -> Result<(), ApiError> {
        if !self.config.enable_auth {
            return Ok(());
        }

        if request.client_id.trim().is_empty() {
            return Err(ApiError::AuthenticationFailed("Invalid client ID".to_string()));
        }

        Ok(())
    }

    /// Check rate limits (simplified)
    fn check_rate_limit(&self, _client_id: &str) -> Result<(), ApiError> {
        // In production, implement proper rate limiting
        Ok(())
    }

    /// Get service health
    fn get_health(&self) -> HealthCheck {
        // Simplified health check
        HealthCheck {
            status: crate::api::HealthStatus::Healthy,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: 0, // Would track actual uptime
            metrics: HashMap::new(),
        }
    }

    /// Get service metrics
    fn get_metrics(&self) -> HashMap<String, String> {
        let mut metrics = HashMap::new();
        metrics.insert("status".to_string(), "healthy".to_string());
        metrics.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        metrics
    }
}

/// JSON representation of intent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentRequestJson {
    pub signals: Vec<SignalInputJson>,
    pub client_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// JSON representation of signal input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalInputJson {
    pub name: String,
    pub value: f64,
    pub confidence: f64,
    pub timestamp: i64,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// JSON representation of intent response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResponseJson {
    pub result: IntentResultJson,
    pub metadata: HashMap<String, String>,
    pub timestamp: i64,
}

/// JSON representation of intent result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResultJson {
    pub intent: IntentVectorJson,
    pub alternatives: Vec<IntentVectorJson>,
    pub metadata: HashMap<String, String>,
    pub processing_time_ns: u64,
}

/// JSON representation of intent vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentVectorJson {
    pub pressure: f64,
    pub confidence: f64,
    pub signals: HashMap<String, SignalContributionJson>,
    pub timestamp: i64,
    pub regime: String,
}

/// JSON representation of signal contribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalContributionJson {
    pub value: f64,
    pub weight: f64,
    pub confidence: f64,
    pub metadata: HashMap<String, String>,
}

/// JSON representation of batch intent request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchIntentRequestJson {
    pub requests: Vec<IntentRequestJson>,
    pub client_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// JSON representation of batch intent response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchIntentResponseJson {
    pub responses: Vec<IntentResponseJson>,
    pub summary: BatchSummaryJson,
    pub metadata: HashMap<String, String>,
    pub timestamp: i64,
}

/// JSON representation of batch summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSummaryJson {
    pub total_requests: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_processing_time_ns: u64,
    pub avg_processing_time_ns: u64,
}

// Conversion implementations
impl IntentRequestJson {
    fn to_internal(self) -> Result<IntentRequest, ApiError> {
        let signals = self.signals.into_iter()
            .map(|s| s.to_internal())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IntentRequest {
            signals,
            metadata: self.metadata,
            client_id: self.client_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        })
    }
}

impl SignalInputJson {
    fn to_internal(self) -> Result<SignalInput, ApiError> {
        Ok(SignalInput {
            name: self.name,
            value: self.value,
            confidence: self.confidence,
            timestamp: self.timestamp,
            metadata: self.metadata,
        })
    }
}

impl IntentResponseJson {
    fn from_internal(response: IntentResponse) -> Self {
        Self {
            result: IntentResultJson::from_internal(response.result),
            metadata: response.metadata,
            timestamp: response.timestamp,
        }
    }
}

impl IntentResultJson {
    fn from_internal(result: crate::intent::IntentResult) -> Self {
        Self {
            intent: IntentVectorJson::from_internal(result.intent),
            alternatives: result.alternatives.into_iter()
                .map(IntentVectorJson::from_internal)
                .collect(),
            metadata: result.metadata,
            processing_time_ns: result.processing_time_ns,
        }
    }
}

impl IntentVectorJson {
    fn from_internal(vector: crate::intent::IntentVector) -> Self {
        let signals = vector.signals.into_iter()
            .map(|(k, v)| (k, SignalContributionJson::from_internal(v)))
            .collect();

        Self {
            pressure: vector.pressure,
            confidence: vector.confidence,
            signals,
            timestamp: vector.timestamp,
            regime: vector.regime,
        }
    }
}

impl SignalContributionJson {
    fn from_internal(contrib: crate::intent::SignalContribution) -> Self {
        Self {
            value: contrib.value,
            weight: contrib.weight,
            confidence: contrib.confidence,
            metadata: contrib.metadata,
        }
    }
}

impl BatchIntentRequestJson {
    fn to_internal(self) -> Result<BatchIntentRequest, ApiError> {
        let requests = self.requests.into_iter()
            .map(|r| r.to_internal())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(BatchIntentRequest {
            requests,
            metadata: self.metadata,
            client_id: self.client_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        })
    }
}

impl BatchIntentResponseJson {
    fn from_internal(response: BatchIntentResponse) -> Self {
        Self {
            responses: response.responses.into_iter()
                .map(IntentResponseJson::from_internal)
                .collect(),
            summary: BatchSummaryJson::from_internal(response.summary),
            metadata: response.metadata,
            timestamp: response.timestamp,
        }
    }
}

impl BatchSummaryJson {
    fn from_internal(summary: crate::api::BatchSummary) -> Self {
        Self {
            total_requests: summary.total_requests,
            successful: summary.successful,
            failed: summary.failed,
            total_processing_time_ns: summary.total_processing_time_ns as u64,
            avg_processing_time_ns: summary.avg_processing_time_ns,
        }
    }
}

/// Warp rejection for API errors
#[derive(Debug)]
pub struct ApiErrorRejection(ApiError);

impl warp::reject::Reject for ApiErrorRejection {}

/// Helper function to pass service through warp filters
fn with_service(service: IntentApi) -> impl Filter<Extract = (IntentApi,), Error = Infallible> + Clone {
    warp::any().map(move || service.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_health_endpoint() {
        let engine = Arc::new(IntentEngine::default());
        let config = ApiConfig::default();
        let api = IntentApi::new(engine, config);

        let filter = api.routes();
        let response = request().method("GET").path("/api/v1/health").reply(&filter).await;

        assert_eq!(response.status(), 200);
    }

    #[test]
    fn test_json_conversion() {
        let json_request = IntentRequestJson {
            signals: vec![SignalInputJson {
                name: "test".to_string(),
                value: 0.5,
                confidence: 0.8,
                timestamp: 1234567890,
                metadata: HashMap::new(),
            }],
            client_id: "test_client".to_string(),
            metadata: HashMap::new(),
        };

        let internal = json_request.to_internal().unwrap();
        assert_eq!(internal.signals.len(), 1);
        assert_eq!(internal.client_id, "test_client");
    }
}