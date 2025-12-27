# Galactus — Signal Lifecycle

## Purpose of This Document

This document defines the **lifecycle of a signal** in Project Galactus.

It exists to:
- Prevent premature productionization
- Enforce research discipline
- Ensure signals remain relevant over time
- Enable graceful deprecation

Signals are not permanent assets.  
They are hypotheses with expiration dates.

---

## Core Principle

A signal is considered **guilty until proven robust**.

Promotion is earned slowly.  
Deprecation is expected.

---

## Signal Lifecycle Stages

Every signal must move through the following stages, in order:

1. Hypothesis  
2. Research Validation  
3. Promotion Candidate  
4. Core Signal  
5. Monitoring  
6. Deprecation  
7. Retirement  

Skipping stages is forbidden.

---

## 1. Hypothesis Stage

### Description

An idea proposed to measure a specific capital behavior.

### Characteristics

- Exists only in research code
- May be informal or exploratory
- May fail quickly

### Requirements

- Clear statement of capital behavior
- Hypothesized constraint or pressure
- Explicit assumptions

No performance claims are required at this stage.

---

## 2. Research Validation Stage

### Description

The signal is tested empirically under controlled conditions.

### Requirements

- Historical validation across:
  - Multiple expiries
  - Different liquidity regimes
  - At least one stress period
- Failure analysis
- Stability assessment

### Prohibited Practices

- Parameter tuning for performance
- Selective reporting
- Ignoring counterexamples

Success here does **not** guarantee promotion.

---

## 3. Promotion Candidate Stage

### Description

The signal is proposed for inclusion in the core engine.

### Requirements

- Formal documentation
- Mathematical definition
- Deterministic formulation
- Alignment with signal philosophy
- Explicit failure modes

A signal may remain a candidate indefinitely.

---

## 4. Core Signal Stage

### Description

The signal is implemented in the Rust core and becomes part of production inference.

### Requirements

- Deterministic Rust implementation
- Unit and golden tests
- Schema alignment
- Versioned configuration

Once promoted, the signal is considered **stable but not permanent**.

---

## 5. Monitoring Stage

### Description

The signal is continuously evaluated for relevance and health.

### Monitoring Dimensions

- Frequency of activation
- Regime sensitivity
- Degradation under stress
- False confidence incidents

Monitoring does not imply optimization.

---

## 6. Deprecation Stage

### Description

The signal is flagged for eventual removal.

### Triggers for Deprecation

- Structural market changes
- Persistent instability
- Redundancy with superior signals
- Violation of design principles

### Behavior

- Signal remains available
- Confidence is reduced
- Consumers are warned

Deprecation is a **success state**, not a failure.

---

## 7. Retirement Stage

### Description

The signal is removed from active inference.

### Requirements

- Documentation update
- Decision log entry
- Archival of historical behavior

Retired signals are not deleted; they are remembered.

---

## Governance and Accountability

- Every signal has an owner
- Lifecycle state must be explicit
- Changes require documentation updates

Signals without ownership are not allowed.

---

## What Is Explicitly Forbidden

- “Temporary” signals
- Undocumented promotion
- Silent behavior changes
- Resurrection without re-validation

---

## Why This Lifecycle Exists

Markets evolve.  
Signals decay.

Without a lifecycle:
- Systems accumulate ghosts
- Confidence inflates
- Understanding erodes

Galactus avoids this by treating signals as **living hypotheses**.

---

## Final Statement

**A signal that cannot die is a signal that should never be born.**
