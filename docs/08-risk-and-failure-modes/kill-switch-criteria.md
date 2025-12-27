# Galactus — Kill Switch Criteria

## Purpose
This document defines the **non-negotiable conditions** under which Galactus must partially or fully halt inference.

The kill switch exists to:
- Prevent false confidence
- Protect downstream consumers
- Preserve trust in inference

Incorrect silence is preferable to confident error.

---

## Definition: Kill Switch

A kill switch is an automatic or manual mechanism that:
- Suppresses inference output
- Freezes signal activation
- Forces the system into a safe state

---

## Mandatory Kill Conditions

Inference **must halt** if any of the following occur:

### 1. Data Integrity Failure
- Corrupted or inconsistent canonical events
- Unresolvable schema mismatch
- Missing core data across required windows

### 2. Time Semantics Violation
- Event time cannot be reliably determined
- Event ordering ambiguity exceeds tolerance

### 3. Regime Indeterminacy
- Regime classification confidence below minimum threshold
- Conflicting regime signals without resolution

### 4. Constraint Misidentification
- Core constraint assumptions invalidated
- Forced flow logic contradicted structurally

### 5. Confidence System Failure
- Confidence calculation unavailable or corrupted
- Confidence not degrading when required

---

## Partial Kill Conditions

Certain failures require **selective suppression**:
- Signal-level deactivation
- Instrument-level silence
- Regime-specific suspension

---

## Recovery Rules

- Kill switches may only be lifted after root cause analysis
- Recovery must be logged
- Silent recovery is forbidden

---

## Final Statement
**Galactus must know when it should stop thinking.**
