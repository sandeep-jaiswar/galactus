# Galactus — Data Sources

## Purpose of This Document

This document defines the **approved data sources** for Project Galactus.

It exists to:
- Establish trust boundaries
- Enforce compliance and reproducibility
- Prevent hidden data dependencies
- Ensure all inference can be audited and replayed

If a data source is not explicitly listed here, it is **forbidden**.

---

## Core Data Principles

All data used by Galactus must be:

1. **Publicly available**
2. **Legally accessible**
3. **Reproducible historically**
4. **Auditable**
5. **Non-personalized**

Galactus does not depend on privileged, paid, or insider data.

---

## Primary Market Data Sources (Approved)

### 1. Cash Market Data

**Description**  
Daily and intraday equity market data from recognized Indian exchanges.

**Examples**
- NSE / BSE bhavcopy
- Equity trade and delivery statistics
- Corporate action-adjusted price data

**Usage**
- Liquidity assessment
- Delivery vs speculation analysis
- Capital absorption estimation

---

### 2. Derivatives Market Data

**Description**  
Public derivatives positioning and pricing data.

**Examples**
- Options chain snapshots
- Open interest (OI) and OI change
- Futures open interest and volume
- Expiry calendars

**Usage**
- Gamma and positioning inference
- Time-to-expiry constraints
- Forced hedging pressure estimation

---

### 3. Corporate Disclosures

**Description**  
Mandatory public disclosures released through official exchange channels.

**Examples**
- Financial results
- Pledge disclosures
- Bulk and block deals
- Corporate actions

**Usage**
- Event identification
- Capital response observation (not narrative interpretation)

---

### 4. Index Methodology and Rebalance Data

**Description**  
Publicly documented index construction and rebalance information.

**Examples**
- NIFTY / SENSEX methodology documents
- Rebalance calendars
- Weight changes

**Usage**
- Forced passive flows
- Index-related capital constraints

---

### 5. Market Calendars

**Description**  
Time-based structural constraints.

**Examples**
- Trading holidays
- Settlement cycles
- Weekly and monthly expiry schedules

**Usage**
- Time normalization
- Constraint modeling

---

## Secondary Data Sources (Conditionally Approved)

These sources may be used **only as proxies**, never as causal truth.

### 6. Public Attention Signals

**Examples**
- Google Trends (symbol-level)
- Public social media volume counts
- Search frequency metrics

**Usage**
- Retail crowding estimation
- Attention-to-liquidity mismatch detection

**Restrictions**
- No sentiment scoring
- No narrative extraction
- No directional inference

---

## Explicitly Forbidden Data Sources

Galactus will **never** use:

- Insider or non-public information
- Broker order books or proprietary flow data
- Paid alternative datasets with restricted redistribution
- User-level or account-level data
- Scraped private communities or closed groups
- News sentiment labels as ground truth
- Analyst recommendations or price targets

Any dependency on these sources invalidates inference.

---

## Data Granularity Rules

- Prefer **event-aligned snapshots** over raw ticks
- Intraday granularity is allowed only when:
  - It captures structural change
  - It improves inference stability
- Granularity must be justified, not assumed

---

## Data Versioning and Lineage

All data must be:
- Versioned
- Timestamped
- Source-attributed

Derived datasets must reference:
- Source datasets
- Transformation logic
- Schema version

No silent backfills are permitted.

---

## Data Quality Expectations

Galactus expects:
- Missing data to be explicit
- Delays to be tolerated
- Noise to be acknowledged

Data is evidence, not truth.

---

## Evolution of Data Sources

New data sources may be added only if:
- They are public and reproducible
- They do not violate non-goals
- Their impact is documented

All additions require documentation updates.

---

## Final Statement

**Galactus is constrained by what it is allowed to know.**

Those constraints are a strength, not a weakness.
