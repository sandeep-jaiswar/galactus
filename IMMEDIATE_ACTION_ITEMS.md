# Immediate Action Items - Production Readiness
## Priority-Based Deployment Roadmap

**Status:** All 4 phases complete. System ready for deployment after Priority 1 fixes.

---

## Priority 1: CRITICAL FIXES (2 hours total)

### 1. Suppress False Pressure in High-Volatility Regimes (1 hour)

**Problem:** 100% false pressure rate during March 2020 stress period

**Current Code Location:** `research/python/src/inference/engine.py` (line ~150)

**Required Change:**
```python
# Current (always fires)
if pressure_signal_strength > 0.7:
    return InferenceDecision.WAIT_FOR_CLEARER_SIGNAL

# Required (regime-aware)
if pressure_signal_strength > 0.7:
    if current_regime == "Panic Volatility" or current_regime == "Circuit Breaker":
        # Suppress false pressure during known high-volatility regimes
        pressure_signal_strength *= 0.2  # Reduce by 80%
    
    if pressure_signal_strength > 0.7:
        return InferenceDecision.WAIT_FOR_CLEARER_SIGNAL
```

**Validation:** Re-run March 2020 test → expect false pressure rate <10%

---

### 2. Add Circuit Breaker as Explicit Regime Trigger (1 hour)

**Problem:** Regime lag 14 days. System didn't recognize COVID crash as regime change.

**Current Code Location:** `research/python/src/inference/regime.py` (line ~80)

**Required Change:**
```python
# Current (only historical volatility)
regime = detect_regime_from_historical_volatility(...)

# Required (add circuit breaker detection)
daily_volatility = calculate_daily_volatility(...)
if daily_volatility > 0.08:  # >8% daily move = circuit breaker
    regime = "Circuit Breaker"  # Immediate regime flag
    confidence *= 0.5  # Reduce confidence 50%
    return regime

# Then fall back to historical detection
regime = detect_regime_from_historical_volatility(...)
```

**Validation:** Re-run March 2020 test → expect regime lag <2 days

---

### 3. Lower Kill-Switch Basis Threshold (30 mins)

**Problem:** Kill-switch never triggered during crisis (0% anticipation)

**Current Code Location:** `research/python/src/inference/kill_switch.py` (line ~40)

**Required Change:**
```python
# Current thresholds
KILL_SWITCH_BASIS_THRESHOLD = 200  # Very conservative
KILL_SWITCH_OI_CONCENTRATION = 0.25  # 25%

# Required thresholds
KILL_SWITCH_BASIS_THRESHOLD = 50   # Lower to >50bps divergence
KILL_SWITCH_CIRCUIT_BREAKER = 0.08  # >8% daily move
KILL_SWITCH_PUT_CALL_RATIO = 1.5    # >1.5 ratio (panic level)

# Combined logic
def should_trigger():
    if basis_divergence > 50 or daily_volatility > 0.08 or put_call_ratio > 1.5:
        return True
    return False
```

**Validation:** Re-run March 2020 test → expect kill-switch triggered 8-10 times on circuit breaker days

---

### 4. Validate Priority 1 Fixes (30 mins)

**Command:**
```bash
cd /media/sandeep/DataDrive/galactus
python run_comprehensive_tests.py > march2020_validation_v2.log 2>&1

# Expected results:
# - False Pressure Rate: <10% (was 100%)
# - Regime Lag: <2 days (was 14 days)
# - Kill-Switch Anticipation: >50% (was 0%)
# - High-Confidence Failure Rate: 0% (maintained)
```

---

## Priority 2: INTEGRATION TASKS (1 week)

### 1. Connect Leading Indicators to Pressure Suppression
**Estimate:** 2 hours  
**Impact:** Further reduce false positives by 50%
```python
# If basis divergence OR put/call ratio shows stress,
# suppress standard pressure detection thresholds
if leading_indicators.stress_signal:
    pressure_threshold *= 2.0  # Require 2x signal strength
```

### 2. Test on 2008 Financial Crisis Data
**Estimate:** 4 hours  
**Impact:** Ensure robustness on even worse scenario (-60% crash)
```
Get 2008 Sep-Oct crisis data:
- Lehman collapse (Sep 15, 2008)
- AIG bailout (Sep 16, 2008)
- TARP vote (Sep 29, 2008)
- Market bottom (Mar 9, 2009)

Run backtest → expect 0% high-confidence failures
```

### 3. Add Regime Transition Confidence Signals
**Estimate:** 2 hours  
**Impact:** Better guidance to clients during uncertain periods

### 4. Implement OI Concentration Liquidity Warnings
**Estimate:** 2 hours  
**Impact:** Proactive alerts before expiry liquidity dries up

---

## Priority 3: PRODUCTION MONITORING (Post-Deployment)

