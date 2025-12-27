# Galactus — Normalization Standards

## Purpose of This Document

This document defines the **normalization standards** for all metrics used in Project Galactus.

It exists to:
- Prevent scale-driven misinterpretation
- Enable cross-instrument comparison
- Preserve interpretability across regimes
- Ensure metrics reflect pressure, not size

Metrics without normalization are not valid.

---

## Core Principle

Raw values are meaningless without context.

Normalization converts raw measurements into **comparable pressure signals**.

### Scale Illusion Problem

**Scale illusion** occurs when metrics from instruments of different sizes, liquidity, or regimes appear comparable but actually represent vastly different market pressures.

#### Common Illusions

1. **Capital Size Illusion**
   - Problem: "10M shares traded" appears large
   - Reality: For a small-cap (50M float), it's 20% of float—huge. For a large-cap (2B float), it's 0.5%—trivial.
   - Solution: Normalize by free float or market cap

2. **Liquidity Illusion**  
   - Problem: "5% price move" appears significant
   - Reality: In a liquid large-cap (tight spreads), it's extreme. In an illiquid small-cap, it's noise.
   - Solution: Normalize by average daily volume or bid-ask spread

3. **Time Illusion**
   - Problem: "100K contracts of OI" appears consistent
   - Reality: With 30 days to expiry, it's diffuse pressure. With 1 day to expiry, it's concentrated force.
   - Solution: Normalize by time-to-expiry with non-linear decay

4. **Volatility Illusion**
   - Problem: "2% daily move" appears meaningful
   - Reality: In a 10% vol regime, it's 2σ event. In a 30% vol regime, it's sub-1σ noise.
   - Solution: Normalize by realized or implied volatility

### The Goal

After proper normalization:
- A "pressure score of 5" means the same thing across all instruments
- Small-cap and large-cap metrics are directly comparable  
- Low-vol and high-vol regimes produce consistent interpretations
- Time urgency is correctly weighted

**Normalization is how Galactus avoids being fooled by scale.**

---

## Mandatory Normalization Dimensions

Every metric must be normalized across one or more of the following dimensions:

1. Capital Size  
2. Liquidity  
3. Time  
4. Volatility (when applicable)

The chosen dimensions must be documented.

---

## 1. Capital Size Normalization

### Purpose

To account for differences in:
- Market capitalization
- Free float
- Outstanding exposure

This prevents **scale illusion**, where larger instruments appear more significant purely due to their size.

### Standard Normalization Methods

#### Method 1A: Free Float Percentage
```
Normalized_OI = (Open_Interest × Contract_Size) / Free_Float_Market_Cap
```

**When to use:**
- Equity futures and options
- Position-based pressure metrics

**Thresholds:**
- < 1%: Low pressure
- 1-5%: Moderate pressure
- 5-10%: High pressure
- > 10%: Extreme pressure (requires investigation)

#### Method 1B: Average Daily Value Ratio
```
Normalized_Flow = Flow_Value / (Average_Daily_Traded_Value × N_days)
```

**When to use:**
- Cash market analysis
- Delivery tracking
- Institutional flow detection

**Parameters:**
- N_days: 10 or 20 (document choice)
- Minimum lookback: 5 trading days

#### Method 1C: Outstanding Shares Percentage
```
Normalized_Delivery = Delivery_Qty / Free_Float_Shares
```

**When to use:**
- Delivery pressure analysis
- Structural position accumulation
- FPI/DII flow normalization

### Rules

- **Absolute values are forbidden** without normalization
- **Free float is preferred** over total shares
- **Use consistent data source** for market cap (same timestamp)
- **Document the normalization base** explicitly
- **Handle corporate actions**: Adjust for splits, bonuses, buybacks

### Edge Cases

1. **Low free float securities** (< 20% of total):
   - Use total shares with documented justification
   - Apply stricter thresholds
   
2. **Newly listed securities** (< 90 days):
   - Use minimum 20-day average
   - Flag as "insufficient history"
   
