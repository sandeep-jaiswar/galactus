# Galactus — System Boundaries

## Purpose of This Document

This document defines the **explicit system boundaries** of Project Galactus.

It exists to:
- Establish the four core layers that define Galactus
- Explicitly identify what is **inside** system scope
- Explicitly identify what is **outside** system scope
- Prevent scope creep and architectural drift

This document serves as the primary reference for understanding what Galactus is and is not responsible for.

---

## Core System Philosophy

Galactus is a **capital-pressure inference system**.

It observes market data, infers capital behavior, validates through research, and outputs explainable insights.

**Galactus does not execute trades or generate strategies.**

These functions belong to downstream systems outside Galactus boundaries.

---

## The Four Core System Boundaries

Galactus consists of **exactly four** architectural boundaries:

1. **Ingestion** — Raw data acquisition and normalization
2. **Inference** — Deterministic capital behavior analysis
3. **Research** — Validation, testing, and discovery
4. **Outputs** — Safe, compliant presentation of results

Each boundary has a clearly defined responsibility, input/output contract, and scope.

---

## 1. Ingestion Boundary

### Responsibility

The ingestion boundary is responsible for:
- Acquiring public market data from authorized sources
- Normalizing timestamps to event-time semantics
- Standardizing identifiers and symbols
- Emitting canonical market events

### What Ingestion Owns

- Data source connectors
- Data quality checks
- Timestamp normalization
- Symbol standardization
- Event emission

### What Ingestion Does NOT Own

- Data interpretation
- Feature computation
- Inference logic
- Business rules
- Capital behavior analysis

### Inputs

- Raw market data feeds (NSE, BSE, exchange APIs)
- Corporate action disclosures
- Index methodology documents
- Market calendars and schedules

### Outputs

- Canonical, immutable market events
- Quality metadata (completeness, timeliness)
- Event streams ready for inference

### Key Principle

**Ingestion transforms but does not interpret.**

Data enters raw and leaves structured, but ingestion makes no judgments about market meaning.

---

## 2. Inference Boundary

### Responsibility

The inference boundary is the **deterministic core** of Galactus.

It is responsible for:
- Canonicalizing market events
- Computing deterministic features
- Inferring capital pressure and forced flows
- Classifying market regimes
- Evaluating confidence and stability
- Producing explainable intent vectors

### What Inference Owns

- Feature computation logic
- Capital pressure models
- Regime classification algorithms
- Confidence evaluation
- Deterministic state management
- Core inference engine (Rust implementation)

### What Inference Does NOT Own

- Data acquisition
- Research and experimentation
- Output presentation
- User-facing APIs
- Strategy formulation
- Trade execution

### Inputs

- Canonical market events (from Ingestion)
- Explicit, versioned configuration
- Historical context (when needed for regime detection)

### Outputs

- Intent vectors (pressure direction, magnitude, confidence)
- Regime classifications (consolidation, expansion, compression)
- Stability and degradation signals
- Explainability metadata

### Key Principle

**Inference is deterministic, reproducible, and explainable.**

The same inputs always produce the same outputs. No randomness, no hidden state, no black boxes.

---

## 3. Research Boundary

### Responsibility

The research boundary supports:
- Feature discovery and hypothesis generation
- Signal validation and backtesting
- Stress testing and failure analysis
- Regime transition studies
- Model exploration and iteration

### What Research Owns

- Exploratory analysis pipelines
- Hypothesis testing frameworks
- Backtesting infrastructure
- Visualization and reporting
- Python-based research notebooks
- Experimental signal development

### What Research Does NOT Own

- Production inference logic
- Live decision systems
- Data source management
- Consumer-facing APIs
- Trade execution logic

### Inputs

- Historical market data
- Inference outputs (intent vectors, regimes)
- External research datasets (when appropriate)

### Outputs

- Research findings and validation reports
- Signal promotion candidates
- Failure mode documentation
- Feature proposals

### Key Principle

**Research discovers truth; production enforces truth.**

Research is iterative, exploratory, and allowed to be messy. Promotion to production requires formal validation and documentation.

---

## 4. Outputs Boundary

### Responsibility

The outputs boundary exposes Galactus results in a **safe and compliant manner**.

It is responsible for:
- Formatting inference outputs
- Enforcing language safety
- Access control and authentication
- API design and versioning
- Output documentation

### What Outputs Owns

- API layer (REST, gRPC, etc.)
- Output formatting and serialization
- Language compliance enforcement
- Consumer authentication
- Output documentation
- Dashboards and visualization interfaces

### What Outputs Does NOT Own

- Inference logic
- Feature computation
- Signal derivation
- Business interpretation
- Trading recommendations
- Execution instructions

### Inputs

- Inference outputs (intent vectors, regimes)
- Research-validated signals
- System metadata

### Outputs

- API responses with intent signals
- Dashboards showing capital pressure
- Reports with regime analysis
- Alerts on regime transitions

### Key Principle

**Outputs present inference, never advice.**

Language is precise, probabilistic, and non-advisory. No buy/sell signals. No price predictions. No actionable instructions.

---

## Explicit Exclusions — What Is Outside System Boundaries

The following are **permanently outside** Galactus system boundaries:

### ❌ 1. Execution Layer (EXCLUDED)

Galactus does **not** include:
- Trade execution logic
- Order management systems
- Broker integrations
- Position management
- Portfolio rebalancing
- Automated trading systems
- Real-time order placement

**Rationale:**

Execution requires:
- Individual capital constraints
- Risk preferences
- Portfolio context
- Regulatory compliance specific to execution
- Real-time operational infrastructure

These factors are **external** to capital inference and belong to downstream systems.

### ❌ 2. Strategy Layer (EXCLUDED)

