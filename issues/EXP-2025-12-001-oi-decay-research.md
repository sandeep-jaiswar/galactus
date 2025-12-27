# EXP-2025-12-001: OI Decay Pressure Signal Research

## Experiment Overview

**Hypothesis**: Derivatives OI decay rate indicates forced position unwinding due to capital constraints

**Capital Pool**: Derivatives market participants (FIIs, domestic institutions, retail)

**Expected Signal**: Rapid OI decay across strike prices signals forced position closure

## Implementation Plan

### 1. Data Requirements
- NSE_FO OI data by strike and expiry
- Time-series OI changes
- Market regime classification
- Volume data for liquidity assessment

### 2. Feature Development
- Implement OI decay computation algorithm
- Add feature isolation and documentation
- Create comprehensive test cases

### 3. Validation Framework
- Walk-forward validation across 2+ years
- Regime-specific performance analysis
- Statistical significance testing
- Failure mode identification

### 4. Stress Testing
- Test during expiry weeks
- Validate in low liquidity conditions
- Check data quality edge cases

## Success Criteria

- Signal shows statistical significance (p < 0.05)
- Performance stable across market regimes
- Clear failure modes identified
- Code ready for promotion consideration

## Files to Create
- `research/python/src/features/oi_decay_pressure.py`
- `research/python/notebooks/experiments/oi_decay_experiment.ipynb`
- `research/python/tests/test_oi_decay.py`

## Definition of Done
- [ ] Hypothesis tested with sufficient data
- [ ] Results peer-reviewed and documented
- [ ] Code follows research framework standards
- [ ] Ready for potential promotion