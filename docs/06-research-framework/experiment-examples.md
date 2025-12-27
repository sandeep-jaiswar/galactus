# Experiment Design Examples

**Practical examples of well-framed experiments in Galactus**

This document provides concrete examples to illustrate proper experiment design, failure documentation, and constraint specification following Galactus principles.

---

## Purpose

These examples demonstrate:
- How to frame capital-behavior hypotheses correctly
- How to document failure modes structurally
- How to specify constraints explicitly
- How to interpret results without performance bias

Use these as reference when designing your own experiments.

---

## Example 1: FII Derivatives Roll-Over Pressure

### Well-Framed Hypothesis

**Hypothesis**: FII derivatives positions approaching monthly expiry face forced roll-over or unwinding pressure that manifests as systematic pressure in the final 3 days before expiry.

### Capital Identification
- **Capital Pool**: Foreign Institutional Investor derivatives positions (index futures and options)
- **Capital Size**: ₹50,000-80,000 crores notional in typical months
- **Liquidity Constraint**: Large positions relative to daily liquidity; cannot exit instantly without market impact

### Constraint Definition
- **Binding Constraint**: Contract expiry (last Thursday of the month)
- **Why Non-Optional**: Positions must be closed, rolled, or exercised; no extension possible
- **Time Horizon**: Final 3 trading days before expiry

### Observable Implications
- **Expected Behavior**: 
  - Increased trading volume in near-month contracts (T-3 to T-1)
  - Basis convergence acceleration
  - Higher intraday volatility in spot
  - Visible order flow imbalance in derivatives
- **Where Observable**: Nifty/BankNifty futures and options, corresponding spot indices
- **Magnitude**: 10-20% increase in volume, 5-10 bps extra basis pressure

### Falsification Criteria
- Volume spike does not materialize in expiry week
- Basis behavior is identical to non-expiry weeks
- No systematic timing pattern (T-3, T-2, T-1)
- Effect absent in low FII exposure months

### Regime Dependency
- **Valid Regimes**: 
  - Normal liquidity periods
  - High FII derivatives exposure (> ₹60,000 crores)
  - Directional trending markets
- **Invalid Regimes**:
  - Extreme volatility shocks (VIX > 35)
  - Low FII participation months
  - Circuit breaker events
- **Transition Behavior**: Effect weakens during regime uncertainty

---

### Constraint Specification Example

**Data Constraints:**
```yaml
Data Sources:
  - NSE derivatives archive (approved)
  - SEBI FII participation reports (public)
  - Event time: T+1 publication delay respected

Schema:
  - Derivatives: canonical-schemas.md v1.2
  - FII positions: participant-flows.schema v2.0

Look-Ahead Prevention:
  - All position data as of T-1 close
  - Publication delay: 1 business day
  - No intraday position inference used
```

**Methodology Constraints:**
```yaml
Determinism:
  - Fixed event window: [T-5, T+2] around expiry
  - No adaptive thresholds
  - Reproducible from raw NSE files

Parameters:
  - Window: 3 days (structural: expiry timing)
  - Volume threshold: None (raw comparison)
  - Normalization: 20-day rolling average

Time Window:
  - 24 months: Jan 2023 - Dec 2024
  - Covers: 2 bull regimes, 1 consolidation, 3 volatility spikes
  - Excludes: Mar 2020 (COVID anomaly)

Controls:
  - Baseline: Mid-month periods (T-20 to T-10)
  - Negative control: Low FII months (< ₹30,000 cr)
  - Counterfactual: Weekly options (no monthly expiry effect expected)
```

**Scope Constraints:**
```yaml
Instruments:
  - Included: Nifty, BankNifty futures (current & near month)
  - Included: ATM options (current month)
  - Excluded: Stock futures (different expiry dynamics)
  - Excluded: Far month contracts (low liquidity)

Regimes:
  - Included: Normal vol (VIX 12-25), High vol (VIX 25-35)
  - Excluded: Extreme vol (VIX > 35)
  - Boundary: Regime shift weeks excluded from analysis

Temporal:
  - Daily analysis (no intraday patterns)
  - 3-day window (T-3 to T-1)
  - Monthly cycles only
```

---

