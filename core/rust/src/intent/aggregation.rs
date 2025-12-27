//! Signal Aggregation Module
//!
//! Implements deterministic aggregation of multiple signals into unified pressure readings.
//! This module ensures reproducible results across different environments and time.
//!
//! # Aggregation Strategy
//!
//! Signals are aggregated using weighted averaging with confidence-based weighting:
//!
//! ```text
//! aggregated_pressure = Σ(signal_value * signal_weight * signal_confidence) / Σ(signal_weight * signal_confidence)
//! aggregated_confidence = min(1.0, Σ(signal_confidence * signal_weight) / Σ(signal_weight))
//! ```
//!
//! # Determinism Guarantees
//!
//! - All operations are pure functions
//! - No floating-point operations that could vary by platform
//! - Consistent ordering through sorted signal names
//! - No external dependencies or random number generation

use std::collections::HashMap;
use super::{SignalInput, SignalContribution, IntentError, IntentConfig};

/// Result of signal aggregation
#[derive(Debug, Clone, PartialEq)]
pub struct AggregationResult {
    /// Aggregated pressure value (-1.0 to 1.0)
    pub pressure: f64,

    /// Aggregated confidence (0.0 to 1.0)
    pub confidence: f64,

    /// Individual signal contributions
    pub contributions: HashMap<String, SignalContribution>,

    /// Aggregation metadata
    pub metadata: HashMap<String, String>,
}

/// Signal aggregator for deterministic pressure computation
#[derive(Debug, Clone)]
pub struct SignalAggregator {
    config: IntentConfig,
}

impl SignalAggregator {
    /// Create a new signal aggregator with the given configuration
    pub fn new(config: IntentConfig) -> Self {
        Self { config }
    }

    /// Create a new aggregator with default configuration
    pub fn default() -> Self {
        Self::new(IntentConfig::default())
    }

    /// Aggregate multiple signals into a unified pressure reading
    ///
    /// # Arguments
    /// * `signals` - Vector of signal inputs to aggregate
    ///
    /// # Returns
    /// Result containing aggregated pressure and confidence, or an error
    ///
    /// # Determinism
    /// Results are deterministic given the same input signals (order doesn't matter)
    pub fn aggregate(&self, signals: Vec<SignalInput>) -> Result<AggregationResult, IntentError> {
        if signals.is_empty() {
            return Err(IntentError::NoSignalsAvailable);
        }

        // Validate all signals
        for signal in &signals {
            self.validate_signal(signal)?;
        }

        // Sort signals by name for deterministic processing
        let mut sorted_signals = signals;
        sorted_signals.sort_by(|a, b| a.name.cmp(&b.name));

        // Compute weighted aggregation
        let (pressure, confidence, contributions) = self.compute_weighted_aggregation(&sorted_signals);

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("signal_count".to_string(), sorted_signals.len().to_string());
        metadata.insert("aggregation_method".to_string(), "weighted_average".to_string());
        metadata.insert("deterministic".to_string(), "true".to_string());

        Ok(AggregationResult {
            pressure,
            confidence,
            contributions,
            metadata,
        })
    }

    /// Validate a single signal input
    fn validate_signal(&self, signal: &SignalInput) -> Result<(), IntentError> {
        // Check signal value range
        if !(-1.0..=1.0).contains(&signal.value) {
            return Err(IntentError::InvalidSignalData(
                format!("Signal '{}' value {} is outside valid range [-1.0, 1.0]", signal.name, signal.value)
            ));
        }

        // Check confidence range
        if !(0.0..=1.0).contains(&signal.confidence) {
            return Err(IntentError::InvalidSignalData(
                format!("Signal '{}' confidence {} is outside valid range [0.0, 1.0]", signal.name, signal.confidence)
            ));
        }

        // Check signal name is not empty
        if signal.name.trim().is_empty() {
            return Err(IntentError::InvalidSignalData(
                "Signal name cannot be empty".to_string()
            ));
        }

        Ok(())
    }

    /// Compute weighted aggregation of signals
    ///
    /// Returns (pressure, confidence, contributions)
    fn compute_weighted_aggregation(&self, signals: &[SignalInput]) -> (f64, f64, HashMap<String, SignalContribution>) {
        let mut total_weighted_pressure = 0.0;
        let mut total_weight = 0.0;
        let mut total_confidence_weight = 0.0;
        let mut contributions = HashMap::new();

        for signal in signals {
            // Get signal weight (use configured weight or default)
            let base_weight = self.config.signal_weights
                .get(&signal.name)
                .copied()
                .unwrap_or(self.config.default_signal_weight);

            // Effective weight = base_weight * signal_confidence
            let effective_weight = base_weight * signal.confidence;

            // Accumulate weighted pressure
            total_weighted_pressure += signal.value * effective_weight;
            total_weight += effective_weight;

            // Accumulate confidence-weighted confidence
            total_confidence_weight += signal.confidence * base_weight;

            // Record contribution
            contributions.insert(signal.name.clone(), SignalContribution {
                value: signal.value,
                weight: effective_weight,
                confidence: signal.confidence,
                metadata: signal.metadata.clone(),
            });
        }

        // Compute final pressure (avoid division by zero)
        let pressure = if total_weight > 0.0 {
            total_weighted_pressure / total_weight
        } else {
            0.0
        };

        // Compute final confidence
        let total_base_weight: f64 = signals.iter()
            .map(|s| self.config.signal_weights
                 .get(&s.name)
                 .copied()
                 .unwrap_or(self.config.default_signal_weight))
            .sum();

        let confidence = if total_base_weight > 0.0 {
            (total_confidence_weight / total_base_weight).min(1.0)
        } else {
            0.0
        };

        (pressure, confidence, contributions)
    }

