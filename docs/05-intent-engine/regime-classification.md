# Galactus — Regime Classification

## Purpose of This Document

This document defines the **regime classification framework** used by the Galactus Intent Engine.

It exists to:
- Provide contextual grounding for all signals
- Prevent misinterpretation of identical pressure under different conditions
- Enable graceful adaptation to changing market structure

Signals do not exist in isolation.  
They exist within regimes.

---

## Definition: Regime

In Galactus, a **regime** is defined as:

> A persistent market state characterized by a distinct configuration of liquidity, volatility, derivatives dominance, and capital constraints.

Regimes describe *how the market behaves*, not *what price does*.

---

## Why Regimes Matter

The same capital pressure can produce:
- No price impact in one regime
- Violent resolution in another

Without regime awareness:
- Signals appear inconsistent
- Confidence becomes misleading
- Inference degrades into pattern-matching

Regime classification conditions all interpretation.

---

## Core Regime Dimensions

Galactus classifies regimes along several orthogonal dimensions:

---

## 1. Liquidity Regime

### States (Indicative)

- High liquidity
- Normal liquidity
- Fragile liquidity
- Illiquid

### Characteristics

- Depth and absorption capacity
- Sensitivity to flows
- Slippage risk

Liquidity regimes determine **impact sensitivity**.

---

## 2. Volatility Regime

### States (Indicative)

- Compressed volatility
- Normal volatility
- Elevated volatility
- Dislocated volatility

### Characteristics

- Stability of price movement
- Sensitivity to shocks
- Option pricing behavior

Volatility regimes condition **pressure amplification**.

---

## 3. Derivatives Dominance Regime

### States (Indicative)

- Derivatives-dominant
- Mixed
- Cash-dominant

### Characteristics

- Influence of options positioning
- Gamma effects
- Hedging-driven behavior

This regime determines **where constraints originate**.

---

## 4. Time Constraint Regime

### States (Indicative)

- Far from deadlines
- Approaching deadlines
- Immediate deadlines (expiry / settlement)

### Characteristics

- Optionality decay
- Urgency of action
- Forced behavior likelihood

Time regimes control **urgency**.

---

## 5. Participation Regime

### States (Indicative)

- Retail-dominant
- Balanced
- Institutional-dominant

### Characteristics

- Crowding risk
- Amplification likelihood
- Absorption behavior

Participation regimes influence **feedback loops**.

---

## Regime Detection (High-Level)

Regimes are detected via:
- Persistent metric configurations
- Structural indicators (not price patterns)
- Event-driven transitions

Regimes are:
- Inferred probabilistically
- Sticky by design
- Resistant to frequent switching

---

## Regime Transitions

Regime changes:
- Are infrequent
- Often event-driven
- May be abrupt or gradual

Transitions must be:
- Explicitly detected
- Logged
- Reflected in confidence

Silent regime shifts are a critical failure mode.

---

## Interaction with Signals

Signals are interpreted **conditional on regime**.

Examples:
- Strong pressure in high-liquidity regime → low impact expectation
- Moderate pressure in fragile-liquidity regime → elevated risk

Signals without regime context are incomplete.

---

## Regime Confidence and Uncertainty

Regime classification includes:
- Confidence scores
- Ambiguity flags
- Transitional states

Uncertain regimes degrade downstream inference confidence.

---

## Failure Modes

Regime classification may fail when:
- Market structure changes rapidly
- Data quality degrades
- Novel instruments dominate flows

Failures must reduce confidence, not force classification.

---

## Evolution of Regimes

Regime definitions may evolve as:
- Market structure changes
- New instruments emerge
- Participation shifts

All changes must be documented.

---

## Final Statement

**Signals explain pressure.  
Regimes explain consequences.**

Galactus needs both to think clearly.