### Failure Mode Documentation Example

#### Failure Mode 1: Insufficient FII Participation

**Description**: Hypothesis fails when FII derivatives exposure is below ₹40,000 crores. Volume spike does not materialize, basis pressure is absent.

**Root Cause**: Constraint only binds at sufficient scale. Below threshold, domestic participants dominate and have different roll patterns.

**Frequency**: 3-4 months per year (typically: August, December low-activity periods)

**Detection**: 
- Check FII position at T-5
- If < ₹40,000 crores → flag as "insufficient capital"
- Confidence should be degraded to LOW

**Impact**: False positive risk if signal activates without checking FII exposure

**Mitigation**: 
```
IF FII_position < 40000:
    confidence = LOW
    reason = "insufficient_scale"
    signal = SILENT
```

**Historical Examples**:
- Aug 2023 expiry: FII at ₹35,000 cr, no volume spike observed
- Dec 2023 expiry: FII at ₹38,000 cr, minimal pressure
- Apr 2024 expiry: FII at ₹72,000 cr, clear pressure (positive case)

---

#### Failure Mode 2: Volatility Shock Dominance

**Description**: During extreme volatility events (VIX > 35), expiry pressure is overwhelmed by broader market panic. Signal is present but undetectable amid noise.

**Root Cause**: Risk management overrides calendar constraints. Participants exit positions regardless of expiry timing.

**Frequency**: 1-2 events per year (typically COVID-like shocks, election surprises)

**Detection**:
- VIX > 35 threshold
- Abnormal spreads (> 50 bps)
- Circuit breakers or trading halts

**Impact**: Signal would show false negative. Pressure exists but cannot be isolated from volatility.

**Mitigation**:
```
IF VIX > 35:
    confidence = DEGRADED
    reason = "volatility_dominates"
    note = "Pressure likely present but not measurable"
```

**Historical Examples**:
- Mar 2020 expiry: VIX at 65, COVID crash
- Feb 2021 expiry: VIX at 38, GameStop contagion
- Oct 2023 expiry: VIX at 33 (borderline, weak signal observed)

---

#### Failure Mode 3: Regime Transition Timing

**Description**: When regime shift coincides with expiry window (T-5 to T+2), causality is ambiguous. Is pressure from expiry or regime change?

**Root Cause**: Cannot distinguish forced flow from regime-driven repositioning when both occur simultaneously.

**Frequency**: 4-6 times per year

**Detection**:
- Regime classifier shows transition in [T-5, T+2] window
- Conflicting signals from regime model
- Unusual cross-asset correlation changes

**Impact**: Attribution error. May incorrectly credit expiry pressure for regime-driven moves.

**Mitigation**:
```
IF regime_transition IN [T-5, T+2]:
    confidence = MEDIUM
    reason = "ambiguous_causality"
    note = "Regime transition coincides with expiry"
```

**Historical Examples**:
- Jun 2023 expiry: Fed pivot signal on T-2
- Nov 2023 expiry: State election results on T-1
- May 2024 expiry: Clean window, clear signal (positive case)

---

### Results Interpretation Example

**Findings** (Structural Focus):

1. **Volume Pattern Confirmed**
   - Average volume increase: 15% in [T-3, T-1] vs [T-20, T-10]
   - Pattern consistent across 18 of 24 months
   - Failures aligned with documented failure modes
   - **Structural Interpretation**: Capital size creates observable market impact during forced unwinding

2. **Basis Convergence**
   - Basis narrows by additional 8 bps in expiry week
   - Effect strongest on T-1 (last day to roll)
   - Absent in low FII months (validates threshold)
   - **Structural Interpretation**: Time urgency accelerates arbitrage convergence

3. **Regime Dependency**
   - Strong effect in normal vol (16/18 months)
   - Weak/absent in high vol (2/6 months)
   - Ambiguous during regime transitions (4/24 months)
   - **Structural Interpretation**: Constraint binds predictably in stable regimes

**What We Did NOT Find**:
- No predictive power for direction (aligns with philosophy)
- No exploitable trading edge (not tested)
- Effect does not persist beyond T+2 (constraint is time-bound)