3. **Illiquid instruments**:
   - Combine with liquidity normalization
   - Never use size normalization alone

---

## 2. Liquidity Normalization

### Purpose

To measure pressure relative to **absorption capacity**.

This prevents **liquidity scale illusion**, where similar flows have vastly different impacts based on market depth.

### Standard Normalization Methods

#### Method 2A: Volume Impact Ratio
```
Normalized_Volume = Volume / Average_Daily_Volume
where Average_Daily_Volume = mean(volume, lookback_days)
```

**When to use:**
- Flow pressure analysis
- Block trade detection
- Unusual activity screening

**Thresholds:**
- < 0.1x: Normal flow
- 0.1-0.3x: Elevated flow
- 0.3-0.5x: High flow
- > 0.5x: Extreme flow (potential price impact)

**Parameters:**
- lookback_days: 20 (standard), 10 (volatile regimes)

#### Method 2B: Depth-Adjusted Flow
```
Normalized_Flow = Flow_Size / Effective_Spread_Depth
where Effective_Spread_Depth = (Bid_Depth + Ask_Depth) / 2
```

**When to use:**
- Intraday pressure metrics
- Market microstructure analysis
- High-frequency regime detection

**Requirements:**
- Order book depth data available
- Timestamp synchronization critical

#### Method 2C: Liquidity Regime Multiplier
```
Adjusted_Threshold = Base_Threshold × Liquidity_Factor
where Liquidity_Factor = f(Bid-Ask_Spread, Depth, Volume_Volatility)
```

**Liquidity Factors:**
- High Liquidity (tight spread, deep book): 1.0
- Normal Liquidity: 1.5
- Low Liquidity (wide spread, thin book): 2.5
- Crisis Liquidity (breakdown): 5.0+

### Rules

- **Liquidity regimes must be classified** before applying thresholds
- **Illiquid instruments require stricter thresholds** (multiply by 2-3x)
- **Time-of-day effects**: Adjust for opening/closing hour volatility
- **Holiday effects**: Apply wider thresholds on pre/post holiday sessions
- **Document regime classification** explicitly

### Edge Cases

1. **Flash events** (sudden liquidity evaporation):
   - Detect via rapid spread widening (> 3x normal)
   - Invalidate metrics during event
   - Resume normalization post-stabilization

2. **Circuit breakers and halts**:
   - Zero liquidity during halt
   - Flag metrics as invalid
   - Restart lookback after resumption

3. **Thin instruments** (ADV < threshold):
   - Require longer lookback (60 days)
   - Use median instead of mean
   - Apply conservative multipliers (3x)

### Anti-Scale-Illusion Check

Before finalizing:
```
if abs(Normalized_Metric_Large_Cap - Normalized_Metric_Small_Cap) < threshold:
    # Check if raw values differ significantly
    if abs(Raw_Metric_Large_Cap - Raw_Metric_Small_Cap) > raw_threshold:
        # Normalization is working correctly
        pass
```

---

## 3. Time Normalization

### Purpose

To reflect urgency and constraint decay.

This prevents **time scale illusion**, where long-horizon pressure appears equivalent to imminent forced flow.

### Standard Normalization Methods

#### Method 3A: Time-to-Expiry Decay
```
Time_Pressure_Factor = 1 / sqrt(Days_To_Expiry)
Normalized_Pressure = Raw_Pressure × Time_Pressure_Factor
```

**When to use:**
- Options expiry pressure
- Futures roll pressure
- Event-driven constraints

**Time Brackets:**
- > 30 days: Low urgency (factor ≈ 0.18)
- 15-30 days: Moderate urgency (factor ≈ 0.26)
- 7-14 days: High urgency (factor ≈ 0.38)
- 3-6 days: Extreme urgency (factor ≈ 0.58)
- 1-2 days: Critical urgency (factor ≈ 1.0)
- Expiry day: Maximum urgency (factor = 2.0)

**Rationale:**
Square root decay reflects empirical observation that pressure accelerates non-linearly.

