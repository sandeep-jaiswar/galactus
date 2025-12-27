# Testing Framework Documentation

This document describes the expanded testing framework for Galactus, covering both Rust production code and Python research code.

## Overview

The testing framework includes:
- **Integration Tests**: Validate interactions between components
- **Performance Tests**: Ensure computational efficiency
- **Chaos Tests**: Verify resilience under adverse conditions

## Rust Tests (`core/rust/tests/`)

### Existing Integration Tests

1. **`confidence_integration_test.rs`**
   - Tests confidence and stability computation determinism
   - Validates geometric mean prevents false confidence
   - Tests silence decision consistency

2. **`failure_analysis_integration_test.rs`**
   - Tests end-to-end failure workflow
   - Validates failure pattern detection
   - Tests deprecation recommendations

### New Tests

#### Performance Tests (`performance_integration_test.rs`)

Tests computational performance to ensure production readiness:

- **`test_confidence_computation_performance`**: Validates confidence computation completes in < 0.1ms per operation
- **`test_failure_analysis_scalability`**: Tests handling of 1000+ failure records
- **`test_stress_scenario_evaluation_performance`**: Ensures stress scenario evaluation is fast
- **`test_concurrent_confidence_assessment`**: Validates thread-safe concurrent operations
- **`test_memory_efficiency_with_large_inputs`**: Tests handling of large datasets

**Running Performance Tests:**
```bash
cd core/rust
cargo test --test performance_integration_test
```

#### Chaos Tests (`chaos_integration_test.rs`)

Tests system resilience under adverse conditions:

- **`test_confidence_with_missing_data`**: Validates graceful degradation with incomplete data
- **`test_confidence_with_contradictory_signals`**: Tests handling of conflicting signals
- **`test_extreme_data_delay`**: Validates response to stale data (600s delay)
- **`test_volatile_inference_values`**: Tests stability with highly volatile inputs
- **`test_stress_scenario_*`**: Validates behavior during market crashes and expiry chaos
- **`test_cascading_failures`**: Tests handling of failure cascades
- **`test_regime_uncertainty_chaos`**: Validates behavior with uncertain regime classification
- **`test_empty_signal_set`**: Tests edge case handling
- **`test_extreme_perturbation_sensitivity`**: Tests response to extreme input variations

**Running Chaos Tests:**
```bash
cd core/rust
cargo test --test chaos_integration_test
```

### Test Results

All Rust tests pass successfully:
- Performance tests: 5 passed
- Chaos tests: 10 passed
- Existing tests: 16 passed

## Python Tests (`research/python/tests/`)

### Existing Research Tests

1. **`test_basis_pressure_research.py`**: Tests basis pressure signal computation
2. **`test_hedge_pressure_research.py`**: Tests hedge pressure detection
3. **`test_oi_decay_research.py`**: Tests OI decay signal
4. **`test_feature_isolation.py`**: Tests feature isolation

### New Tests

#### Integration Tests (`test_signal_integration.py`)

Tests signal integration and cross-validation:

**Test Classes:**
- `TestSignalIntegration`: Basic signal combination and aggregation
- `TestCrossSignalValidation`: Cross-validation between different signal types
- `TestDataIntegrationPipeline`: End-to-end data pipeline testing
- `TestPromotionReadiness`: Validates signals ready for production promotion
- `TestRegimeCompatibility`: Tests signal behavior under different market regimes

**Key Tests:**
- Signal conflict detection
- Confidence propagation through combinations
- Data quality impact on signals
- Determinism validation
- Performance requirements (< 10ms)

**Running Integration Tests:**
```bash
cd research/python
python3 -m pytest tests/test_signal_integration.py -v
```

#### Performance Tests (`test_performance.py`)

Tests computational efficiency of research signals:

**Test Classes:**
- `TestComputationPerformance`: Single and batch computation latency
- `TestScalabilityPerformance`: Linear scaling validation
- `TestResourceUtilization`: CPU and memory efficiency
- `TestComputationalComplexity`: Algorithm complexity verification
- `TestCacheEfficiency`: Caching mechanism efficiency

**Key Performance Targets:**
- Single signal computation: < 1ms
- Batch 1000 operations: < 10ms
- Historical analysis (100 periods): < 10ms
- Large dataset (10k samples): < 100ms

**Running Performance Tests:**
```bash
cd research/python
python3 -m pytest tests/test_performance.py -v
```

#### Chaos Tests (`test_chaos.py`)

Tests resilience and edge case handling:

**Test Classes:**
- `TestDataQualityFailures`: Missing, delayed, contradictory, corrupted data
- `TestExtremMarketConditions`: Volatility, crashes, liquidity crises
- `TestEdgeCases`: Zero values, negative values, extreme values
- `TestSystemFailures`: Memory pressure, timeouts, cascading failures
- `TestNumericalStability`: Floating point precision, overflow/underflow
- `TestRegimeTransitions`: Rapid regime changes, uncertainty
- `TestTemporalChaos`: Out-of-order data, duplicates, gaps

