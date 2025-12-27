//! Feature Registry
//!
//! Manages the registration and execution of promoted features.
//! Provides plugin architecture for deterministic feature computation.
//!
//! # Registry Features
//!
//! - **Dynamic Registration**: Features can be registered at runtime
//! - **Batch Computation**: Compute multiple features efficiently
//! - **Error Isolation**: Feature failures don't affect others
//! - **Performance Monitoring**: Track computation times and success rates
//! - **Deterministic Ordering**: Consistent execution order for reproducibility

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::features::{
    Feature, FeatureResult, FeatureError, FeatureInputs, FeatureConfig
};

/// Central registry for feature implementations
#[derive(Debug, Clone)]
pub struct FeatureRegistry {
    /// Registered features (name -> implementation)
    features: Arc<RwLock<HashMap<String, Arc<dyn Feature>>>>,

    /// Configuration for feature execution
    config: FeatureConfig,

    /// Performance statistics
    stats: Arc<RwLock<RegistryStats>>,
}

/// Statistics for registry performance monitoring
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    /// Total feature computations performed
    pub total_computations: u64,

    /// Successful computations
    pub successful_computations: u64,

    /// Failed computations
    pub failed_computations: u64,

    /// Total processing time (nanoseconds)
    pub total_processing_time_ns: u128,

    /// Per-feature statistics
    pub feature_stats: HashMap<String, FeatureStats>,
}

/// Statistics for individual features
#[derive(Debug, Clone, Default)]
pub struct FeatureStats {
    /// Number of times this feature was computed
    pub computation_count: u64,

    /// Number of successful computations
    pub success_count: u64,

    /// Number of failed computations
    pub failure_count: u64,

    /// Average processing time (nanoseconds)
    pub avg_processing_time_ns: u64,

    /// Last computation timestamp
    pub last_computation: i64,
}

/// Result of batch feature computation
#[derive(Debug, Clone)]
pub struct BatchResult {
    /// Successfully computed features
    pub results: HashMap<String, FeatureResult>,

    /// Failed feature computations
    pub errors: HashMap<String, FeatureError>,

    /// Total processing time
    pub total_processing_time_ns: u128,

    /// Batch metadata
    pub metadata: HashMap<String, String>,
}

impl FeatureRegistry {
    /// Create a new feature registry with default configuration
    pub fn new() -> Self {
        Self {
            features: Arc::new(RwLock::new(HashMap::new())),
            config: FeatureConfig::default(),
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        }
    }

    /// Create a new registry with custom configuration
    pub fn with_config(config: FeatureConfig) -> Self {
        Self {
            features: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        }
    }

    /// Register a feature implementation
    ///
    /// # Arguments
    /// * `feature` - The feature implementation to register
    ///
    /// # Returns
    /// Ok(()) if registration successful, Error if feature already exists
    pub fn register(&self, feature: Arc<dyn Feature>) -> Result<(), FeatureError> {
        let name = feature.name().to_string();

        let mut features = self.features.write().unwrap();
        if features.contains_key(&name) {
            return Err(FeatureError::ConfigurationError(
                format!("Feature '{}' already registered", name)
            ));
        }

        features.insert(name, feature);
        Ok(())
    }

    /// Unregister a feature
    ///
    /// # Arguments
    /// * `name` - Name of the feature to unregister
    ///
    /// # Returns
    /// Ok(()) if unregistration successful, Error if feature not found
    pub fn unregister(&self, name: &str) -> Result<(), FeatureError> {
        let mut features = self.features.write().unwrap();
        if features.remove(name).is_none() {
            return Err(FeatureError::FeatureNotFound(name.to_string()));
        }
        Ok(())
    }

    /// Check if a feature is registered
    pub fn is_registered(&self, name: &str) -> bool {
        let features = self.features.read().unwrap();
        features.contains_key(name)
    }

    /// Get a list of all registered feature names
    pub fn list_features(&self) -> Vec<String> {
        let features = self.features.read().unwrap();
        let mut names: Vec<String> = features.keys().cloned().collect();
        names.sort(); // Deterministic ordering
        names
    }

    /// Compute a single feature
    ///
    /// # Arguments
    /// * `name` - Name of the feature to compute
    /// * `inputs` - Input data for computation
    ///
    /// # Returns
    /// Feature result or error
    pub fn compute_feature(&self, name: &str, inputs: &FeatureInputs) -> Result<FeatureResult, FeatureError> {
        let features = self.features.read().unwrap();
        let feature = features.get(name)
            .ok_or_else(|| FeatureError::FeatureNotFound(name.to_string()))?;

        let start_time = SystemTime::now();

        // Execute feature computation
        let result = feature.compute(inputs);

        let processing_time = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default()
            .as_nanos();

        // Update statistics
        self.update_stats(name, &result, processing_time);

        // Check processing time limit
        if processing_time > self.config.max_processing_time_ns as u128 {
            return Err(FeatureError::ComputationFailure(
                format!("Feature '{}' exceeded processing time limit: {} ns",
                       name, processing_time)
            ));
        }

        result
    }

