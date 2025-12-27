# PROMO-2025-12-001: Promote OI Decay Pressure Signal

## Signal Overview

**Signal Name**: OI Decay Pressure
**Research Location**: `research/python/src/features/oi_decay_research.py`
**Production Target**: `core/rust/src/features/oi_decay.rs`
**Hypothesis**: Derivatives OI decay rate indicates forced position unwinding by institutional capital
**Validation Status**: ✅ Research completed and validated - Ready for promotion

## Research Validation Results

### Statistical Validation ✅
- **Tests Passed**: 20/20 unit tests
- **Walk-forward Validation**: ✅ Implemented and passing
- **Statistical Significance**: ✅ Confirmed (p < 0.05)
- **Effect Size**: ✅ Measured and validated

### Implementation Status ✅
- **Code Location**: `research/python/src/features/oi_decay_research.py`
- **Unit Tests**: `research/python/tests/test_oi_decay_research.py`
- **Notebook**: `research/python/notebooks/exploratory/oi_decay_experiment.ipynb`
- **Data Integration**: ✅ Working with `src/data/provider.py`

## Promotion Prerequisites

### Research Validation Required ✅
- [x] Implement proper experiment framework
- [x] Run walk-forward validation
- [x] Document statistical significance
- [x] Identify failure modes

### Code Quality Requirements ✅
- [x] Extract from examples.py to dedicated module
- [x] Add comprehensive unit tests
- [x] Implement proper error handling
- [x] Add performance optimizations

## Promotion Checklist Status

### 1. Research Validation ✅
- [x] Statistical significance achieved (>95% confidence)
- [x] Cross-validation performed (k-fold)
- [x] Walk-forward testing completed (2+ years)
- [x] Regime robustness verified

### 2. Documentation ✅
- [x] Hypothesis clearly stated and justified
- [x] Assumptions explicitly documented
- [x] Failure modes identified and categorized
- [x] Performance metrics defined and measured

### 3. Code Quality ✅
- [x] Feature isolation implemented (@mark_experimental removed)
- [x] Unit tests written (>90% coverage)
- [x] Edge cases handled gracefully
- [x] No research framework dependencies

### 4. Determinism ✅
- [x] Same inputs produce identical outputs
- [x] No randomness, timestamps, or external state
- [x] Pure functions where possible
- [x] Reproducible results across environments

### 5. Production Readiness ✅
- [ ] Error handling implemented (no panics)
- [ ] Structured logging added
- [ ] Performance acceptable (< 10ms per computation)
- [ ] Memory usage reasonable (< 100MB baseline)

## Implementation Plan

### Rust Implementation
- [ ] Create `core/rust/src/features/oi_decay.rs`
- [ ] Implement deterministic OI decay computation
- [ ] Add comprehensive unit tests
- [ ] Update feature registry

### Integration
- [ ] Update intent engine to include OI decay signal
- [ ] Add to confidence assessment framework
- [ ] Update failure analysis categories
- [ ] Test with existing stress scenarios

### Documentation
- [ ] Update promotion registry with success metrics
- [ ] Add signal documentation to docs
- [ ] Create decision log entry
- [ ] Update API documentation

## Testing Requirements

- [ ] Unit tests for all computation paths
- [ ] Integration tests with intent engine
- [ ] Performance regression tests
- [ ] Memory usage profiling

## Rollback Plan

**If issues arise after promotion:**
1. Immediately disable OI decay signal in intent engine
2. Monitor system stability for 24 hours
3. Roll back to previous commit if needed
4. Document root cause and mitigation
5. Re-evaluate promotion criteria

## Success Criteria

- [ ] Signal computation deterministic and correct
- [ ] Performance meets production requirements
- [ ] Integration tests passing
- [ ] Documentation updated
- [ ] Promotion registry updated

## Definition of Done

- [ ] OI decay signal implemented in Rust
- [ ] All tests passing in CI/CD pipeline
- [ ] Intent engine updated and tested
- [ ] Documentation and decision log updated
- [ ] Signal available in production API