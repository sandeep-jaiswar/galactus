# Galactus — Feature Discovery Rules

## Purpose of This Document

This document defines the **rules for feature discovery** in Project Galactus.

It exists to:
- Prevent feature sprawl
- Enforce capital-behavior grounding
- Avoid outcome-driven feature mining
- Maintain a clean boundary between research and production

Features are hypotheses, not assets.

---

## Definition: Feature

In Galactus, a **feature** is:

> A measurable transformation of canonical data that represents a specific aspect of capital behavior or constraint.

A feature is not:
- A signal
- A decision rule
- An optimization artifact

Features exist to support inference, not to replace it.

---

## Feature Eligibility Criteria

A feature is eligible for exploration only if:

1. It maps to a clearly defined capital behavior
2. It is derived from approved data sources
3. Its computation is deterministic
4. Its meaning is interpretable without performance results

Features that fail any criterion are rejected.

---

## Feature Discovery Scope

Feature discovery may explore:

- New ways to quantify known constraints
- Better normalization of existing metrics
- Improved detection of regime transitions
- Alternative representations of pressure dynamics

Feature discovery may not explore:
- Price-only transformations
- Outcome-optimized embeddings
- Uninterpretable composite scores

---

## Hypothesis Requirement

Every feature must be associated with a hypothesis that states:

- What capital behavior it represents
- Why this behavior should matter
- Under what conditions it should fail

Features without hypotheses are exploratory only and may not be promoted.

---

## Isolation Requirement

Feature discovery must:
- Occur only in the research layer
- Never alter production inference
- Never reuse production code paths implicitly

Isolation violations invalidate results.

---

## Parameter Discipline

Features must:
- Minimize parameter count
- Avoid hard-coded thresholds
- Justify any parameter economically

Features requiring frequent retuning are fragile and discouraged.

---

## Evaluation Without Optimization

Feature evaluation focuses on:
- Stability across regimes
- Structural plausibility
- Failure behavior

Performance optimization is not the primary criterion.

---

## Feature Documentation

Every feature must be documented with:

- Description and rationale
- Data inputs and transformations
- Assumptions
- Known failure modes
- Example interpretations

Undocumented features do not exist.

---

## Feature Comparison Rules

Features may be compared only if:
- They represent the same underlying mechanism
- Comparisons preserve interpretability
- Differences are explained structurally

Ranking features by performance alone is forbidden.

---

## Promotion Boundary

Features may cross into production only when:
- They satisfy promotion criteria
- They are converted into signals via the signal lifecycle
- Documentation is updated prior to implementation

Feature promotion is rare by design.

---

## Feature Retirement

Features that:
- Fail validation
- Become redundant
- Lose structural relevance

Must be retired and documented.

---

## Practical Implementation

### Isolation Mechanisms

To enforce these rules, Galactus provides:

1. **Feature Package** (`research/python/src/features/`)
   - Isolated Python package for feature exploration
   - Automatic production environment blocking
   - Mandatory isolation decorators

2. **`@mark_experimental` Decorator**
   - Enforces hypothesis documentation
   - Tracks assumptions and failure modes
   - Emits warnings on use
   - Prevents production usage

3. **FeatureIsolationContext**
   - Context manager for safe feature usage
   - Tracks feature usage
   - Enforces environment checks

### Usage Example

```python
from features.isolation import mark_experimental, FeatureIsolationContext

@mark_experimental(
    hypothesis="OI decay rate indicates forced position unwinding",
    assumptions=[
        "Clean derivatives data available",
        "Sufficient liquidity (>1000 contracts)",
        "Normal market regime"
    ],
    failure_modes=[
        "False signals during expiry weeks",
        "Sensitive to data quality",
        "Fails in low volume"
    ]
)
def compute_oi_decay_pressure(current_oi, previous_oi, time_delta):
    """Compute capital pressure from OI decay patterns."""
    # Feature implementation
    total_decay = sum(
        max(0, prev - current_oi.get(strike, 0))
        for strike, prev in previous_oi.items()
    )
    total_build = sum(
        max(0, current_oi.get(strike, 0) - prev)
        for strike, prev in previous_oi.items()
    )
    total = total_decay + total_build
    return (total_build - total_decay) / total if total > 0 else 0.0

# Use the feature
with FeatureIsolationContext() as ctx:
    pressure = compute_oi_decay_pressure(current, previous, 1.0)
```

### Enforcement

Features are enforced through:

1. **Automatic blocking**: Production environment prevents feature imports
2. **CI validation**: `scripts/validate_feature_isolation.sh`
3. **Tests**: Comprehensive isolation mechanism tests
4. **Documentation**: Example notebook and README

See:
- `research/python/src/features/README.md` - Usage guide
- `research/python/notebooks/exploratory/feature-discovery-example.ipynb` - Examples
- `research/python/tests/test_feature_isolation.py` - Tests

---

## Final Statement

**Features exist to clarify thinking, not to inflate models.**

In Galactus, fewer features mean clearer inference.

**Implementation**: Features are isolated in `research/python/src/features/` with enforced boundaries.
