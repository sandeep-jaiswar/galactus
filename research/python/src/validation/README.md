# Walk-Forward Validation Framework

## Overview

This module implements the walk-forward validation framework for Project Galactus as defined in [`docs/07-backtesting-and-validation/walk-forward-validation.md`](../../../docs/07-backtesting-and-validation/walk-forward-validation.md).

**Purpose**: Ensure temporal honesty in signal validation by enforcing event-time fidelity and frozen logic during evaluation.

## Core Principles

1. **Event-Time Fidelity**: All validation aligns to event time with no future information leakage
2. **Logic Freezing**: Signal logic, parameters, and thresholds are frozen before validation
3. **Sequential Exposure**: Inference evaluated one window at a time in chronological order
4. **Regime Coverage**: Validation must span multiple market regimes
5. **Documentation**: All failures and observations must be documented

## Module Structure

```
validation/
├── __init__.py          # Public API exports
├── config.py            # ValidationConfig dataclass
├── walk_forward.py      # Core validator orchestration
└── rules.py             # Validation rule enforcement
```

## Quick Start

```python
from validation import WalkForwardValidator, ValidationConfig

# 1. Create frozen configuration
config = ValidationConfig(
    training_window_days=90,
    validation_window_days=30,
    step_size_days=30,
)

# 2. Initialize validator
validator = WalkForwardValidator(config)

# 3. Freeze logic (must be done before validation)
validator.freeze_logic(
    logic_code="def signal(x): return x > threshold",
    parameters={"lookback": 20},
    thresholds={"threshold": 0.5},
    regime_definitions={}
)

# 4. Set training window
validator.set_training_window(
    start_time=datetime(2023, 1, 1),
    end_time=datetime(2023, 3, 31),
    justification="Q1 2023 hypothesis discovery"
)

# 5. Add validation windows with regime labels
validator.add_validation_window(
    start_time=datetime(2023, 4, 1),
    end_time=datetime(2023, 4, 30),
    regime_labels=["high_liquidity", "low_volatility"]
)

# 6. Evaluate each window
def inference_func(start, end):
    # Apply frozen logic to data
    return {"observations": "..."}

validator.evaluate_window(0, inference_func)

# 7. Finalize and get results
result = validator.finalize(documentation="Validation summary...")
```

See [`examples/walk_forward_validation_example.py`](../examples/walk_forward_validation_example.py) for complete usage.

## Key Components

### ValidationConfig

Frozen configuration for validation parameters. Cannot be modified after creation.

**Required Parameters**:
- `training_window_days`: Duration of training window
- `validation_window_days`: Duration of each validation window
- `step_size_days`: Step between validation windows

**Enforcement Flags** (defaults ensure temporal honesty):
- `enforce_event_time=True`: Cannot be disabled
- `respect_disclosure_delays=True`: Respect data availability delays
- `require_multiple_liquidity_regimes=True`: Require liquidity diversity
- `require_multiple_volatility_regimes=True`: Require volatility diversity
- `require_expiry_period=True`: Must include expiry-heavy period
- `require_regime_transition=True`: Must include regime transition

### WalkForwardValidator

Main orchestrator for walk-forward validation.

**Key Methods**:
- `freeze_logic()`: Freeze signal logic before validation
- `set_training_window()`: Define training/discovery window
- `add_validation_window()`: Add validation window sequentially
- `evaluate_window()`: Evaluate a single window
- `finalize()`: Complete validation and get results

**Enforced Rules**:
- Logic must be frozen before evaluation
- Training window cannot overlap validation windows
- Validation windows must be chronological
- Overlapping windows require justification
- Regime coverage must meet requirements

### ValidationResult

Complete validation outcome with pass/fail decision.

**Attributes**:
- `passed`: Boolean indicating if validation passed
- `failures`: List of validation failures
- `warnings`: List of warnings (e.g., no documented failures)
- `regime_coverage`: Count of windows per regime type
- `total_validation_days`: Total days covered by validation
- `documentation`: Required documentation of results

## Validation Rules

### EventTimeValidator

Enforces event-time fidelity and detects look-ahead bias.

```python
from validation.rules import EventTimeValidator

validator = EventTimeValidator()

# Check data access respects event-time
violation = validator.validate_data_access(
    data_timestamp=datetime(2023, 1, 1, 9, 0),
    access_timestamp=datetime(2023, 1, 1, 10, 0),
    disclosure_delay_hours=2  # Required delay
)

if violation:
    print(f"Look-ahead bias detected: {violation.description}")
```

