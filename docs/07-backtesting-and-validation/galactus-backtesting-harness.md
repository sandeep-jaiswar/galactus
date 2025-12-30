# Galactus Backtesting Harness

## Design & Operating Model

---

## 1. Purpose of the Harness

The Galactus Backtesting Harness exists to answer **one question only**:

> *"Given what was knowable at time T, did Galactus describe the market state honestly?"*

It does **not** exist to:

* Measure PnL
* Compare strategies
* Optimize parameters
* Evaluate prediction accuracy

If the harness ever starts answering "would this have made money?", it has failed.

---

## 2. Core Design Principles

The harness must enforce the following:

1. **Event-time correctness** — No future leakage; replay exactly as events arrived
2. **Deterministic replay** — Same events, same order, same results
3. **Frozen logic** — No parameter tuning during replay
4. **No outcome awareness** — Galactus doesn't see future; evaluator doesn't score correctness by price
5. **Failure-first evaluation** — Study failures, not wins
6. **Confidence honesty over correctness** — Did Galactus suppress confidence when uncertain?

These principles are enforced by **structure**, not discipline.

---

## 3. High-Level Architecture

```text
Historical Events
(canonical, ordered)
        │
        ▼
Event Replay Engine
(event-time clock)
        │
        ▼
Galactus Core (Frozen)
(regime + pressure logic)
        │
        ▼
Inference Snapshots
(state + confidence)
        │
        ▼
Evaluation Layer
(structural checks only)
        │
        ▼
Failure Ledger + Metrics
(non-PnL dashboards)
```

---

## 4. Components (Detailed)

---

## 4.1 Event Replay Engine

### Responsibility

Replays historical market events **exactly as they would have appeared in real time**.

### Inputs

* Canonical market events (price, liquidity, OI)
* Derivatives events (expiry, rebalance)
* Reference events (holidays, circuit breakers)
* Data quality events (gaps, lateness)

### Guarantees

* **Strict event-time ordering** — No batch aggregation, no end-of-day shortcuts
* **No future leakage** — Replay respects the clock at inference time
* **Deterministic sequencing** — Same seed → same event order
* **Configurable replay speed** — 1x real-time or accelerated for research
* **Late-event handling** — Events arriving after inference trigger re-computation

### Forbidden

* Batch aggregation (grouping events by day)
* End-of-day summaries
* Look-ahead alignment
* Reordering events for convenience

**Rule:** If replay cannot be trusted, the test is invalid.

---

## 4.2 Frozen Galactus Core

### Responsibility

Run Galactus **exactly as production would**, with:

* Frozen signal definitions
* Frozen regime logic
* Frozen confidence rules
* Frozen kill-switch criteria

### Rules

* ❌ No parameter tuning
* ❌ No dynamic thresholds
* ❌ No adaptive learning
* ❌ No exception overrides
* ✅ Same code as production (or marked branch)
* ✅ Same data validation rules
* ✅ Same kill-switch triggers

If you feel the urge to "just tweak something", stop the run and document why.

---

## 4.3 Inference Snapshot Recorder

At each evaluation tick (typically 5-minute bar), the harness records a **snapshot**:

```json
{
  "timestamp": "2023-06-15T10:30:00Z",
  "event_sequence": 12847,
  "instrument": "NIFTY",
  "regime": {
    "classification": "Normal Derivatives Dominance",
    "confidence": 0.92,
    "supporting_signals": ["OI_decay", "basis_pressure"]
  },
  "capital_pressure": {
    "detected": true,
    "intensity": 0.78,
    "confidence": 0.85,
    "sources": ["forced_flow", "expiry_proximity"]
  },
  "forced_flow": {
    "estimated_magnitude": 1200,
    "confidence": 0.81
  },
  "liquidity": {
    "bid_ask_spread": 12.5,
    "depth": 450,
    "quality": "normal"
  },
  "overall_confidence": 0.82,
  "stability_indicator": 0.88,
  "silenced": false,
  "kill_switch_status": "active",
  "data_quality": {
    "missing_sources": [],
    "staleness_warnings": [],
    "validation_failures": []
  }
}
```

Snapshots are:

* **Immutable** — Never modified after creation
* **Append-only** — Form a ledger
* **Replayable** — Can reconstruct the entire inference timeline
* **Timestamped** — Both event-time and record-time

They form the **ground truth of what Galactus believed at that moment**.

---

## 5. What Is Evaluated (Very Important)

The harness evaluates **inference quality**, not outcomes. Price is context, never a judge.

---

## 5.1 Regime Evaluation

### Questions Asked

* Was the regime classification reasonable given market structure at that time?
* Did regime transitions occur **near** structural breaks (not after)?
* Did Galactus cling to old regimes too long (regime lag)?
* How often did regimes conflict with each other?

### Metrics