Galactus does **not** include:
- Strategy formulation
- Asset allocation decisions
- Portfolio construction
- Risk-adjusted optimization
- Position sizing
- Entry/exit timing decisions
- PnL optimization

**Rationale:**

Strategy requires:
- Investor-specific goals
- Capital availability
- Risk tolerance
- Time horizons
- Tax considerations
- Regulatory constraints

These are **individual decisions** that cannot be generalized by a capital inference system.

### Why These Exclusions Matter

By explicitly excluding execution and strategy:

1. **Regulatory Clarity**
   - Galactus remains a research tool, not an advisory service
   - No SEBI registration requirements for execution
   - Clear liability boundaries

2. **Architectural Simplicity**
   - Focused scope prevents feature creep
   - Core inference remains pure and testable
   - No coupling to execution infrastructure

3. **Generalizability**
   - Outputs serve multiple downstream use cases
   - No assumptions about individual capital constraints
   - Reusable across different strategy implementations

4. **Maintainability**
   - Small, focused system is easier to validate
   - Determinism is preserved
   - Clear upgrade and deprecation paths

---

## System Boundary Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                         GALACTUS SYSTEM                          │
│                                                                   │
│  ┌─────────────┐      ┌─────────────┐      ┌─────────────┐     │
│  │             │      │             │      │             │     │
│  │  INGESTION  │─────▶│  INFERENCE  │─────▶│   OUTPUTS   │     │
│  │             │      │             │      │             │     │
│  │  Raw Data   │      │ Deterministic│      │  Safe APIs  │     │
│  │Normalization│      │   Capital   │      │  Language   │     │
│  │             │      │  Pressure   │      │  Compliant  │     │
│  └─────────────┘      │  Analysis   │      └─────────────┘     │
│                        │             │                           │
│                        └──────┬──────┘                           │
│                               │                                  │
│                               │ Outputs                          │
│                               ▼                                  │
│                        ┌─────────────┐                           │
│                        │             │                           │
│                        │  RESEARCH   │                           │
│                        │             │                           │
│                        │ Validation  │                           │
│                        │  Testing    │                           │
│                        │  Discovery  │                           │
│                        └─────────────┘                           │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘

                           OUTSIDE SCOPE
                      (Downstream Systems)

           ┌──────────────────────────────────────┐
           │      ❌ EXECUTION LAYER               │
           │  - Trade execution                   │
           │  - Order management                  │
           │  - Broker integration                │
           └──────────────────────────────────────┘

           ┌──────────────────────────────────────┐
           │      ❌ STRATEGY LAYER                │
           │  - Strategy formulation               │
           │  - Portfolio construction             │
           │  - Position sizing                    │
           └──────────────────────────────────────┘
```

---

## Cross-Boundary Communication

### Allowed Patterns

1. **Ingestion → Inference**
   - Event streams
   - Immutable data

2. **Inference → Outputs**
   - Intent vectors
   - Regime classifications

3. **Inference → Research**
   - Historical outputs
   - Validation datasets

4. **Research → Inference**
   - Signal promotion (via formal process)
   - Configuration updates (versioned)

### Forbidden Patterns

- **Circular dependencies** between boundaries
- **Shared mutable state** across boundaries
- **Direct database access** bypassing boundaries
- **Inference logic** embedded in Outputs
- **Execution logic** embedded anywhere in Galactus

---

## Boundary Enforcement

### At Development Time

- Code organization mirrors boundaries
- Module imports enforce dependency direction
- Build system validates boundary compliance

### At Runtime

- APIs enforce boundary contracts
- Invalid cross-boundary calls fail explicitly
- Monitoring tracks boundary violations

### At Review Time

- Design reviews check boundary compliance
- Code reviews verify single-responsibility
- Architecture decisions reference this document

---

## When Boundaries May Change

Boundary changes require:

1. **Formal proposal** explaining need
2. **Vision compliance check** against core documents
3. **Impact analysis** on existing components
4. **Documentation update** including rationale
5. **Decision log entry** for traceability

Boundary drift without documentation is **forbidden**.

---

## Relationship to Other Architecture Documents

This document establishes **what** the system boundaries are.

Related documents define **how** boundaries operate:

- **[`high-level-design.md`](high-level-design.md)** — Detailed layer design
- **[`component-boundaries.md`](component-boundaries.md)** — Component responsibilities
- **[`event-driven-architecture.md`](event-driven-architecture.md)** — Data flow semantics
- **[`rust-vs-python-contract.md`](rust-vs-python-contract.md)** — Technology separation

All documents must remain consistent with these system boundaries.

---

## Success Criteria

Galactus system boundaries are successful when:

1. **Clarity**
   - Anyone can determine if a feature is in/out of scope
   - No ambiguity about component ownership

2. **Stability**
   - Boundaries remain stable across releases
   - Changes are rare and well-justified

3. **Simplicity**
   - Each boundary has a single, clear purpose
   - No overlapping responsibilities

4. **Enforceability**
   - Boundary violations are detectable
   - Enforcement is automated where possible

---

## Final Statement

**Galactus has exactly four boundaries: Ingestion, Inference, Research, and Outputs.**

**Execution and Strategy are permanently outside these boundaries.**

This separation is not a limitation—it is Galactus's architectural foundation.

By knowing what we are **not**, we can be excellent at what we **are**:

**A deterministic capital-pressure inference system for Indian financial markets.**

---

## Document Authority

This document has equal authority with:
- [`docs/00-vision-and-non-goals/vision.md`](../00-vision-and-non-goals/vision.md)
- [`docs/00-vision-and-non-goals/explicit-non-goals.md`](../00-vision-and-non-goals/explicit-non-goals.md)
- [`VISION_LOCK.md`](../../VISION_LOCK.md)

Any architectural decision that conflicts with these boundaries must be rejected.
