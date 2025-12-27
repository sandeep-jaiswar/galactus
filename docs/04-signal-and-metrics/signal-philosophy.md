# Galactus — Signal Philosophy

## Purpose of This Document

This document defines the **philosophy of signals** in Project Galactus.

It exists to:
- Prevent indicator sprawl
- Enforce capital-behavior grounding
- Ensure signals remain interpretable and durable
- Separate inference from action

If a metric does not align with this philosophy, it is not a valid Galactus signal.

---

## What a Signal Means in Galactus

In Galactus, a **signal** is:

> A deterministic, explainable measurement that quantifies a constraint or pressure acting on capital within a defined context and time window.

Signals are **descriptive**, not prescriptive.

They describe *what pressure exists*, not *what to do about it*.

---

## What a Signal Is Not

A Galactus signal is **not**:

- A buy or sell instruction
- A price forecast
- A probability of profit
- A pattern detector
- A shortcut to decision-making

Any metric that implies action without context is invalid.

---

## Explicit Rejections

Galactus **explicitly rejects** the following classes of signals:

### 1. Indicator-Based Signals

**Rejected:**
- RSI-based signals
- MACD crossovers
- Moving average crosses (SMA, EMA)
- Bollinger Band touches
- Stochastic oscillators
- Any traditional technical indicator used as a primary signal

**Rationale:**  
Indicators describe price outcomes, not capital constraints. They collapse diverse capital dynamics into a single dimension and encourage reactive rather than structural interpretation.

### 2. Predictive Signals

**Rejected:**
- Price forecasts or predictions
- Target price estimates
- Return probability distributions
- Time-series extrapolations
- Pattern-based future projections
- Confidence intervals for price outcomes

**Rationale:**  
Galactus infers pressure, not outcomes. Price prediction obscures causal structure and creates false confidence. Predicting "what will happen" is fundamentally outside scope.

### 3. Price-Derived Signals

**Rejected:**
- Signals derived purely from historical price
- Rate-of-change indicators (price-only)
- Price momentum signals
- Trend strength measures based on price alone
- Price pattern recognition outputs

**Rationale:**  
Price is a consequence of capital behavior, not a cause. Price-derived signals lack grounding in structural constraints and fail to explain *why* capital must act.

### 4. Black-Box Signals

**Rejected:**
- Uninterpretable ML model outputs
- Neural network embeddings without explanation
- Proprietary "alpha scores" without decomposition
- Opaque composite metrics

**Rationale:**  
Every signal must be explainable in terms of capital behavior and constraints. Black-box outputs violate the fundamental explainability requirement.

### Enforcement

Any signal that:
- Falls into these rejected categories
- Cannot articulate which capital is constrained and why
- Requires prediction of future outcomes
- Depends solely on price history

**Must be rejected, regardless of historical performance.**

---

## Signal Primitives

All valid signals must be reducible to one or more of the following primitives:

1. **Constraint**
   - Time, liquidity, regulatory, or structural limits on capital

2. **Pressure**
   - Magnitude and direction of forced or incentivized capital action

3. **Imbalance**
   - Asymmetry between opposing capital forces

4. **Absorption Capacity**
   - Market’s ability to accommodate capital without price impact

5. **Stability**
   - Persistence or fragility of observed pressure

Signals that cannot be expressed in these terms are rejected.

---

## Capital Behavior Requirement

Every signal must explicitly answer:

1. Which capital is affected?
2. What constraint is binding?
3. Why action cannot be delayed?
4. Over what time window does this matter?

If these questions cannot be answered clearly, the signal is invalid.

---

## Determinism Requirement

All signals must be:

- Deterministic
- Replayable
- Independent of system state outside documented inputs

Signals that depend on:
- Random initialization
- Implicit ordering
- Hidden smoothing

are forbidden.

---

## Context Dependency

Signals are **contextual**, not universal.

Valid context dimensions include:
- Time to expiry
- Liquidity regime
- Instrument type
- Market regime

A signal without explicit context is incomplete.

---

## Signal Scope and Modesty

Galactus signals are intentionally modest.

They may:
- Be directional or non-directional
- Be strong or weak
- Be inconclusive

They are allowed to say:
- “Pressure exists”
- “Pressure is increasing”
- “Pressure is unresolved”

They are not allowed to say:
- “This will happen”

---

## Signal Composition

Signals may be:
- Atomic (single mechanism)
- Composite (multiple mechanisms)

Composite signals must:
- Preserve interpretability
- Expose component contributions
- Avoid opaque aggregation

Black-box composition is forbidden.

---

## Time Awareness

All signals must declare:
- Their effective time horizon
- When they expire or decay
- Conditions under which they become invalid

Timeless signals are not allowed.

---

## Failure Awareness

Signals must document:
- Known failure modes
- Conditions under which they degrade
- Regimes where they are unreliable

Undocumented failure is unacceptable.

---

## Signal Lifecycle

Signals progress through stages:
1. Hypothesis (research only)
2. Validated (research)
3. Promoted (core)
4. Monitored (production)
5. Deprecated (sunset)

For the complete lifecycle definition with transition criteria, see [`signal-lifecycle.md`](signal-lifecycle.md).

Lifecycle management is mandatory.

---

## Why This Philosophy Exists

Without a strict philosophy:
- Indicators multiply
- Explanations decay
- Confidence inflates
- Trust erodes

Galactus avoids this by being **deliberately restrictive**.

---

## Final Statement

**A Galactus signal does not tell you what will happen.**

It tells you **what pressure exists, and why**.
