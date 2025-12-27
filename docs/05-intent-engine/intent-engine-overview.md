# Galactus — Intent Engine Overview

## Purpose of This Document

This document defines the **Intent Engine**, the core inference component of Project Galactus.

It exists to:
- Translate market events into capital-intent inference
- Enforce deterministic, explainable reasoning
- Separate observation from interpretation
- Provide stable, auditable outputs for downstream consumers

The Intent Engine is the *brain* of Galactus, not its hands.

---

## What the Intent Engine Is

The Intent Engine is a **deterministic inference system** that:

- Consumes canonical market events
- Computes capital pressure metrics
- Infers intent and regime states
- Emits structured, non-actionable outputs

It reasons about **constraints and pressure**, not prices or trades.

---

## What the Intent Engine Is Not

The Intent Engine is explicitly **not**:

- A trading engine
- A signal generator for execution
- A prediction model
- A real-time decision system
- A black-box optimizer

Any attempt to extend the Intent Engine beyond inference is a violation of design principles.

---

## Inputs to the Intent Engine

The engine consumes:

- Canonical market events (from ingestion)
- Explicit configuration (versioned)
- Schema-stable reference data (e.g., calendars)

Inputs are:
- Immutable
- Time-aligned
- Replayable

The engine does not fetch data directly.

---

## Core Responsibilities

The Intent Engine is responsible for the following steps:

1. **Event Canonicalization**
2. **Deterministic Feature Computation**
3. **Capital Pressure Inference**
4. **Regime Classification**
5. **Confidence and Stability Evaluation**

Each step is isolated and testable.

---

## Outputs of the Intent Engine

The engine emits **intent vectors**, which include:

- Capital pressure magnitude
- Pressure directionality (if applicable)
- Regime state
- Confidence score
- Explicit assumptions and limitations

Outputs are:
- Descriptive
- Contextual
- Non-prescriptive

---

## Determinism Guarantee

Given:
- The same event stream
- The same configuration
- The same engine version

The Intent Engine must always produce identical outputs.

This guarantee is non-negotiable.

---

## Explainability Requirement

Every output must be traceable to:

- Specific input events
- Explicit metrics
- Documented assumptions

If an output cannot be explained in market-structure terms, it is invalid.

---

## State Handling

The Intent Engine:
- Is stateless by default
- Accepts state explicitly when required
- Does not maintain hidden internal state

State transitions must be explicit and auditable.

---

## Time Semantics

The engine reasons strictly in **event time**.

- Late events trigger recomputation
- Time windows are explicit
- Decay is modeled, not assumed

---

## Failure and Uncertainty Handling

The Intent Engine:
- Surfaces uncertainty explicitly
- Degrades confidence under poor data
- Is allowed to produce no inference

Silence is preferable to false confidence.

---

## Performance Characteristics

The engine is designed for:
- Predictable latency
- Throughput aligned with event frequency
- Stability under stress conditions (expiry, volatility spikes)

It does not optimize for microsecond latency.

---

## Boundaries and Interfaces

The Intent Engine:
- Consumes events via defined interfaces
- Emits intent vectors via stable schemas
- Is isolated from presentation and research layers

Boundary violations are architectural failures.

---

## Evolution Strategy

The Intent Engine evolves through:
- Addition of new signals (via lifecycle)
- Schema versioning
- Controlled deprecation

Its core responsibility remains constant.

---

## Final Statement

**The Intent Engine does not decide what to do.**

It decides **what is happening**—clearly, deterministically, and honestly.
