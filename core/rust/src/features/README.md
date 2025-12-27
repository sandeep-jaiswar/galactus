# Feature Computation Framework

The Feature Computation Framework provides a plugin architecture for implementing and managing promoted signals in the Galactus production Rust core.

## Overview

The framework enables:

- **Plugin Architecture**: Modular feature implementations
- **Deterministic Computation**: Reproducible results across environments
- **Performance Monitoring**: Track computation times and success rates
- **Error Isolation**: Feature failures don't affect others
- **Batch Processing**: Efficient computation of multiple features

## Architecture

### Core Components

1. **Feature Registry** (`registry.rs`): Manages feature registration and execution
2. **Feature Trait** (`mod.rs`): Standard interface for feature implementations
3. **Batch Processing**: Compute multiple features efficiently
4. **Statistics Tracking**: Monitor performance and reliability

### Feature Lifecycle

```
Research → Validation → Promotion → Implementation → Registration → Production
```

## Usage

### Basic Feature Implementation

```rust
use galactus_core::features::{Feature, FeatureResult, FeatureError, FeatureInputs};

pub struct MyFeature {
    // Feature-specific configuration
}

impl Feature for MyFeature {
    fn name(&self) -> &str {
        "my_feature"
    }

    fn description(&self) -> &str {
        "Description of what this feature computes"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn compute(&self, inputs: &FeatureInputs) -> Result<FeatureResult, FeatureError> {
        // Implement deterministic computation logic
        let value = compute_feature_value(inputs)?;
        let confidence = compute_confidence(inputs)?;

        Ok(FeatureResult {
            name: self.name().to_string(),
            value,
            confidence,
            timestamp: inputs.timestamp,
            metadata: HashMap::new(),
            processing_time_ns: 0, // Will be set by registry
        })
    }
}
```

### Registry Usage

```rust
use galactus_core::features::{FeatureRegistry, FeatureConfig};
use std::sync::Arc;

// Create registry
let mut config = FeatureConfig::default();
config.max_processing_time_ns = 500_000; // 500μs limit
let registry = FeatureRegistry::with_config(config);

// Register features
let my_feature = Arc::new(MyFeature::new());
registry.register(my_feature)?;

// Compute single feature
let inputs = create_feature_inputs();
let result = registry.compute_feature("my_feature", &inputs)?;

// Compute multiple features
let feature_names = vec!["feature1".to_string(), "feature2".to_string()];
let batch_result = registry.compute_batch(&feature_names, &inputs);
```

## Feature Interface

All features must implement the `Feature` trait:

```rust
pub trait Feature: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn version(&self) -> &str;
    fn compute(&self, inputs: &FeatureInputs) -> Result<FeatureResult, FeatureError>;
}
```

### Requirements

- **Thread Safe**: `Send + Sync` for concurrent execution
- **Deterministic**: Same inputs produce same outputs
- **Pure Functions**: No side effects or external state
- **Error Handling**: Comprehensive error reporting

## Input Data Structure

Features receive data through `FeatureInputs`:

```rust
pub struct FeatureInputs {
    pub market_data: HashMap<String, MarketDataPoint>,    // Spot prices, volumes
    pub options_data: HashMap<String, OptionChain>,       // Option chains
    pub futures_data: HashMap<String, FuturesData>,       // Futures contracts
    pub context: HashMap<String, String>,                 // Additional context
    pub timestamp: i64,                                   // Computation timestamp
}
```

Features should only access data they actually need.

## Output Format

Features return `FeatureResult`:

```rust
pub struct FeatureResult {
    pub name: String,                           // Feature name
    pub value: f64,                            // Feature value (-1.0 to 1.0)
    pub confidence: f64,                       // Confidence (0.0 to 1.0)
    pub timestamp: i64,                        // Computation timestamp
    pub metadata: HashMap<String, String>,     // Additional metadata
    pub processing_time_ns: u64,               // Actual processing time
}
```

## Error Handling

Features can return `FeatureError`:

- `FeatureNotFound`: Feature not registered
- `InvalidInput`: Bad input data
- `ComputationFailure`: Computation error
- `ConfigurationError`: Configuration issue
- `ResourceExhaustion`: Memory/time limits exceeded

## Batch Processing

Compute multiple features efficiently:

```rust
let batch_result = registry.compute_batch(&feature_names, &inputs);

// Check results
for (name, result) in &batch_result.results {
    println!("{}: {} (confidence: {})", name, result.value, result.confidence);
}

// Check errors
for (name, error) in &batch_result.errors {
    eprintln!("{} failed: {}", name, error);
}
```

### Determinism

- Features processed in sorted name order
- Results are reproducible
- No random number generation
- Consistent floating-point operations

## Performance Monitoring

Registry tracks statistics:

```rust
let stats = registry.stats();
println!("Total computations: {}", stats.total_computations);
println!("Success rate: {:.2}%",
    (stats.successful_computations as f64 / stats.total_computations as f64) * 100.0);

// Per-feature stats
for (name, feature_stats) in &stats.feature_stats {
    println!("{}: {} computations, avg time: {} ns",
        name,
        feature_stats.computation_count,
        feature_stats.avg_processing_time_ns
    );
}
```

## Configuration

Customize registry behavior:

```rust
let mut config = FeatureConfig::default();
config.max_processing_time_ns = 1_000_000;  // 1ms limit
config.enable_logging = true;
config.fail_fast = false;  // Continue on errors
config.feature_configs.insert(
    "my_feature".to_string(),
    HashMap::from([("param".to_string(), "value".to_string())])
);
```

## Testing

Features should have comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_computation() {
        let feature = MyFeature::new();
        let inputs = create_test_inputs();

        let result = feature.compute(&inputs).unwrap();
        assert!(result.value >= -1.0 && result.value <= 1.0);
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[test]
    fn test_deterministic_computation() {
        let feature = MyFeature::new();
        let inputs = create_test_inputs();

        let result1 = feature.compute(&inputs).unwrap();
        let result2 = feature.compute(&inputs).unwrap();

        assert_eq!(result1.value, result2.value);
        assert_eq!(result1.confidence, result2.confidence);
    }
}
```

## Safety & Determinism

### Memory Safety
- Rust guarantees prevent memory corruption
- No unsafe code in framework
- Automatic resource management

### Determinism Guarantees
- Pure functions only
- No external state dependencies
- Consistent ordering
- No floating-point operations that vary by platform

### Performance
- Low overhead registry
- Efficient batch processing
- Configurable time limits
- Statistics tracking

## Integration with Intent Engine

Features integrate with the intent engine:

```rust
// Compute features
let batch_result = registry.compute_batch(&["oi_decay", "hedge_pressure"], &inputs);

// Convert to intent signals
let signals: Vec<SignalInput> = batch_result.results.values()
    .map(|result| SignalInput {
        name: result.name.clone(),
        value: result.value,
        confidence: result.confidence,
        timestamp: result.timestamp,
        metadata: result.metadata.clone(),
    })
    .collect();

// Process with intent engine
let intent_result = intent_engine.process(signals)?;
```

## Examples

See `examples.rs` in the intent module for comprehensive usage examples.

## Dependencies

The framework uses only Rust standard library for maximum portability and minimal dependencies.