| Metric | Interpretation | Target |
|--------|-----------------|--------|
| Regime Lag (days) | Delay between structural break and regime shift | < 1 day |
| Regime Transition Frequency | Changes per month | 8-15 |
| Regime Conflict Rate | % time multiple regimes flagged | < 5% |

### Failure Pattern

High regime lag + high confidence = **silent failure** (worst case).

---

## 5.2 Pressure Detection Evaluation

### Questions Asked

* Did Galactus detect pressure when **constraints were binding** (expiry, rebalance)?
* Did it **avoid** detecting pressure when none existed (false positives)?
* Was forced flow correctly separated from discretionary flow?

### Key Metric

**False Pressure Rate** — Percentage of inferences claiming high capital pressure when market behaved normally.

High false pressure rate + high confidence = **trains noise as signal**.

---

## 5.3 Confidence Calibration Evaluation (PRIMARY METRIC)

This is the **most important evaluation**.

### Questions Asked

* When Galactus was **wrong**, was confidence **low**?
* Did confidence degrade during **ambiguity** (conflicting signals)?
* Did high confidence correlate with **stability** and **repeatability**?
* When did confidence **collapse**? Was it justified?

### What We Track

```text
High Confidence + Wrong = SEVERE FAILURE (red flag)
High Confidence + Right = acceptable (not punished)
Low Confidence + Wrong = acceptable (system worked as designed)
Low Confidence + Right = acceptable (system worked as designed)
```

### Implementation

For every snapshot, record:

```rust
pub struct ConfidenceEvaluation {
    pub timestamp: EventTime,
    pub overall_confidence: f64,
    pub confidence_by_signal: BTreeMap<String, f64>,
    pub regime_confidence: f64,
    pub pressure_confidence: f64,
    
    // Post-hoc evaluation (only for historical analysis)
    pub structural_stability_realized: f64,  // How stable was the market?
    pub confidence_calibration_error: f64,   // Did confidence match realized stability?
}
```

---

## 5.4 Silence Evaluation

Silence is a **valid, correct output**.

### Questions Asked

* Did Galactus go **silent** when assumptions broke?
* Did it suppress inference during **data issues** (gaps, delays)?
* Did it avoid speaking during **regime conflict** (two regimes equally valid)?

### Metric

**Correct Silence Rate** — Percentage of times Galactus silenced when it should have.

```text
Silence during data gap + market moved = correct behavior
Silence during regime conflict = correct behavior
Silence during stress = correct behavior
```

---

## 5.5 Stress Survival Evaluation

Replay **known stress periods**:

* COVID crash (2020-03-23)
* Election volatility (2024-06-04)
* Expiry compressions
* Liquidity collapses
* Circuit breaker events

### Questions Asked

* Did Galactus degrade confidence appropriately?
* Did it hallucinate structure (high confidence during stress)?
* Did kill switches trigger at appropriate thresholds?
* Did it go silent when it should have?

### Pass Condition

Confidence reduced by ≥ 40% during stress periods, or system silenced entirely.

---

## 5.6 Kill-Switch Evaluation

### Questions Asked

* When were kill switches triggered?
* Were they triggered **before** major inference failures (protective)?
* Or **after** (too late)?
* How often were they triggered incorrectly (false positives)?

### Metric

**Kill-Switch Anticipation** — Percentage of kill-switch triggers that preceded 30-minute instability window.

---

## 6. What the Harness Explicitly Does NOT Do

The harness never:

* ❌ Compares inference to price movement
* ❌ Scores directional correctness ("did it predict the move?")
* ❌ Computes returns or PnL
* ❌ Labels outcomes as "right" or "wrong" based on price
* ❌ Optimizes parameters for historical fit
* ❌ Ranks signals by historical correlation

Price is **context**, not a **judge**.

---

## 7. Failure Ledger (Mandatory Output)

Every run produces a **Failure Ledger**, a table of every structural failure identified.

### Example

| Timestamp        | Event Seq | Instrument | Regime           | Confidence | Evaluation | Failure Type   | Root Cause | Silenced |
| ---------------- | --------- | ---------- | ---------------- | ---------- | ---------- | -------------- | ---------- | -------- |
| 2020-03-23 10:15 | 45821     | NIFTY      | Stable Liquidity | 0.92       | Wrong      | Overconfidence | Regime lag | No       |
| 2020-03-24 09:30 | 46104     | NIFTY      | Stress Shock     | 0.45       | Wrong      | Acceptable     | Ambiguity  | No       |
| 2024-06-04 15:00 | 123456    | NIFTY      | Election Shock   | 0.00       | N/A        | N/A            | N/A        | Yes      |

Failures are:

* **Logged** — Never deleted
* **Classified** — Into structured categories (regime lag, overconfidence, false pressure, etc.)
* **Aggregated** — Patterns emerge across time
* **Used for learning** — Feed into improvement loop

This ledger is more valuable than any performance chart.

---

## 8. Improvement Loop Integration

The harness feeds directly into improvement:

