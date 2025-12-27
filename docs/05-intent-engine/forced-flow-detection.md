# Galactus — Forced Flow Detection

## Purpose of This Document

This document defines **forced flow** within Project Galactus and describes how the Intent Engine detects it.

It exists to:
- Distinguish mandatory capital action from discretionary behavior
- Prevent misclassification of reactive moves as intent
- Enable reliable identification of structural pressure

Forced flow is the most reliable source of inference in Galactus.

---

## Definition: Forced Flow

In Galactus, **forced flow** is defined as:

> Capital movement that must occur due to binding constraints, regardless of price, opinion, or preference.

Forced flow is non-negotiable capital behavior.

---

## Forced vs Discretionary Flow

### Forced Flow

- Triggered by structural constraints
- Time-bound and deadline-driven
- Often price-insensitive
- Mechanically executed

Examples:
- Dealer hedging near option expiry
- Index rebalancing trades
- Margin call liquidations
- Settlement-driven adjustments

---

### Discretionary Flow

- Optional and opinion-driven
- Can be delayed or avoided
- Sensitive to price and liquidity
- Strategy-dependent

Examples:
- Tactical trades
- Portfolio rebalancing without deadlines
- Retail speculation

Galactus prioritizes forced flow inference.

---

## Characteristics of Forced Flow

Forced flows exhibit several identifiable characteristics:

1. **Time Compression**
   - Action accelerates as deadlines approach

2. **Price Insensitivity**
   - Execution occurs despite unfavorable prices

3. **Predictable Windows**
   - Often tied to known calendar events

4. **Mechanical Execution**
   - Rule-based rather than discretionary

---

## Primary Sources of Forced Flow

Galactus explicitly models the following forced flow sources:

### 1. Derivatives Hedging

- Gamma-driven dealer hedging
- Vega and delta adjustments
- Expiry-driven unwinds

### 2. Index and Passive Flows

- Index inclusions/exclusions
- Weight rebalances
- ETF creation/redemption

### 3. Margin and Risk Constraints

- Margin calls
- Risk limit breaches
- Volatility-triggered de-risking

### 4. Settlement and Regulatory Events

- Settlement deadlines
- Regulatory compliance actions

---

## Detection Methodology (High-Level)

Forced flow detection relies on **confluence**, not single indicators.

Key detection elements include:

- Presence of binding constraints
- Size of capital subject to constraint
- Proximity to deadline
- Lack of discretionary delay behavior
- Alignment across multiple signals

No single metric is sufficient.

---

## Differentiating Forced Flow from Noise

Galactus avoids misclassification by:

- Normalizing flows by liquidity
- Conditioning inference on time urgency
- Rejecting short-lived, unstructured bursts
- Requiring structural justification

Sudden volume without constraint context is not forced flow.

---

## Directionality of Forced Flow

Forced flow may be:
- Directional (net buying or selling)
- Neutral (pinning, stabilizing)
- Volatility-enhancing

Direction is inferred from **mechanism**, not price movement.

---

## Confidence Assessment

Forced flow detection outputs include:
- Confidence levels
- Assumption lists
- Known ambiguity flags

Ambiguous cases degrade confidence rather than forcing classification.

---

## Failure Modes

Forced flow detection may fail when:
- Constraints are misidentified
- Capital size is underestimated
- Liquidity collapses unexpectedly
- Exogenous shocks override structure

Failures must be explicit.

---

## Interaction with Other Engine Components

Forced flow detection informs:
- Capital pressure computation
- Regime classification
- Confidence evaluation

It does not operate in isolation.

---

## What Galactus Explicitly Avoids

Galactus does not:
- Infer forced flow from price momentum alone
- Label all large moves as forced
- Assume derivatives always dominate

Forced flow is situational.

---

## Final Statement

**When capital is forced, markets reveal structure.**

Galactus exists to detect those moments clearly.
