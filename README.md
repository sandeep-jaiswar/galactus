# Project Galactus

**A capital-pressure inference engine for the Indian financial markets**

---

## What is Galactus?

Galactus is a deterministic system that infers forced and strategic capital behavior in the Indian financial markets using public data.

**Galactus does not predict prices.**  
**Galactus does not issue trading instructions.**  
**Galactus infers, explains, and waits.**

---

## Core Vision (Locked)

The core vision of Galactus is **permanently established** in:

📜 **[`docs/00-vision-and-non-goals/vision.md`](docs/00-vision-and-non-goals/vision.md)**

This document is the **single source of truth** for all architectural and design decisions.

Any feature, proposal, or change that contradicts the core vision **must be rejected** unless the vision document itself is explicitly revised through the formal revision process.

---

## What Galactus Is

- An **inference engine**, not a prediction engine
- A **capital-behavior model**, not a charting system
- A **deterministic system**, not a probabilistic guessing machine
- A **research-informed platform**, not a tip or signal service

---

## What Galactus Is NOT

Galactus **explicitly excludes**:

- ❌ **Trading systems** — No buy/sell signals, no execution logic
- ❌ **Advisory tools** — No personalized investment advice
- ❌ **Prediction engines** — No price forecasting or target estimation
- ❌ **Execution logic** — No broker integration or order placement

See the complete list in **[`docs/00-vision-and-non-goals/explicit-non-goals.md`](docs/00-vision-and-non-goals/explicit-non-goals.md)**

---

## Quick Start (Docker)

```bash
# Clone and start production environment
git clone <repository-url>
cd galactus
./start_production.sh
```

This will:
- Build all Docker containers
- Start the complete Galactus stack
- Set up monitoring and dashboards
- Make APIs available for inference

**Service URLs:**
- **API**: http://localhost:8080
- **Grafana**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9090

---

## Architecture Overview

Galactus runs as a containerized microservices architecture:

- **galactus-core**: Rust inference engine with HTTP/gRPC APIs
- **prometheus**: Metrics collection and alerting
- **grafana**: Monitoring dashboards and visualization
- **galactus-research**: Python research environment (optional)

All services are orchestrated with Docker Compose and include health checks, logging, and monitoring.

---

## Intended Users

Galactus is designed for:

- Researchers studying market structure
- Systematic investors seeking explainable signals
- Internal decision-support systems
- Analysts who value probabilistic reasoning over tips

It is **not** designed for retail speculation or gamified trading.

---

## Documentation Structure

The complete documentation is organized in **[`docs/`](docs/)**:

### 🎯 Foundation (Start Here)
- **[`00-vision-and-non-goals/`](docs/00-vision-and-non-goals/)** — Core vision, non-goals, and design principles

### 📚 Theory and Architecture
- **[`01-market-theory/`](docs/01-market-theory/)** — Capital behavior models and market structure
- **[`02-system-architecture/`](docs/02-system-architecture/)** — System design and component boundaries

### 🔧 Implementation
- **[`03-data-and-schemas/`](docs/03-data-and-schemas/)** — Data sources, schemas, and quality rules
- **[`04-signal-and-metrics/`](docs/04-signal-and-metrics/)** — Signal philosophy and metric taxonomy
- **[`05-intent-engine/`](docs/05-intent-engine/)** — Core inference engine design

### 🔬 Research and Validation
- **[`06-research-framework/`](docs/06-research-framework/)** — Research methodology and promotion criteria
- **[`07-backtesting-and-validation/`](docs/07-backtesting-and-validation/)** — Validation frameworks and stress testing

### 🛡️ Risk and Compliance
- **[`08-risk-and-failure-modes/`](docs/08-risk-and-failure-modes/)** — Known risks and kill-switch criteria
- **[`09-compliance-and-language/`](docs/09-compliance-and-language/)** — Language guidelines and output restrictions

### 📋 Operations and Evolution
- **[`10-operational-playbooks/`](docs/10-operational-playbooks/)** — Operational procedures
- **[`11-decision-log/`](docs/11-decision-log/)** — Architectural decision records
- **[`12-roadmap-and-deprecation/`](docs/12-roadmap-and-deprecation/)** — Evolution and sunset policies

