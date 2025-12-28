//! Intent Engine Usage Examples
//!
//! This module demonstrates how to use the intent engine for capital pressure inference.
//! These examples show the deterministic processing of promoted signals.

use crate::intent::{IntentConfig, IntentEngine, MarketRegime, SignalInput};
use std::collections::HashMap;

/// Example: Basic intent processing with OI decay and hedge pressure signals
pub fn example_basic_processing() -> Result<(), Box<dyn std::error::Error>> {
    // Create intent engine with default configuration
    let engine = IntentEngine::default();

    // Create signal inputs (simulating promoted signals)
    let signals = vec![
        SignalInput {
            name: "oi_decay".to_string(),
            value: -0.4, // Moderate selling pressure from OI decay
            confidence: 0.85,
            timestamp: 1703123456, // Example timestamp
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("source".to_string(), "research_signal_001".to_string());
                meta.insert("decay_rate".to_string(), "0.15".to_string());
                meta
            },
        },
        SignalInput {
            name: "hedge_pressure".to_string(),
            value: 0.2, // Light buying pressure from hedge adjustments
            confidence: 0.75,
            timestamp: 1703123456,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("source".to_string(), "research_signal_002".to_string());
                meta.insert("hedge_ratio".to_string(), "1.2".to_string());
                meta
            },
        },
    ];

    // Process signals to get intent vector
    let result = engine.process(signals)?;

    println!("Intent Processing Result:");
    println!("  Pressure: {:.3}", result.intent.pressure);
    println!("  Confidence: {:.3}", result.intent.confidence);
    println!("  Regime: {}", result.intent.regime);
    println!("  Signals: {}", result.intent.signals.len());
    println!("  Alternatives: {}", result.alternatives.len());
    println!("  Processing time: {} ns", result.processing_time_ns);

    // The result should show moderate selling pressure with good confidence
    assert!(result.intent.pressure < 0.0); // Negative pressure (selling)
    assert!(result.intent.confidence > 0.7); // Good confidence
    assert!(!result.intent.signals.is_empty());

    Ok(())
}

/// Example: Processing with custom configuration and regime awareness
pub fn example_custom_config() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom configuration
    let mut config = IntentConfig::default();
    config.min_confidence_threshold = 0.8;
    config.signal_weights.insert("oi_decay".to_string(), 1.5); // Higher weight for OI decay
    config.enable_alternatives = true;
    config.max_alternatives = 3;

    let engine = IntentEngine::new(config);

    // Signals indicating bear market conditions
    let signals = vec![
        SignalInput {
            name: "oi_decay".to_string(),
            value: -0.8, // Strong selling pressure
            confidence: 0.9,
            timestamp: 1703123456,
            metadata: HashMap::new(),
        },
        SignalInput {
            name: "basis_pressure".to_string(),
            value: -0.6, // Additional selling pressure
            confidence: 0.8,
            timestamp: 1703123456,
            metadata: HashMap::new(),
        },
    ];

    let result = engine.process(signals)?;

    println!("Bear Market Processing:");
    println!("  Pressure: {:.3}", result.intent.pressure);
    println!("  Confidence: {:.3}", result.intent.confidence);
    println!("  Regime: {}", result.intent.regime);

    // Should detect bear regime and show strong selling pressure
    assert_eq!(result.intent.regime, "bear");
    assert!(result.intent.pressure < -0.5);
    assert!(result.intent.confidence > 0.8);

    Ok(())
}

/// Example: Error handling for invalid inputs
pub fn example_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let engine = IntentEngine::default();

    // Test empty signals
    let result = engine.process(vec![]);
    assert!(result.is_err());
    println!("Empty signals error: {:?}", result.unwrap_err());

    // Test invalid signal value
    let invalid_signals = vec![SignalInput {
        name: "invalid".to_string(),
        value: 1.5, // Invalid: outside [-1.0, 1.0]
        confidence: 0.8,
        timestamp: 1703123456,
        metadata: HashMap::new(),
    }];

    let result = engine.process(invalid_signals);
    assert!(result.is_err());
    println!("Invalid signal error: {:?}", result.unwrap_err());

    Ok(())
}

/// Example: Demonstrating deterministic behavior
pub fn example_determinism() -> Result<(), Box<dyn std::error::Error>> {
    let engine = IntentEngine::default();

    let signals1 = vec![
        SignalInput {
            name: "z_signal".to_string(),
            value: 0.5,
            confidence: 0.8,
            timestamp: 1703123456,
            metadata: HashMap::new(),
        },
        SignalInput {
            name: "a_signal".to_string(),
            value: 0.3,
            confidence: 0.9,
            timestamp: 1703123456,
            metadata: HashMap::new(),
        },
    ];

    let signals2 = vec![
        SignalInput {
            name: "a_signal".to_string(),
            value: 0.3,
            confidence: 0.9,
            timestamp: 1703123456,
            metadata: HashMap::new(),
        },
        SignalInput {
            name: "z_signal".to_string(),
            value: 0.5,
            confidence: 0.8,
            timestamp: 1703123456,
            metadata: HashMap::new(),
        },
    ];

    let result1 = engine.process(signals1)?;
    let result2 = engine.process(signals2)?;

    // Results should be identical despite different order
    assert_eq!(result1.intent.pressure, result2.intent.pressure);
    assert_eq!(result1.intent.confidence, result2.intent.confidence);

    println!("Determinism verified: same results regardless of signal order");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_processing() {
        example_basic_processing().unwrap();
    }

    #[test]
    fn test_custom_config() {
        example_custom_config().unwrap();
    }

    #[test]
    fn test_error_handling() {
        example_error_handling().unwrap();
    }

    #[test]
    fn test_determinism() {
        example_determinism().unwrap();
    }
}
