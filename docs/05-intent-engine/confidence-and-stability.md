# Galactus — Confidence and Stability

## Purpose of This Document

This document defines how **confidence and stability** are measured, represented, and surfaced by the Galactus Intent Engine.

It exists to:
- Prevent false precision
- Make uncertainty explicit
- Differentiate strong inference from fragile inference
- Encourage appropriate downstream interpretation

Confidence is not optimism.  
Stability is not permanence.

---

## Core Principle

Galactus does not aim to be confident.  
It aims to be **correctly uncertain**.

---

## Definition: Confidence

In Galactus, **confidence** represents:

> The degree to which current inference is supported by data quality, structural consistency, and regime alignment.

Confidence does **not** represent:
- Probability of profit
- Likelihood of price movement
- Strength of conviction

It represents **trustworthiness of inference**.

---

## Definition: Stability

**Stability** represents:

> The persistence and robustness of inferred capital pressure over time and across small perturbations.

A stable inference:
- Persists across adjacent events
- Is not highly sensitive to minor data changes
- Degrades gradually rather than collapsing

---

## Dimensions of Confidence

Confidence is computed as a function of multiple dimensions:

---

## 1. Data Quality Confidence

Based on:
- Completeness of input data
- Timeliness of events
- Consistency across sources

Poor data quality directly degrades confidence.

---

## 2. Structural Alignment Confidence

Based on:
- Clear presence of binding constraints
- Alignment with known market mechanics
- Absence of contradictory signals

Weak structural grounding reduces confidence.

---

## 3. Regime Consistency Confidence

Based on:
- Clarity of regime classification
- Stability of regime over time
- Alignment between signal and regime

Ambiguous or transitional regimes degrade confidence.

---

## 4. Signal Agreement Confidence

Based on:
- Confluence of independent signals
- Absence of offsetting pressures
- Transparency of aggregation

Single-signal inference carries lower confidence.

---

## Dimensions of Stability

Stability evaluates how inference behaves over time.

---

## 1. Temporal Stability

- Persistence across consecutive events
- Resistance to short-lived noise
- Gradual decay when conditions change

---

## 2. Sensitivity Stability

- Low sensitivity to small data perturbations
- Robustness to minor schema or timing variations

---

## 3. Regime Stability

- Inference remains coherent within a regime
- Predictable behavior at regime boundaries

---

## Confidence and Stability Outputs

All intent outputs must include:

- Explicit confidence score
- Stability indicator
- Assumption list
- Known ambiguity flags

No output is complete without uncertainty metadata.

---

## Degradation Rules

When uncertainty increases:

- Confidence must degrade
- Stability indicators must weaken
- Output language must soften

Silence is acceptable when confidence drops below minimum thresholds.

---

## Hard Confidence Floors

Inference must be suppressed when:

- Data quality is critically degraded
- Regime classification is indeterminate
- Core assumptions are violated

Producing confident output under these conditions is forbidden.

---

## Interaction with Consumers

Downstream systems must:
- Respect confidence indicators
- Avoid reinterpreting low-confidence inference as actionable
- Surface uncertainty transparently

Confidence is part of the signal, not decoration.

---

## Failure Modes

Confidence assessment may fail if:
- Data quality issues are not detected
- Regime transitions are missed
- Novel market behavior emerges

Failure must reduce confidence, not mask it.

---

## Evolution of Confidence Models

Confidence logic may evolve as:
- New failure modes are identified
- Market structure changes
- Better diagnostics are developed

All changes must be documented and versioned.

---

## Final Statement

**Galactus does not hide uncertainty.**

When confidence is low, Galactus speaks softly—or not at all.
