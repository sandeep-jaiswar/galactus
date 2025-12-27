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