#### Method 3B: Rate-of-Change Normalization
```
Normalized_Rate = (Current_Value - Previous_Value) / Time_Delta
```

**When to use:**
- Momentum and acceleration metrics
- Pressure buildup detection
- Regime transition signals

**Time_Delta standards:**
- Intraday: 1 hour, 15 minutes
- Daily: 1 trading day
- Weekly: 5 trading days
- Never use calendar days (only trading days)

#### Method 3C: Session-Relative Timing
```
Session_Factor = f(Time_Since_Open, Time_To_Close)
Normalized_Flow = Raw_Flow × Session_Factor
```

**Session Factors:**
- First 15 min: 2.0 (opening volatility)
- 15 min - 1 hr: 1.5
- Mid-session: 1.0 (baseline)
- Last hour: 1.3
- Last 15 min: 2.5 (closing pressure)

**When to use:**
- Intraday flow analysis
- MOC/MOO order pressure
- Settlement-driven activity

#### Method 3D: Event Proximity Adjustment
```
Event_Urgency = Base_Pressure × (1 + Proximity_Factor)
where Proximity_Factor = e^(-days_to_event / decay_constant)
```

**Events requiring adjustment:**
- Index rebalancing
- Dividend ex-dates
- Earnings announcements
- Regulatory deadlines

**Decay constants:**
- Index rebalance: 5 days
- Dividend ex-date: 3 days
- Earnings: 7 days

### Rules

- **Time horizon must be explicit** in metric definition
- **Time decay must be modeled**, not assumed linear
- **Trading days only** (exclude weekends/holidays)
- **Calendar effects**: Adjust for monthly/quarterly patterns
- **Document decay function** and rationale

### Edge Cases

1. **Overnight risk** (multi-day gaps):
   - Adjust for overnight holding cost
   - Factor in weekend/holiday risk premium
   - Apply gap risk multiplier (1.2-1.5x)

2. **Expiry week anomalies**:
   - Non-linear acceleration common
   - Use empirical decay curves when available
   - Conservative: double urgency factor

3. **Event uncertainty** (scheduled but undefined):
   - Use wider confidence bands
   - Reduce signal strength proportionally
   - Document assumption explicitly

### Anti-Scale-Illusion Check

Time normalization must ensure:
```
# Pressure measured 1 day before expiry should be significantly higher
# than the same raw pressure measured 30 days before expiry
assert Pressure_1_day > Pressure_30_days × 5
```

### Combining Time Normalizations

When multiple time factors apply:
```
Combined_Factor = Product(Individual_Factors)
# NOT Sum(Individual_Factors)
```

**Example:**
Expiry-day + closing-hour pressure:
```
Total_Factor = Expiry_Factor(2.0) × Session_Factor(2.5) = 5.0
```

---

## 4. Volatility Normalization (Conditional)

### Purpose

To contextualize movement under different regimes.

This prevents **volatility scale illusion**, where similar absolute moves have different significance in calm vs volatile regimes.

### Standard Normalization Methods

#### Method 4A: Realized Volatility Adjustment
```
Normalized_Move = Price_Change / Realized_Volatility
where Realized_Volatility = std_dev(returns, lookback_period)
```

**When to use:**
- Price impact assessment
- Unusual move detection
- Regime-relative pressure

**Thresholds (in standard deviations):**
- < 1σ: Normal move
- 1-2σ: Elevated move
- 2-3σ: Significant move
- > 3σ: Extreme move (investigate)

**Lookback periods:**
- Standard: 20 days
- Volatile regime: 10 days
- Stable regime: 30 days

#### Method 4B: Implied Volatility Context
```
Normalized_Pressure = Flow_Pressure / Implied_Volatility
```

**When to use:**
- Options-related pressure
- Market expectation context
- Forward-looking adjustments

**Requirements:**
- Valid IV surface available
- ATM IV for directional pressure
- Skew-adjusted IV for tail pressure

