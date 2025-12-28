//! Intent Engine Core
//!
//! Main inference engine that combines signal aggregation with regime-aware processing
//! to produce final capital intent vectors with confidence scoring.
//!
//! # Inference Pipeline
//!
//! 1. **Signal Validation**: Ensure all inputs are valid and consistent
//! 2. **Regime Detection**: Classify current market regime
//! 3. **Signal Aggregation**: Combine signals with regime-adjusted weights
//! 4. **Intent Mapping**: Map aggregated pressure to intent vector
//! 5. **Confidence Calculation**: Compute overall confidence score
//! 6. **Alternative Generation**: Create alternative interpretations (optional)
//!
//! # Regime Awareness
//!
//! The engine adjusts signal weights and confidence calculations based on market regime:
//! - **Bull Market**: Emphasize momentum signals
//! - **Bear Market**: Emphasize defensive signals
//! - **Sideways**: Balance all signals equally
//! - **High Volatility**: Increase confidence thresholds
//!
//! # Determinism
//!
//! All computations are deterministic and reproducible across environments.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use super::{
    IntentVector, IntentResult, IntentError, IntentConfig,
    SignalInput
};
use super::aggregation::{SignalAggregator, AggregationResult};

/// Market regime classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarketRegime {
    /// Strong upward momentum
    Bull,
    /// Strong downward momentum
    Bear,
    /// Range-bound trading
    Sideways,
    /// High volatility environment
    Volatile,
    /// Unknown or mixed conditions
    Mixed,
}

impl std::fmt::Display for MarketRegime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketRegime::Bull => write!(f, "bull"),
            MarketRegime::Bear => write!(f, "bear"),
            MarketRegime::Sideways => write!(f, "sideways"),
            MarketRegime::Volatile => write!(f, "volatile"),
            MarketRegime::Mixed => write!(f, "mixed"),
        }
    }
}

/// Main intent inference engine
#[derive(Debug, Clone)]
pub struct IntentEngine {
    aggregator: SignalAggregator,
    config: IntentConfig,
}

impl IntentEngine {
    /// Create a new intent engine with the given configuration
    pub fn new(config: IntentConfig) -> Self {
        Self {
            aggregator: SignalAggregator::new(config.clone()),
            config,
        }
    }

    /// Create a new engine with default configuration
    pub fn default() -> Self {
        Self::new(IntentConfig::default())
    }

    /// Process signals to generate intent vector
    ///
    /// # Arguments
    /// * `signals` - Vector of signal inputs
    ///
    /// # Returns
    /// Intent result with primary vector and alternatives, or an error
    ///
    /// # Determinism
    /// Results are deterministic given the same inputs
    pub fn process(&self, signals: Vec<SignalInput>) -> Result<IntentResult, IntentError> {
        let start_time = SystemTime::now();

        // Validate inputs
        if signals.is_empty() {
            return Err(IntentError::NoSignalsAvailable);
        }

        // Detect market regime
        let regime = self.detect_regime(&signals)?;

        // Adjust configuration for regime
        let regime_config = self.adjust_config_for_regime(&self.config, &regime);

        // Create regime-adjusted aggregator
        let regime_aggregator = SignalAggregator::new(regime_config);

        // Aggregate signals
        let aggregation_result = regime_aggregator.aggregate(signals)?;

        // Map to intent vector
        let intent_vector = self.create_intent_vector(&aggregation_result, &regime);

        // Generate alternatives if enabled
        let alternatives = if self.config.enable_alternatives {
            self.generate_alternatives(&intent_vector, &aggregation_result)?
        } else {
            Vec::new()
        };

        // Create metadata
        let mut metadata = HashMap::new();
        metadata.insert("regime".to_string(), regime.to_string());
        metadata.insert("signal_count".to_string(), aggregation_result.contributions.len().to_string());
        metadata.insert("alternatives_generated".to_string(), alternatives.len().to_string());

        // Calculate processing time
        let processing_time_ns = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default()
            .as_nanos() as u64;

        Ok(IntentResult {
            intent: intent_vector,
            alternatives,
            metadata,
            processing_time_ns,
        })
    }

