# Galactus — Research Methodology

## Purpose of This Document

This document defines the **research methodology** used in Project Galactus.

It exists to:
- Prevent p-hacking and overfitting
- Enforce discipline in hypothesis testing
- Separate insight from coincidence
- Ensure research results are reproducible and honest

Research is how Galactus learns.  
Methodology is how it avoids self-deception.

---

## Core Research Philosophy

Galactus research is guided by three beliefs:

1. **Most hypotheses are wrong**
2. **Performance is not proof**
3. **Understanding matters more than optimization**

Research exists to invalidate ideas quickly, not to confirm them eagerly.

---

## Hypothesis-Driven Research

All research must begin with an explicit hypothesis.

A valid hypothesis must state:
- The capital behavior being tested
- The constraint or mechanism involved
- The expected observable effect
- The conditions under which it should fail

Exploration without hypothesis is permitted, but promotion without hypothesis is forbidden.

---

## Research Stages

Research proceeds through the following stages:

1. Exploration  
2. Hypothesis Formulation  
3. Controlled Testing  
4. Failure Analysis  
5. Validation Across Regimes  

Skipping stages is not allowed.

---

## Data Discipline

Research must:
- Use the same canonical data sources as production
- Respect schema versions
- Avoid forward-looking bias
- Document any data limitations

Synthetic or external data must be clearly labeled.

---

## Avoiding Look-Ahead Bias

All research must:
- Align strictly to event time
- Avoid using future information implicitly
- Respect disclosure and publication delays

Any look-ahead contamination invalidates results.

---

## Parameter Discipline

Parameters must:
- Be few and interpretable
- Have economic justification
- Remain stable across regimes

Hyperparameter tuning for performance is strongly discouraged.

---

## Evaluation Philosophy

Research evaluation focuses on:
- Stability across regimes
- Behavior under stress
- Failure characteristics

Single-number metrics (e.g., accuracy) are insufficient.

---

## Counterfactual Thinking

Every research result must consider:
- What would disprove this?
- Under what conditions does this fail?
- Is the result structurally plausible?

Results without counterfactual analysis are incomplete.

---

## Reproducibility Requirements

All research must be:
- Reproducible from raw data
- Version-controlled
- Documented sufficiently for re-execution

If a result cannot be reproduced, it does not exist.

---

## Negative Results

Negative results are:
- Valuable
- Documented
- Preserved

Discarding failed hypotheses is a success, not a loss.

---

## Promotion Readiness

Research is considered promotion-ready only when:
- The hypothesis is structurally sound
- Behavior is stable across regimes
- Failure modes are understood
- Results are reproducible

Performance alone is insufficient.

---

## Ethical Guardrails

Research must:
- Avoid personalized inference
- Respect compliance boundaries
- Avoid outcome-driven framing

Insight must not cross into advice.

---

## Related Documentation

For detailed experiment design guidance:
- [`experiment-design.md`](experiment-design.md) — Complete experiment framework
- [`experiment-template.md`](experiment-template.md) — Documentation template
- [`experiment-checklist.md`](experiment-checklist.md) — Validation checklist
- [`experiment-examples.md`](experiment-examples.md) — Practical examples

For promotion process:
- [`promotion-checklist.md`](promotion-checklist.md) — Promotion requirements
- [`backtesting-guidelines.md`](backtesting-guidelines.md) — Validation standards

---

## Final Statement

**Research in Galactus exists to reduce ignorance, not to create confidence.**

Truth survives scrutiny. Hypotheses do not.