#### Method 4C: Volatility Regime Factor
```
Regime_Adjusted_Threshold = Base_Threshold × Volatility_Factor
where Volatility_Factor = Current_Vol / Long_Term_Vol
```

**Volatility Factors:**
- Ultra-low vol (< 0.5x LT avg): Factor = 2.0 (tighter thresholds)
- Low vol (0.5-0.8x): Factor = 1.5
- Normal vol (0.8-1.2x): Factor = 1.0
- High vol (1.2-2.0x): Factor = 0.7
- Extreme vol (> 2.0x): Factor = 0.5 (looser thresholds)

**Long-term average:**
- Use 252-day (1 year) rolling average
- Minimum history: 60 days

#### Method 4D: VIX-Relative Adjustment (Index instruments)
```
Normalized_Signal = Raw_Signal × (Base_VIX / Current_VIX)
where Base_VIX = 15 (normal regime baseline)
```

**When to use:**
- Index futures/options
- Systematic risk context
- Cross-regime comparison

**VIX Brackets:**
- < 12: Complacent regime (amplify signals)
- 12-20: Normal regime (no adjustment)
- 20-30: Elevated regime (dampen signals)
- > 30: Crisis regime (significant dampening)

### Rules

- **Volatility normalization must be justified** per metric
- **Not all metrics require volatility adjustment** (only those comparing across regimes)
- **Document which volatility measure** (realized, implied, VIX)
- **Specify lookback period** explicitly
- **Handle volatility transitions** (regime shifts)

### When NOT to Use Volatility Normalization

1. **Structural constraints** (e.g., physical delivery requirements)
   - These are independent of volatility regime
   
2. **Time-bound obligations** (e.g., expiry-day settlement)
   - Constraint doesn't change with volatility
   
3. **Regulatory requirements**
   - Fixed thresholds regardless of vol regime

### Edge Cases

1. **Volatility regime shifts** (rapid vol expansion):
   - Use windowed vol (max 10 days lookback)
   - Detect shift via vol-of-vol metric
   - Adjust normalization mid-regime

2. **Zero/near-zero volatility** (unusual stability):
   - Set minimum vol floor (e.g., 5% annualized)
   - Flag as anomalous regime
   - Avoid division by near-zero

3. **Volatility clusters** (GARCH effects):
   - Use EWMA for current regime sensitivity
   - Lambda = 0.94 (standard)
   - Update daily

### Anti-Scale-Illusion Check

Volatility normalization must ensure:
```
# A 2% move in 10% vol regime should equal a 4% move in 20% vol regime
assert abs(Normalized_Move_Low_Vol - Normalized_Move_High_Vol) < tolerance
where:
    Normalized_Move_Low_Vol = 2% / 10% = 0.2
    Normalized_Move_High_Vol = 4% / 20% = 0.2
```

### Combining with Other Normalizations

Volatility normalization typically applies **last**:
```
Final_Metric = (Raw_Metric / Size_Factor) / Liquidity_Factor / Vol_Factor
```

**Exception:** Time pressure may interact with volatility:
```
# For time-sensitive metrics in volatile regimes:
Final_Metric = (Raw / Size) × Time_Decay × Vol_Adjustment
```

### Documentation Requirements

For each volatility-normalized metric, document:
1. Which volatility measure (realized/implied/VIX)
2. Lookback period and calculation method
3. Why volatility adjustment is necessary
4. How metric behaves at vol extremes (backtest)

---

## Cross-Normalization Rules

### Order of Normalization

Apply normalizations in the following sequence:

1. **Capital Size** (first - establishes base scale)
2. **Liquidity** (second - contextualizes absorption)
3. **Time** (third - adds urgency weighting)
4. **Volatility** (last - regime contextualization)

**Rationale:** Each normalization builds on the previous, creating a hierarchy from structure to context.

### Standard Composite Formula

```
Normalized_Metric = (Raw_Metric / Capital_Size_Factor) 
                    / Liquidity_Factor
                    × Time_Urgency_Factor
                    / Volatility_Factor
```