### LogicFreezeValidator

Detects parameter or threshold changes during validation.

```python
from validation.rules import LogicFreezeValidator

validator = LogicFreezeValidator(frozen_logic)

# Check if parameter changed
violation = validator.check_parameter_change(
    "threshold", old_value=0.5, new_value=0.6
)

if violation:
    print(f"Parameter changed: {violation.description}")
```

### RegimeCoverageValidator

Validates regime coverage across validation windows.

```python
from validation.rules import RegimeCoverageValidator

validator = RegimeCoverageValidator()

window_regimes = [
    ["high_liquidity", "low_volatility"],
    ["low_liquidity", "high_volatility"],
    ["expiry_heavy", "regime_transition"]
]

violations = validator.validate_coverage(window_regimes)
```

### ForbiddenPracticesDetector

Detects practices that invalidate validation:
- Unjustified window exclusion
- Retroactive regime reclassification
- Result smoothing across windows
- Retroactive fixes during validation

```python
from validation.rules import ForbiddenPracticesDetector

detector = ForbiddenPracticesDetector()

# Detect retroactive changes
violation = detector.detect_retroactive_reclassification(
    window_index=3,
    original_regimes=["high_volatility"],
    new_regimes=["normal_volatility"]  # Changed!
)
```

## Forbidden Practices

The following practices **invalidate** walk-forward validation:

❌ **Parameter adjustment mid-validation**
```python
# WRONG - changing parameters during validation
validator.freeze_logic(..., parameters={"threshold": 0.5})
# ... evaluate some windows ...
validator.freeze_logic(..., parameters={"threshold": 0.6})  # FORBIDDEN!
```

❌ **Threshold tuning based on outcomes**
```python
# WRONG - tuning based on observed results
result = validator.evaluate_window(0, inference_func)
if result.observations['performance'] < target:
    # Adjust threshold  # FORBIDDEN!
```

❌ **Retroactive regime reclassification**
```python
# WRONG - changing regime labels after seeing results
validator.add_validation_window(..., regime_labels=["high_volatility"])
# ... evaluate ...
# Oh, that didn't work well, let's call it "transition" instead  # FORBIDDEN!
```

❌ **Selective window exclusion**
```python
# WRONG - excluding windows with poor results
for i in range(total_windows):
    result = validator.evaluate_window(i, inference_func)
    if result.observations['performance'] < threshold:
        continue  # Skip bad windows  # FORBIDDEN!
```

❌ **Result smoothing**
```python
# WRONG - averaging or smoothing results across windows
raw_results = [0.8, 0.3, 0.9, 0.2]
reported_results = [mean(raw_results)] * len(raw_results)  # FORBIDDEN!
```

## Testing

Run the test suite:

```bash
# From research/python directory
python tests/validation/test_walk_forward.py
python tests/validation/test_rules.py
```

All tests should pass before using the framework.

## Design Notes

### Why Frozen Logic?

Frozen logic prevents silent adaptation during validation. Any change to logic, parameters, or thresholds invalidates the validation because it introduces hindsight bias.

### Why Regime Coverage?

Validation limited to favorable regimes is insufficient. Signals must demonstrate stable behavior across diverse market conditions including stress periods, expiry days, and regime transitions.

### Why Document Failures?

Signals that "never fail" are suspect. Documented failures demonstrate honest evaluation and help understand signal limitations.

### Why No Overlapping Windows?

Overlapping windows can introduce dependencies between validation results. Non-overlapping windows ensure each evaluation is independent unless explicitly justified.

## Integration with Galactus

This is a **research-layer tool** for validating signals before promotion to production.

**Promotion Path**:
1. Develop signal in Python research layer
2. Validate using walk-forward framework
3. Document results and failures
4. If validation passes, proceed with promotion checklist
5. Implement validated logic in Rust core

See [`docs/06-research-framework/promotion-checklist.md`](../../../docs/06-research-framework/promotion-checklist.md) for promotion requirements.

## References

- [Walk-Forward Validation](../../../docs/07-backtesting-and-validation/walk-forward-validation.md) - Complete specification
- [Backtest Design](../../../docs/07-backtesting-and-validation/backtest-design.md) - Backtesting principles
- [Promotion Checklist](../../../docs/06-research-framework/promotion-checklist.md) - Requirements for promotion

## Philosophy

> **Walk-forward validation ensures Galactus never cheats time.**
> 
> If a signal cannot survive the future honestly, it does not belong in the present.

— From `docs/07-backtesting-and-validation/walk-forward-validation.md`
