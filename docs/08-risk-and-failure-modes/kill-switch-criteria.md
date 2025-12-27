# Galactus — Kill Switch Criteria

## Purpose
This document defines the **non-negotiable conditions** under which Galactus must partially or fully halt inference.

The kill switch exists to:
- Prevent false confidence
- Protect downstream consumers
- Preserve trust in inference

Incorrect silence is preferable to confident error.

---

## Definition: Kill Switch

A kill switch is an automatic or manual mechanism that:
- Suppresses inference output
- Freezes signal activation
- Forces the system into a safe state

---

## Mandatory Kill Conditions

Inference **must halt** if any of the following occur:

### 1. Data Integrity Failure
- Corrupted or inconsistent canonical events
- Unresolvable schema mismatch
- Missing core data across required windows

**Thresholds:**
- Data completeness must be ≥ 80%
- Schema validation must pass
- Data integrity checks must pass

### 2. Time Semantics Violation
- Event time cannot be reliably determined
- Event ordering ambiguity exceeds tolerance

**Thresholds:**
- Maximum time ambiguity: 60 seconds
- Event ordering must be consistent

### 3. Regime Indeterminacy
- Regime classification confidence below minimum threshold
- Conflicting regime signals without resolution

**Thresholds:**
- Regime confidence must be ≥ 0.30
- No unresolved regime conflicts allowed

### 4. Constraint Misidentification
- Core constraint assumptions invalidated
- Forced flow logic contradicted structurally

**Thresholds:**
- Constraint assumptions must be valid
- Forced flow logic must be consistent

### 5. Confidence System Failure
- Confidence calculation unavailable or corrupted
- Confidence not degrading when required

**Thresholds:**
- Overall confidence must be ≥ 0.30
- Data quality confidence must be ≥ 0.50
- Confidence values must be finite (no NaN or infinity)

---

## Partial Kill Conditions

Certain failures require **selective suppression**:
- Signal-level deactivation
- Instrument-level silence
- Regime-specific suspension

---

## Implementation

The kill switch is implemented in `core/rust/src/kill_switch.rs` as a deterministic evaluation system.

### Core Components

1. **KillSwitchEvaluator**: Main evaluation engine
2. **KillCondition**: Represents triggered conditions
3. **KillSwitchDecision**: Evaluation result (Proceed/Warn/Suppress/Halt)
4. **KillSwitchConfig**: Configurable thresholds

### Usage Example

```rust
use galactus_core::kill_switch::*;
use galactus_core::confidence::*;

// Create evaluator with default thresholds
let evaluator = KillSwitchEvaluator::default();

// Prepare inputs
let data_quality = DataQualityMetrics { /* ... */ };
let regime = RegimeAssessment { /* ... */ };
let structural = StructuralValidation { /* ... */ };
let confidence = OverallConfidence { /* ... */ };
let stability = StabilityIndicator { /* ... */ };

// Evaluate kill switch conditions
let decision = evaluator.evaluate(
    &data_quality,
    &regime,
    &structural,
    &confidence,
    &stability,
);

match decision {
    KillSwitchDecision::Proceed => {
        // Safe to proceed with inference
    }
    KillSwitchDecision::ProceedWithWarnings(warnings) => {
        // Proceed but attach warnings to output
    }
    KillSwitchDecision::PartialSuppress(conditions) => {
        // Suppress specific signals/instruments
    }
    KillSwitchDecision::Halt(conditions) => {
        // Must halt inference completely
        // Log conditions and enter safe state
    }
}
```

### Integration with Intent Engine

The kill switch is evaluated during intent processing:

1. **Pre-processing validation**: Check data quality and time semantics
2. **Regime validation**: Verify regime classification confidence
3. **Structural validation**: Ensure constraint and flow consistency
4. **Confidence validation**: Verify confidence system health
5. **Final decision**: Halt, suppress, warn, or proceed

---

## Recovery Rules

- Kill switches may only be lifted after root cause analysis
- Recovery must be logged
- Silent recovery is forbidden

### Recovery Process

1. **Identify root cause**: Determine what triggered kill switch
2. **Fix underlying issue**: Correct data, configuration, or logic
3. **Verify fix**: Run validation tests
4. **Document recovery**: Log the incident and resolution
5. **Resume operation**: Gradually re-enable inference

---

## Monitoring and Alerting

Kill switch activations must be:
- Logged with full context
- Alerted to operations team
- Tracked in metrics dashboards
- Reviewed in post-incident analysis

---

## Testing

All kill switch conditions must have:
- Unit tests for each trigger condition
- Integration tests with intent engine
- Stress tests for edge cases
- Recovery validation tests

See `core/rust/src/kill_switch.rs` for comprehensive test suite.

---

## Final Statement
**Galactus must know when it should stop thinking.**