**Note the operations:**
- Division for size and liquidity (reducing to comparable scale)
- Multiplication for time (amplifying urgency)
- Division for volatility (contextualizing regime)

### Partial Normalization

Not all dimensions apply to every metric:

| Metric Type | Size | Liquidity | Time | Volatility |
|-------------|------|-----------|------|------------|
| OI Pressure | ✓ | ✓ | ✓ | Optional |
| Delivery Flow | ✓ | ✓ | ✗ | ✗ |
| Gamma Exposure | ✓ | Optional | ✓ | ✓ |
| Roll Pressure | ✓ | ✓ | ✓ | Optional |

**Document which normalizations apply and justify omissions.**

### Interpretability Preservation

After multi-dimensional normalization:
- **Units must remain clear** (e.g., "% of free float per day per volatility unit")
- **Thresholds must be documented** for the composite metric
- **Reversibility required**: Can you explain what the normalized value represents in raw terms?

### Composite Normalization Validation

Test composite metrics against these criteria:

1. **Rank preservation**: High raw pressure should remain high after normalization
2. **Scale independence**: Doubling all inputs shouldn't change relative rankings
3. **Interpretability**: A domain expert can explain what "0.5" means
4. **Stability**: Small input changes produce small output changes

### Example: Multi-Dimensional Normalization

**Scenario:** Open Interest Pressure near expiry

```
Raw_OI_Change = 1,000,000 contracts
Contract_Size = 100 shares
Free_Float = 500,000,000 shares
Avg_Daily_Volume = 5,000,000 shares
Days_To_Expiry = 2
Current_Volatility = 25%
Normal_Volatility = 20%

Step 1 - Capital Size:
Size_Normalized = (1M × 100) / 500M = 0.20 (20% of free float)

Step 2 - Liquidity:
Liquidity_Factor = 5M shares/day
Liquidity_Normalized = 0.20 / (5M/500M) = 0.20 / 0.01 = 20 days of volume

Step 3 - Time:
Time_Factor = 1 / sqrt(2) = 0.707 → apply as multiplier for urgency
Time_Adjusted = 20 × 1.414 = 28.3 pressure units

Step 4 - Volatility (optional here, but shown for completeness):
Vol_Adjusted = 28.3 × (20/25) = 22.6 final pressure units
```

**Interpretation:** The OI change represents 22.6 "standard pressure units" - meaning it's equivalent to 22.6 days of normal volume pressure, adjusted for imminent expiry and elevated volatility.

### Documentation Requirements

For any multi-dimensional normalization:
1. List all applied normalizations in order
2. Document omitted normalizations with justification  
3. Provide formula with actual values plugged in
4. State interpretation in plain language
5. Define threshold bands (low/medium/high/extreme)

---

## Forbidden Normalization Practices

### 1. Z-Score Normalization Without Context

**Problem:** Removes interpretability and meaning.

```
# FORBIDDEN
Z_Score = (Value - Mean) / StdDev
```

**Why forbidden:**
- Loses units and real-world meaning
- Changes with sample period
- No threshold interpretability
- Regime-dependent

**Allowed alternative:**
Use percentile ranks **within homogeneous groups** with documented distributions.

### 2. Percentile Ranks Across Unrelated Instruments

**Problem:** Comparing apples to oranges.

```
# FORBIDDEN
Rank_Across_All_Stocks = percentile(metric, all_universe)
```

**Why forbidden:**
- Structural differences masked
- Size/liquidity/sector effects ignored
- Meaningless cross-instrument comparison

**Allowed alternative:**
Percentile ranks **within sector/size/liquidity cohorts** with documented grouping.

### 3. Min-Max Scaling

**Problem:** Distorted by outliers and loses actual scale.

```
# FORBIDDEN  
Scaled = (Value - Min) / (Max - Min)
```

**Why forbidden:**
- Single outlier distorts entire range
- No reference to market reality
- Changes with every new extreme

**Allowed alternative:**
Use domain-specific bounds with market meaning (e.g., 0% to 100% of free float).