    /// Detect market regime from signal patterns
    fn detect_regime(&self, signals: &[SignalInput]) -> Result<MarketRegime, IntentError> {
        if signals.is_empty() {
            return Ok(MarketRegime::Mixed);
        }

        // Simple regime detection based on signal consensus
        // In production, this would use more sophisticated analysis
        let avg_pressure: f64 = signals.iter().map(|s| s.value).sum::<f64>() / signals.len() as f64;
        let pressure_std: f64 = {
            let variance = signals.iter()
                .map(|s| (s.value - avg_pressure).powi(2))
                .sum::<f64>() / signals.len() as f64;
            variance.sqrt()
        };

        // High consensus signals
        let consensus_signals: Vec<_> = signals.iter()
            .filter(|s| s.confidence > 0.8)
            .collect();

        if consensus_signals.len() >= 2 {
            let consensus_avg = consensus_signals.iter()
                .map(|s| s.value)
                .sum::<f64>() / consensus_signals.len() as f64;

            if consensus_avg > 0.3 {
                return Ok(MarketRegime::Bull);
            } else if consensus_avg < -0.3 {
                return Ok(MarketRegime::Bear);
            }
        }

        // High volatility detection
        if pressure_std > 0.5 {
            return Ok(MarketRegime::Volatile);
        }

        // Default to sideways/mixed
        if pressure_std < 0.2 {
            Ok(MarketRegime::Sideways)
        } else {
            Ok(MarketRegime::Mixed)
        }
    }

    /// Adjust configuration based on detected market regime
    fn adjust_config_for_regime(&self, base_config: &IntentConfig, regime: &MarketRegime) -> IntentConfig {
        let mut config = base_config.clone();

        match regime {
            MarketRegime::Bull => {
                // In bull markets, emphasize momentum signals
                config.signal_weights.insert("momentum".to_string(), 1.2);
                config.min_confidence_threshold = 0.6; // Lower threshold
            }
            MarketRegime::Bear => {
                // In bear markets, emphasize defensive signals
                config.signal_weights.insert("defensive".to_string(), 1.2);
                config.min_confidence_threshold = 0.6;
            }
            MarketRegime::Volatile => {
                // In volatile markets, increase confidence requirements
                config.min_confidence_threshold = 0.8;
                config.default_signal_weight = 0.3; // Reduce default weight
            }
            MarketRegime::Sideways => {
                // In sideways markets, balance all signals
                // Keep default weights
            }
            MarketRegime::Mixed => {
                // In mixed conditions, slight increase in confidence threshold
                config.min_confidence_threshold = 0.75;
            }
        }

        config
    }

    /// Create intent vector from aggregation result
    fn create_intent_vector(&self, aggregation: &AggregationResult, regime: &MarketRegime) -> IntentVector {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        // Apply final confidence adjustments based on regime
        let final_confidence = self.adjust_confidence_for_regime(aggregation.confidence, &regime);

        IntentVector {
            pressure: aggregation.pressure,
            confidence: final_confidence,
            signals: aggregation.contributions.clone(),
            timestamp,
            regime: regime.to_string(),
        }
    }

    /// Adjust confidence based on market regime
    fn adjust_confidence_for_regime(&self, base_confidence: f64, regime: &MarketRegime) -> f64 {
        match regime {
            MarketRegime::Volatile => {
                // Reduce confidence in volatile markets
                (base_confidence * 0.8).min(1.0)
            }
            MarketRegime::Mixed => {
                // Slightly reduce confidence in mixed conditions
                (base_confidence * 0.9).min(1.0)
            }
            _ => base_confidence,
        }
    }