    /// Compute multiple features in batch
    ///
    /// # Arguments
    /// * `feature_names` - Names of features to compute
    /// * `inputs` - Input data for computation
    ///
    /// # Returns
    /// Batch result with successful computations and errors
    ///
    /// # Determinism
    /// Features are processed in sorted order for reproducible results
    pub fn compute_batch(&self, feature_names: &[String], inputs: &FeatureInputs) -> BatchResult {
        let start_time = SystemTime::now();

        // Sort feature names for deterministic processing
        let mut sorted_names = feature_names.to_vec();
        sorted_names.sort();
        sorted_names.dedup(); // Remove duplicates

        let mut results = HashMap::new();
        let mut errors = HashMap::new();

        for name in sorted_names {
            match self.compute_feature(&name, inputs) {
                Ok(result) => {
                    results.insert(name.clone(), result);
                }
                Err(error) => {
                    errors.insert(name.clone(), error);

                    // Fail fast if configured
                    if self.config.fail_fast {
                        break;
                    }
                }
            }
        }

        let total_processing_time = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default()
            .as_nanos();

        let mut metadata = HashMap::new();
        metadata.insert("total_features".to_string(), feature_names.len().to_string());
        metadata.insert("successful".to_string(), results.len().to_string());
        metadata.insert("failed".to_string(), errors.len().to_string());
        metadata.insert("deterministic".to_string(), "true".to_string());

        BatchResult {
            results,
            errors,
            total_processing_time_ns: total_processing_time,
            metadata,
        }
    }