### 4. Normalization That Obscures Units

**Problem:** Cannot reverse-engineer real values.

```
# FORBIDDEN (if final units unclear)
Mystery_Metric = (A / B) × (C - D) / E^2
```

**Why forbidden:**
- Unexplainable to stakeholders
- Cannot validate reasonableness
- Debugging impossible

**Required:**
Every normalized metric must state final units (e.g., "% of free float per day").

### 5. Adaptive Normalization Without Bounds

**Problem:** Normalization factors change unpredictably.

```
# FORBIDDEN
Dynamic_Factor = recent_mean / all_time_mean
Normalized = Raw / Dynamic_Factor
```

**Why forbidden:**
- Non-deterministic if lookback undefined
- Can flip meaning during regime shifts
- Testing/backtesting unreliable

**Allowed alternative:**
Fixed lookback periods with documented regime adjustments.

### 6. Logarithmic Compression Without Justification

**Problem:** Arbitrary transformation.

```
# FORBIDDEN (without capital-behavior justification)
Log_Normalized = log(Raw_Value)
```

**Why forbidden:**
- Unless power-law behavior demonstrated
- Changes interpretation without reason
- May hide non-linear effects

**Allowed when:**
Power-law or multiplicative processes proven (e.g., price returns, compounding).

### 7. Industry-Standard Technical Indicators

**Problem:** Not grounded in capital behavior.

```
# FORBIDDEN
RSI_Normalized = RSI / 100
Bollinger_Pct = (Price - Lower_Band) / (Upper_Band - Lower_Band)
```

**Why forbidden:**
- Violates signal philosophy (see forbidden-metrics.md)
- Price-derived without capital context
- Pattern-based, not constraint-based

### 8. Benchmarking Against Market Index (Unjustified)

**Problem:** Imposes correlation assumption.

```
# FORBIDDEN (without capital-behavior link)
Relative_Strength = Stock_Return / Index_Return
```

**Why forbidden:**
- Assumes beta relationship
- Ignores idiosyncratic capital constraints
- Sector/factor effects masked

**Allowed when:**
Explicitly modeling index-related forced flow (e.g., index rebalancing pressure).

---

## Stability and Robustness Checks

Normalized metrics must pass the following tests before promotion to core.

### 1. Regime Stability Test

**Requirement:** Metric meaning must not flip across regimes.

**Test procedure:**
```
# Define regimes: Low Vol, Normal Vol, High Vol, Crisis
for regime in regimes:
    normalized_values = apply_normalization(raw_data[regime])
    assert sign(normalized_values) is consistent
    assert threshold_ordering(normalized_values) preserved
```

**Failure mode:**
If high pressure becomes low pressure after normalization in a different regime, the normalization is invalid.

### 2. Outlier Resistance Test

**Requirement:** Single extreme value should not distort normalization.

**Test procedure:**
```
baseline_normalized = normalize(data)
data_with_outlier = data + [extreme_value]
outlier_normalized = normalize(data_with_outlier)

# Check: 95% of values should change by < 10%
assert percentile(abs(outlier_normalized - baseline_normalized), 95) < 0.10
```

**Remediation:**
Use robust statistics (median, winsorization, trimmed mean) if outliers distort.

### 3. Interpretability at Extremes Test

**Requirement:** Metric must remain meaningful at 1st and 99th percentiles.

**Test procedure:**
```
extreme_low = percentile(normalized_metric, 1)
extreme_high = percentile(normalized_metric, 99)

# Both extremes must be explainable
assert can_explain_in_capital_behavior_terms(extreme_low)
assert can_explain_in_capital_behavior_terms(extreme_high)
```

**Example failure:**
Normalized value of "800% of free float" is implausible and indicates broken normalization.

### 4. Scale Independence Test

**Requirement:** Relative magnitudes preserved across size ranges.

