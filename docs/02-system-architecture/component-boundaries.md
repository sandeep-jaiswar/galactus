# Galactus — Component Boundaries

## Purpose of This Document

This document defines the **responsibility boundaries** between major components of Project Galactus.

It exists to:
- Prevent scope leakage between components
- Enforce separation of concerns
- Maintain long-term architectural integrity

If a component violates its boundary, it is considered a design failure, not an implementation detail.

---

## Boundary Philosophy

Each component in Galactus must:
- Have a single, well-defined responsibility
- Communicate only through explicit interfaces
- Remain replaceable without system-wide rewrites

Components may evolve internally, but **their contracts must remain stable**.

---

## High-Level Component Map

The primary components are:

1. Data Ingestion
2. Streaming & Persistence
3. Intent Engine (Core)
4. Intent State Store
5. Research & Validation
6. Consumer / API Layer

No component may bypass another without explicit justification.

---

## 1. Data Ingestion Component

### Owns
- Data fetching
- Timestamp normalization
- Identifier normalization
- Event emission

### Does NOT Own
- Feature computation
- Data interpretation
- Inference or aggregation

### Allowed Outputs
- Canonical market events

### Forbidden Behaviors
- Dropping data silently
- Deriving metrics
- Making assumptions about market meaning

---

## 2. Streaming & Persistence Component

### Owns
- Event durability
- Event ordering
- Replay capability
- Schema versioning

### Does NOT Own
- Business logic
- Feature computation
- Inference

### Allowed Outputs
- Ordered event streams
- Immutable historical datasets

### Forbidden Behaviors
- Mutating historical data
- Enriching events with inference
- Schema changes without versioning

---

## 3. Intent Engine (Core)

### Owns
- Canonicalization of events
- Deterministic feature computation
- Capital pressure inference
- Regime classification
- Confidence and stability evaluation

### Does NOT Own
- Data fetching
- Visualization
- Research experimentation
- User-specific logic

### Allowed Inputs
- Canonical market events
- Explicit configuration

### Allowed Outputs
- Intent vectors
- Regime states
- Explainability metadata

### Forbidden Behaviors
- Price prediction
- Actionable trade recommendations
- Accessing raw data sources directly
- Maintaining hidden internal state

---

## 4. Intent State Store

### Owns
- Persistence of inference outputs
- Historical intent timelines
- Versioned intent datasets

### Does NOT Own
- Inference logic
- Signal transformation
- Data mutation

### Allowed Inputs
- Intent Engine outputs

### Forbidden Behaviors
- In-place updates
- Derived analytics
- Backfilling without versioning

---

## 5. Research & Validation Component

### Owns
- Feature discovery
- Hypothesis testing
- Backtesting
- Stress testing
- Failure analysis

### Does NOT Own
- Production inference
- Live decisioning
- Data mutation in production stores

### Allowed Inputs
- Historical raw data
- Intent outputs
- Derived research datasets

### Forbidden Behaviors
- Modifying production inference
- Writing directly to core state stores
- Bypassing promotion rules

---

## 6. Consumer / API Layer

### Owns
- Data presentation
- Access control
- Output formatting
- Language safety enforcement

### Does NOT Own
- Inference logic
- Signal derivation
- Business interpretation

### Allowed Inputs
- Intent state store outputs
- Aggregated inference summaries

### Forbidden Behaviors
- Translating inference into advice
- Hiding uncertainty
- Re-framing outputs as trading signals

---

## Cross-Boundary Rules

### Explicit Interfaces Only
- All cross-component communication must use versioned interfaces
- No shared mutable state

### No Bidirectional Dependencies
- Components may depend only on upstream abstractions
- Cyclic dependencies are forbidden

### Fail Closed
- When boundary assumptions break, components must fail explicitly
- Silent degradation is not acceptable

---

## Boundary Enforcement Mechanisms

Boundaries should be enforced through:
- Code organization
- Build-time checks
- API contracts
- Documentation requirements

Violations are architectural bugs.

---

## When Boundaries May Change

Boundary changes require:
- Documentation updates
- Explicit rationale
- Review against vision and non-goals

Boundary drift without documentation is not permitted.

---

## Final Statement

**Clear boundaries allow Galactus to remain correct even as it grows.**

Coupling is the enemy of inference.
