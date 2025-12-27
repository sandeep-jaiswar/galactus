# Galactus — Explicit Non-Goals

## Purpose of This Document

This document defines what **Galactus will never attempt to do**.

It exists to:
- Prevent scope creep disguised as “enhancements”
- Protect the core vision from commercial or ideological pressure
- Provide an authoritative reference for rejecting proposals

If a feature aligns with these non-goals, it must be rejected **without debate**.

---

## Why Non-Goals Matter

Most complex systems fail not because of poor execution, but because they:
- Attempt to solve adjacent problems
- Gradually absorb conflicting objectives
- Accumulate features that dilute their core advantage

Galactus is intentionally narrow.

Its strength comes from **what it refuses to become**.

---

## Category 1 — Trading & Execution (Hard No)

Galactus will **never**:

- Generate buy/sell signals
- Suggest entries, exits, targets, or stop-losses
- Recommend position sizing or leverage
- Execute trades or integrate with broker APIs
- Optimize for PnL, hit rate, or trade frequency

**Rationale**  
Trading decisions are downstream choices that depend on risk appetite, capital, and constraints external to Galactus.

Galactus provides inference, not action.

---

## Category 2 — Price Prediction & Forecasting

Galactus will **never**:

- Predict future prices or price ranges
- Forecast returns
- Estimate upside/downside targets
- Compete with time-series prediction models

This includes:
- Linear regression on price
- ARIMA-style forecasting
- Deep learning price models
- Pattern-based chart prediction

**Rationale**  
Price is an outcome, not a primitive.  
Predicting it obscures the causal structure Galactus is designed to reveal.

---

## Category 3 — Indicator Aggregation Platforms

Galactus will **never**:

- Repackage traditional indicators (RSI, MACD, VWAP, EMAs) as core signals
- Act as a configurable indicator dashboard
- Provide indicator “confluence” scoring

Indicators may appear in **research contexts only**, and only to explain *why they fail* or how they proxy deeper mechanics.

**Rationale**  
Indicator aggregation optimizes for familiarity, not truth.

---

## Category 4 — Black-Box Intelligence

Galactus will **never**:

- Use uninterpretable models as default decision-makers
- Rely on opaque deep learning systems without causal explanation
- Produce outputs that cannot be explained in market-structure terms

This includes:
- End-to-end neural networks trained on price alone
- Models that outperform but cannot be reasoned about
- Signals that cannot articulate *why capital must act*

**Rationale**  
Explainability is a core feature, not a trade-off.

---

## Category 5 — Retail Engagement & Gamification

Galactus will **never**:

- Gamify market analysis
- Optimize for engagement metrics
- Provide “confidence scores” designed to excite users
- Use emotionally charged language

This includes:
- Alerts designed to trigger urgency
- Color-coding implying actionability
- Leaderboards or performance bragging

**Rationale**  
Clarity beats excitement.  
Restraint beats persuasion.

---

## Category 6 — Personalized Advice

Galactus will **never**:

- Provide personalized investment advice
- Tailor outputs to individual portfolios
- Optimize recommendations based on user-specific constraints

**Rationale**  
Personalization crosses regulatory boundaries and compromises objectivity.

Galactus operates at the **market-structure level**, not the individual level.

---

## Category 7 — Latency Arms Race

Galactus will **never**:

- Compete on microsecond or millisecond latency
- Engage in order-book microstructure games
- Attempt to front-run via speed

**Rationale**  
Galactus is designed for **structural inference**, not speed advantage.

Correctness under stress matters more than being first.

---

## Category 8 — Narrative & News Interpretation Engines

Galactus will **never**:

- Act as a sentiment analysis engine for news or social media
- Attempt to interpret narratives as causal drivers
- Compete with news-based trading systems

Public attention data may be used **only as a proxy for retail crowding**, not as narrative truth.

**Rationale**  
Narratives explain behavior retroactively; they rarely predict constraints.

---

## Category 9 — Over-Optimization & Hyper-Tuning

Galactus will **never**:

- Optimize parameters solely for backtest performance
- Allow extensive hyperparameter tuning without economic justification
- Treat backtest metrics as proof of correctness

**Rationale**  
Robustness beats optimization.  
Stability beats sharpness.

---

## Category 10 — Silent Scope Expansion

Galactus will **never**:

- Add major capabilities without documentation updates
- Introduce features without explicit rationale
- Allow “temporary” hacks to become permanent

Any violation of this principle is considered **technical and conceptual debt**.

---

## Enforcement Rule

If a proposal:
- Conflicts with this document, or
- Requires redefining a non-goal to justify itself

Then the proposal must be rejected **before implementation**.

This document outranks:
- Feature requests
- Performance goals
- Commercial pressure

---

## Final Statement

**Galactus succeeds by being narrow, disciplined, and principled.**

Everything it does well comes from the many things it refuses to do.