**Test procedure:**
```
# Apply to small-cap and large-cap with same raw pressure
small_cap_normalized = normalize(raw_pressure, small_cap_params)
large_cap_normalized = normalize(raw_pressure, large_cap_params)

# Normalized values should be comparable
assert abs(small_cap_normalized - large_cap_normalized) < threshold
```

**This is the core anti-scale-illusion test.**

### 5. Time Consistency Test

**Requirement:** Normalization stable over time (for same raw conditions).

**Test procedure:**
```
# Apply normalization to similar market conditions 6 months apart
normalized_t0 = normalize(raw_data_t0, params_t0)
normalized_t6 = normalize(raw_data_t6, params_t6)

# If raw conditions similar, normalized should be similar
if conditions_similar(t0, t6):
    assert abs(normalized_t0 - normalized_t6) < threshold
```

**Failure mode:**
Normalization factors drifting due to creeping data issues.

### 6. Reversibility Test

**Requirement:** Can reconstruct approximate raw value from normalized value.

**Test procedure:**
```
normalized = normalize(raw, context)
reconstructed_raw = denormalize(normalized, context)

# Should recover within acceptable tolerance
assert abs(reconstructed_raw - raw) / raw < 0.05  # 5% tolerance
```

**Why important:**
Validates that normalization hasn't lost critical information.

### 7. Monotonicity Test

**Requirement:** Increasing raw pressure should produce increasing normalized pressure.

**Test procedure:**
```
for raw_1, raw_2 in sorted_pairs(raw_data):
    if raw_1 < raw_2:
        assert normalize(raw_1) < normalize(raw_2)
```

**Failure mode:**
Non-monotonic normalization indicates incorrect formula or division by correlated factor.

### 8. Distribution Sanity Check

**Requirement:** Normalized distribution should be explainable.

**Test procedure:**
```
distribution = histogram(normalized_values)

# Check for:
- Reasonable range (not all values near 0 or infinity)
- Expected shape (e.g., right-skewed for pressure metrics)
- No unexpected modes or gaps
```

**Warning signs:**
- Bimodal without market explanation
- Heavy clustering at bounds
- Unexplained gaps in distribution

### Robustness Validation Requirements

Before a normalized metric enters production:

1. ✅ Pass all 8 stability tests above
2. ✅ Backtest across at least 2 market regimes
3. ✅ Validate on at least 3 instruments of different sizes
4. ✅ Document failure modes discovered during testing
5. ✅ Define monitoring thresholds for production
6. ✅ Establish rollback criteria if normalization degrades

**Metrics that fail multiple tests are rejected.**

---

## Documentation Requirements

Every normalized metric must have complete documentation including:

### 1. Metric Definition

```
Metric Name: [Descriptive name]
Purpose: [What pressure/constraint it measures]
Category: [Constraint/Pressure/Imbalance/Liquidity/Stability]
```

### 2. Raw Input Specification

```
Raw Inputs:
- Input_1: [name, source, units]
- Input_2: [name, source, units]
- ...

Data Requirements:
- Minimum history: [X days/events]
- Update frequency: [real-time/daily/weekly]
- Data quality checks: [specific validations]
```

### 3. Normalization Steps (Detailed)

```
Step 1 - Capital Size:
Formula: [exact formula with variable definitions]
Parameters: [free float source, market cap timestamp]
Why: [capital-behavior justification]

Step 2 - Liquidity:
Formula: [exact formula]
Parameters: [lookback period, volume type]
Why: [absorption capacity justification]

Step 3 - Time:
Formula: [exact formula]
Parameters: [decay function, time unit]
Why: [urgency justification]

Step 4 - Volatility (if applicable):
Formula: [exact formula]
Parameters: [vol measure, lookback]
Why: [regime context justification]
```

### 4. Resulting Units and Interpretation

```
Final Units: [e.g., "% of free float per day"]

Interpretation Scale:
- < 1.0: [interpretation]
- 1.0 - 5.0: [interpretation]
- 5.0 - 10.0: [interpretation]
- > 10.0: [interpretation]

Real-World Example:
Value = 7.3
Meaning: [Plain English explanation with market context]
```