**Alternative Explanation Considered**:
"Volume spike is just general expiry activity, not FII-specific"
- **Rebuttal**: Effect scales with FII position size, not present in low-FII months
- **Rebuttal**: Pattern timing (T-3 to T-1) matches FII roll window, not general expiry (which peaks T-1 only)

---

### Recommendation Example

**Decision**: ☑ **REFINE** — Promising but needs improvements

**Rationale**:
- Hypothesis is structurally sound and validated in normal regimes
- Failure modes are understood and documented
- However: needs better FII position data granularity
- However: regime transition handling needs improvement

**Specific Improvements Needed**:
1. Add intraday FII position proxy (currently T-1 only)
2. Refine regime transition detection to avoid [T-5, T+2] overlap
3. Test across multiple years to confirm multi-cycle stability
4. Develop confidence scoring model for FII threshold

**Estimated Effort**: 2-3 weeks

**Priority**: HIGH — core to derivatives pressure framework

**Not Promotable Yet Because**:
- Data granularity insufficient for real-time application
- Confidence model needs formalization
- Edge cases (transition handling) need cleaner solution

---

## Example 2: Mutual Fund Inflow Deployment Lag

### Well-Framed Hypothesis

**Hypothesis**: Domestic mutual fund equity inflows experience 3-5 day deployment lag, creating predictable but low-confidence buying pressure in liquid large-caps during inflow windows.

### Capital Identification
- **Capital Pool**: Equity mutual fund SIP and lump-sum inflows
- **Capital Size**: ₹15,000-25,000 crores per month
- **Liquidity Constraint**: Must deploy systematically; cannot hold cash beyond regulatory limits

### Constraint Definition
- **Binding Constraint**: SEBI mandate to maintain < 5% cash in equity funds
- **Why Non-Optional**: Regulatory requirement with penalties
- **Time Horizon**: 3-5 business days post-inflow

### Observable Implications
- **Expected Behavior**:
  - Systematic buying in Nifty50 constituents during deployment window
  - Concentration in high-liquidity names (top 20 by volume)
  - No directional informativeness (buying is mechanical)
- **Where Observable**: Large-cap cash market, particularly top 20 liquid names
- **Magnitude**: 2-5 bps temporary impact on liquid names, fades within 2 weeks

### Falsification Criteria
- Buying pattern is random, not concentrated in [T+2, T+5] window
- Effect present even when inflows are negative
- No relationship between inflow magnitude and buying intensity
- Pattern absent in all regimes

### Regime Dependency
- **Valid Regimes**: All normal market conditions
- **Invalid Regimes**: None (regulatory constraint is regime-independent)
- **Transition Behavior**: Effect persists through regime changes (mechanical deployment)

---

### Failure Mode: Deployment Timing Uncertainty

**Description**: While constraint exists, exact timing of deployment varies by fund house. Window is noisy: some deploy T+1, others T+5, making aggregate signal weak.

**Root Cause**: Each AMC has own deployment schedule. Aggregate effect is dispersed over 5-day window, reducing detectability.

**Frequency**: Always present (inherent limitation)

**Detection**: 
- Signal strength varies by month
- No clear peak day within [T+2, T+5]
- Standard deviation of deployment day > 1.5 days

**Impact**: Low signal-to-noise ratio. Effect exists but often undetectable.

**Mitigation**:
```
confidence = LOW  # Always, due to timing dispersion
note = "Effect exists structurally but timing uncertain"
application = "Contextual awareness only, not actionable signal"
```

**Historical Examples**: All months show this pattern. Effect is structurally sound but practically weak.

---

## Example 3: Poorly Framed Experiment (Anti-Pattern)

### ❌ What NOT To Do

**Bad Hypothesis**: "Nifty shows higher returns in the 5 days after FII buying"

**Why This Is Wrong**:

1. **Outcome-Led Framing**
   - Framed around price return, not capital behavior
   - "Higher returns" is a performance metric, not structural mechanism
   - Could be random correlation

2. **No Capital Constraint**
   - What forces FII to buy?
   - Why can't they delay?
   - What prevents reversals?

3. **No Mechanism**
   - Why would buying cause sustained returns?
   - Is it information, pressure, or coincidence?
   - What's the structural reason?

