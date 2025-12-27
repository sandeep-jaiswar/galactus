//! Intent Engine Module
//!
//! Core inference logic for Galactus capital pressure detection.
//! This module implements deterministic intent inference from promoted signals.
//!
//! # Architecture
//!
//! The intent engine processes signals through three main components:
//! 1. **Signal Aggregation**: Combines multiple signal inputs
//! 2. **Intent Inference**: Maps aggregated signals to capital intent
//! 3. **Confidence Scoring**: Provides reliability metrics
//!
//! # Determinism Requirements
//!
//! All computations must be:
//! - **Deterministic**: Same inputs → same outputs
//! - **Pure functions**: No external state or side effects
//! - **Reproducible**: Across environments and time
//!
//! # Safety
//!
//! This module contains the core business logic and must maintain:
//! - Memory safety (Rust guarantees)
//! - Type safety (compile-time verification)
//! - Business logic correctness (comprehensive testing)

pub mod engine;
pub mod aggregation;

#[cfg(feature = "examples")]
pub mod examples;

// Re-export main types for convenience
pub use engine::IntentEngine;
pub use aggregation::{SignalAggregator, AggregationResult};

// Core types for intent representation
use std::collections::HashMap;

/// Represents a capital intent vector
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IntentVector {
    /// Overall capital pressure (-1.0 to 1.0)
    /// Negative = selling pressure, Positive = buying pressure
    pub pressure: f64,

    /// Confidence in the pressure reading (0.0 to 1.0)
    pub confidence: f64,

    /// Component signals that contributed to this intent
    pub signals: HashMap<String, SignalContribution>,

    /// Timestamp of intent computation
    pub timestamp: i64,

    /// Market regime context
    pub regime: String,
}

/// Contribution of a single signal to the intent vector
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignalContribution {
    /// Signal value (-1.0 to 1.0)
    pub value: f64,

    /// Signal weight in final aggregation (0.0 to 1.0)
    pub weight: f64,

    /// Signal confidence (0.0 to 1.0)
    pub confidence: f64,

    /// Signal-specific metadata
    pub metadata: HashMap<String, String>,
}

/// Result of intent computation
#[derive(Debug, Clone, PartialEq)]
pub struct IntentResult {
    /// Primary intent vector
    pub intent: IntentVector,

    /// Alternative interpretations (if applicable)
    pub alternatives: Vec<IntentVector>,

    /// Computation metadata
    pub metadata: HashMap<String, String>,

    /// Processing time in nanoseconds
    pub processing_time_ns: u64,
}

/// Errors that can occur during intent processing
#[derive(Debug, Clone, PartialEq)]
pub enum IntentError {
    /// No signals available for processing
    NoSignalsAvailable,

    /// Signal data is invalid or corrupted
    InvalidSignalData(String),

    /// Aggregation failed due to inconsistent signal data
    AggregationFailure(String),

    /// Confidence calculation failed
    ConfidenceFailure(String),

    /// Regime detection failed
    RegimeFailure(String),
}

impl std::fmt::Display for IntentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntentError::NoSignalsAvailable => write!(f, "No signals available for intent computation"),
            IntentError::InvalidSignalData(msg) => write!(f, "Invalid signal data: {}", msg),
            IntentError::AggregationFailure(msg) => write!(f, "Aggregation failure: {}", msg),
            IntentError::ConfidenceFailure(msg) => write!(f, "Confidence calculation failure: {}", msg),
            IntentError::RegimeFailure(msg) => write!(f, "Regime detection failure: {}", msg),
        }
    }
}

impl std::error::Error for IntentError {}

/// Configuration for intent engine behavior
#[derive(Debug, Clone)]
pub struct IntentConfig {
    /// Minimum confidence threshold for valid intent (0.0 to 1.0)
    pub min_confidence_threshold: f64,

    /// Maximum number of alternative interpretations to generate
    pub max_alternatives: usize,

    /// Signal weights for aggregation (signal_name -> weight)
    pub signal_weights: HashMap<String, f64>,

    /// Default weight for unknown signals
    pub default_signal_weight: f64,

    /// Enable alternative intent generation
    pub enable_alternatives: bool,
}

impl Default for IntentConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.7,
            max_alternatives: 3,
            signal_weights: HashMap::new(),
            default_signal_weight: 0.5,
            enable_alternatives: true,
        }
    }
}

/// Signal input for intent processing
#[derive(Debug, Clone)]
pub struct SignalInput {
    /// Signal name (must match promoted signal names)
    pub name: String,

    /// Signal value (-1.0 to 1.0)
    pub value: f64,

    /// Signal confidence (0.0 to 1.0)
    pub confidence: f64,

    /// Signal timestamp (Unix timestamp)
    pub timestamp: i64,

    /// Additional signal metadata
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_vector_creation() {
        let intent = IntentVector {
            pressure: 0.5,
            confidence: 0.8,
            signals: HashMap::new(),
            timestamp: 1234567890,
            regime: "normal".to_string(),
        };

        assert_eq!(intent.pressure, 0.5);
        assert_eq!(intent.confidence, 0.8);
        assert_eq!(intent.regime, "normal");
    }

    #[test]
    fn test_intent_config_defaults() {
        let config = IntentConfig::default();
        assert_eq!(config.min_confidence_threshold, 0.7);
        assert_eq!(config.max_alternatives, 3);
        assert_eq!(config.default_signal_weight, 0.5);
        assert!(config.enable_alternatives);
    }
}