### 5. Anti-Scale-Illusion Validation

```
Validation Results:
- Scale Independence: [PASS/FAIL - evidence]
- Regime Stability: [PASS/FAIL - evidence]
- Outlier Resistance: [PASS/FAIL - evidence]
- Monotonicity: [PASS/FAIL - evidence]

Tested Scenarios:
- Small-cap vs Large-cap: [normalized values comparable]
- Low-vol vs High-vol: [properly adjusted]
- Near-expiry vs Far-expiry: [urgency weighted correctly]
```

### 6. Known Limitations

```
Failure Modes:
1. [Condition where normalization breaks]
   - Detection: [how to identify]
   - Mitigation: [what to do]
   
2. [Another failure mode]
   ...

Invalid Regimes:
- [Market condition where metric shouldn't be used]
- [Another invalid scenario]
```

### 7. Edge Cases and Special Handling

```
Edge Case 1: [Description]
- Occurs when: [condition]
- Handling: [special normalization or rejection]
- Example: [concrete instance]

Edge Case 2: [Description]
...
```

### 8. Parameter Configuration

```
Configurable Parameters:
- Param_1: [name]
  - Default: [value]
  - Range: [min, max]
  - Sensitivity: [impact of changes]
  
- Param_2: [name]
  ...

Regime-Specific Overrides:
- Low Liquidity Regime: [parameter adjustments]
- High Volatility Regime: [parameter adjustments]
- Crisis Regime: [parameter adjustments or metric disable]
```

### 9. Backtesting and Validation

```
Backtest Period: [start date - end date]
Instruments Tested: [list of instruments with size/liquidity diversity]
Regimes Covered: [bull, bear, sideways, crisis, etc.]

Key Findings:
- [Finding 1 with supporting evidence]
- [Finding 2]
- ...

Performance Under Stress:
- Market condition: [specific stress scenario]
  - Metric behavior: [how it performed]
  - Issues found: [any problems]
```

### 10. Monitoring and Maintenance

```
Production Monitoring:
- Metric: [what to monitor]
  - Threshold: [alert threshold]
  - Action: [what to do if breached]

Review Schedule: [quarterly/semi-annual/annual]

Deprecation Criteria:
- [Condition 1 that would require metric retirement]
- [Condition 2]
```

### Documentation Template Location

A complete template is maintained at:
```
docs/04-signal-and-metrics/templates/normalized-metric-spec.md
```

### Documentation Review Checklist

Before promoting a normalized metric:
- [ ] All 10 sections completed
- [ ] Formulas validated against implementation
- [ ] Units clearly stated and interpretable
- [ ] Edge cases tested and documented
- [ ] Failure modes identified
- [ ] Anti-scale-illusion tests passed
- [ ] Backtest results included
- [ ] Peer review completed
- [ ] Monitoring plan defined

---

## Final Statement

**Normalization is not a cosmetic step.**

It is how Galactus turns data into meaning.

### Core Mandates

1. **All metrics must be normalized** across at least one dimension
2. **Normalization must prevent scale illusion** (test and validate)
3. **Units must remain interpretable** after normalization
4. **Failure modes must be documented** before production
5. **Cross-instrument comparability** is the primary goal

### Red Flags

A normalization is **invalid** if:
- You cannot explain the final value to a domain expert
- Small-cap and large-cap produce incomparable results
- The metric flips meaning across regimes
- Outliers distort the entire distribution
- You cannot reverse-engineer approximate raw values

### Success Criteria

A normalization is **valid** if:
- Same real pressure → same normalized value (across instruments)
- Different real pressure → different normalized value (within instrument)
- Interpretation stable across time and regimes
- Stakeholders can act on normalized values with confidence

**When in doubt, normalize more conservatively.**  
**Better to under-normalize than to create false equivalences.**

---

This document is the **definitive standard** for all metric normalization in Project Galactus.

Violations of these standards are grounds for metric rejection, regardless of empirical performance.
