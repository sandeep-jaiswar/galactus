# Galactus — Experiment Design

## Purpose

This document defines the **intent, constraints, and discipline** governing **experiment design** within Project Galactus.

It exists to ensure that all experiments:
- Advance structural understanding of markets
- Remain grounded in capital behavior
- Avoid outcome-driven or performance-led bias
- Produce results that are reproducible, interpretable, and promotable

Experiments are not vehicles for validation.  
They are tools for **learning and falsification**.

---

## Scope

### This Document Covers

This document governs:

- How experiments are framed and justified
- What constitutes a valid experiment in Galactus
- Constraints on data, methodology, and interpretation
- How experiments relate to features, signals, and promotion
- What experimental results are considered meaningful

### This Document Does NOT Cover

This document explicitly does **not** cover:

- Trading strategy design
- Execution or PnL optimization
- Hyperparameter tuning for performance
- UI, visualization, or presentation concerns
- Promotion criteria (covered separately in the Promotion Checklist)

Experiments are **pre-decision artifacts**, not production candidates.

---

## Definition: Experiment

In Galactus, an **experiment** is defined as:

> A controlled investigation designed to test whether a specific capital behavior or constraint produces observable, structurally consistent effects under defined conditions.

An experiment must aim to **invalidate or refine a hypothesis**, not to confirm it.

---

## Core Principles

### 1. Determinism Over Cleverness

- Experiments must be reproducible end-to-end
- Same inputs must produce the same results
- No hidden randomness, sampling tricks, or adaptive logic

Clever experiments that cannot be replayed are invalid.

---

### 2. Inference Over Prediction

- Experiments test *mechanisms*, not forecasts
- Outcomes are evaluated structurally, not financially
- “Did pressure exist?” matters more than “Did price move?”

Predictive success without explanation is insufficient.

---

### 3. Capital Behavior as First-Class Truth

Every experiment must explicitly define:

- What capital is involved
- What constraint is being tested
- Why this constraint should force or bias action
- Over what time window the effect should appear

Experiments without capital grounding are rejected.

---

### 4. Failure Is a Valid Outcome

Experiments are successful if they:
- Disprove a hypothesis
- Reveal instability
- Expose regime dependence
- Identify incorrect assumptions

Experiments that “work everywhere” are suspect.

---

## Experiment Framing Requirements

Every experiment must clearly document:

- **Hypothesis**  
  What specific capital behavior is being tested

- **Mechanism**  
  Why the behavior should exist structurally

- **Expected Observations**  
  What evidence would support or refute the hypothesis

- **Failure Conditions**  
  When and why the experiment should fail

Experiments without explicit failure conditions are incomplete.

---

## Data and Methodology Constraints

All experiments must:

- Use approved, public data sources
- Respect canonical schemas and event time
- Avoid look-ahead bias explicitly
- Declare all assumptions and simplifications

Any deviation must be documented and justified.

---

## Evaluation Standards

Experiments are evaluated on:

- Structural consistency across regimes
- Stability under stress and edge conditions
- Interpretability of results
- Alignment with documented market theory

Performance metrics (PnL, accuracy, returns) are **secondary and optional**, never primary.

---

## Relationship to Features and Signals

- Experiments may produce **insights**
- Insights may lead to **feature hypotheses**
- Features may eventually become **signals** via the signal lifecycle

No experiment directly produces a production signal.

---

## Invalid Experiment Patterns (Explicit)

The following invalidate an experiment:

- Outcome-led framing (“This works because returns are high”)
- Selective time period analysis
- Parameter tuning after observing results
- Silent data exclusions
- Narrative explanations without structural grounding

These are considered research failures.

---

## Documentation Requirements

Every experiment must produce:

- A written summary of intent and findings
- Explicit documentation of failures
- Clear statement of what was learned
- Recommendation (promote, refine, reject)

Undocumented experiments do not exist.

---

## Open Questions

Open questions are expected and encouraged.

They must be:
- Explicitly stated
- Tracked across iterations
- Revisited as new evidence emerges

Untracked ambiguity is technical debt.

---

## Revision History

- v1.0 — Formalized experiment design discipline aligned with Galactus research framework  
- v0.1 — Initial draft