---

## Core Principles

1. **Determinism over cleverness** — Same inputs always produce same outputs
2. **Inference over prediction** — We model what capital *must* do, not what price *might* do
3. **Capital behavior is the truth** — Price is a secondary artifact
4. **Explainability is mandatory** — Every output must be explainable in market-structure terms
5. **Silence is valid** — "No meaningful inference" is preferable to false confidence

See **[`docs/00-vision-and-non-goals/design-principles.md`](docs/00-vision-and-non-goals/design-principles.md)** for the complete list.

---

## Language Philosophy

Galactus uses precise, non-advisory language:

### ✅ Allowed
- Capital pressure
- Forced flow
- Regime classification
- Constraint analysis
- Confidence degradation

### ❌ Forbidden
- Buy / Sell signals
- Bullish / Bearish predictions
- Entry / Exit recommendations
- Price targets
- Actionable advice

---

## Technical Architecture

- **Rust** — Production inference engine (deterministic, fast, correct)
- **Python** — Research, experimentation, and discovery (exploratory, iterative)

**Python discovers truth. Rust enforces truth.**

### Directory Structure

```
galactus/
├── core/rust/              # Production inference engine (deterministic core)
└── research/python/        # Research and experimentation (discovery layer)
```

**The boundary between these is sacred and enforced.**

See **[`docs/02-system-architecture/rust-vs-python-contract.md`](docs/02-system-architecture/rust-vs-python-contract.md)** and **[`QUICK-REFERENCE.md`](docs/02-system-architecture/QUICK-REFERENCE.md)**

---

## Development Roadmap

### ✅ **Completed (Q4 2024)**
- **Architecture Foundation**: Rust/Python boundary enforcement implemented
- **Data Layer**: Canonical schemas, ingestion pipeline, and validation
- **Core Frameworks**: Regime classification, confidence assessment, failure analysis, stress scenarios
- **Research Framework**: Experiment templates, promotion checklists, validation tools

### 🔄 **Current Phase (Q1 2025): Signal Development**
- **Research Experiments**: Validate OI decay, hedge pressure, and basis signals
- **Signal Promotion**: Move first validated signal to production Rust core
- **Intent Engine**: Implement core inference logic

### 🎯 **Next Phase (Q2 2025): Production Infrastructure**
- **API Layer**: gRPC/HTTP interfaces for external consumers
- **Monitoring**: Production observability and alerting
- **State Management**: Intent vector persistence and querying

### 📋 **Active Issues**
See **[`UPCOMING_TASKS.md`](UPCOMING_TASKS.md)** for detailed implementation roadmap and **[`issues/`](issues/)** for specific task definitions.

**Priority Order:**
1. Research signal validation and promotion
2. Intent engine core implementation  
3. API layer and external interfaces
4. Monitoring and operational readiness

---

## Success Criteria

Galactus is successful if:

- Its outputs remain interpretable under stress
- Its signals degrade gracefully during regime shifts
- Its architecture resists ad-hoc feature creep
- Its conclusions can be explained without charts
- It avoids producing confident nonsense

**Performance alone is not success.**

---

## Vision Immutability

The core vision defined in **[`docs/00-vision-and-non-goals/`](docs/00-vision-and-non-goals/)** is considered **locked**.

Changes to the vision require:
- Evidence of material market structure changes, or
- Invalidation of core assumptions by empirical data
- Explicit revision through formal process
- Documentation in the decision log

See **[`VISION_LOCK.md`](VISION_LOCK.md)** for the enforcement policy.

---

## Contributing

Before contributing, you **must** read and understand:

1. **[Vision](docs/00-vision-and-non-goals/vision.md)** — What Galactus is and is not
2. **[Non-Goals](docs/00-vision-and-non-goals/explicit-non-goals.md)** — What will never be built
3. **[Design Principles](docs/00-vision-and-non-goals/design-principles.md)** — How decisions are made

Any contribution that conflicts with these documents will be rejected.

---

## License

[To be determined]

---

## Final Statement

**Galactus does not guess.**  
**Galactus does not predict.**  
**Galactus infers, explains, and waits.**

Markets move when capital pressure becomes unavoidable.  
Galactus exists to measure that inevitability.
