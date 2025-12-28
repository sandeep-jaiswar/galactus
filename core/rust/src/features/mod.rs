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

pub mod basis_pressure;
pub mod hedge_pressure;
#[cfg(test)]
pub mod integration_tests;
pub mod oi_decay;
pub mod registry;

// Re-export main types for convenience
pub use basis_pressure::{BasisPressureConfig, BasisPressureFeature, BasisPressureResult};
pub use hedge_pressure::{HedgePressureConfig, HedgePressureFeature, HedgePressureResult};
pub use oi_decay::{OIDecayConfig, OIDecayFeature, OIDecayResult};
pub use registry::{
    Feature, FeatureConfig, FeatureError, FeatureInputs, FeatureRegistry, FeatureResult,
    FuturesData, MarketDataPoint, OptionChain, StrikeData,
};

/// Create a feature registry with all promoted signals registered
///
/// This function initializes a FeatureRegistry and registers all three promoted
/// signals: OI decay, hedge pressure, and basis pressure.
///
/// # Returns
/// A FeatureRegistry with all features registered and ready for use
pub fn create_promoted_feature_registry() -> Result<FeatureRegistry, FeatureError> {
    let registry = FeatureRegistry::new();

    // Register OI decay feature
    let oi_decay = Arc::new(OIDecayFeature::new());
    registry.register(oi_decay)?;

    // Register hedge pressure feature
    let hedge_pressure = Arc::new(HedgePressureFeature::new());
    registry.register(hedge_pressure)?;

    // Register basis pressure feature
    let basis_pressure = Arc::new(BasisPressureFeature::new());
    registry.register(basis_pressure)?;

    Ok(registry)
}

// Core types for feature computation
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
