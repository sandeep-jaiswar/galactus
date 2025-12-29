# Galactus Backtesting Framework

Deterministic replay and structural evaluation of inference outputs.

## Quick Start

### Python Research Mode

```python
from galactus.backtesting import BacktestHarness, InferenceSnapshot

# Create harness
harness = BacktestHarness(
    galactus_version="0.1.0",
    run_name="my_backtest"
)

# Create snapshots (from live inference)
snapshots = [...]  # List of InferenceSnapshot objects

# Run evaluation
metrics = harness.run_backtest(snapshots)

# Export results
harness.export_results("./results/")
```

### Example Script

```bash
cd research/python
python -m backtesting.example
```

## Running Tests

### Rust

```bash
cd core/rust
cargo test --lib backtesting
```

### Python

```bash
cd research/python
python -m pytest tests/backtesting/
```

## CI Integration

The harness runs automatically on pull requests that modify:
- Core inference logic (regime, confidence, pressure)
- Backtesting code itself

View results in the GitHub Actions logs and PR comments.

## Key Concepts

### Snapshots

Immutable, append-only records of what Galactus believed at each moment.

```python
snapshot = InferenceSnapshot(
    snapshot_id="snap_0001",
    event_timestamp=datetime.now(timezone.utc),
    instrument="NIFTY",
    regime=RegimeSnapshot(...),
    capital_pressure=PressureSnapshot(...),
    overall_confidence=0.85,
    silenced=False,
    kill_switch_status="active",
)
```

### Failure Ledger

Categorized, immutable record of all identified failures.

Categories:
- `regime_lag` — Regime changes lagged structural breaks
- `false_pressure` — Capital pressure detected incorrectly
- `overconfidence` — High confidence despite uncertainty
- `incorrect_silence` — Should have spoken but stayed silent
- `incorrect_activation` — Should have silenced but kept speaking
- `kill_switch_late` — Kill-switch triggered too late
- `kill_switch_early` — Kill-switch triggered without justification

### Metrics (Non-PnL)

| Metric | Target | Interpretation |
|--------|--------|-----------------|
| High-Confidence Failure Rate | < 2% | How often was Galactus wrong while confident? |
| False Pressure Rate | < 5% | How often did it detect phantom pressure? |
| Regime Lag (days) | < 1 | How long to detect regime changes? |
| Silence Correctness Rate | > 85% | Did it silence when it should? |
| Kill-Switch Anticipation | > 60% | Did kill-switch precede instability? |
| Confidence Calibration Error | < 0.15 | Did confidence match realized stability? |

## Design Principles

1. **Event-time correctness** — No future leakage
2. **Deterministic replay** — Same events → same results
3. **Frozen logic** — No parameter tuning
4. **No outcome awareness** — Price is context, never a judge
5. **Failure-first** — Study failures, not wins
6. **Confidence honesty** — Did Galactus suppress when uncertain?

## What the Harness Does NOT Do

❌ Measure PnL
❌ Compare to price movement
❌ Score directional accuracy
❌ Optimize parameters
❌ Rank signals by correlation

If the harness ever starts answering "would this have made money?", it has failed.

## Outputs

### Files

- `metrics.json` — Summary metrics
- `failures.json` / `failures.csv` — Failure ledger
- `snapshots.jsonl` / `snapshots.csv` — Inference snapshots

### Dashboard

Key metrics are displayed in GitHub PR comments for visibility.

## Regression Thresholds

CI fails builds if:

- High-confidence failure rate increases > 0.5%
- False pressure rate increases > 2%
- Regime lag increases > 12 hours
- Kill-switch anticipation decreases > 10%
- Silence correctness decreases > 5%

## Documentation

- [Design & Operating Model](../../docs/07-backtesting-and-validation/galactus-backtesting-harness.md)
- [Failure Analysis Framework](../../docs/07-backtesting-and-validation/failure-analysis.md)
- [Stress Scenarios](../../docs/07-backtesting-and-validation/stress-scenarios.md)

## Contributing

When adding features:

1. Update the failure categories if needed (in `types.rs` / `types.py`)
2. Add test snapshots covering edge cases
3. Validate against historical stress periods
4. Document the design decision

## Contact

For questions about the harness design, see the architecture docs or open an issue.

---

> The backtesting harness exists to teach Galactus humility.
> Humility means confidence that degrades, silence when assumptions break, and structural correctness over correctness.