**Key Scenarios:**
- 10% market crash simulation
- 5% liquidity spread (crisis)
- Flash crash and recovery
- Circuit breaker scenarios
- Extreme volatility (10%+ moves)

**Running Chaos Tests:**
```bash
cd research/python
python3 -m pytest tests/test_chaos.py -v
```

### Test Results

All new Python tests pass successfully:
- Integration tests: 24 passed
- Performance tests: 26 passed  
- Chaos tests: 23 passed

## Running All Tests

### Rust Tests
```bash
cd core/rust
cargo test
```

### Python Tests
```bash
cd research/python
python3 -m pytest tests/test_signal_integration.py tests/test_performance.py tests/test_chaos.py -v
```

## Test Coverage

### Rust Coverage

- **Confidence System**: Determinism, degradation, geometric mean, silence decisions
- **Failure Analysis**: Pattern detection, deprecation logic, learning reports
- **Stress Scenarios**: Category classification, severity levels, suppression logic
- **Performance**: Sub-millisecond operations, concurrent safety, memory efficiency
- **Chaos**: Missing data, contradictions, extreme values, cascading failures

### Python Coverage

- **Signal Integration**: Combination, aggregation, conflict detection
- **Cross-Validation**: Signal correlation, independence, interaction
- **Performance**: Computation latency, scalability, vectorization
- **Chaos**: Data quality failures, extreme conditions, edge cases
- **Regime Compatibility**: Behavior across normal, volatile, illiquid, expiry regimes

## Test Principles

### 1. Determinism
All tests verify that same inputs produce same outputs. This is critical for production deployment.

### 2. Performance Targets
- Rust: Sub-millisecond for core operations
- Python: < 10ms for signal computations
- Scalability: Linear or better complexity

### 3. Graceful Degradation
Systems must degrade gracefully under adverse conditions:
- Confidence scores reflect data quality
- Silence decisions trigger appropriately
- No crashes on edge cases

### 4. Edge Case Handling
Tests cover:
- Zero values
- Negative values
- Extreme values (very large/small)
- Missing data
- Contradictory signals
- Empty datasets

### 5. Production Readiness
Tests validate:
- Thread safety for concurrent operations
- Memory efficiency with large datasets
- Proper error propagation
- Bounds checking (confidence [0,1], pressure [-1,1])

## CI/CD Integration

### Pre-commit Checks
```bash
# Rust: Compile and test
cd core/rust
cargo fmt --check
cargo clippy
cargo test

# Python: Run new tests
cd research/python
python3 -m pytest tests/test_signal_integration.py tests/test_performance.py tests/test_chaos.py
```

### Continuous Integration
The test suite is designed for CI/CD pipelines:
- Fast execution (< 1 second total)
- Deterministic results
- Clear failure messages
- No external dependencies required

## Adding New Tests

### Rust Tests

1. Create test file in `core/rust/tests/`
2. Import necessary modules from `galactus_core`
3. Use `#[test]` attribute
4. Follow existing patterns for test structure

Example:
```rust
use galactus_core::confidence::*;

#[test]
fn test_new_feature() {
    // Arrange
    let input = /* ... */;
    
    // Act
    let result = compute_something(&input);
    
    // Assert
    assert!(result.is_valid());
}
```

### Python Tests

1. Create test file in `research/python/tests/`
2. Use `pytest` conventions
3. Organize tests into classes by functionality
4. Include docstrings

Example:
```python
import pytest

class TestNewFeature:
    """Test new feature implementation."""
    
    def test_basic_behavior(self):
        """Test basic behavior."""
        result = compute_something()
        assert result > 0
```

## Performance Benchmarking

For detailed performance analysis:

```bash
# Rust: Use criterion (when enabled)
cd core/rust
cargo bench

# Python: Use pytest-benchmark
cd research/python
python3 -m pytest tests/test_performance.py --benchmark-only
```

## Troubleshooting

### Rust Test Failures

**Compilation Errors:**
```bash
cargo clean
cargo build
cargo test
```

**Specific Test:**
```bash
cargo test test_name -- --nocapture
```

### Python Test Failures

**Import Errors:**
```bash
pip install pytest numpy
```

**Verbose Output:**
```bash
python3 -m pytest tests/test_file.py -v -s
```

**Specific Test:**
```bash
python3 -m pytest tests/test_file.py::TestClass::test_method -v
```

## Test Maintenance

### Review Schedule
- **Weekly**: Review failed tests in CI
- **Monthly**: Update performance baselines
- **Quarterly**: Review chaos scenarios for market changes

### Updating Tests
When modifying code:
1. Update relevant tests first
2. Ensure tests fail appropriately
3. Implement changes
4. Verify tests pass
5. Add new edge case tests if needed

## References

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Pytest Documentation](https://docs.pytest.org/)
- [Galactus Design Principles](../docs/00-vision-and-non-goals/design-principles.md)
- [Galactus Testing Philosophy](../docs/07-backtesting-and-validation/)
