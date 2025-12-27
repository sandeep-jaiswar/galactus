# Galactus — Promotion Checklist

## Purpose of This Document

This document defines the **mandatory checklist** that any feature, metric, or signal must pass **before promotion** from the research layer (Python) into the core Intent Engine (Rust).

It exists to:
- Prevent premature productionization
- Enforce structural integrity
- Block performance-driven shortcuts
- Preserve determinism and explainability

Failure to satisfy **any single item** blocks promotion.

---

## Promotion Scope

This checklist applies to promotion of:

- Features → Signals
- Research Signals → Core Signals
- New inference logic → Intent Engine modules
- Changes to existing core signals

This checklist does **not** apply to:
- Exploratory notebooks
- Research-only visualizations
- One-off analyses

---

## Section 1 — Structural Validity (Non-Negotiable)

### 1.1 Capital Behavior Mapping

The proposal explicitly answers:

- [ ] What capital is involved?
- [ ] What constraint is binding?
- [ ] Why is action required (not optional)?
- [ ] What prevents delay?

If any answer is vague or narrative-based → **Reject**

---

### 1.2 Market Structure Alignment

- [ ] The mechanism aligns with documented market theory
- [ ] It does not contradict Indian market structure assumptions
- [ ] It respects derivatives dominance where applicable

If alignment requires reinterpretation of theory → **Reject**

---

## Section 2 — Determinism & Reproducibility

### 2.1 Deterministic Definition

- [ ] Mathematical / logical definition is explicit
- [ ] No randomness or implicit state
- [ ] Same inputs always produce same outputs

If determinism depends on environment or ordering → **Reject**

---

### 2.2 Replayability

- [ ] Can be recomputed from raw canonical events
- [ ] Does not depend on transient system state
- [ ] Historical inference can be replayed exactly

If replay is approximate → **Reject**

---

## Section 3 — Data Discipline

### 3.1 Data Source Compliance

- [ ] Uses only approved public data sources
- [ ] No paid, proprietary, or privileged data
- [ ] No personalized or user-level data

Violation of data policy → **Immediate Reject**

---

### 3.2 Schema Compliance

- [ ] Uses canonical schemas
- [ ] Respects schema versioning
- [ ] Handles missing data explicitly

Schema shortcuts are forbidden.

---

## Section 4 — Normalization & Scaling

### 4.1 Normalization Justification

- [ ] Capital size normalized
- [ ] Liquidity normalized
- [ ] Time urgency modeled explicitly
- [ ] Volatility normalization justified (if used)

Raw or absolute metrics → **Reject**

---

### 4.2 Cross-Instrument Validity

- [ ] Meaning is preserved across instruments
- [ ] Scale does not dominate interpretation
- [ ] Illiquid instruments handled explicitly

If interpretation changes silently → **Reject**

---

## Section 5 — Regime Awareness

### 5.1 Regime Conditioning

- [ ] Applicable regimes are documented
- [ ] Known invalid regimes are documented
- [ ] Behavior under regime transition is understood

Signals without regime scope are incomplete.

---

### 5.2 Regime Failure Handling

- [ ] Confidence degrades under regime ambiguity
- [ ] Signal does not force activation across regimes

If regime awareness is implicit → **Reject**

---

## Section 6 — Failure Analysis (Mandatory)

### 6.1 Known Failure Modes

- [ ] At least 3 realistic failure scenarios documented
- [ ] Structural reasons for failure explained
- [ ] Failure does not rely on “unexpected price action”

If failure modes are unknown → **Reject**

---

### 6.2 Historical Failure Evidence

- [ ] Documented periods where the signal failed
- [ ] Explanation of why failure occurred
- [ ] No selective omission of bad periods

Signals that “never fail” are invalid.

---

## Section 7 — Backtesting Integrity

### 7.1 Backtest Framing

- [ ] Backtests framed around structure, not profit
- [ ] Event-time aligned
- [ ] No forward-looking bias

PnL-centric framing → **Reject**

---

### 7.2 Stability Evidence

- [ ] Behavior consistent across multiple regimes
- [ ] No fragile parameter dependence
- [ ] Performance not driven by a single period

Overfitting indicators → **Reject**

---

## Section 8 — Interpretability & Explainability

### 8.1 Explanation Test

A knowledgeable analyst can explain:
- [ ] Why the signal activated
- [ ] What pressure it represents
- [ ] Why confidence is what it is

If explanation requires charts → **Reject**

---

### 8.2 Component Transparency

- [ ] All sub-metrics are visible
- [ ] No opaque aggregation
- [ ] Contributions are inspectable

Black-box logic → **Reject**

---

## Section 9 — Confidence & Stability Integration

### 9.1 Confidence Logic

- [ ] Confidence calculation is explicit
- [ ] Confidence degrades under uncertainty
- [ ] Hard stop conditions are defined

Overconfident defaults → **Reject**

---

### 9.2 Stability Behavior

- [ ] Signal persistence is understood
- [ ] Decay behavior is modeled
- [ ] Abrupt collapse scenarios are documented

Timeless signals are invalid.

---

## Section 10 — Rust Readiness (Final Gate)

### 10.1 Rust Implementability

- [ ] Logic is implementable in pure Rust
- [ ] No Python-only dependencies
- [ ] Performance characteristics are predictable

If logic requires Python flexibility → **Reject**

---

### 10.2 Testing Requirements

- [ ] Unit tests defined
- [ ] Golden input/output cases defined
- [ ] Edge cases enumerated

Untestable logic → **Reject**

---

## Final Promotion Decision

Promotion requires:

- ✅ **ALL checklist items satisfied**
- 📄 **Documentation updated**
- 🧠 **Decision logged**
- 🔒 **Scope explicitly bounded**

If promotion requires “we’ll refine later” → **Reject**

---

## Final Statement

**Promotion is not a reward.  
It is a commitment to correctness under scrutiny.**

Galactus promotes slowly so it can remain trusted forever.
