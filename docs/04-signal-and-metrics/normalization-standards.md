# Galactus — Normalization Standards

## Purpose of This Document

This document defines the **normalization standards** for all metrics used in Project Galactus.

It exists to:
- Prevent scale-driven misinterpretation
- Enable cross-instrument comparison
- Preserve interpretability across regimes
- Ensure metrics reflect pressure, not size

Metrics without normalization are not valid.

---

## Core Principle

Raw values are meaningless without context.

Normalization converts raw measurements into **comparable pressure signals**.

---

## Mandatory Normalization Dimensions

Every metric must be normalized across one or more of the following dimensions:

1. Capital Size  
2. Liquidity  
3. Time  
4. Volatility (when applicable)

The chosen dimensions must be documented.

---

## 1. Capital Size Normalization

### Purpose

To account for differences in:
- Market capitalization
- Free float
- Outstanding exposure

### Examples

- OI change as a percentage of free float
- Flow size relative to market cap

### Rules

- Absolute values are forbidden without normalization
- Free float is preferred over total shares

---

## 2. Liquidity Normalization

### Purpose

To measure pressure relative to **absorption capacity**.

### Examples

- Volume relative to average daily traded value
- Delivery volume relative to typical delivery

### Rules

- Liquidity regimes must be considered
- Illiquid instruments require stricter thresholds

---

## 3. Time Normalization

### Purpose

To reflect urgency and constraint decay.

### Examples

- Pressure per unit time
- Time-to-expiry adjusted metrics
- Rate of change per trading session

### Rules

- Time horizon must be explicit
- Time decay must be modeled, not assumed

---

## 4. Volatility Normalization (Conditional)

### Purpose

To contextualize movement under different regimes.

### Examples

- Pressure relative to implied or realized volatility
- Flow impact normalized by recent volatility

### Rules

- Volatility normalization must be justified
- Not all metrics require volatility adjustment

---

## Cross-Normalization Rules

- Multiple normalizations may be applied sequentially
- Order of normalization must be documented
- Composite normalization must remain interpretable

---

## Forbidden Normalization Practices

- Z-score normalization without context
- Percentile ranks across unrelated instruments
- Normalization that obscures units or meaning

---

## Stability and Robustness Checks

Normalized metrics must be:
- Stable across regimes
- Resistant to outliers
- Interpretable at extremes

Metrics that flip meaning under normalization are invalid.

---

## Documentation Requirements

Every metric must document:
- Raw input definition
- Normalization steps
- Resulting units
- Interpretation guidance

---

## Final Statement

**Normalization is not a cosmetic step.**

It is how Galactus turns data into meaning.
