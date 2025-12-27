# Galactus — Capital Pressure Model

## Purpose of This Document

This document formalizes the **Capital Pressure Model**, the core quantitative framework used by Galactus to measure and infer the magnitude, direction, and urgency of forced capital flows.

It defines:
- The formal structure of capital pressure
- The four components that determine pressure magnitude
- How these components interact and compound
- Rules for pressure aggregation and decay
- The relationship between pressure and market impact

This is the **mathematical and conceptual foundation** for all capital pressure inference in Galactus.

---

## What Is Capital Pressure?

**Capital pressure** is the magnitude and directionality of required capital action that results from binding constraints within a specific time window.

Pressure arises when capital:
- Must act (constraint is binding)
- Cannot delay (time urgency exists)
- Faces limited alternatives (absorption capacity is constrained)

Capital pressure is **not** the same as:
- Price momentum (which may occur without pressure)
- Trading volume (which may be discretionary)
- Sentiment or opinion (which lacks binding force)

---

## Formal Definition

Capital pressure is conceptually determined by four interacting components:

### 1. Constraint Severity

**Constraint severity** measures how binding a constraint is on capital behavior.

Severity is high when:
- The constraint cannot be avoided or delayed
- Violating the constraint incurs significant cost or risk
- The capital has no alternative actions available
- The constraint is structural (rules-based) rather than discretionary

Examples:
- **High severity**: Options expiry (must settle or roll)
- **High severity**: Margin call (must post collateral or liquidate)
- **Medium severity**: Index rebalancing (must track index, but timing flexible)
- **Low severity**: Discretionary portfolio adjustment (can be deferred)

Constraint severity determines **whether** capital must act.

---

### 2. Capital Size

**Capital size** measures the magnitude of capital affected by the constraint.

Size is relevant because:
- Larger flows create greater market impact
- Large capital cannot move without revealing intent
- Size affects the feasibility of absorption by the market

Capital size must account for:
- Gross notional exposure
- Net directional exposure after hedging
- Leverage or implicit multipliers
- Hidden or off-balance-sheet positions

Examples:
- Large index fund rebalancing (billions in notional)
- Options dealer gamma hedging (directional flow amplification)
- Retail liquidation cascade (individually small, collectively large)

Capital size determines **how much** pressure exists.

---

### 3. Time Urgency

**Time urgency** measures the time window within which capital must act.

Urgency is high when:
- The deadline is imminent and non-negotiable
- Delay increases cost or risk materially
- The constraint tightens over time
- Multiple deadlines converge simultaneously

Time urgency affects:
- The speed of required execution
- The market's ability to absorb flow smoothly
- The likelihood of price impact and slippage
- The predictability of action timing

Examples:
- **High urgency**: Same-day expiry settlement
- **High urgency**: Intraday margin call
- **Medium urgency**: End-of-week rebalancing
- **Low urgency**: Quarterly repositioning with flexible timing

Time urgency determines **when** capital must act.

---

### 4. Absorption Context

**Absorption context** measures the market's capacity to absorb capital flow without material price impact.

Absorption capacity is determined by:
- Liquidity depth (order book size, market maker presence)
- Volatility regime (high volatility reduces absorption)
- Competing flows (offsetting pressure increases absorption)
- Market structure (auction vs continuous, circuit breakers, etc.)

Absorption capacity is **not static**:
- It degrades during stress
- It varies by time of day and market conditions
- It may vanish suddenly (liquidity withdrawal)

Examples:
- **High absorption**: Deep, liquid market with two-sided flow
- **Medium absorption**: Normal conditions, moderate depth
- **Low absorption**: Illiquid market, one-sided flow, or high volatility
- **Near-zero absorption**: Market panic, circuit breakers, or liquidity vacuum

Absorption context determines the **likely market impact** of pressure.

---

## Conceptual Formulation

Capital pressure can be expressed conceptually as:

```
Capital Pressure ∝ (Constraint Severity × Capital Size × Time Urgency) / Absorption Context
```

This is a **conceptual model**, not a literal formula.

All implementations must preserve this relationship.

---

## Directionality of Pressure

Capital pressure may be:
- Directional (net buying or selling pressure)
- Neutral (pinning or stabilizing pressure)
- Volatility-expanding (amplification risk)

Directionality is derived from **mechanism**, not price trend.

---

## Pressure Decay and Resolution

Capital pressure is not static.

It may:
- Increase as constraints tighten
- Decay as positions are unwound
- Resolve abruptly at deadlines
- Transform into a different pressure type

Decay and resolution must be modeled explicitly.

---

## Aggregation Rules

When multiple pressures exist:

- Pressures may compound
- Pressures may offset
- Pressures may operate on different time horizons

Aggregation must preserve:
- Component visibility
- Interpretability
- Determinism

Blind summation is forbidden.

---

## Pressure Stability

Pressure stability refers to:
- Persistence over time
- Sensitivity to small changes
- Vulnerability to regime shifts

Unstable pressure must reduce confidence.

---

## Failure Modes

The capital pressure model may fail when:
- Constraints are misidentified
- Capital size is misestimated
- Liquidity vanishes unexpectedly
- Exogenous shocks override structure

Failure must degrade inference, not be hidden.

---

## Relationship to Signals

Signals in Galactus are:
- Measurements or transformations of capital pressure
- Contextual views of pressure dynamics

No signal exists without a pressure interpretation.

---

## Limits of the Model

This model does not claim:
- Perfect causality
- Complete observability
- Guaranteed outcomes

It is a **useful abstraction**, not a universal truth.

---

## Revision Policy

Changes to this model require:
- Evidence of structural invalidation
- Documentation updates
- Review against vision and design principles

---

## Final Statement

**Markets move when capital pressure becomes unavoidable.**

Galactus exists to measure that inevitability.
