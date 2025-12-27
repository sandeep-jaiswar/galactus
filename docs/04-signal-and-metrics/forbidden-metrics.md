# Galactus — Forbidden Metrics Registry

## Purpose of This Document

This document defines the **classes of metrics that are permanently forbidden** in Project Galactus.

It exists to:
- Prevent regression into indicator-based systems
- Eliminate ambiguous or misleading measurements
- Protect interpretability and causal reasoning
- Provide an authoritative basis for rejection

Performance does not justify violation of this document.

**This registry is permanent and immutable.** Metrics listed here are forbidden indefinitely.

---

## Quick Reference Registry

The following table provides a quick lookup of permanently forbidden metric categories and key examples:

| Category | Key Forbidden Metrics | Why Forbidden |
|----------|----------------------|---------------|
| **Pure Price-Derived** | RSI, MACD, Moving Averages, Bollinger Bands | Describe outcomes, not causes |
| **Pattern Recognition** | Candlestick patterns, Chart formations, Head-and-shoulders | Observer-dependent, lack causal grounding |
| **Profit-Oriented** | Expected return, PnL, Sharpe ratio (as inputs), Hit rate | Optimize hindsight, inappropriate as causal inputs |
| **Black-Box Composite** | Neural embeddings, Proprietary alpha scores, Unlabeled factor blends | Obscure causality, cannot be audited |
| **Sentiment & Narrative** | News sentiment scores, Social media sentiment, Analyst opinion aggregation | Explain retroactively, inconsistent |
| **Relative Ranking** | Percentile ranks (without context), Cross-sectional z-scores, Leaderboards | Obscure absolute pressure, false comparability |
| **Over-Fitted** | Hand-tuned thresholds, Frequently re-optimized metrics, Parameter-sensitive signals | Unstable, encode noise |

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

## Permanently Forbidden Metrics — Explicit List

The following specific metrics are **permanently and explicitly forbidden**:

### Technical Indicators (Pure Price-Derived)
- **RSI (Relative Strength Index)** — Momentum oscillator without capital grounding
- **MACD (Moving Average Convergence Divergence)** — Price momentum indicator
- **Simple Moving Averages (SMA)** — Price smoothing without constraint context
- **Exponential Moving Averages (EMA)** — Weighted price averaging
- **Bollinger Bands** — Volatility bands derived solely from price
- **Stochastic Oscillator** — Momentum indicator
- **Rate of Change (ROC)** — Price-only momentum measure

### Chart Patterns
- **Head and Shoulders** — Visual pattern recognition
- **Double Top/Bottom** — Visual pattern recognition
- **Triangle Patterns** — Ascending, descending, symmetrical triangles
- **Flag and Pennant Patterns** — Continuation patterns
- **Wedge Patterns** — Rising and falling wedges
- **Cup and Handle** — Visual formation
- **Candlestick Patterns** — Doji, hammer, engulfing, etc.

### Sentiment Scores
- **News Sentiment Scores** — Text-based emotional polarity
- **Social Media Sentiment** — Twitter/Reddit sentiment aggregation
- **Fear and Greed Index** — Composite sentiment measure
- **Analyst Opinion Scores** — Aggregated analyst ratings
- **Market Mood Indicators** — Derived from narrative interpretation

### Black-Box Embeddings
- **Neural Network Embeddings** — Latent representations without interpretability
- **Autoencoder Features** — Compressed representations
- **Proprietary Alpha Scores** — Vendor-provided opaque scores
- **Unlabeled Factor Blends** — Composite scores without component transparency
- **Deep Learning Features** — Hidden layer outputs without explanation

### Other Forbidden Categories
- **Fibonacci Retracements** — Arbitrary numerical sequences
- **Elliott Wave Counts** — Subjective wave pattern interpretation
- **Ichimoku Cloud** — Multi-component indicator system
- **Parabolic SAR** — Stop and reverse indicator
- **Average Directional Index (ADX)** — Trend strength indicator

**This list is not exhaustive.** Any metric falling into the forbidden categories above must be rejected, even if not explicitly named here.

---

## Enforcement Rule

If a proposed metric:
- Falls into any forbidden category
- Appears on the permanently forbidden list
- Cannot be grounded in capital behavior and constraints
- Requires justification to bypass this document

Then the metric **must be rejected immediately**.

**There are no exceptions.**

**Performance does not justify inclusion.**

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
