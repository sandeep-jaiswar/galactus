# Feature Discovery Layer

**This directory contains EXPERIMENTAL feature prototypes ONLY.**

## Purpose

This is where features are discovered, explored, and validated **before** promotion to production.

All code here is:
- Experimental
- Unvalidated
- Subject to change or deletion
- **NOT FOR PRODUCTION USE**

## Rules

### 1. Isolation Requirement

All features MUST be isolated from production:
- Use the `@mark_experimental` decorator
- Never import production inference code
- Never call live production systems
- Never write to production state

### 2. Hypothesis Requirement

Every feature must have:
- A clear hypothesis stating what capital behavior it represents
- Documented assumptions
- Known failure modes

Example:
```python
from features.isolation import mark_experimental

@mark_experimental(
    hypothesis="Forced unwinding shows in derivatives OI decay rate",
    assumptions=[
        "Clean derivatives data available",
        "Sufficient market liquidity",
        "Normal regime (not crisis)"
    ],
    failure_modes=[
        "Fails during low volume periods",
        "Sensitive to data quality issues",
        "May give false signals during expiry"
    ]
)
def compute_oi_decay_pressure(data):
    """Compute capital pressure from OI decay patterns."""
    # Implementation here
    pass
```

### 3. Documentation Requirement

Features must be documented with:
- Purpose and rationale
- Data inputs and transformations
- Assumptions
- Known failure modes
- Example usage

### 4. No Production Logic

Features MUST NOT:
- Re-implement production Rust code
- Call production APIs directly
- Access production databases
- Trigger real trading decisions
- Provide investment advice

### 5. Determinism Not Required (Yet)

Research features MAY:
- Use randomness for exploration
- Be non-deterministic
- Change frequently
- Be messy or experimental

**But**: Before promotion, they must become deterministic.

## Feature Lifecycle

```
1. Exploration → Feature created with @mark_experimental
2. Hypothesis Testing → Validate across regimes
3. Failure Analysis → Document how it breaks
4. Promotion Decision → Complete checklist
5. Rust Implementation → Deterministic production code
```

## Usage Examples

### Creating a New Feature

```python
from features.isolation import mark_experimental

@mark_experimental(
    hypothesis="Call-put OI ratio indicates forced hedging pressure",
    assumptions=["Options market data available", "Active derivatives trading"],
    failure_modes=["Fails in low liquidity", "Sensitive to expiry effects"]
)
def compute_hedge_pressure(calls_oi, puts_oi, spot_price):
    """
    Compute forced hedging pressure from option OI imbalance.
    
    Args:
        calls_oi: Call open interest by strike
        puts_oi: Put open interest by strike
        spot_price: Current spot price
    
    Returns:
        Hedge pressure metric (normalized)
    """
    # Compute imbalance
    imbalance = (calls_oi - puts_oi) / (calls_oi + puts_oi)
    
    # Normalize by distance from spot
    # ... implementation details ...
    
    return pressure_metric
```

### Using Features in Notebooks

```python
import warnings
from features.isolation import FeatureIsolationContext, mark_experimental

# Create isolation context
with FeatureIsolationContext() as ctx:
    # Use experimental features
    result = compute_hedge_pressure(calls, puts, spot)
    
    # Warnings will be emitted automatically
    print(f"Hedge pressure: {result}")
```

### Listing All Features

```python
import features.examples as examples_module
from features.isolation import list_experimental_features

# Get all experimental features
all_features = list_experimental_features(examples_module)

for name, metadata in all_features.items():
    print(f"{name}:")
    print(f"  Hypothesis: {metadata.hypothesis}")
    print(f"  Created: {metadata.created}")
    print(f"  Assumptions: {len(metadata.assumptions)}")
    print()
```

## Protection Mechanisms

### 1. Environment Guard

Features automatically check for production environment:
```python
import os
os.environ["GALACTUS_ENV"] = "production"

# This will raise FeatureIsolationError
from features import mark_experimental  # ERROR!
```

### 2. Decorator Enforcement

The `@mark_experimental` decorator:
- Prevents production use
- Emits warnings on each use
- Tracks metadata
- Requires hypothesis

### 3. Import Guard

The package itself prevents imports in production:
```python
# In production environment, this fails immediately
import features  # FeatureIsolationError raised
```

## Anti-Patterns

### ❌ Using Features in Production

**WRONG:**
```python
# In production code
from features.examples import compute_hedge_pressure
result = compute_hedge_pressure(live_data)  # VIOLATION!
```

**RIGHT:**
```python
# Features must be promoted to core/rust first
# Then call Rust implementation
import subprocess
result = subprocess.run(["./core/rust/target/release/compute_pressure"])
```

### ❌ Bypassing Isolation

**WRONG:**
```python
# Removing the decorator to "make it work"
def compute_hedge_pressure(data):  # NO ISOLATION!
    pass
```

**RIGHT:**
```python
# Keep decorator during research
@mark_experimental(hypothesis="...", ...)
def compute_hedge_pressure(data):
    pass

# Only remove after promotion to Rust
```

### ❌ No Hypothesis

**WRONG:**
```python
@mark_experimental(hypothesis="Testing something")  # Vague!
def some_feature(data):
    pass
```

**RIGHT:**
```python
@mark_experimental(
    hypothesis="Basis spread widening indicates forced arbitrage unwinding",
    assumptions=["Futures and spot data aligned", "Normal trading hours"],
    failure_modes=["Fails during circuit breakers"]
)
def compute_basis_pressure(futures, spot):
    pass
```

## Promotion Process

When a feature is ready for production:

1. **Complete the checklist**: See `docs/06-research-framework/promotion-checklist.md`
2. **Document thoroughly**: Update feature documentation
3. **Define Rust spec**: Specify how it will be implemented
4. **Get approval**: Review with maintainers
5. **Implement in Rust**: Deterministic production code
6. **Archive research code**: Keep for reference

**After promotion, the Python version is archived, not used.**

## Directory Structure

```
features/
├── __init__.py         # Package initialization with guards
├── README.md           # This file
├── isolation.py        # Isolation mechanisms and decorators
├── examples.py         # Example features (for reference)
└── [prototype_name].py # Individual feature prototypes
```

## Environment Variables

- `GALACTUS_ENV`: Set to "production" to prevent feature imports
  - Default: unset (allows research use)
  - In production deployments: Set to "production"

## Testing Features

Features should have basic sanity tests:

```python
def test_hedge_pressure_basic():
    """Sanity check for hedge pressure feature."""
    # Use synthetic data
    calls = [100, 200, 300]
    puts = [150, 250, 350]
    spot = 18500
    
    result = compute_hedge_pressure(calls, puts, spot)
    
    # Basic checks
    assert result is not None
    assert -1.0 <= result <= 1.0  # Normalized
```

**Note**: Research tests are NOT replacements for production tests.

## When to Delete Features

Delete features that:
- Have been promoted to production
- Failed validation and won't be promoted
- Are superseded by better approaches
- Are no longer relevant

**Document deletion** in notebooks or commit messages.

## References

- [Feature Discovery Rules](../../../docs/06-research-framework/feature-discovery-rules.md)
- [Promotion Checklist](../../../docs/06-research-framework/promotion-checklist.md)
- [Research Methodology](../../../docs/06-research-framework/research-methodology.md)
- [Rust vs Python Contract](../../../docs/02-system-architecture/rust-vs-python-contract.md)

## Final Statement

**Features exist to clarify thinking, not to inflate models.**

Explore freely. Validate thoroughly. Promote rarely.
