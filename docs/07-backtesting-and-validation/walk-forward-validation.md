# Galactus — Walk-Forward Validation

## Purpose

This document defines the **walk-forward validation framework** used in Project Galactus.

It exists to:
- Validate structural stability over time
- Detect regime-dependent decay early
- Prevent hidden look-ahead bias
- Ensure research behavior matches production behavior

Walk-forward validation is not about proving performance.  
It is about proving **temporal honesty**.

---

## Scope

### This Document Covers

- What walk-forward validation means in Galactus
- How walk-forward windows are constructed
- What is allowed and forbidden during walk-forward testing
- How results must be interpreted
- How walk-forward validation gates promotion

### This Document Does NOT Cover

- Strategy optimization
- Rolling parameter tuning
- PnL maximization techniques
- Adaptive or self-learning systems

Galactus does **not** adapt silently.

---

## Definition: Walk-Forward Validation

In Galactus, **walk-forward validation** is defined as:

> A sequential evaluation process where inference logic is frozen, applied forward in event time, and evaluated strictly using information available up to each point.

The system is treated as if it were **live**, even when tested historically.

---

## Core Principles

### 1. Event-Time Fidelity

- All validation aligns to event time
- No future events are visible, even implicitly
- Disclosure and reporting delays are respected

Any look-ahead contamination invalidates results.

---

### 2. Logic Freezing

During walk-forward validation:

- Signal logic is frozen
- Parameters are frozen
- Thresholds are frozen
- Regime definitions are frozen

Adaptive behavior is forbidden unless explicitly modeled and documented.

---

### 3. Sequential Exposure

Inference must be evaluated:

- One event window at a time
- In chronological order
- Without backfilling or retroactive correction

Retroactive “fixes” are not allowed during validation.

---

## Walk-Forward Window Design

### Training / Discovery Window

- Used only for hypothesis formulation
- Ends **before** walk-forward evaluation begins
- Must not overlap with validation windows

Results here are not considered evidence.

---

### Walk-Forward Evaluation Windows

- Applied sequentially
- Each window uses only prior information
- Window size must be explicitly justified

Overlapping windows must be documented and justified.

---

## Regime Coverage Requirements

Walk-forward validation must include:

- Multiple liquidity regimes
- Multiple volatility regimes
- At least one expiry-heavy period
- At least one regime transition window

Validation limited to calm periods is insufficient.

---

## Allowed Observations During Walk-Forward

During validation, you may observe:

- Whether pressure was detected
- Whether forced flow was identified
- How confidence evolved
- How inference degraded or resolved

You may **not** change logic based on observations.

---

## Forbidden Practices (Explicit)

The following invalidate walk-forward validation:

- Parameter adjustment mid-validation
- Threshold tuning based on observed outcomes
- Reclassifying regimes retroactively
- Selectively excluding bad windows
- Smoothing results across windows

These are forms of hindsight bias.

---

## Evaluation Criteria

Walk-forward validation evaluates:

- Structural consistency across time
- Stability of signal activation
- Correct suppression during invalid regimes
- Confidence degradation behavior
- Known failure mode manifestation

It does **not** evaluate profitability.

---

## Failure Expectations

Failures during walk-forward validation are expected and valuable.

Failures must be:
- Documented
- Categorized (data, regime, model, confidence)
- Used to refine scope or invalidate hypotheses

Ignoring failures invalidates validation.

---

## Documentation Requirements

Each walk-forward validation must document:

- Window definitions and rationale
- Frozen logic and configuration
- Observed behavior per window
- Regime context
- Failures and anomalies
- Conclusion (promote, refine, reject)

Undocumented validation does not exist.

---

## Relationship to Promotion

A signal **cannot be promoted** unless:

- Walk-forward validation shows stable behavior
- Failures are understood and acceptable
- No hidden adaptivity is present
- Confidence behaves conservatively

Backtests without walk-forward validation are incomplete.

---

## Acceptable Outcomes

It is acceptable for a signal to:

- Activate infrequently
- Go silent for long periods
- Fail under certain regimes
- Be rejected after validation

It is unacceptable for a signal to:
- Appear consistently “right”
- Avoid documented failure
- Improve through silent adjustment

---

## Final Statement

**Walk-forward validation ensures Galactus never cheats time.**

If a signal cannot survive the future honestly,  
it does not belong in the present.
