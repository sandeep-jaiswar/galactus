# Galactus — Forbidden Metrics

## Purpose of This Document

This document defines the **classes of metrics that are explicitly forbidden** in Project Galactus.

It exists to:
- Prevent regression into indicator-based systems
- Eliminate ambiguous or misleading measurements
- Protect interpretability and causal reasoning
- Provide an authoritative basis for rejection

Performance does not justify violation of this document.

---

## Core Principle

If a metric cannot be grounded in **capital behavior and constraints**, it does not belong in Galactus.

Some metrics are seductive precisely because they appear to work—until they fail silently.

---

## Category 1 — Pure Price-Derived Metrics

### Description

Metrics derived solely from historical price, without reference to capital structure.

### Examples

- RSI
- MACD
- Simple and exponential moving averages
- Bollinger Bands
- Rate of change (price-only)

### Reason for Rejection

These metrics:
- Describe outcomes, not causes
- Collapse diverse capital dynamics into a single dimension
- Encourage reactive interpretation

---

## Category 2 — Pattern Recognition Metrics

### Description

Metrics that attempt to infer meaning from visual or statistical price patterns.

### Examples

- Candlestick patterns
- Chart formations (head-and-shoulders, triangles)
- Trendline break detectors

### Reason for Rejection

Patterns:
- Are observer-dependent
- Lack causal grounding
- Fail under regime shifts

---

## Category 3 — Profit-Oriented Metrics (as Inputs)

### Description

Metrics that optimize or reference financial outcomes.

### Examples

- Expected return
- PnL
- Sharpe ratio
- Hit rate
- Maximum drawdown

### Reason for Rejection

These metrics:
- Optimize hindsight
- Collapse inference into outcome chasing
- Are inappropriate as causal inputs

They may be used **only for evaluation**, never for inference.

---

## Category 4 — Black-Box Composite Scores

### Description

Metrics produced by opaque aggregation or machine learning without interpretability.

### Examples

- Neural network embeddings
- Proprietary “alpha scores”
- Unlabeled factor blends

### Reason for Rejection

These metrics:
- Obscure causality
- Cannot be audited
- Fail explainability requirements

---

## Category 5 — Sentiment and Narrative Metrics

### Description

Metrics derived from textual interpretation or emotional labeling.

### Examples

- News sentiment scores
- Social media sentiment polarity
- Analyst opinion aggregation

### Reason for Rejection

Narratives explain behavior retroactively and inconsistently.

Galactus observes **reaction**, not rhetoric.

---

## Category 6 — Relative Ranking Metrics Without Context

### Description

Metrics that rank instruments without absolute grounding.

### Examples

- Percentile ranks
- Cross-sectional z-scores
- Leaderboards

### Reason for Rejection

Relative ranking:
- Obscures absolute pressure
- Creates false comparability
- Encourages action without context

---

## Category 7 — Over-Fitted or Hyper-Tuned Metrics

### Description

Metrics whose parameters are optimized solely for historical performance.

### Examples

- Indicators with hand-tuned thresholds
- Metrics requiring frequent re-optimization
- Signals sensitive to small parameter changes

### Reason for Rejection

These metrics:
- Are unstable
- Encode noise
- Fail under regime shifts

---

## Enforcement Rule

If a proposed metric:
- Falls into any forbidden category
- Requires justification to bypass this document

Then the metric must be rejected.

There are no exceptions.

---

## Evolution of Forbidden Metrics

This list may grow, but only with:
- Explicit rationale
- Documentation update
- Alignment with design principles

Removal from this list requires extraordinary justification.

---

## Final Statement

**Galactus avoids many metrics not because they never work,  
but because they work for the wrong reasons.**
