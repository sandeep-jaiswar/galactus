# Galactus — Capital Behavior Model

## Purpose of This Document

This document defines the **capital behavior model** used by Galactus.

It formalizes:
- What constitutes “capital” in the market
- How capital behaves under constraints
- The difference between discretionary and forced action
- Why capital behavior is a more reliable primitive than price

All signals and inferences in Galactus must be explainable using this model.

---

## Core Premise

Markets are not driven by opinions.  
They are driven by **capital under constraints**.

Price is the visible outcome of:
- Capital that *chooses* to act
- Capital that is *forced* to act
- Capital that is *prevented* from acting

Galactus models these forces explicitly.

---

## Definition: Capital

In Galactus, **capital** refers to any pool of funds that can influence market prices through allocation, hedging, or liquidation.

Capital is characterized by:
- Size
- Mandate
- Constraints
- Time horizon
- Ability to delay action

Capital is **not** homogeneous.

---

## Categories of Capital

Galactus distinguishes between the following high-level capital types:

### 1. Discretionary Capital

Capital that:
- Can choose whether to act
- Can delay decisions
- Is guided by strategy, opinion, or mandate

Examples:
- Long-only institutional investors
- Proprietary trading desks
- Retail investors

Discretionary capital **responds to opportunity**, but is not obligated to act.

---

### 2. Forced Capital

Capital that:
- Must act regardless of opinion
- Is driven by structural or regulatory constraints
- Has limited or no discretion over timing

Examples:
- Options dealer hedging flows
- Margin calls
- Index rebalancing flows
- ETF creation/redemption
- Expiry-related settlements

Forced capital **acts because it has no alternative**.

---

### 3. Reactive Capital

Capital that:
- Acts in response to observed price or volatility
- Is rule-based rather than opinion-based
- Often amplifies existing moves

Examples:
- Risk-parity strategies
- Volatility targeting funds
- CTA-style trend followers

Reactive capital is discretionary in design but **mechanical in execution**.

---

## Capital Constraints

Capital behavior is governed by constraints.

Common constraints include:
- Time (expiry, settlement cycles)
- Risk limits
- Margin requirements
- Liquidity availability
- Regulatory or index rules

A **constraint** becomes meaningful when it:
- Forces action
- Prevents delay
- Overrides discretionary preference

Galactus focuses on identifying **when constraints become binding**.

---

## Forced Action vs Optional Action

A critical distinction in Galactus:

- **Optional action**: Capital may act if conditions are attractive.
- **Forced action**: Capital must act even if conditions are unattractive.

Forced action creates **predictable pressure**.

Galactus is optimized to detect these moments.

---

## Capital Pressure

**Capital Pressure** is defined as:

> The magnitude and direction of required capital action resulting from binding constraints within a given time window.

Capital pressure can be:
- Directional or neutral
- Concentrated or distributed
- Stable or transient

Capital pressure does **not** guarantee price movement, but it:
- Increases the probability of market impact
- Reduces the range of possible outcomes

---

## Interaction Between Capital Types

Markets are shaped by interaction effects:

- Forced capital often initiates movement
- Reactive capital amplifies movement
- Discretionary capital may fade or follow

Galactus does not assume any single capital type dominates at all times.

Instead, it infers **which capital type is currently in control**.

---

## Why Price Is Insufficient

Price alone cannot distinguish between:
- Forced buying vs enthusiastic buying
- Liquidity-driven moves vs conviction-driven moves
- Temporary pressure vs regime change

Identical price movements can arise from entirely different capital dynamics.

Galactus treats price as **evidence**, not as explanation.

---

## Implications for Signal Design

Under this model:

- Signals must map to identifiable capital behavior
- Indicators without capital interpretation are invalid
- Metrics must reference constraints, not patterns

A valid Galactus signal answers:
- *Which capital is constrained?*
- *Why must it act now?*
- *What prevents it from delaying?*

---

## Limits of the Model

This model does **not** claim:
- Complete market predictability
- Exhaustive identification of all capital flows
- Perfect attribution of causality

It is a **useful abstraction**, not a complete theory.

Galactus prefers a correct partial model to an incorrect complete one.

---

## Revision Policy

This model may be revised only if:
- Structural market mechanics change materially
- New evidence invalidates core assumptions

All revisions must be explicit and logged.

---

## Final Statement

**Markets move when capital loses the ability to wait.**

Galactus exists to identify those moments.
