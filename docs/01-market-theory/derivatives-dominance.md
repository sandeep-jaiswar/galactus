# Galactus — Derivatives Dominance

## Purpose of This Document

This document explains **why derivatives markets dominate price behavior** in Indian equities and indices, and how this dominance creates observable, repeatable capital pressure.

It defines:
- The role of derivatives in price formation
- The behavior of derivative-linked capital
- What Galactus models explicitly
- What Galactus deliberately ignores

All derivative-related signals must align with this document.

---

## Core Observation

In Indian markets, **derivatives are not a side market**.

They are the **primary mechanism through which price constraints are expressed**.

Cash markets frequently reflect the resolution of pressures created in derivatives, not the other way around.

---

## Why Derivatives Dominate in India

Several structural factors contribute:

- Extremely high options participation
- Weekly index expiries
- Low cost of leverage
- Retail preference for convex payoffs
- Institutional use of derivatives for hedging and exposure management

As a result:
- A large portion of capital exposure exists off the cash market
- Hedging activity often exceeds directional intent
- Price is constrained by positioning rather than conviction

---

## Key Actors in the Derivatives Ecosystem

### 1. Options Buyers (Primarily Retail)

Characteristics:
- Seek convex outcomes
- Accept frequent small losses
- Cluster around round strikes and narratives

Their behavior:
- Creates concentrated open interest
- Does not directly move price
- Forces other participants to hedge

---

### 2. Options Sellers / Dealers

Characteristics:
- Typically neutral on direction
- Sensitive to gamma and vega exposure
- Manage risk dynamically

Their behavior:
- Creates **forced hedging flows**
- Responds mechanically to price movement
- Can pin or accelerate price near key strikes

Dealers are a primary source of **forced capital** in Galactus’ model.

---

### 3. Institutional Hedgers

Characteristics:
- Use derivatives to manage portfolio risk
- Often indifferent to short-term price
- Operate under mandate and risk constraints

Their activity:
- Introduces structural demand for options
- Amplifies hedging-related flows

---

## Gamma and Price Constraints

A central mechanism in derivatives dominance is **gamma exposure**.

When gamma is:
- **Positive**: price movement is dampened, mean-reverting behavior is more likely
- **Negative**: price movement is amplified, trending behavior is more likely

As expiry approaches:
- Gamma effects intensify
- Time constraints remove optionality
- Hedging becomes increasingly mechanical

Galactus treats gamma-related flows as **constraint-driven pressure**, not sentiment.

---

## Strike Concentration and Pinning

High open interest at specific strikes creates:
- Liquidity magnets
- Hedging equilibria
- Temporary price stability near expiry

Pinning is:
- Time-bound
- Mechanically driven
- Independent of fundamental opinion

Galactus models **the conditions under which pinning is likely**, not pinning as a rule.

---

## Weekly Expiries and Time Compression

Weekly expiries compress:
- Risk management windows
- Decision timelines
- Error tolerance

This compression:
- Increases forced behavior
- Reduces discretionary adjustment
- Makes capital behavior more observable

Galactus explicitly incorporates **time-to-expiry** as a core normalization factor.

---

## What Galactus Models Explicitly

Galactus models:
- Open interest concentration and change
- Gamma exposure distribution
- Time decay pressure
- Hedging-induced capital flows
- Interaction between derivatives and cash liquidity

These are modeled as **pressure gradients**, not directional bets.

---

## What Galactus Ignores Deliberately

Galactus does **not** model:
- Individual option trades
- Implied volatility as a standalone signal
- Option “sentiment”
- Retail payoff narratives

These are either noisy or derivative of deeper mechanics.

---

## Failure Modes in Derivatives Modeling

Galactus acknowledges that:
- Sudden news can overwhelm derivative constraints
- Liquidity can disappear unexpectedly
- Extreme positioning can unwind non-linearly

Derivative dominance is **situational**, not absolute.

---

## Implications for System Design

Because derivatives dominate:
- Options data is first-class input
- Expiry calendars are structural constraints
- Intraday behavior cannot be analyzed without derivative context

Any system ignoring derivatives is structurally blind in Indian markets.

---

## Final Statement

**In Indian markets, price often moves to satisfy derivatives constraints, not to express belief.**

Galactus exists to quantify those constraints.