4. **Unfalsifiable**
   - "5 days" is arbitrary
   - "Higher" is vague
   - No structural reason for failure

5. **Regime-Agnostic**
   - Assumes same behavior everywhere
   - No boundaries of applicability

### ✅ How To Fix It

**Better Hypothesis**: "FII buying during derivatives roll-over windows (T-3 to T-1 before expiry) represents forced position adjustment rather than informational trading, and should not exhibit sustained directional follow-through beyond expiry (T+2)."

**Why This Is Better**:
- Focuses on mechanism (forced vs informed)
- Specifies capital constraint (expiry timing)
- Falsifiable (follow-through test)
- Has structural explanation
- Regime-aware (derivatives roll-over context)

---

## Example 4: Constraint Specification for Edge Case

### Experiment: Dividend Announcement Front-Running

**Challenge**: Testing if unusual buying before dividend announcements reflects information leakage.

**Constraint Specification**:

```yaml
Ethical Boundaries:
  - NO analysis of individual trader behavior
  - NO inference about specific entities
  - Aggregate market-level patterns only
  
Data Privacy:
  - No order-level data (too granular)
  - Use only aggregate volume/price data
  - No broker or participant identification

Look-Ahead Prevention:
  - Announcement time: official BSE/NSE timestamp
  - Analysis window: up to T-5 (before announcement)
  - No hindsight selection of dividend stocks

Scope Limitation:
  - Top 100 stocks only (liquid names)
  - Exclude: stocks with scheduled dividend calendar (predictable)
  - Focus: surprise dividends (unscheduled)

Signal Application:
  - This is research ONLY
  - NOT for producing trading signals
  - Purpose: understand market structure integrity
  - Output: yes/no on pattern existence, no actionable recommendations
```

**Why These Constraints Matter**:
- Prevents crossing into surveillance or enforcement (not our mandate)
- Maintains public data only principle
- Avoids personalization or individual inference
- Keeps focus on structural understanding, not exploitation

---

## Key Takeaways from Examples

### Good Experiment Characteristics

✅ **Capital-Behavior Focused**
- Identifies specific capital pool
- Explains why action is forced
- Defines timing and magnitude structurally

✅ **Falsifiable**
- Clear conditions for failure
- Counterfactual tests included
- Not designed to "always work"

✅ **Regime-Aware**
- Specifies where hypothesis applies
- Acknowledges boundaries
- Documents transition behavior

✅ **Honest About Limitations**
- Documents known failure modes
- Acknowledges weak signals as weak
- Prefers silence to false confidence

✅ **Structurally Grounded**
- References market theory
- Explains mechanisms
- Not just pattern observation

### Red Flags (Anti-Patterns)

🚫 **Performance-Led**: "This strategy returns X%"  
🚫 **Narrative-Only**: "Smart money is buying"  
🚫 **Unfalsifiable**: "Works in all conditions"  
🚫 **Mechanism-Free**: "We observe correlation"  
🚫 **Cherry-Picked**: "In these specific months..."  

---

## Using These Examples

1. **Before Designing Experiment**:
   - Read Example 1 or 2 for well-framed hypothesis structure
   - Use as template for your own capital behavior identification
   - Check your hypothesis against Example 3 anti-patterns

2. **When Documenting Constraints**:
   - Refer to constraint specification examples
   - Adapt YAML format to your experiment
   - Ensure all boundary conditions explicit

3. **When Analyzing Failures**:
   - Use failure mode examples as documentation template
   - Ensure you have structural explanations
   - Include historical examples for each failure

4. **When Writing Recommendations**:
   - Follow Example 1 recommendation format
   - Be honest about "Refine" or "Reject"
   - Specify concrete improvements needed

---

## References

- [`experiment-design.md`](experiment-design.md) — Complete framework
- [`experiment-template.md`](experiment-template.md) — Blank template
- [`experiment-checklist.md`](experiment-checklist.md) — Validation checklist
- [`research-methodology.md`](research-methodology.md) — Research principles

---

**Remember**: These examples show the ideal. Real experiments may be messier, but should strive toward these standards.

The goal is not perfection but **honest, structured, capital-grounded research** that advances understanding.
