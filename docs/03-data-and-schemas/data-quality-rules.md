# Galactus — Data Quality Rules

## Purpose of This Document

This document defines the **data quality standards** for Project Galactus.

It exists to:
- Prevent silent data corruption
- Make uncertainty explicit
- Protect inference integrity
- Ensure honest degradation under stress

Galactus does not require perfect data, but it requires **honest data**.

---

## Core Belief

Imperfect data is unavoidable.  
Dishonest inference is not.

Galactus prefers:
- Explicit uncertainty
- Degraded confidence
- Silence when necessary

Over false precision.

---

## Data Quality Dimensions

Galactus evaluates data quality across the following dimensions:

1. Completeness  
2. Timeliness  
3. Consistency  
4. Accuracy  
5. Coverage  

Each dimension affects inference confidence.

---

## 1. Completeness

### Definition

Whether all expected fields and records are present.

### Rules

- Missing fields must be explicit
- Partial datasets must be flagged
- Incomplete events degrade confidence

Silent assumptions about missing data are forbidden.

---

## 2. Timeliness

### Definition

Whether data arrives within acceptable temporal bounds.

### Rules

- Event time is authoritative
- Delayed data must be flagged
- Inference may be recomputed upon late arrival

Processing delays must not alter inference logic.

---

## 3. Consistency

### Definition

Whether data aligns across sources and time.

### Rules

- Schema consistency is mandatory
- Sudden structural breaks must be flagged
- Conflicting data must degrade confidence

Consistency violations are inference-relevant events.

---

## 4. Accuracy

### Definition

Whether data reflects reality within known limitations.

### Rules

- Public data is assumed accurate unless contradicted
- Corrections must be treated as new events
- No retroactive mutation of historical data

Accuracy uncertainty must be surfaced, not smoothed.

---

## 5. Coverage

### Definition

Whether data adequately represents the universe being analyzed.

### Rules

- Sparse coverage must be flagged
- Low-liquidity instruments require stricter thresholds
- Inference must scale with coverage quality

Coverage gaps limit inference scope.

---

## Quality Indicators

All events and derived outputs should include:

- Completeness indicator
- Confidence score
- Assumption annotations

Quality metadata is first-class data.

---

## Degradation Rules

When data quality degrades:

- Confidence must degrade proportionally
- Inference scope may narrow
- Silence is permitted and encouraged

Under no circumstances should degraded data produce confident inference.

---

## Hard Stop Conditions

Inference must halt or be suppressed when:

- Core data sources are unavailable
- Schema integrity is violated
- Event ordering cannot be resolved
- Data corruption is detected

Proceeding in these conditions is a system failure.

---

## Research vs Production Tolerance

- Research may tolerate lower quality data
- Production inference has stricter thresholds
- Threshold differences must be documented

Research shortcuts must never leak into production.

---

## Monitoring and Alerting

Data quality issues should be:
- Logged
- Monitored
- Reviewed post-incident

Quality failures are learning opportunities, not nuisances.

---

## Final Statement

**Galactus is only as confident as its data deserves.**

When the data lies, Galactus stays silent.
