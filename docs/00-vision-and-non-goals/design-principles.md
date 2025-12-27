# Galactus — Design Principles

## Purpose of This Document

This document defines the **design principles** that govern all decisions in Project Galactus.

It exists to:
- Guide decision-making under ambiguity
- Resolve trade-offs consistently
- Maintain coherence as the system evolves

When multiple solutions are technically valid, **the one that best aligns with these principles must be chosen**.

**This document is locked.** See [`VISION_LOCK.md`](../../VISION_LOCK.md) for the immutability policy.

---

## Principle 1 — Determinism Is a Feature, Not an Implementation Detail

Galactus prioritizes deterministic behavior over all other technical qualities.

- Same inputs must always produce the same outputs
- No hidden state
- No time-dependent behavior unless explicitly modeled
- No randomness in production logic without explicit seeding and documentation

**Rationale**  
Trust in inference requires reproducibility.  
If outputs cannot be replayed and explained, they cannot be relied upon.

---

## Principle 2 — Inference Over Prediction

Galactus infers **constraints, pressures, and intent** rather than predicting outcomes.

- Outputs describe *why capital must act*
- Outputs do not describe *what price will do*
- Probabilities are acceptable; forecasts are not

**Rationale**  
Prediction obscures causality.  
Inference preserves it.

---

## Principle 3 — Capital Behavior Is the Primitive

Price, volume, and indicators are secondary artifacts.

Galactus treats the following as first-class primitives:
- Forced flows
- Liquidity constraints
- Derivative positioning
- Regulatory and structural mechanics

Any signal must be explainable in terms of **capital behavior**, not statistical coincidence.

---

## Principle 4 — Explainability Is Mandatory

Every output must be explainable using:
- Market structure
- Capital mechanics
- Explicit assumptions

If a result cannot be explained clearly to a knowledgeable analyst, it is considered invalid—even if it performs well historically.

**Rationale**  
Unexplainable correctness is indistinguishable from luck.

---

## Principle 5 — Separation of Discovery and Enforcement

Galactus enforces a strict boundary:

- **Python discovers truth**
- **Rust enforces truth**

Research code is allowed to be:
- Exploratory
- Iterative
- Imperfect

Production code must be:
- Minimal
- Deterministic
- Boring

**Rationale**  
Discovery requires freedom.  
Enforcement requires discipline.

---

## Principle 6 — Stability Beats Sharpness

Galactus prefers:
- Broad validity over peak performance
- Graceful degradation over brittle precision
- Fewer signals over many marginal ones

A signal that works “most of the time” but fails catastrophically is inferior to one that works modestly but consistently.

---

## Principle 7 — Silence Is a Valid Output

Galactus is allowed—and encouraged—to output:
- “No meaningful inference”
- “Insufficient evidence”
- “Regime uncertainty”

The absence of inference is preferable to false confidence.

**Rationale**  
Most market time contains no exploitable structural pressure.

---

## Principle 8 — Explicit Failure Is Better Than Hidden Failure

Galactus must:
- Detect when its assumptions break
- Surface uncertainty explicitly
- Avoid masking failures with smoothing or heuristics

Failure modes should be documented, not patched silently.

---

## Principle 9 — Market Structure Over Narrative

Galactus does not reason about:
- Stories
- Opinions
- News sentiment as truth

Narratives may be observed only as **crowding or attention proxies**, never as causal drivers.

---

## Principle 10 — Documentation Is a First-Class Artifact

If a design choice cannot be:
- Written down
- Defended
- Revisited

Then it should not exist.

Code without documentation is considered incomplete.

---

## Principle 11 — Minimalism in the Core

The core engine should:
- Do fewer things, extremely well
- Avoid feature accumulation
- Resist “just in case” logic

Complexity belongs in research, not in production.

---

## Principle 12 — Time Awareness

Galactus acknowledges that:
- Markets evolve
- Structures decay
- Signals expire

No component is permanent.  
All components must be allowed to sunset.

---

## Enforcement Rule

When principles conflict, they are resolved in this order:

1. Determinism
2. Inference over prediction
3. Explainability
4. Stability
5. Performance

This ordering is intentional and non-negotiable.

---

## Final Statement

**Galactus is designed to be correct, calm, and conservative.**

Speed, excitement, and confidence are secondary to truth.
