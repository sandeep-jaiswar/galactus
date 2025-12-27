# Galactus — Backtesting Guidelines

## Purpose of This Document

This document defines the **backtesting guidelines** for Project Galactus.

It exists to:
- Prevent outcome-driven research
- Avoid false confidence from historical artifacts
- Ensure backtests measure structural validity, not profit
- Preserve consistency between research and production

Backtests are diagnostic tools, not proof engines.

---

## Core Principle

Backtesting does not prove that a signal will work.  
It only tests whether a hypothesis is **structurally plausible and stable**.

---

## What Backtesting Is Allowed to Evaluate

Backtesting in Galactus may evaluate:

- Presence and persistence of capital pressure
- Correct detection of forced flows
- Stability across regimes
- Timing alignment with known constraints
- Failure behavior under stress

Backtesting may **not** evaluate trading profitability as a primary goal.

---

## Explicit Non-Goals of Backtesting

Backtesting is not used to:

- Maximize returns
- Optimize entry/exit rules
- Tune thresholds for performance
- Compare signals by PnL

Any backtest framed around profit invalidates the research.

---

## Event-Time Alignment

All backtests must:

- Use event time, not processing time
- Respect disclosure and reporting delays
- Avoid implicit future information

Any look-ahead bias invalidates results.

---

## Regime-Aware Testing

Backtests must be segmented by:
- Liquidity regimes
- Volatility regimes
- Derivatives dominance regimes
- Stress periods (expiries, shocks)

Aggregate performance across regimes is misleading.

---

## Window Discipline

Testing windows must be:
- Explicitly defined
- Justified structurally
- Consistent across runs

Rolling windows without rationale are forbidden.

---

## Parameter Handling

Parameters must:
- Be fixed before testing
- Remain stable across regimes
- Be few and interpretable

Parameter tuning during backtests is not allowed.

---

## Counterfactual Testing

Every backtest must include:
- Conditions where the signal should fail
- Scenarios with opposing pressure
- Periods of known noise

A signal that never fails is suspect.

---

## Negative and Null Results

Backtests that show:
- No effect
- Inconsistent behavior
- Frequent false positives

Must be documented and preserved.

Discarding negative results is forbidden.

---

## Reproducibility Requirements

All backtests must:
- Be reproducible from raw data
- Record schema and engine versions
- Capture configuration explicitly

Results without provenance do not exist.

---

## Interpretation Rules

Backtest results must be interpreted as:
- Evidence of structural alignment or misalignment
- Diagnostic feedback for refinement
- Input to promotion decisions

They are not marketing artifacts.

---

## Promotion Threshold

A signal may be considered for promotion only if:
- Backtests confirm structural consistency
- Failure modes are understood
- Behavior is stable across regimes

Performance is a secondary consideration.

---

## Final Statement

**Backtests reveal structure.  
They do not guarantee outcomes.**

Galactus backtests to understand, not to persuade.
