# Galactus — Stress Scenarios

## Purpose

This document defines the **stress scenarios** used in Project Galactus to validate the robustness, stability, and honesty of signals, features, and inference logic.

It exists to ensure that Galactus:
- Does not overfit to calm or “normal” market conditions
- Exposes hidden fragility early
- Degrades gracefully under extreme or abnormal conditions
- Avoids false confidence during structural stress

Stress testing is not about prediction.  
It is about **survivability of inference**.

---

## Scope

### This Document Covers

- What qualifies as a stress scenario
- Categories of stress relevant to Indian markets
- How stress scenarios are applied in backtesting and validation
- How results must be interpreted and documented
- Acceptable and unacceptable behavior under stress

### This Document Does NOT Cover

- Trading strategy stress tests
- PnL drawdown analysis
- Execution or slippage modeling
- Risk management for portfolios

Stress scenarios test **inference integrity**, not profitability.

---

## Definition: Stress Scenario

In Galactus, a **stress scenario** is defined as:

> A market condition in which one or more core assumptions about liquidity, participation, timing, or constraints are violated, compressed, or distorted.

Stress scenarios are expected to **break some signals**.  
That is the point.

---

## Why Stress Scenarios Matter

Most inference failures occur:
- Near expiries
- During liquidity collapse
- When regimes shift abruptly
- When multiple constraints bind simultaneously

Signals that only work in benign conditions are not production-ready.

---

## Core Stress Scenario Categories

Galactus defines the following mandatory stress categories.

---

## 1. Liquidity Stress Scenarios

### Description

Conditions where market absorption capacity collapses or becomes highly uneven.

### Examples

- Sudden volume drop in mid-cap or small-cap stocks
- Liquidity cliffs around key strikes
- One-sided order flow with no counterparties
- Trading halts or partial sessions

### Expected Behavior

- Capital pressure may increase sharply
- Confidence must degrade
- Signals may suppress activation

Failure to degrade confidence is unacceptable.

---

## 2. Expiry Compression Scenarios

### Description

Conditions near weekly or monthly expiries where time constraints dominate behavior.

### Examples

- Last-day or last-hour option expiry
- Extreme strike concentration near expiry
- Rapid gamma regime flips

### Expected Behavior

- Forced flow detection should intensify
- Time urgency must dominate normalization
- Signals must decay or resolve cleanly post-expiry

Signals persisting after expiry are invalid.

---

## 3. Volatility Shock Scenarios

### Description

Sudden and significant volatility expansion or contraction.

### Examples

- Unexpected macro announcements
- Large gap opens
- Volatility spikes without proportional liquidity

### Expected Behavior

- Regime classification may transition
- Confidence must degrade during transition
- Signals must not extrapolate pre-shock behavior

Overconfident continuity across shocks is a failure.

---

## 4. Regime Transition Scenarios

### Description

Periods where the market shifts from one structural regime to another.

### Examples

- Cash-dominant to derivatives-dominant transition
- Institutional withdrawal or re-entry
- Retail participation surges or collapses

### Expected Behavior

- Transitional regimes must be detected
- Signal confidence must weaken
- Some signals may become temporarily invalid

Silent regime transition is a critical failure.

---

## 5. Constraint Overlap Scenarios

### Description

Multiple binding constraints acting simultaneously.

### Examples

- Expiry coinciding with index rebalance
- Margin stress during volatility spikes
- Settlement pressure plus liquidity stress

### Expected Behavior

- Pressure aggregation must remain interpretable
- Offsetting pressures must be visible
- Blind summation is forbidden

Opaque aggregation under overlap is invalid.

---

## 6. Data Degradation Scenarios

### Description

Scenarios where data quality is compromised.

### Examples

- Delayed derivatives data
- Missing bhavcopy entries
- Schema changes mid-period

### Expected Behavior

- Confidence must degrade immediately
- Inference may halt
- Assumptions must be surfaced

Proceeding with high confidence is unacceptable.

---

## Stress Scenario Application Rules

### Mandatory Coverage

Every promotion candidate must be tested against:

- At least one scenario from each stress category
- At least one historical real-world example
- At least one synthetic or constructed stress window (if applicable)

Skipping categories requires explicit justification.

---

### Evaluation Criteria Under Stress

Stress scenarios are evaluated on:

- Structural coherence of inference
- Confidence and stability degradation behavior
- Absence of false precision
- Correct identification of failure modes

Performance during stress is irrelevant.

---

## Acceptable Outcomes Under Stress

Under stress, it is acceptable for Galactus to:

- Reduce confidence sharply
- Suppress signal activation
- Output “no meaningful inference”
- Explicitly fail closed

It is **not** acceptable to:
- Maintain high confidence
- Produce confident directional inference
- Mask uncertainty with smoothing

---

## Documentation Requirements

Each stress scenario test must document:

- Scenario description
- Why it is stressful structurally
- Observed inference behavior
- Failures or anomalies
- Lessons learned
- Impact on promotion decision

Undocumented stress tests do not exist.

---

## Relationship to Promotion

A signal **cannot be promoted** unless:

- Its behavior under stress is understood
- Failure modes are acceptable and documented
- Confidence degradation behaves as expected

Stress survivability outweighs normal-period performance.

---

## Final Statement

**Stress does not invalidate Galactus.  
Silence, humility, and graceful degradation validate it.**

Signals that survive stress without lying are worth keeping.