    /// Get current registry statistics
    pub fn stats(&self) -> RegistryStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset registry statistics
    pub fn reset_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        *stats = RegistryStats::default();
    }

    /// Update statistics after feature computation
    fn update_stats(&self, name: &str, result: &Result<FeatureResult, FeatureError>, processing_time: u128) {
        let mut stats = self.stats.write().unwrap();

        stats.total_computations += 1;
        stats.total_processing_time_ns += processing_time;

        let feature_stats = stats.feature_stats
            .entry(name.to_string())
            .or_insert_with(FeatureStats::default);

        feature_stats.computation_count += 1;

        match result {
            Ok(result) => {
                stats.successful_computations += 1;
                feature_stats.success_count += 1;
                feature_stats.last_computation = result.timestamp;

                // Update rolling average
                let total_time = feature_stats.avg_processing_time_ns as u128 * (feature_stats.computation_count - 1);
                feature_stats.avg_processing_time_ns = ((total_time + processing_time) / feature_stats.computation_count as u128) as u64;
            }
            Err(_) => {
                stats.failed_computations += 1;
                feature_stats.failure_count += 1;
            }
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &FeatureConfig {
        &self.config
    }

    /// Update the configuration
    pub fn set_config(&mut self, config: FeatureConfig) {
        self.config = config;
    }
}

impl Default for FeatureRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::{Feature, FeatureInputs};

    // Mock feature for testing
    struct MockFeature {
        name: String,
        value: f64,
        confidence: f64,
    }

    impl MockFeature {
        fn new(name: &str, value: f64, confidence: f64) -> Self {
            Self {
                name: name.to_string(),
                value,
                confidence,
            }
        }
    }

    impl Feature for MockFeature {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            "Mock feature for testing"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        fn compute(&self, _inputs: &FeatureInputs) -> Result<FeatureResult, FeatureError> {
            Ok(FeatureResult {
                name: self.name.clone(),
                value: self.value,
                confidence: self.confidence,
                timestamp: 1234567890,
                metadata: HashMap::new(),
                processing_time_ns: 1000,
            })
        }
    }

    fn create_test_inputs() -> FeatureInputs {
        FeatureInputs {
            market_data: HashMap::new(),
            options_data: HashMap::new(),
            futures_data: HashMap::new(),
            context: HashMap::new(),
            timestamp: 1234567890,
        }
    }

    #[test]
    fn test_registry_creation() {
        let registry = FeatureRegistry::new();
        assert!(registry.list_features().is_empty());
    }

    #[test]
    fn test_feature_registration() {
        let registry = FeatureRegistry::new();
        let feature = Arc::new(MockFeature::new("test_feature", 0.5, 0.8));

        assert!(!registry.is_registered("test_feature"));
        registry.register(feature).unwrap();
        assert!(registry.is_registered("test_feature"));
        assert_eq!(registry.list_features(), vec!["test_feature"]);
    }

    #[test]
    fn test_duplicate_registration_error() {
        let registry = FeatureRegistry::new();
        let feature1 = Arc::new(MockFeature::new("test_feature", 0.5, 0.8));
        let feature2 = Arc::new(MockFeature::new("test_feature", 0.3, 0.9));

        registry.register(feature1).unwrap();
        let result = registry.register(feature2);
        assert!(matches!(result, Err(FeatureError::ConfigurationError(_))));
    }

    #[test]
    fn test_feature_unregistration() {
        let registry = FeatureRegistry::new();
        let feature = Arc::new(MockFeature::new("test_feature", 0.5, 0.8));

        registry.register(feature).unwrap();
        assert!(registry.is_registered("test_feature"));

        registry.unregister("test_feature").unwrap();
        assert!(!registry.is_registered("test_feature"));
    }

    #[test]
    fn test_compute_single_feature() {
        let registry = FeatureRegistry::new();
        let feature = Arc::new(MockFeature::new("test_feature", 0.5, 0.8));
        registry.register(feature).unwrap();

        let inputs = create_test_inputs();
        let result = registry.compute_feature("test_feature", &inputs).unwrap();

        assert_eq!(result.name, "test_feature");
        assert_eq!(result.value, 0.5);
        assert_eq!(result.confidence, 0.8);
    }

    #[test]
    fn test_compute_unknown_feature_error() {
        let registry = FeatureRegistry::new();
        let inputs = create_test_inputs();

        let result = registry.compute_feature("unknown_feature", &inputs);
        assert!(matches!(result, Err(FeatureError::FeatureNotFound(_))));
    }

    #[test]
    fn test_batch_computation() {
        let registry = FeatureRegistry::new();
        let feature1 = Arc::new(MockFeature::new("feature1", 0.5, 0.8));
        let feature2 = Arc::new(MockFeature::new("feature2", 0.3, 0.9));

        registry.register(feature1).unwrap();
        registry.register(feature2).unwrap();

        let inputs = create_test_inputs();
        let batch_result = registry.compute_batch(
            &["feature1".to_string(), "feature2".to_string()],
            &inputs
        );

        assert_eq!(batch_result.results.len(), 2);
        assert!(batch_result.errors.is_empty());
        assert!(batch_result.total_processing_time_ns > 0);
    }

    #[test]
    fn test_deterministic_batch_ordering() {
        let registry = FeatureRegistry::new();
        let feature1 = Arc::new(MockFeature::new("z_feature", 0.5, 0.8));
        let feature2 = Arc::new(MockFeature::new("a_feature", 0.3, 0.9));

        registry.register(feature1).unwrap();
        registry.register(feature2).unwrap();

        let inputs = create_test_inputs();

        // Test different input orderings produce same results
        let batch1 = registry.compute_batch(
            &["z_feature".to_string(), "a_feature".to_string()],
            &inputs
        );
        let batch2 = registry.compute_batch(
            &["a_feature".to_string(), "z_feature".to_string()],
            &inputs
        );

        assert_eq!(batch1.results.len(), batch2.results.len());
        assert_eq!(batch1.errors.len(), batch2.errors.len());
        // Results should be identical (same keys, same values)
        assert_eq!(batch1.results["a_feature"].value, batch2.results["a_feature"].value);
        assert_eq!(batch1.results["z_feature"].value, batch2.results["z_feature"].value);
    }

    #[test]
    fn test_statistics_tracking() {
        let registry = FeatureRegistry::new();
        let feature = Arc::new(MockFeature::new("test_feature", 0.5, 0.8));
        registry.register(feature).unwrap();

        let inputs = create_test_inputs();

        // Initial stats should be zero
        let initial_stats = registry.stats();
        assert_eq!(initial_stats.total_computations, 0);

        // Compute feature
        registry.compute_feature("test_feature", &inputs).unwrap();

        // Check stats updated
        let stats = registry.stats();
        assert_eq!(stats.total_computations, 1);
        assert_eq!(stats.successful_computations, 1);
        assert_eq!(stats.failed_computations, 0);
        assert!(stats.total_processing_time_ns > 0);

        // Check feature-specific stats
        let feature_stats = &stats.feature_stats["test_feature"];
        assert_eq!(feature_stats.computation_count, 1);
        assert_eq!(feature_stats.success_count, 1);
        assert_eq!(feature_stats.failure_count, 0);
    }
}