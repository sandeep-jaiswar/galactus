# Galactus — High-Level Design (HLD)

## Purpose of This Document

This document defines the **high-level system architecture** of Project Galactus.

It explains:
- How Galactus is structured
- How data flows through the system
- Where responsibilities begin and end
- Why specific architectural choices were made

This document is the primary reference for:
- System boundaries
- Component ownership
- Technology selection rationale

Any architectural change must be justified against this document.

---

## System Objective (Restated)

Galactus is a **capital-intent inference system** for Indian financial markets.

Its objective is to:
- Ingest public market data
- Infer forced and strategic capital behavior
- Produce deterministic, explainable intent metrics
- Expose those metrics safely and compliantly

Galactus does **not**:
- Predict prices
- Execute trades
- Optimize for latency arbitrage

---

## Architectural Principles

The architecture is governed by the following principles:

1. **Event-driven, not tick-driven**
2. **Deterministic core**
3. **Separation of research and production**
4. **Replayability as a first-class feature**
5. **Explainability over complexity**

---

## High-Level System Overview

At a high level, Galactus consists of six major layers:

1. Data Ingestion
2. Streaming & Persistence
3. Intent Engine (Core)
4. Intent State Store
5. Research & Validation
6. Consumer / API Layer

Each layer has a clearly defined responsibility and interface.

---

## 1. Data Ingestion Layer

### Responsibility

The ingestion layer is responsible for:
- Fetching public market data
- Normalizing timestamps and identifiers
- Emitting **market events**

This layer performs **no inference**.

---

### Inputs

Indicative inputs include:
- Cash market bhavcopy
- Options chain snapshots
- Corporate disclosures
- Index methodology and rebalance calendars
- Market calendars and expiry schedules

---

### Outputs

- Canonical, time-aligned **market events**
- Events are immutable once emitted

---

### Design Characteristics

- Batch-oriented where appropriate
- Event-time semantics over processing-time semantics
- Idempotent ingestion
- Lossless by default

---

## 2. Streaming & Persistence Layer

### Responsibility

This layer decouples ingestion from computation and ensures:
- Durability
- Replayability
- Temporal ordering

---

### Components

Indicative technologies:
- Kafka / Redpanda for event streams
- Object storage (S3 / GCS) for raw data
- Columnar storage (Iceberg / Delta) for structured data

---

### Design Characteristics

- Append-only
- Versioned schemas
- No in-place mutation
- Replayable at any point in time

This layer is the **historical memory** of Galactus.

---

## 3. Intent Engine (Core)

### Responsibility

The Intent Engine is the **deterministic core** of Galactus.

It is responsible for:
- Canonicalizing market events
- Computing deterministic features
- Inferring capital pressure
- Classifying regime states
- Producing explainable intent vectors

---

### Language Choice

The core engine is implemented in **Rust**.

Rationale:
- Determinism
- Memory safety
- Predictable concurrency
- Reproducible execution

---

### Internal Modules (Indicative)

- `canonicalizer`
- `capital_pressure`
- `forced_flow_detector`
- `regime_classifier`
- `confidence_evaluator`

Each module:
- Has a single responsibility
- Exposes pure or explicitly state-passed functions
- Is independently testable

---

### Inputs

- Canonical market events
- Explicit configuration (versioned)

---

### Outputs

- Intent vectors (pressure, directionality, confidence)
- Regime states
- Metadata describing stability and assumptions

The engine never emits:
- Buy/sell signals
- Price forecasts
- Actionable instructions

---

## 4. Intent State Store

### Responsibility

The intent state store persists **derived inference outputs** for:

- Longitudinal analysis
- Backtesting
- Regime transition analysis
- Research validation

---

### Data Stored

- Per-symbol intent timelines
- Per-expiry derivative pressure maps
- Aggregate market regime states

---

### Design Characteristics

- Columnar and time-partitioned
- Optimized for analytical reads
- Immutable once written (new versions appended)

---

## 5. Research & Validation Layer

### Responsibility

This layer supports:
- Feature discovery
- Hypothesis testing
- Signal validation
- Stress and failure analysis

---

### Language Choice

This layer is implemented in **Python**.

Rationale:
- Fast iteration
- Rich analytical ecosystem
- Visualization and exploration

---

### Constraints

- Research code cannot modify production inference
- Promotion to core follows documented promotion rules
- Research results must be reproducible

This layer is **allowed to be messy**.  
The core is not.

---

## 6. Consumer / API Layer

### Responsibility

Expose Galactus outputs to downstream consumers in a **safe and compliant manner**.

---

### Consumers

Indicative consumers include:
- Internal dashboards
- Research notebooks
- Advisory-grade analytics
- Alerting systems (non-actionable)

---

### Output Characteristics

- Probabilistic and descriptive
- Explainable
- Language-safe (no trading instructions)

---

## Cross-Cutting Concerns

### Determinism & Replayability

- All inference can be replayed from raw events
- Backtests and live runs share code paths
- Configuration is versioned and auditable

---

### Failure Handling

- Missing or degraded data is surfaced explicitly
- Uncertain inference is allowed
- Silence is preferable to false precision

---

### Compliance

- Public data only
- No personalization
- SEBI-safe output language enforced at API boundaries

---

## Explicit Non-Goals (Architectural)

The architecture does **not** support:
- Ultra-low-latency trading
- Real-time order book inference
- User-specific decision optimization
- Black-box inference pipelines

---

## Evolution Strategy

The architecture is designed to:
- Start simple
- Grow modularly
- Allow components to be deprecated cleanly

No component is permanent.

---

## Final Statement

**Galactus is designed to think clearly, move deliberately, and explain itself under stress.**

This architecture exists to make that possible.