    /// Generate alternative intent interpretations
    fn generate_alternatives(&self, primary: &IntentVector, aggregation: &AggregationResult) -> Result<Vec<IntentVector>, IntentError> {
        let mut alternatives = Vec::new();

        // Alternative 1: Conservative interpretation (reduce pressure magnitude)
        if primary.pressure.abs() > 0.1 {
            let mut alt1 = primary.clone();
            alt1.pressure *= 0.7; // Reduce by 30%
            alt1.confidence *= 0.9; // Slightly reduce confidence
            alternatives.push(alt1);
        }

        // Alternative 2: Aggressive interpretation (increase pressure magnitude)
        if primary.pressure.abs() < 0.8 {
            let mut alt2 = primary.clone();
            alt2.pressure *= 1.2; // Increase by 20%
            alt2.confidence *= 0.85; // Reduce confidence due to aggressiveness
            alternatives.push(alt2);
        }

        // Alternative 3: Equal weight interpretation (ignore configured weights)
        if aggregation.contributions.len() > 1 {
            let equal_weight_config = {
                let mut config = self.config.clone();
                config.signal_weights.clear();
                config.default_signal_weight = 1.0;
                config
            };

            let equal_aggregator = SignalAggregator::new(equal_weight_config);
            let signals: Vec<SignalInput> = aggregation.contributions.iter()
                .map(|(name, contrib)| SignalInput {
                    name: name.clone(),
                    value: contrib.value,
                    confidence: contrib.confidence,
                    timestamp: primary.timestamp,
                    metadata: contrib.metadata.clone(),
                })
                .collect();

            if let Ok(equal_aggregation) = equal_aggregator.aggregate(signals) {
                let mut alt3 = self.create_intent_vector(&equal_aggregation, &MarketRegime::Mixed);
                alt3.regime = "equal_weight".to_string();
                alternatives.push(alt3);
            }
        }

        // Limit to max_alternatives
        alternatives.truncate(self.config.max_alternatives);

        Ok(alternatives)
    }

    /// Get the current configuration
    pub fn config(&self) -> &IntentConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: IntentConfig) {
        self.config = config.clone();
        self.aggregator = SignalAggregator::new(config);
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
    fn test_regime_detection_bull() {
        let engine = IntentEngine::default();
        let signals = vec![
            create_test_signal("momentum", 0.8, 0.9),
            create_test_signal("volume", 0.6, 0.85),
        ];

        let regime = engine.detect_regime(&signals).unwrap();
        assert_eq!(regime, MarketRegime::Bull);
    }

    #[test]
    fn test_regime_detection_bear() {
        let engine = IntentEngine::default();
        let signals = vec![
            create_test_signal("momentum", -0.7, 0.9),
            create_test_signal("volume", -0.5, 0.85),
        ];

        let regime = engine.detect_regime(&signals).unwrap();
        assert_eq!(regime, MarketRegime::Bear);
    }

    #[test]
    fn test_regime_detection_volatile() {
        let engine = IntentEngine::default();
        let signals = vec![
            create_test_signal("signal1", 0.8, 0.9),
            create_test_signal("signal2", -0.6, 0.85),
        ];

        let regime = engine.detect_regime(&signals).unwrap();
        assert_eq!(regime, MarketRegime::Volatile);
    }

    #[test]
    fn test_intent_processing() {
        let engine = IntentEngine::default();
        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.8),
            create_test_signal("hedge_pressure", 0.2, 0.7),
        ];

        let result = engine.process(signals).unwrap();

        assert!(result.intent.pressure >= -1.0 && result.intent.pressure <= 1.0);
        assert!(result.intent.confidence >= 0.0 && result.intent.confidence <= 1.0);
        assert!(!result.intent.regime.is_empty());
        assert!(result.processing_time_ns > 0);
    }

    #[test]
    fn test_alternatives_generation() {
        let mut config = IntentConfig::default();
        config.enable_alternatives = true;
        config.max_alternatives = 2;

        let engine = IntentEngine::new(config);
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
            create_test_signal("signal2", 0.3, 0.9),
        ];

        let result = engine.process(signals).unwrap();
        assert!(!result.alternatives.is_empty());
        assert!(result.alternatives.len() <= 2);
    }

    #[test]
    fn test_empty_signals_error() {
        let engine = IntentEngine::default();
        let result = engine.process(vec![]);
        assert!(matches!(result, Err(IntentError::NoSignalsAvailable)));
    }

    #[test]
    fn test_deterministic_processing() {
        let engine = IntentEngine::default();
        let signals1 = vec![
            create_test_signal("z_signal", 0.5, 0.8),
            create_test_signal("a_signal", 0.3, 0.9),
        ];
        let signals2 = vec![
            create_test_signal("a_signal", 0.3, 0.9),
            create_test_signal("z_signal", 0.5, 0.8),
        ];

        let result1 = engine.process(signals1).unwrap();
        let result2 = engine.process(signals2).unwrap();

        assert_eq!(result1.intent.pressure, result2.intent.pressure);
        assert_eq!(result1.intent.confidence, result2.intent.confidence);
        assert_eq!(result1.intent.regime, result2.intent.regime);
    }
}