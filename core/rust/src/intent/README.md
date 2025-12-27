# Intent Engine

The Intent Engine is the core inference component of Galactus, responsible for deterministic capital pressure detection from promoted signals.

## Overview

The intent engine processes multiple capital pressure signals to produce a unified intent vector that represents the aggregate market pressure from institutional capital flows. The engine is designed for:

- **Determinism**: Same inputs always produce same outputs
- **Regime Awareness**: Adjusts processing based on market conditions
- **Confidence Scoring**: Provides reliability metrics for all outputs
- **Alternative Interpretations**: Generates multiple possible readings

## Architecture

### Core Components

1. **Signal Aggregation** (`aggregation.rs`): Combines multiple signals using weighted averaging
2. **Intent Inference** (`engine.rs`): Applies regime-aware processing and confidence calculation
3. **Configuration** (`mod.rs`): Defines processing parameters and behavior

### Processing Pipeline

```
Signals → Validation → Regime Detection → Aggregation → Intent Mapping → Confidence → Alternatives
```

## Usage

### Basic Processing

```rust
use galactus_core::intent::{IntentEngine, SignalInput};

let engine = IntentEngine::default();

let signals = vec![
    SignalInput {
        name: "oi_decay".to_string(),
        value: -0.4,  // Selling pressure
        confidence: 0.85,
        timestamp: 1703123456,
        metadata: HashMap::new(),
    },
    SignalInput {
        name: "hedge_pressure".to_string(),
        value: 0.2,   // Buying pressure
        confidence: 0.75,
        timestamp: 1703123456,
        metadata: HashMap::new(),
    },
];

let result = engine.process(signals)?;
println!("Pressure: {:.3}, Confidence: {:.3}", result.intent.pressure, result.intent.confidence);
```

### Custom Configuration

```rust
use galactus_core::intent::{IntentEngine, IntentConfig};

let mut config = IntentConfig::default();
config.min_confidence_threshold = 0.8;
config.signal_weights.insert("oi_decay".to_string(), 1.5);

let engine = IntentEngine::new(config);
```

## Signal Format

Signals must conform to the following format:

- **Value**: f64 in range [-1.0, 1.0]
  - Negative = selling pressure
  - Positive = buying pressure
  - Magnitude indicates strength

- **Confidence**: f64 in range [0.0, 1.0]
  - 0.0 = no confidence
  - 1.0 = complete confidence

- **Name**: String identifier (must match promoted signal names)

- **Timestamp**: Unix timestamp (i64)

## Regime Detection

The engine automatically detects market regimes:

- **Bull**: Strong upward consensus
- **Bear**: Strong downward consensus
- **Sideways**: Low volatility, mixed signals
- **Volatile**: High signal dispersion
- **Mixed**: Default classification

Regime detection adjusts signal weights and confidence thresholds for optimal performance.

## Output Format

### Intent Vector

```rust
pub struct IntentVector {
    pub pressure: f64,           // [-1.0, 1.0] aggregate pressure
    pub confidence: f64,         // [0.0, 1.0] confidence score
    pub signals: HashMap<String, SignalContribution>,  // Component signals
    pub timestamp: i64,          // Processing timestamp
    pub regime: String,          // Detected market regime
}
```

### Intent Result

```rust
pub struct IntentResult {
    pub intent: IntentVector,                    // Primary intent
    pub alternatives: Vec<IntentVector>,         // Alternative interpretations
    pub metadata: HashMap<String, String>,       // Processing metadata
    pub processing_time_ns: u64,                 // Performance metric
}
```

## Error Handling

The engine returns `IntentError` for various failure conditions:

- `NoSignalsAvailable`: No input signals provided
- `InvalidSignalData`: Signal values outside valid ranges
- `AggregationFailure`: Signal combination failed
- `ConfidenceFailure`: Confidence calculation error
- `RegimeFailure`: Regime detection failed

## Determinism Guarantees

- All floating-point operations use deterministic algorithms
- Signal processing order is normalized (sorted by name)
- No random number generation
- No external state dependencies
- Results are reproducible across environments

## Performance

- Designed for low-latency inference (< 1ms typical)
- Memory efficient (no allocations in hot path)
- Optimized for concurrent processing
- Processing time tracked in nanoseconds

## Testing

Run the test suite:

```bash
cargo test --package galactus-core
```

Tests cover:
- Unit tests for all components
- Integration tests for full pipeline
- Determinism verification
- Error condition handling
- Performance benchmarks

## Examples

See `examples.rs` for comprehensive usage examples including:

- Basic processing
- Custom configuration
- Error handling
- Determinism verification
- Regime-aware processing

## Dependencies

The intent engine uses only Rust standard library features for maximum portability and minimal dependency footprint.