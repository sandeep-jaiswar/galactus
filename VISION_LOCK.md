# Vision Lock — Galactus Core Vision Immutability Policy

---

## Purpose of This Document

This document establishes the **immutability policy** for the core vision of Project Galactus.

It exists to:
- Prevent unauthorized or casual changes to the foundational vision
- Establish a formal process for vision evolution
- Protect the system from scope creep, feature drift, and architectural compromise
- Serve as the ultimate enforcement mechanism for architectural decisions

---

## Locked Vision Documents

The following documents constitute the **locked core vision** of Galactus:

1. **[`docs/00-vision-and-non-goals/vision.md`](docs/00-vision-and-non-goals/vision.md)**
   - Defines what Galactus is
   - Establishes core beliefs and principles
   - Sets success criteria

2. **[`docs/00-vision-and-non-goals/explicit-non-goals.md`](docs/00-vision-and-non-goals/explicit-non-goals.md)**
   - Defines what Galactus will never become
   - Establishes hard boundaries:
     - **No trading systems**
     - **No advisory tools**
     - **No prediction engines**
     - **No execution logic**

3. **[`docs/00-vision-and-non-goals/design-principles.md`](docs/00-vision-and-non-goals/design-principles.md)**
   - Establishes decision-making framework
   - Defines trade-off resolution hierarchy
   - Sets architectural constraints

---

## What "Locked" Means

### Immutability
- These documents represent the **permanent foundation** of Galactus
- Changes are prohibited except through the formal revision process
- No feature, proposal, or optimization may contradict these documents
- If a decision conflicts with the vision, **the decision is wrong**

### Enforcement Priority
The locked vision documents have **absolute priority** over:
- Feature requests
- Performance requirements
- Commercial pressure
- Technical convenience
- Stakeholder preferences
- Competitive dynamics

### Rejection Authority
Any proposal that:
- Conflicts with the core vision
- Redefines a non-goal to justify itself
- Requires "temporary" vision compromise
- Obscures the fundamental purpose

**Must be rejected immediately, without implementation.**

---

## Explicit Exclusions (Non-Negotiable)

The following are **permanently excluded** from Galactus:

### ❌ Category 1: Trading & Execution
- Buy/sell signal generation
- Entry/exit recommendations
- Position sizing or leverage advice
- Trade execution or broker integration
- Order placement logic
- PnL optimization

**Rationale:** Galactus provides inference, not action. Trading decisions depend on external constraints beyond system scope.

### ❌ Category 2: Price Prediction & Forecasting
- Future price prediction
- Price target estimation
- Return forecasting
- Time-series price models
- Pattern-based chart prediction
- Upside/downside range estimation
- Prediction guarantees or warranties
- Performance promises or accuracy assurances

**Rationale:** Price is an outcome, not a primitive. Predicting it obscures causal structure. Guarantees on future outcomes create false expectations and are incompatible with market uncertainty.

### ❌ Category 3: Advisory Tools
- Personalized investment advice
- Portfolio-specific recommendations
- User-tailored outputs
- Individual risk profiling
- Custom strategy suggestions

**Rationale:** Advisory services cross regulatory boundaries and compromise objectivity.

### ❌ Category 4: Alpha Generation & Performance Competition
- Claiming to generate alpha or excess returns
- Competing on performance metrics (Sharpe ratio, win rate, returns)
- Optimizing for benchmark outperformance
- Marketing as a trading edge or competitive advantage
- Promising superior returns or outperformance
- Measuring success primarily through PnL metrics

**Rationale:** Alpha generation requires trading decisions and execution beyond Galactus scope. Galactus provides structural inference, not investment performance. Performance competition encourages overfitting.

### ❌ Category 5: Execution Logic
- Automated trade execution
- Real-time order management
- Broker API integration
- Position management systems
- Risk management execution

**Rationale:** Execution belongs downstream, outside Galactus scope.

---

## Core Identity (Permanent)

Galactus is and will always be:

- A **capital-pressure inference engine**
- A **deterministic system** (same inputs → same outputs)
- A **structural analysis tool** (market mechanics, not price patterns)
- A **research-informed platform** (evidence-based, not intuition-driven)

Galactus produces:
- Measures of capital pressure and constraints
- Regime classifications and stability indicators
- Confidence metrics and uncertainty signals
- Explanations grounded in market structure

---

## Revision Process (How Vision Can Change)

