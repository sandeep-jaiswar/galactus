//! Feature Computation Framework
//!
//! Plugin architecture for computing promoted signals in the production Rust core.
//! All features implemented here must have passed the promotion checklist.
//!
//! # Architecture
//!
//! The feature framework provides:
//! 1. **Feature Registry**: Manages available feature implementations
//! 2. **Plugin Interface**: Standard interface for feature computation
//! 3. **Deterministic Execution**: Reproducible results across environments
//! 4. **Error Handling**: Comprehensive error reporting and recovery
//!
//! # Safety & Determinism
//!
//! - All computations must be deterministic (same inputs → same outputs)
//! - No external state or side effects
//! - Memory safe (Rust guarantees)
//! - Type safe (compile-time verification)
//!
//! # Feature Lifecycle
//!
//! 1. **Research**: Signal developed and validated in Python
//! 2. **Promotion**: Passes promotion checklist and gets stakeholder approval
//! 3. **Implementation**: Rust implementation added to this module
//! 4. **Integration**: Feature registered and available to intent engine

pub mod registry;

// Re-export main types for convenience
pub use registry::{FeatureRegistry, FeatureResult, FeatureError};

// Core types for feature computation
use std::collections::HashMap;

/// Represents the result of a feature computation
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureResult {
    /// Feature name (must match promoted signal name)
    pub name: String,

    /// Feature value (-1.0 to 1.0)
    /// Interpretation depends on feature type (pressure, momentum, etc.)
    pub value: f64,

    /// Confidence in the computation (0.0 to 1.0)
    pub confidence: f64,

    /// Timestamp of computation
    pub timestamp: i64,

    /// Computation metadata
    pub metadata: HashMap<String, String>,

    /// Processing time in nanoseconds
    pub processing_time_ns: u64,
}

/// Errors that can occur during feature computation
#[derive(Debug, Clone, PartialEq)]
pub enum FeatureError {
    /// Feature not found in registry
    FeatureNotFound(String),

    /// Invalid input data for feature computation
    InvalidInput(String),

    /// Computation failed due to data issues
    ComputationFailure(String),

    /// Configuration error
    ConfigurationError(String),

    /// Resource exhaustion (memory, etc.)
    ResourceExhaustion(String),
}

impl std::fmt::Display for FeatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureError::FeatureNotFound(name) => write!(f, "Feature '{}' not found", name),
            FeatureError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            FeatureError::ComputationFailure(msg) => write!(f, "Computation failure: {}", msg),
            FeatureError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            FeatureError::ResourceExhaustion(msg) => write!(f, "Resource exhaustion: {}", msg),
        }
    }
}

impl std::error::Error for FeatureError {}

/// Trait for feature computation implementations
///
/// All promoted features must implement this trait to be usable
/// in the production inference engine.
pub trait Feature: Send + Sync {
    /// Get the feature name (must match promotion checklist)
    fn name(&self) -> &str;

    /// Get feature description and metadata
    fn description(&self) -> &str;

    /// Get the version of this feature implementation
    fn version(&self) -> &str;

    /// Compute the feature value from input data
    ///
    /// # Arguments
    /// * `inputs` - Input data required for computation
    ///
    /// # Returns
    /// Feature result or error
    ///
    /// # Determinism
    /// Must produce identical results for identical inputs
    fn compute(&self, inputs: &FeatureInputs) -> Result<FeatureResult, FeatureError>;
}

/// Input data structure for feature computation
///
/// This structure provides all data that features might need.
/// Features should only access data they actually require.
#[derive(Debug, Clone)]
pub struct FeatureInputs {
    /// Market data (prices, volumes, etc.)
    pub market_data: HashMap<String, MarketDataPoint>,

    /// Option chain data (strikes, expiries, etc.)
    pub options_data: HashMap<String, OptionChain>,

    /// Futures data for basis calculations
    pub futures_data: HashMap<String, FuturesData>,

    /// Additional context data
    pub context: HashMap<String, String>,

    /// Computation timestamp
    pub timestamp: i64,
}

/// Single market data point
#[derive(Debug, Clone)]
pub struct MarketDataPoint {
    pub symbol: String,
    pub price: f64,
    pub volume: u64,
    pub timestamp: i64,
    pub metadata: HashMap<String, String>,
}

/// Option chain data
#[derive(Debug, Clone)]
pub struct OptionChain {
    pub underlying: String,
    pub expiry: i64,
    pub strikes: Vec<StrikeData>,
    pub metadata: HashMap<String, String>,
}

/// Data for a specific strike
#[derive(Debug, Clone)]
pub struct StrikeData {
    pub strike: f64,
    pub call_bid: f64,
    pub call_ask: f64,
    pub put_bid: f64,
    pub put_ask: f64,
    pub open_interest: u64,
    pub volume: u64,
}

/// Futures contract data
#[derive(Debug, Clone)]
pub struct FuturesData {
    pub symbol: String,
    pub price: f64,
    pub open_interest: u64,
    pub timestamp: i64,
    pub metadata: HashMap<String, String>,
}

/// Configuration for feature computation
#[derive(Debug, Clone)]
pub struct FeatureConfig {
    /// Maximum processing time per feature (nanoseconds)
    pub max_processing_time_ns: u64,

    /// Enable detailed logging
    pub enable_logging: bool,

    /// Fail fast on first error
    pub fail_fast: bool,

    /// Feature-specific configurations
    pub feature_configs: HashMap<String, HashMap<String, String>>,
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            max_processing_time_ns: 1_000_000, // 1ms
            enable_logging: false,
            fail_fast: true,
            feature_configs: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_result_creation() {
        let result = FeatureResult {
            name: "test_feature".to_string(),
            value: 0.5,
            confidence: 0.8,
            timestamp: 1234567890,
            metadata: HashMap::new(),
            processing_time_ns: 50000,
        };

        assert_eq!(result.name, "test_feature");
        assert_eq!(result.value, 0.5);
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_feature_config_defaults() {
        let config = FeatureConfig::default();
        assert_eq!(config.max_processing_time_ns, 1_000_000);
        assert!(!config.enable_logging);
        assert!(config.fail_fast);
    }
}