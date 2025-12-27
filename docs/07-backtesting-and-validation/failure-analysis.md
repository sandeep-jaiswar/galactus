# Galactus — Failure Analysis

## Purpose of This Document

This document defines the **failure analysis framework** for Project Galactus.

It exists to:
- Identify when and why inference fails
- Prevent repetition of known mistakes
- Distinguish model failure from data failure
- Convert breakdowns into structural learning

Failure is not an exception in Galactus.  
It is a signal.

## Implementation

The failure analysis framework is implemented in the Rust core at:
**`core/rust/src/failure_analysis/`**

The implementation provides:
- **Structured categorization** of all failure types
- **Complete documentation** of failure metadata
- **Pattern detection** for repeated failures
- **Learning loop** that feeds into research and signal refinement

See module documentation for usage examples and API details.

---

## Core Belief

Every incorrect inference contains more information than a correct one.

Ignoring failure is the fastest way to lose structural understanding.

---

## Definition: Failure

In Galactus, a **failure** occurs when:

- Inference contradicts observable capital behavior
- Confidence remains high despite incorrect context
- A signal activates outside its valid regime
- A known constraint fails to materialize as expected

Failure is defined structurally, not financially.

---

## Categories of Failure

Galactus classifies failures into the following categories:

---

## 1. Data Failures

### Description

Failures caused by data issues.

Examples:
- Missing or delayed events
- Incorrect schema interpretation
- Stale or corrupted inputs

### Response

- Degrade confidence
- Improve data diagnostics
- Do not adjust inference logic to compensate

---

## 2. Model Assumption Failures

### Description

Failures caused by incorrect or outdated assumptions.

Examples:
- Constraint no longer binding
- Market structure change
- Participant behavior shift

### Response

- Re-examine assumptions
- Update documentation
- Consider model revision or deprecation

---

## 3. Regime Misclassification

### Description

Failures caused by incorrect regime inference.

Examples:
- Treating fragile liquidity as normal
- Missing regime transitions
- Overstaying in an outdated regime

### Response

- Improve regime detection
- Increase uncertainty during transitions
- Add explicit transition states

---

## 4. Signal Misapplication

### Description

Failures where a valid signal is applied in an invalid context.

Examples:
- Using expiry-driven signals far from expiry
- Applying derivatives pressure logic in cash-dominant regimes

### Response

- Tighten applicability rules
- Narrow signal scope
- Improve documentation

---

## 5. Overconfidence Failures

### Description

Failures where inference sounded more certain than evidence allowed.

Examples:
- High confidence under degraded data
- Suppressed ambiguity flags
- Ignored conflicting signals

### Response

- Strengthen confidence degradation rules
- Add new hard stop conditions

---

## Failure Detection Mechanisms

Failures may be detected via:
- Post-event analysis
- Research review
- Monitoring alerts
- Consumer feedback (non-actionable)

Detection does not require consensus.

---

## Failure Documentation

Every failure must be documented with:

- Description of failure
- Category
- Root cause analysis
- Affected signals or components
- Corrective action (if any)

Undocumented failures are repeated failures.

---

## Learning Loop

Failure analysis feeds into:
- Research hypotheses
- Signal refinement
- Regime evolution
- Documentation updates

Learning must be explicit.

---

## What Failure Analysis Must Not Do

Failure analysis must not:
- Optimize signals for hindsight performance
- Justify violations of design principles
- Lead to ad-hoc patches

Fixes must address cause, not symptoms.

---

## Failure as a Gatekeeper

Repeated failures may trigger:
- Signal deprecation
- Model revision
- Architectural review

Ignoring repeated failure is unacceptable.

---

## Cultural Enforcement

Galactus encourages:
- Early admission of failure
- Explicit uncertainty
- Documentation over justification

Confidence without humility is a bug.

---

## Final Statement

**Galactus improves not by being right often,  
but by understanding clearly when it is wrong.**

---

## Using the Implementation

The failure analysis framework is available as a Rust module in the production core.

### Basic Usage

```rust
use galactus_core::failure_analysis::*;

// Create an analyzer
let mut analyzer = FailureAnalyzer::new();

// Record a failure
let failure = FailureRecord::new(
    FailureCategory::DataFailure,
    "Missing OI data for NSE FO".to_string(),
    "Data provider API timeout".to_string(),
    vec!["oi_signal".to_string(), "derivatives_pressure".to_string()],
);
analyzer.record_failure(failure);

// Detect patterns
let patterns = analyzer.detect_patterns();

// Generate learning report
let report = analyzer.generate_learning_report();
```

### Querying Failures

```rust
// Get failures by category
let data_failures = analyzer.recorder().get_by_category(&FailureCategory::DataFailure);

// Check for repeated failures
if analyzer.has_repeated_failures("problematic_signal") {
    // Take corrective action
}

// Check if component should be deprecated
if analyzer.should_deprecate_component("bad_signal") {
    // Trigger deprecation process
}
```

### Integration Points

The failure analysis framework integrates with:
- **Research Framework**: Failures inform hypothesis generation
- **Signal Lifecycle**: Repeated failures trigger signal deprecation
- **Confidence System**: Failure patterns influence confidence degradation rules
- **Documentation**: All failures must be documented with full context

For complete API documentation, see: `core/rust/src/failure_analysis/mod.rs`