### Permitted Grounds for Revision
Vision documents may be revised **only** when:

1. **Market structure materially changes**
   - Regulatory framework changes fundamentally
   - Market microstructure evolves significantly
   - New forcing mechanisms emerge

2. **Core assumptions are empirically invalidated**
   - Evidence shows fundamental model failure
   - Structural relationships break permanently
   - Causality assumptions proven incorrect

3. **Technical impossibility discovered**
   - Determinism becomes technically infeasible
   - Data requirements become permanently unavailable

### Revision Requirements
Any revision must include:

1. **Formal proposal document** explaining:
   - What changed in market structure
   - Why current vision is invalidated
   - Evidence supporting the change
   - Impact on existing system

2. **Decision log entry** in [`docs/11-decision-log/decision-log.md`](docs/11-decision-log/decision-log.md)

3. **Stakeholder review** with explicit approval

4. **Documentation cascade**
   - Update all dependent documents
   - Identify affected components
   - Plan migration path

5. **Version tracking**
   - Preserve previous version
   - Document revision history
   - Explain rationale

### Prohibited Revision Grounds
Vision **may not** be revised due to:
- Poor performance in backtests
- Commercial pressure or competitive dynamics
- Technical implementation difficulty
- Stakeholder preference changes
- Desire for additional features
- Market timing or tactical considerations

---

## Conflict Resolution Hierarchy

When trade-offs arise, the following hierarchy applies (highest to lowest priority):

1. **Vision compliance** — Does it align with core vision?
2. **Non-goals enforcement** — Does it violate explicit non-goals?
3. **Design principles** — Does it follow design principles hierarchy?
4. **Technical quality** — Is it deterministic, explainable, stable?
5. **Performance** — Does it perform well empirically?

This ordering is **permanent and non-negotiable**.

---

## Enforcement Mechanisms

### Development Process
- All proposals must reference vision documents
- Code reviews must verify vision compliance
- Architecture decisions require vision justification

### Documentation
- New documents must link to core vision
- Conflicting content triggers mandatory reconciliation
- Vision references required in major components

### Governance
- Vision compliance is a blocking requirement
- Non-compliant features may not merge
- Violations trigger rollback and review

---

## Anti-Patterns (Automatic Rejection)

The following patterns **automatically violate** the vision lock:

### 🚫 Feature Drift
- "Just a small trading feature..."
- "Only a basic buy/sell signal..."
- "We could add advisory as optional..."

### 🚫 Scope Creep
- "Since we have capital data, why not predict price..."
- "Users want recommendations, so..."
- "Competitors offer execution, we should too..."

### 🚫 Vision Erosion
- "The vision is too restrictive..."
- "Times have changed, we need to adapt..."
- "Let's temporarily bypass this..."

### 🚫 Definitional Drift
- Redefining "inference" to include "prediction"
- Redefining "capital pressure" as "price momentum"
- Redefining "analysis" as "advice"

**All such proposals must be rejected immediately.**

---

## Benefits of Vision Lock

By maintaining vision immutability, Galactus achieves:

1. **Architectural coherence** — All components serve unified purpose
2. **Scope protection** — Feature creep cannot compromise core identity
3. **Liability clarity** — Non-advisory stance remains unambiguous
4. **Technical focus** — Resources directed at core competency
5. **Long-term viability** — Foundation remains stable across iterations
6. **Trust preservation** — Users understand permanent boundaries

---

## Monitoring and Audit

### Quarterly Review
- Verify vision compliance across codebase
- Check for scope drift in new features
- Audit language in outputs and documentation
- Review decision log for vision consistency

### Version Control
- Tag vision document versions
- Track revision history explicitly
- Maintain change justification log

### Compliance Checks
- Automated language scanning (no forbidden terms)
- Architectural boundary validation
- Documentation consistency checks

---

## Final Statement

**The core vision of Galactus is locked.**

This lock is not a limitation—it is Galactus's greatest strength.

By refusing to become everything, Galactus can be excellent at one thing:

**Inferring forced and strategic capital behavior in financial markets.**

Everything else is out of scope, by design, permanently.

---

## Document Authority

This document has the same authority level as the vision documents themselves.

Any attempt to:
- Circumvent this lock
- Redefine its terms
- Override its enforcement

**Is considered a vision violation.**

---

**Galactus does not predict.**  
**Galactus does not advise.**  
**Galactus does not trade.**

**This is permanent.**
