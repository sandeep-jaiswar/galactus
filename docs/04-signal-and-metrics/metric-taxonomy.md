# Galactus — Metric Taxonomy

## Purpose of This Document

This document defines the **taxonomy of metrics** permitted in Project Galactus.

It exists to:
- Classify valid metric types
- Prevent accidental reliance on indicators
- Enforce consistency across signals
- Make rejection of invalid metrics explicit

Metrics are the building blocks of signals.  
Not all metrics deserve to exist.

---

## Metric vs Signal

- A **metric** is a quantitative measurement
- A **signal** is an interpreted metric grounded in capital behavior

All signals are composed of one or more metrics, but not all metrics qualify for signal construction.

---

## Allowed Metric Categories

**Only the following five (5) categories are permitted.**

Any metric that does not fit into one of these categories **must be rejected**, regardless of performance or appeal.

---

## 1. Constraint Metrics

### Definition

Metrics that quantify **binding constraints** on capital.

### Examples

- Time-to-expiry
- Margin utilization ratios
- Settlement proximity
- Index rebalance windows

### Purpose

To identify **when capital loses optionality**.

---

## 2. Pressure Metrics

### Definition

Metrics that measure **required or incentivized capital action**.

### Examples

- Gamma-induced hedging pressure
- Forced flow ratios
- Net open interest imbalance
- Roll pressure near expiry

### Purpose

To estimate **magnitude and direction of pressure**.

---

## 3. Imbalance Metrics

### Definition

Metrics that capture **asymmetry between opposing forces**.

### Examples

- Call vs put OI imbalance
- Delivery vs speculation ratios
- Long vs short exposure imbalance

### Purpose

To reveal **structural asymmetry**, not sentiment.

---

## 4. Liquidity Metrics

### Definition

Metrics that describe **absorption capacity**.

### Examples

- Average daily traded value
- Free float-adjusted volume
- Delivery concentration
- Depth proxies

### Purpose

To contextualize pressure impact.

---

## 5. Stability Metrics

### Definition

Metrics that assess **persistence or fragility** of observed conditions.

### Examples

- Duration of pressure persistence
- Variance of positioning over time
- Rate of pressure decay

### Purpose

To distinguish transient noise from structural conditions.

---

## Category Enforcement

**These five categories are exhaustive and exclusive.**

- **Constraint**: Binding constraints on capital
- **Pressure**: Required or incentivized capital action
- **Imbalance**: Asymmetry between opposing forces
- **Liquidity**: Absorption capacity
- **Stability**: Persistence or fragility of conditions

Any proposed metric that:
- Does not clearly fit into exactly one of these five categories
- Requires creation of a new category
- Blends multiple categories without clear decomposition

Must be **rejected immediately**, without exception.

---

## Forbidden Metric Categories

The following categories are explicitly forbidden.

---

## 1. Pure Price Indicators

Examples:
- RSI
- MACD
- Moving averages
- Bollinger Bands

**Reason**  
They describe outcomes, not causes.

---

## 2. Pattern Recognition Metrics

Examples:
- Candlestick patterns
- Chart formations
- Trendline breaks

**Reason**  
They lack causal grounding.

---

## 3. Profit-Oriented Metrics

Examples:
- Expected return
- Sharpe ratio (as signal input)
- Win rate

**Reason**  
They optimize outcomes, not understanding.

---

## 4. Black-Box Feature Embeddings

Examples:
- Latent neural embeddings
- Uninterpretable composite scores

**Reason**  
They violate explainability requirements.

---

## Metric Composition Rules

- Metrics may be combined only if each component is valid
- Weighting must be explicit
- Normalization must be documented

Opaque aggregation is forbidden.

---

## Metric Normalization Requirements

All metrics must declare:
- Units
- Normalization basis
- Time horizon

Metrics without normalization context are invalid.

---

## Metric Lifecycle

Metrics follow the same lifecycle as signals:
- Hypothesized
- Validated
- Promoted
- Monitored
- Deprecated

Unused metrics must be removed.

---

## Enforcement

Any metric that:
- Falls outside allowed categories
- Cannot be explained via capital behavior

Must be rejected, regardless of performance.

---

## Final Statement

**Metrics exist to explain pressure, not to predict outcomes.**

Galactus measures causes, not charts.