### 1. Kill-Switch Activation Log
Track when kill-switch triggers in production:
```json
{
  "timestamp": "2025-12-29T09:15:00Z",
  "trigger_reason": "basis_divergence",
  "basis_points": 85,
  "confidence_before": 0.75,
  "duration_hours": 2.5,
  "market_event": "earnings_surprise"
}
```

### 2. Pressure Signal Tracking
Monitor false pressure rate in production:
- Target: <5% false positive rate
- Alert if: >10% false positive rate for 2+ days
- Action: Review regime detection thresholds

### 3. Regime Transition Monitoring
Track how well system detects regime changes:
- Target: <1 day lag on regime changes
- Alert if: >3 day lag detected
- Action: Review circuit breaker detection sensitivity

### 4. Confidence Calibration Feedback
Monthly review:
- Expected vs actual success rate
- Adjust confidence degradation schedule
- Update regime-specific thresholds

---

## Deployment Checklist

### Before Going Live

```
CODE QUALITY
  [ ] Run all tests: python run_comprehensive_tests.py
  [ ] Check code coverage (target: >80%)
  [ ] Lint all files: flake8 research/python/
  [ ] Type check: mypy research/python/
  
VALIDATION
  [ ] Jan 2020 calm period: 0% high-confidence failures
  [ ] Mar 2020 stress period: 0% high-confidence failures (after Priority 1 fixes)
  [ ] 2008 financial crisis: 0% high-confidence failures
  [ ] Kill-switch triggers appropriately (>50bps OR >8% daily)
  
INFRASTRUCTURE
  [ ] GalactusDataProvider tested with live data
  [ ] Monitoring/alerting configured
  [ ] Log aggregation working
  [ ] Backup/recovery procedures documented
  
DOCUMENTATION
  [ ] API documentation complete
  [ ] Deployment guide written
  [ ] Runbook for kill-switch activation created
  [ ] Confidence degradation rationale documented
  
TEAM
  [ ] All team members trained on new features
  [ ] On-call rotation established
  [ ] Escalation procedure documented
```

---

## Estimated Timeline to Production

| Phase | Tasks | Effort | Status |
|-------|-------|--------|--------|
| **Critical Fixes** | False pressure suppression, circuit breaker, kill-switch | 2 hours | READY NOW |
| **Validation** | March 2020 re-test, 2008 test | 2 hours | READY NOW |
| **Integration** | Leading indicators link, liquidity warnings | 4 hours | This week |
| **Monitoring Setup** | Dashboards, alerts, logging | 2 hours | This week |
| **Final QA** | End-to-end testing with live data | 4 hours | This week |
| **Deployment** | Canary → Prod rollout | 2 hours | End of week |

**Total:** 16 hours of engineering work  
**Go-Live Date:** End of week (Friday) if all Priority 1 fixes validated

---

## Key Metrics to Watch Post-Deployment

```
CRITICAL (Daily)
  ├─ Kill-Switch: Check if triggered, reason, duration
  ├─ High-Confidence Failures: Should be <1%
  └─ False Pressure Rate: Should be <5%

IMPORTANT (Weekly)
  ├─ Regime Lag: Measure vs manual assessment
  ├─ Confidence Calibration: Expected vs actual accuracy
  └─ Basis Divergence: Track during expiry periods

TRENDING (Monthly)
  ├─ System uptime: Target >99.9%
  ├─ API latency: Target <100ms
  ├─ Data quality issues: Track and remediate
  └─ Model performance: Quarterly retraining decision
```

---

## Success Criteria

**Go-Live is successful if:**

1. ✅ Zero high-confidence failures on day 1
2. ✅ Kill-switch triggers <5 times/day (normal activity)
3. ✅ False pressure rate <5% (down from 100% in March test)
4. ✅ Regime transitions detected within 1 day
5. ✅ All monitoring alerts integrated and tested
6. ✅ Team confident in operation and troubleshooting

**Rollback triggers:**
- High-confidence failure rate >5% on any day
- Kill-switch stuck in triggered state >1 hour
- Regime lag >3 days on actual regime changes
- Data feed disconnection >30 minutes
- Confidence calibration error >0.5

---

## Document Locations

- **Comprehensive Assessment:** [COMPREHENSIVE_PHASE_ASSESSMENT.md](COMPREHENSIVE_PHASE_ASSESSMENT.md)
- **Jan 2020 Results:** [BACKTEST_RESULTS_JAN2020.md](BACKTEST_RESULTS_JAN2020.md)
- **Real Data Implementation:** [REAL_DATA_IMPLEMENTATION.md](REAL_DATA_IMPLEMENTATION.md)
- **Test Results (Mar 2020):** `backtest_results_march2020_stress/`

---

## Next: Execute Priority 1 Fixes

Ready to implement the 3 critical fixes? They're straightforward changes to:
1. `research/python/src/inference/engine.py` (pressure suppression)
2. `research/python/src/inference/regime.py` (circuit breaker detection)
3. `research/python/src/inference/kill_switch.py` (threshold adjustment)

Once complete, re-run tests to validate March 2020 improvements.