    /// Get the current configuration
    pub fn config(&self) -> &IntentConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: IntentConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::IntentConfig;

    fn create_test_signal(name: &str, value: f64, confidence: f64) -> SignalInput {
        SignalInput {
            name: name.to_string(),
            value,
            confidence,
            timestamp: 1234567890,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_empty_signals_error() {
        let aggregator = SignalAggregator::default();
        let result = aggregator.aggregate(vec![]);
        assert!(matches!(result, Err(IntentError::NoSignalsAvailable)));
    }

    #[test]
    fn test_single_signal_aggregation() {
        let aggregator = SignalAggregator::default();
        let signal = create_test_signal("test_signal", 0.5, 0.8);

        let result = aggregator.aggregate(vec![signal]).unwrap();

        assert_eq!(result.pressure, 0.5);
        assert_eq!(result.confidence, 0.8);
        assert_eq!(result.contributions.len(), 1);
        assert_eq!(result.contributions["test_signal"].value, 0.5);
    }

    #[test]
    fn test_multiple_signals_equal_weight() {
        let aggregator = SignalAggregator::default();
        let signals = vec![
            create_test_signal("signal1", 0.6, 0.9),
            create_test_signal("signal2", 0.4, 0.7),
        ];

        let result = aggregator.aggregate(signals).unwrap();

        // Expected pressure: (0.6*0.5*0.9 + 0.4*0.5*0.7) / (0.5*0.9 + 0.5*0.7) = (0.27 + 0.14) / (0.45 + 0.35) = 0.41 / 0.8 = 0.5125
        assert!((result.pressure - 0.5125).abs() < 1e-10);

        // Expected confidence: min(1.0, (0.9*0.5 + 0.7*0.5) / (0.5 + 0.5)) = min(1.0, 0.8 / 1.0) = 0.8
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_weighted_signals() {
        let mut config = IntentConfig::default();
        config.signal_weights.insert("important".to_string(), 1.0);
        config.signal_weights.insert("normal".to_string(), 0.5);

        let aggregator = SignalAggregator::new(config);
        let signals = vec![
            create_test_signal("important", 0.6, 0.9),
            create_test_signal("normal", 0.4, 0.7),
        ];

        let result = aggregator.aggregate(signals).unwrap();

        // Expected pressure: (0.6*1.0*0.9 + 0.4*0.5*0.7) / (1.0*0.9 + 0.5*0.7) = (0.54 + 0.14) / (0.9 + 0.35) = 0.68 / 1.25 = 0.544
        assert!((result.pressure - 0.544).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_signal_value() {
        let aggregator = SignalAggregator::default();
        let signal = SignalInput {
            name: "invalid".to_string(),
            value: 1.5, // Invalid: outside [-1.0, 1.0]
            confidence: 0.8,
            timestamp: 1234567890,
            metadata: HashMap::new(),
        };

        let result = aggregator.aggregate(vec![signal]);
        assert!(matches!(result, Err(IntentError::InvalidSignalData(_))));
    }

    #[test]
    fn test_invalid_signal_confidence() {
        let aggregator = SignalAggregator::default();
        let signal = SignalInput {
            name: "invalid".to_string(),
            value: 0.5,
            confidence: 1.2, // Invalid: outside [0.0, 1.0]
            timestamp: 1234567890,
            metadata: HashMap::new(),
        };

        let result = aggregator.aggregate(vec![signal]);
        assert!(matches!(result, Err(IntentError::InvalidSignalData(_))));
    }

    #[test]
    fn test_deterministic_ordering() {
        let aggregator = SignalAggregator::default();
        let signals1 = vec![
            create_test_signal("z_signal", 0.6, 0.9),
            create_test_signal("a_signal", 0.4, 0.7),
        ];
        let signals2 = vec![
            create_test_signal("a_signal", 0.4, 0.7),
            create_test_signal("z_signal", 0.6, 0.9),
        ];

        let result1 = aggregator.aggregate(signals1).unwrap();
        let result2 = aggregator.aggregate(signals2).unwrap();

        assert_eq!(result1.pressure, result2.pressure);
        assert_eq!(result1.confidence, result2.confidence);
    }
}