1. **Run harness** on historical period (3-6 months)
2. **Review failures** (not wins) — Study the ledger
3. **Classify failures** by category (regime lag? false pressure? overconfidence?)
4. **Decide** on correction:
   * Narrow signal scope
   * Add confidence degradation rule
   * Increase silence during ambiguity
   * Tighten kill-switch threshold
   * Update documentation
5. **Update docs** — Document the change and reasoning
6. **Freeze logic** again — Mark the new version
7. **Re-run harness** — Validate improvement without overfitting

### Success Criteria

* ❌ Improvement = "more correct predictions"
* ✅ Improvement = "fewer high-confidence failures"
* ✅ Improvement = "more silence during ambiguity"
* ✅ Improvement = "faster regime transitions"

If improvement increases silence but reduces high-confidence failures → **success**.

---

## 9. Minimal Metrics Dashboard (Non-PnL)

### Allowed Metrics

| Metric | Why | Target |
|--------|-----|--------|
| High-confidence failure rate | Overconfidence detector | < 2% |
| Regime lag (avg) | Lag in regime detection | < 1 day |
| False pressure rate | Avoid training noise as signal | < 5% |
| Silence correctness rate | Did system silence appropriately? | > 85% |
| Kill-switch anticipation | Were triggers protective? | > 60% |
| Confidence calibration error | Did confidence match realized stability? | < 0.15 |

### Forbidden Metrics

* ❌ Accuracy (% correct predictions)
* ❌ Hit rate
* ❌ Sharpe ratio
* ❌ Maximum drawdown
* ❌ Total return
* ❌ Correlation with price
* ❌ Win/loss ratio

---

## 10. Operational Modes

### Mode A — Research Validation

* **Language**: Python harness
* **Speed**: Accelerated (10-100x)
* **Scope**: 1-3 month slices
* **Purpose**: Exploratory, interactive
* **Gate**: None (research phase)

### Mode B — Pre-Promotion Validation

* **Language**: Rust harness
* **Speed**: 1x real-time (deterministic)
* **Scope**: Full 6-month period
* **Purpose**: Validation gate for promotion
* **Gate**: Strict (must pass before code merge)

### Mode C — Regression Protection

* **Language**: Rust
* **Speed**: 1x real-time
* **Scope**: Last 3 months
* **Purpose**: CI gate, prevent regression
* **Gate**: Enforced (CI fails if regression detected)

---

## 11. CI Integration (Future-Proof)

The harness should eventually run automatically:

```yaml
# .github/workflows/backtest-regression.yml
name: Backtest Regression Tests

on:
  pull_request:
    paths:
      - 'core/rust/src/regime/**'
      - 'core/rust/src/features/**'
      - 'core/rust/src/confidence/**'

jobs:
  backtest-regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build Rust harness
        run: cargo build --release -p galactus-core
      
      - name: Download historical events
        run: scripts/download_backtest_data.sh
      
      - name: Run backtest harness
        run: cargo test --release --test backtest_harness
      
      - name: Validate failure ledger
        run: python research/backtesting/validate_ledger.py
      
      - name: Check regression
        run: |
          python research/backtesting/check_regression.py \
            --baseline=baseline_metrics.json \
            --current=current_metrics.json \
            --fail-on-regression
```

Fail builds if:

* High-confidence failure rate increases > 0.5%
* Regime lag increases > 12 hours
* False pressure rate increases > 2%
* Kill-switch anticipation decreases > 10%

---

## 12. Implementation Checklist

- [ ] Event replay engine (Rust)
- [ ] Snapshot recorder (Rust)
- [ ] Structural evaluator (Rust + Python)
- [ ] Failure ledger enrichment (Rust)
- [ ] Python harness orchestrator
- [ ] Baseline metrics computation
- [ ] Regression detection
- [ ] CI integration
- [ ] Documentation (this file)
- [ ] Stress scenario harness (separate)

---

## 13. Final Principle (Do Not Forget)

> **The backtesting harness exists to teach Galactus humility.**
>
> Humility means:
> - Confidence that degrades under ambiguity
> - Silence when assumptions break
> - Fast regime transitions
> - Structural correctness over predictive accuracy
> - Low false positive rates on capital pressure
>
> If the harness starts measuring "profit", it has failed.
> If the harness starts optimizing "accuracy", it has failed.
> If the harness starts validating "wins", it has failed.
>
> Humility is the only valid metric.

---

## 14. Links & References

* [Failure Analysis Framework](failure-analysis.md)
* [Stress Scenarios](stress-scenarios.md)
* [Walk-Forward Validation](walk-forward-validation.md)
* [Signal Lifecycle](../04-signal-and-metrics/signal-lifecycle.md)
* [Event-Driven Architecture](../02-system-architecture/event-driven-architecture.md)
* [Research Methodology](../06-research-framework/research-methodology.md)

---

## 15. Change Log

| Date | Change | Author |
|------|--------|--------|
| 2025-12-29 | Initial design | Galactus Team |
