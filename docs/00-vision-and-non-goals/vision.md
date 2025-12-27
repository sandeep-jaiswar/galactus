# Galactus — Vision

## Purpose of This Document

This document defines the **core vision** of Project Galactus.

It exists to:
- Anchor all future design and research decisions
- Prevent ideological and architectural drift
- Serve as the ultimate reference when trade-offs arise

If a future decision contradicts this document, **the decision is wrong** unless this document is explicitly revised.

**This document is locked.** See [`VISION_LOCK.md`](../../VISION_LOCK.md) for the immutability policy and formal revision process.

---

## One-Sentence Vision

**Galactus is a deterministic system that infers forced and strategic capital behavior in the Indian financial markets using public data, without predicting prices or issuing trading instructions.**

---

## The Problem Galactus Solves

Most market systems optimize for **price movement**.

This is a flawed abstraction.

Price is:
- A lagging artifact
- A composite outcome of multiple capital constraints
- Often dominated by derivatives, liquidity mechanics, and forced flows

In the Indian market specifically:
- Retail participation is disproportionately high
- Derivatives volume dominates cash markets
- Forced capital (expiry, index rebalancing, margin mechanics) routinely overwhelms discretionary intent

Existing tools:
- React to price
- Overfit indicators
- Conflate noise with signal
- Encourage discretionary interpretation

**Galactus exists to model what capital must do, not what price might do.**

---

## What Galactus Is

Galactus is:

- An **inference engine**, not a prediction engine
- A **capital-behavior model**, not a charting system
- A **deterministic system**, not a probabilistic guessing machine
- A **research-informed platform**, not a tip or signal service

It produces:
- Measures of capital pressure
- Regime classifications
- Stability and confidence indicators
- Explanations grounded in market structure

---

## What Galactus Is Not

Galactus is **explicitly not**:

- A trading platform
- A buy/sell signal generator
- A price target engine
- A high-frequency or latency-arbitrage system
- A discretionary or intuition-driven tool
- A black-box machine learning system

Any feature or proposal that pushes Galactus toward these outcomes must be rejected.

---

## Core Belief System

Galactus is built on the following beliefs:

1. **Capital constraints drive markets**
   - Forced flows matter more than opinions
   - Structural mechanics outweigh narratives in short-to-medium horizons

2. **Inference beats prediction**
   - Knowing *why* something must happen is more robust than guessing *what* will happen
   - Probabilistic intent is more useful than point forecasts

3. **Determinism creates trust**
   - The same inputs must always produce the same outputs
   - Reproducibility matters more than novelty

4. **Most of the time, the correct action is inaction**
   - Galactus must be comfortable producing “no meaningful inference”
   - Silence is preferable to false precision

---

## Intended Users

Galactus is designed for:

- Researchers studying market structure
- Systematic investors seeking explainable signals
- Internal decision-support systems
- Analysts who value probabilistic reasoning over tips

It is **not** designed for:
- Retail speculation
- Gamified trading
- Signal-copying behavior

---

## Success Criteria

Galactus is successful if:

- Its outputs remain interpretable under stress
- Its signals degrade gracefully during regime shifts
- Its architecture resists ad-hoc feature creep
- Its conclusions can be explained without charts
- It avoids producing confident nonsense

Performance alone is **not** success.

---

## Long-Term Aspiration

In the long term, Galactus aims to:

- Become a reference framework for capital-behavior analysis
- Generalize beyond equities to other asset classes
- Serve as a foundation for compliant, explainable market intelligence

Galactus does not aim to “beat the market.”  
It aims to **understand it correctly**.

---

## Revision Philosophy

This document may evolve, but only when:

- Market structure materially changes, or
- Core assumptions are invalidated by evidence

Revisions must be explicit, justified, and recorded.

---

## Final Statement

**Galactus does not guess.  
Galactus does not predict.  
Galactus infers, explains, and waits.**
