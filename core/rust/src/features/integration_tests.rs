//! Integration Tests for Promoted Signals
//!
//! Tests that verify promoted signals work correctly.
//! These tests ensure the complete pipeline from feature computation to registry integration.

use crate::features::registry::Feature;
use crate::features::{
    BasisPressureFeature, FeatureInputs, FeatureRegistry, FuturesData, HedgePressureFeature,
    MarketDataPoint, OIDecayFeature, OptionChain, StrikeData,
};
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_hedge_pressure_feature_registration() {
        let mut registry = FeatureRegistry::new();
        let hedge_pressure = Arc::new(HedgePressureFeature::new());

        // Register the feature
        registry.register(hedge_pressure).unwrap();

        // Verify it's registered
        assert!(registry.is_registered("hedge_pressure"));
        assert!(registry
            .list_features()
            .contains(&"hedge_pressure".to_string()));
    }

    #[test]
    fn test_basis_pressure_feature_registration() {
        let mut registry = FeatureRegistry::new();
        let basis_pressure = Arc::new(BasisPressureFeature::new());

        // Register the feature
        registry.register(basis_pressure).unwrap();

        // Verify it's registered
        assert!(registry.is_registered("basis_pressure"));
        assert!(registry
            .list_features()
            .contains(&"basis_pressure".to_string()));
    }

    #[test]
    fn test_oi_decay_feature_registration() {
        let mut registry = FeatureRegistry::new();
        let oi_decay = Arc::new(OIDecayFeature::new());

        // Register the feature
        registry.register(oi_decay).unwrap();

        // Verify it's registered
        assert!(registry.is_registered("oi_decay"));
        assert!(registry.list_features().contains(&"oi_decay".to_string()));
    }

    #[test]
    fn test_hedge_pressure_computation_with_real_data() {
        let hedge_pressure = HedgePressureFeature::new();

        // Create test data: call-heavy imbalance
        let calls_oi = [
            (100.0, 2000), // At-the-money calls
            (105.0, 1500), // Out-of-money calls
            (110.0, 800),  // Further OTM calls
        ];

        let puts_oi = [
            (95.0, 500),  // Out-of-money puts
            (100.0, 600), // At-the-money puts
            (105.0, 200), // In-the-money puts
        ];

        let spot_price = 100.0;

        let result = hedge_pressure
            .compute_pressure(&calls_oi, &puts_oi, spot_price)
            .unwrap();

        // Should show positive pressure (call-heavy)
        assert!(result.pressure > 0.0);
        assert_eq!(result.call_oi_total, 4300);
        assert_eq!(result.put_oi_total, 1300);
        assert!(result.imbalance_ratio > 0.5); // Strong call bias
        assert!(result.confidence > 0.0);
        assert_eq!(result.strike_count, 4); // 3 call strikes + 1 unique put strike
    }

    #[test]
    fn test_basis_pressure_computation_with_real_data() {
        let basis_pressure = BasisPressureFeature::new();

        // Create test data: futures trading at premium (contango)
        let spot_price = 100.0;
        let futures_price = 102.0; // 2% premium
        let time_to_expiry = 30.0 / 365.0; // 30 days
        let risk_free_rate = 0.05; // 5%

        let result = basis_pressure
            .compute_pressure(
                futures_price,
                spot_price,
                time_to_expiry * 365.0,
                Some(risk_free_rate),
            )
            .unwrap();

        // Should show positive pressure (futures overpriced relative to fair value)
        assert!(result.pressure > 0.0);
        assert!(result.actual_basis > result.fair_basis); // Actual basis should be higher than fair basis
        assert!(result.divergence_pct > 0.0); // Positive divergence
        assert!(result.confidence > 0.0);
        assert!(result.time_to_expiry_days > 0.0);
    }

    #[test]
    fn test_oi_decay_feature_integration() {
        let oi_decay = OIDecayFeature::new();

        // Create OI data in context (current implementation expects this)
        let mut context = HashMap::new();
        context.insert("current_oi_95.0".to_string(), "1000".to_string());
        context.insert("current_oi_100.0".to_string(), "800".to_string());
        context.insert("current_oi_105.0".to_string(), "600".to_string());
        context.insert("previous_oi_95.0".to_string(), "900".to_string());
        context.insert("previous_oi_100.0".to_string(), "850".to_string());
        context.insert("previous_oi_105.0".to_string(), "650".to_string());
        context.insert("time_delta_hours".to_string(), "1.0".to_string());

        let inputs = FeatureInputs {
            market_data: HashMap::new(),
            options_data: HashMap::new(),
            futures_data: HashMap::new(),
            context,
            timestamp: 1703123456,
        };

        // Compute the feature
        let result = oi_decay.compute(&inputs).unwrap();

        // Verify result structure
        assert_eq!(result.name, "oi_decay");
        assert!(result.value <= 1.0 && result.value >= -1.0); // Should be in valid range
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
        assert_eq!(result.timestamp, 1703123456);
        assert!(!result.metadata.is_empty()); // Should have computation metadata
    }

    #[test]
    fn test_basis_pressure_feature_integration() {
        let basis_pressure = BasisPressureFeature::new();

        // Create realistic futures and spot data
        let mut futures_data = HashMap::new();
        let mut market_data = HashMap::new();

        // Add futures data
        futures_data.insert(
            "default".to_string(),
            FuturesData {
                symbol: "TEST".to_string(),
                price: 102.0,
                open_interest: 50000,
                timestamp: 1703123456,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("time_to_expiry_days".to_string(), "30".to_string());
                    meta
                },
            },
        );

        // Add spot market data with correct symbol
        market_data.insert(
            "SPOT".to_string(),
            MarketDataPoint {
                symbol: "SPOT".to_string(),
                price: 100.0,
                volume: 50000,
                timestamp: 1703123456,
                metadata: HashMap::new(),
            },
        );

        let inputs = FeatureInputs {
            market_data,
            options_data: HashMap::new(),
            futures_data,
            context: HashMap::new(),
            timestamp: 1703123456,
        };

        // Compute the feature
        let result = basis_pressure.compute(&inputs).unwrap();

        // Verify result structure
        assert_eq!(result.name, "basis_pressure");
        assert!(result.value <= 1.0 && result.value >= -1.0); // Should be in valid range
        assert!(result.confidence > 0.0 && result.confidence <= 1.0);
        assert_eq!(result.timestamp, 1703123456);
        assert!(!result.metadata.is_empty()); // Should have computation metadata
    }

    #[test]
    fn test_multiple_features_in_registry() {
        let mut registry = FeatureRegistry::new();

        // Register OI decay feature
        let oi_decay = Arc::new(OIDecayFeature::new());
        registry.register(oi_decay).unwrap();

        // Register hedge pressure feature
        let hedge_pressure = Arc::new(HedgePressureFeature::new());
        registry.register(hedge_pressure).unwrap();

        // Register basis pressure feature
        let basis_pressure = Arc::new(BasisPressureFeature::new());
        registry.register(basis_pressure).unwrap();

        // Verify registry state
        assert!(registry.is_registered("oi_decay"));
        assert!(registry.is_registered("hedge_pressure"));
        assert!(registry.is_registered("basis_pressure"));
        assert_eq!(registry.list_features().len(), 3);
    }

    #[test]
    fn test_deterministic_computation() {
        let oi_decay = OIDecayFeature::new();

        // Create identical inputs
        let inputs1 = FeatureInputs {
            market_data: HashMap::new(),
            options_data: HashMap::new(),
            futures_data: HashMap::new(),
            context: HashMap::new(),
            timestamp: 1703123456,
        };

        let inputs2 = inputs1.clone();

        // Compute multiple times
        let result1 = oi_decay.compute(&inputs1).unwrap();
        let result2 = oi_decay.compute(&inputs2).unwrap();

        // Results should be identical
        assert_eq!(result1.value, result2.value);
        assert_eq!(result1.confidence, result2.confidence);
        assert_eq!(result1.metadata, result2.metadata);
    }

    #[test]
    fn test_feature_error_handling() {
        let oi_decay = OIDecayFeature::new();

        // Create inputs with no OI data
        let inputs = FeatureInputs {
            market_data: HashMap::new(),
            options_data: HashMap::new(), // Empty - should return insufficient data result
            futures_data: HashMap::new(),
            context: HashMap::new(),
            timestamp: 1703123456,
        };

        // This should succeed but return zero pressure due to insufficient data
        let result = oi_decay.compute(&inputs).unwrap();
        assert_eq!(result.value, 0.0);
        assert_eq!(result.confidence, 0.0);
    }
}
