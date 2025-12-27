# Failure Analysis Module

This module implements the failure analysis framework as defined in:
[`docs/07-backtesting-and-validation/failure-analysis.md`](../../../docs/07-backtesting-and-validation/failure-analysis.md)

## Overview

The failure analysis framework provides a structured approach to:
- **Categorize** inference failures into well-defined types
- **Document** failures with complete metadata and context
- **Detect patterns** in repeated failures
- **Trigger actions** based on failure analysis (deprecation, revision, review)
- **Feed the learning loop** to improve system quality over time

## Philosophy

> "Every incorrect inference contains more information than a correct one."

Galactus treats failure as a signal, not an exception. This module ensures every failure is captured, categorized, and converted into learning.

## Components

### 1. Types (`types.rs`)

Defines core types:
- **`FailureCategory`**: Five categories of failures with specific response strategies
- **`FailureRecord`**: Complete record of a failure with metadata
- **`FailurePattern`**: Detected patterns from analyzing multiple failures
- **`FailureTrigger`**: Actions to take based on patterns

### 2. Recorder (`recorder.rs`)

Manages failure storage and retrieval:
- Record failures with unique IDs
- Query by category, component, time range
- Track addressed vs unaddressed failures
- Simple in-memory storage (to be extended for persistence)

### 3. Analyzer (`analyzer.rs`)

Analyzes failures and generates insights:
- Detect patterns across failures
- Identify repeated failures for components
- Recommend deprecation or revision
- Generate learning reports for research team

## Usage Examples

### Recording a Failure

```rust
use galactus_core::failure_analysis::*;

let mut analyzer = FailureAnalyzer::new();

// Record a data failure
let failure = FailureRecord::new(
    FailureCategory::DataFailure,
    "Missing OI data for last 2 hours".to_string(),
    "Data provider outage".to_string(),
    vec!["oi_signal".to_string(), "derivatives_pressure".to_string()],
);

let failure_id = analyzer.record_failure(failure);
```

### Marking Failures as Addressed

```rust
// Add corrective action when recording
let failure = FailureRecord::new(
    FailureCategory::SignalMisapplication,
    "Expiry signal fired 10 days before expiry".to_string(),
    "Signal applied outside valid temporal window".to_string(),
    vec!["expiry_pressure_signal".to_string()],
).with_corrective_action("Narrowed signal applicability window to 3 days".to_string());

analyzer.record_failure(failure);

// Or mark later
analyzer.recorder_mut().mark_addressed(&failure_id, "Fixed in v2.1".to_string());
```

### Detecting Patterns

```rust
// Analyze all failures
let patterns = analyzer.detect_patterns();

for pattern in patterns {
    println!("Category: {:?}", pattern.category);
    println!("Count: {}", pattern.count);
    println!("Requires action: {}", pattern.requires_action);
    
    if let Some(trigger) = pattern.recommended_trigger {
        println!("Recommended: {:?}", trigger);
    }
}
```

### Checking Component Health

```rust
// Check if a component has repeated failures
if analyzer.has_repeated_failures("problematic_signal") {
    println!("Warning: Signal has repeated failures");
}

// Check if component should be deprecated
if analyzer.should_deprecate_component("bad_signal") {
    println!("Critical: Signal should be deprecated");
}
```

### Generating Learning Reports

```rust
let report = analyzer.generate_learning_report();

println!("Total failures: {}", report.total_failures);
println!("Unaddressed: {}", report.unaddressed_failures);

// Components needing attention
for component in report.components_needing_attention {
    println!("⚠️  Attention needed: {}", component);
}

// Deprecation candidates
for component in report.deprecation_candidates {
    println!("❌ Should deprecate: {}", component);
}
```

### Querying Failures

```rust
// By category
let data_failures = analyzer.recorder()
    .get_by_category(&FailureCategory::DataFailure);

// By component
let signal_failures = analyzer.recorder()
    .get_by_component("oi_signal");

// By time range (Unix timestamps)
let recent = analyzer.recorder()
    .get_by_time_range(start_time, end_time);

// Unaddressed only
let unaddressed = analyzer.recorder()
    .get_unaddressed();
```

## Failure Categories

### 1. DataFailure
**Examples**: Missing data, delays, corruption  
**Response**: Degrade confidence, improve diagnostics, don't adjust inference logic

### 2. ModelAssumptionFailure
**Examples**: Outdated assumptions, structural changes  
**Response**: Re-examine assumptions, update docs, consider revision

### 3. RegimeMisclassification
**Examples**: Missing transitions, wrong regime detection  
**Response**: Improve detection, increase uncertainty during transitions

### 4. SignalMisapplication
**Examples**: Valid signal in wrong context  
**Response**: Tighten applicability rules, narrow scope

### 5. OverconfidenceFailure
**Examples**: High confidence despite poor evidence  
**Response**: Strengthen confidence degradation rules

## Integration Points

### With Research Framework
- Failures inform hypothesis generation
- Pattern detection guides research priorities
- Learning reports feed into experiment design

### With Signal Lifecycle
- Repeated failures trigger signal review
- Persistent failures trigger deprecation
- Addressed failures validate improvements

### With Confidence System
- Failure patterns influence confidence rules
- Category-specific degradation strategies
- Hard stops for critical failure patterns

### With Documentation
- All failures must be documented
- Root cause analysis required
- Corrective actions tracked

## Design Principles

1. **Determinism**: Same inputs → same categorization
2. **Completeness**: Every failure fully documented
3. **Transparency**: Nothing hidden or explained away
4. **Learning**: Failures convert to structural improvements
5. **Action-oriented**: Patterns trigger concrete steps

## Testing

Comprehensive tests ensure:
- Correct categorization of failures
- Deterministic pattern detection
- Proper threshold-based triggering
- Learning report accuracy
- Query functionality

Run tests:
```bash
cargo test failure_analysis
```

## Future Enhancements

- Persistent storage (database/structured logs)
- Time-series analysis of failure rates
- Cross-correlation between failure types
- Automated alerting for critical patterns
- Integration with monitoring systems

## References

- [Failure Analysis Documentation](../../../docs/07-backtesting-and-validation/failure-analysis.md)
- [Research Framework](../../../docs/06-research-framework/)
- [Signal Lifecycle](../../../docs/04-signal-and-metrics/signal-lifecycle.md)
