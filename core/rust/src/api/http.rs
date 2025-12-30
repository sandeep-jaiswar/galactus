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

use crate::api::{
    ApiConfig, ApiError, BatchIntentRequest, BatchIntentResponse, HealthCheck, IntentRequest,
    IntentResponse,
};
use crate::features::FeatureRegistry;
use crate::intent::{IntentEngine, SignalInput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

/// HTTP Intent API implementation
#[derive(Clone)]
pub struct IntentApi {
    /// Feature registry for signal computation
    registry: Arc<FeatureRegistry>,

    /// Intent engine for computation
    engine: Arc<IntentEngine>,

    /// Service configuration
    config: ApiConfig,
}

impl IntentApi {
    /// Create a new HTTP intent API
    pub fn new(
        registry: Arc<FeatureRegistry>,
        engine: Arc<IntentEngine>,
        config: ApiConfig,
    ) -> Self {
        Self {
            registry,
            engine,
            config,
        }
    }

    /// Create warp filters for the API
    pub fn routes(
        &self,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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
        let intent_request = request
            .to_internal()
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
        let batch_request = request
            .to_internal()
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

        // If no market_data provided, try to backfill from research HTTP adapter
        let mut market_data = request.market_data;
        if market_data.is_empty() {
            if let Ok(base_url) = std::env::var("RESEARCH_PROVIDER_URL") {
                // Use context 'symbols' if present (comma-separated), otherwise default to NIFTY
                let symbols: Vec<String> = request
                    .context
                    .get("symbols")
                    .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
                    .unwrap_or_else(|| vec!["NIFTY".to_string()]);

                let client = crate::data::research_client::ResearchClient::new(&base_url);
                for sym in symbols {
                    match client.fetch_historical(&sym, 1) {
                        Ok(bars) => {
                            if let Some(last) = bars.last() {
                                let now_ts = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs() as i64;

                                market_data.insert(
                                    sym.clone(),
                                    crate::features::MarketDataPoint {
                                        symbol: sym.clone(),
                                        price: last.close,
                                        volume: last.volume,
                                        timestamp: now_ts,
                                        metadata: {
                                            let mut m = std::collections::HashMap::new();
                                            m.insert(
                                                "data_source".to_string(),
                                                "research_http".to_string(),
                                            );
                                            m.insert("research_url".to_string(), base_url.clone());
                                            m
                                        },
                                    },
                                );
                            }
                        }
                        Err(_) => {
                            // ignore fetch errors and proceed
                        }
                    }
                }
            }
        }

        // Create feature inputs from request data (with possible backfilled market_data)
        let feature_inputs = crate::features::FeatureInputs {
            market_data,
            options_data: request.options_data,
            futures_data: request.futures_data,
            context: request.context,
            timestamp: request.timestamp,
        };

        // Compute all registered features
        let feature_names = self.registry.list_features();
        let batch_result = self.registry.compute_batch(&feature_names, &feature_inputs);

        // Convert feature results to signal inputs
        let mut signals = Vec::new();
        for (name, result) in &batch_result.results {
            signals.push(self.feature_result_to_signal_input(name.clone(), result.clone()));
        }

        // Log any feature computation errors but continue
        if !batch_result.errors.is_empty() {
            // In production, this should be logged properly
            eprintln!("Feature computation errors: {:?}", batch_result.errors);
        }

        // Process with intent engine
        let result = self.engine.process(signals)?;

        // Create response
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let mut metadata = HashMap::new();
        metadata.insert(
            "server_version".to_string(),
            env!("CARGO_PKG_VERSION").to_string(),
        );
        metadata.insert(
            "features_computed".to_string(),
            batch_result.results.len().to_string(),
        );
        metadata.insert(
            "feature_errors".to_string(),
            batch_result.errors.len().to_string(),
        );

        Ok(IntentResponse {
            result,
            metadata,
            timestamp,
        })
    }

    /// Convert feature result to signal input
    fn feature_result_to_signal_input(
        &self,
        name: String,
        result: crate::features::FeatureResult,
    ) -> SignalInput {
        SignalInput {
            name,
            value: result.value,
            confidence: result.confidence,
            timestamp: result.timestamp,
            metadata: result.metadata,
        }
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
        let total_requests = request.requests.len();
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
            total_requests,
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
        // Validate that we have some market data to work with
        if request.market_data.is_empty()
            && request.options_data.is_empty()
            && request.futures_data.is_empty()
        {
            return Err(ApiError::InvalidRequest(
                "No market data provided".to_string(),
            ));
        }

        // Check size limits (simplified - in production would check total data size)
        let total_data_points =
            request.market_data.len() + request.options_data.len() + request.futures_data.len();
        if total_data_points > 1000 {
            // Arbitrary limit
            return Err(ApiError::InvalidRequest(format!(
                "Too much data: {} data points (max: 1000)",
                total_data_points
            )));
        }

        if request.client_id.trim().is_empty() {
            return Err(ApiError::InvalidRequest("Client ID required".to_string()));
        }

        // Basic validation of market data
        for (symbol, data) in &request.market_data {
            if symbol.trim().is_empty() {
                return Err(ApiError::InvalidRequest(
                    "Empty symbol in market data".to_string(),
                ));
            }
            if data.price <= 0.0 {
                return Err(ApiError::InvalidRequest(format!(
                    "Invalid price {} for symbol {}",
                    data.price, symbol
                )));
            }
        }

        // Basic validation of options data
        for (underlying, chain) in &request.options_data {
            if underlying.trim().is_empty() {
                return Err(ApiError::InvalidRequest(
                    "Empty underlying in options data".to_string(),
                ));
            }
            if chain.strikes.is_empty() {
                return Err(ApiError::InvalidRequest(format!(
                    "No strikes provided for options chain {}",
                    underlying
                )));
            }
        }

        // Basic validation of futures data
        for (symbol, data) in &request.futures_data {
            if symbol.trim().is_empty() {
                return Err(ApiError::InvalidRequest(
                    "Empty symbol in futures data".to_string(),
                ));
            }
            if data.price <= 0.0 {
                return Err(ApiError::InvalidRequest(format!(
                    "Invalid price {} for futures symbol {}",
                    data.price, symbol
                )));
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
            return Err(ApiError::InvalidRequest(format!(
                "Batch too large: {} requests (max: {})",
                request.requests.len(),
                self.config.max_batch_size
            )));
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
            return Err(ApiError::AuthenticationFailed(
                "Invalid client ID".to_string(),
            ));
        }

        Ok(())
    }

    /// Authenticate batch request
    fn authenticate_request_batch(&self, request: &BatchIntentRequest) -> Result<(), ApiError> {
        if !self.config.enable_auth {
            return Ok(());
        }

        if request.client_id.trim().is_empty() {
            return Err(ApiError::AuthenticationFailed(
                "Invalid client ID".to_string(),
            ));
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
    pub market_data: HashMap<String, MarketDataPointJson>,
    pub options_data: HashMap<String, OptionChainJson>,
    pub futures_data: HashMap<String, FuturesDataJson>,
    #[serde(default)]
    pub context: HashMap<String, String>,
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

/// JSON representation of market data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataPointJson {
    pub symbol: String,
    pub price: f64,
    pub volume: u64,
    pub timestamp: i64,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// JSON representation of option chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChainJson {
    pub underlying: String,
    pub expiry: i64,
    pub strikes: Vec<StrikeDataJson>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// JSON representation of strike data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrikeDataJson {
    pub strike: f64,
    pub call_bid: f64,
    pub call_ask: f64,
    pub put_bid: f64,
    pub put_ask: f64,
    pub open_interest: u64,
    pub volume: u64,
}

/// JSON representation of futures data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuturesDataJson {
    pub symbol: String,
    pub price: f64,
    pub open_interest: u64,
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
    #[allow(clippy::wrong_self_convention)]
    fn to_internal(self) -> Result<IntentRequest, ApiError> {
        let market_data = self
            .market_data
            .into_iter()
            .map(|(k, v)| (k, v.to_internal()))
            .collect();

        let options_data = self
            .options_data
            .into_iter()
            .map(|(k, v)| (k, v.to_internal()))
            .collect();

        let futures_data = self
            .futures_data
            .into_iter()
            .map(|(k, v)| (k, v.to_internal()))
            .collect();

        Ok(IntentRequest {
            market_data,
            options_data,
            futures_data,
            context: self.context,
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
    #[allow(dead_code)]
    #[allow(clippy::wrong_self_convention)]
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

impl MarketDataPointJson {
    #[allow(clippy::wrong_self_convention)]
    fn to_internal(self) -> crate::features::MarketDataPoint {
        crate::features::MarketDataPoint {
            symbol: self.symbol,
            price: self.price,
            volume: self.volume,
            timestamp: self.timestamp,
            metadata: self.metadata,
        }
    }
}

impl OptionChainJson {
    #[allow(clippy::wrong_self_convention)]
    fn to_internal(self) -> crate::features::OptionChain {
        crate::features::OptionChain {
            underlying: self.underlying,
            expiry: self.expiry,
            strikes: self.strikes.into_iter().map(|s| s.to_internal()).collect(),
            metadata: self.metadata,
        }
    }
}

impl StrikeDataJson {
    #[allow(clippy::wrong_self_convention)]
    fn to_internal(self) -> crate::features::StrikeData {
        crate::features::StrikeData {
            strike: self.strike,
            call_bid: self.call_bid,
            call_ask: self.call_ask,
            put_bid: self.put_bid,
            put_ask: self.put_ask,
            open_interest: self.open_interest,
            volume: self.volume,
        }
    }
}

impl FuturesDataJson {
    #[allow(clippy::wrong_self_convention)]
    fn to_internal(self) -> crate::features::FuturesData {
        crate::features::FuturesData {
            symbol: self.symbol,
            price: self.price,
            open_interest: self.open_interest,
            timestamp: self.timestamp,
            metadata: self.metadata,
        }
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
            alternatives: result
                .alternatives
                .into_iter()
                .map(IntentVectorJson::from_internal)
                .collect(),
            metadata: result.metadata,
            processing_time_ns: result.processing_time_ns,
        }
    }
}

impl IntentVectorJson {
    fn from_internal(vector: crate::intent::IntentVector) -> Self {
        let signals = vector
            .signals
            .into_iter()
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
    #[allow(clippy::wrong_self_convention)]
    fn to_internal(self) -> Result<BatchIntentRequest, ApiError> {
        let requests = self
            .requests
            .into_iter()
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
            responses: response
                .responses
                .into_iter()
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
#[allow(dead_code)]
pub struct ApiErrorRejection(ApiError);

impl warp::reject::Reject for ApiErrorRejection {}

/// Helper function to pass service through warp filters
fn with_service(
    service: IntentApi,
) -> impl Filter<Extract = (IntentApi,), Error = Infallible> + Clone {
    warp::any().map(move || service.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::create_promoted_feature_registry;
    use warp::test::request;

    #[tokio::test]
    async fn test_health_endpoint() {
        let registry = Arc::new(create_promoted_feature_registry().unwrap());
        let engine = Arc::new(IntentEngine::default());
        let config = ApiConfig::default();
        let api = IntentApi::new(registry, engine, config);

        let filter = api.routes();
        let response = request()
            .method("GET")
            .path("/api/v1/health")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
    }

    #[test]
    fn test_json_conversion() {
        let json_request = IntentRequestJson {
            market_data: HashMap::new(),
            options_data: HashMap::new(),
            futures_data: HashMap::new(),
            context: HashMap::new(),
            client_id: "test_client".to_string(),
            metadata: HashMap::new(),
        };

        let internal = json_request.to_internal().unwrap();
        assert_eq!(internal.market_data.len(), 0);
        assert_eq!(internal.client_id, "test_client");
    }
}
