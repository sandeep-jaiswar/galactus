# Galactus — Retail vs Institutional Dynamics

## Purpose of This Document

This document defines how **retail and institutional participants interact** in Indian markets, and why their interaction produces repeatable capital pressure patterns.

Galactus does not judge participants by sophistication.  
It models them by **constraints, scale, and reaction function**.

All crowding, attention, and flow-based signals must align with this framework.

---

## Core Observation

Retail and institutional participants do not compete on equal footing.

They differ fundamentally in:
- Capital size
- Time horizon
- Risk tolerance
- Constraint severity
- Ability to absorb adverse movement

Markets move when these differences collide.

---

## Retail Capital — Characteristics

Retail capital is characterized by:

- Small average position size
- High leverage via derivatives
- Short time horizons
- High sensitivity to recent price movement
- Limited capacity to absorb drawdowns

Retail behavior is:
- Reactive, not initiatory
- Attention-driven
- Outcome-focused rather than process-focused

Retail capital rarely **forces** markets, but it frequently **amplifies** existing pressure.

---

## Institutional Capital — Characteristics

Institutional capital is characterized by:

- Large position sizes
- Mandated strategies and constraints
- Longer time horizons
- Risk controls over conviction
- Responsibility to manage liquidity impact

Institutional behavior is:
- Constraint-driven
- Process-oriented
- Often indifferent to short-term price noise

Institutions may act slowly, but when constrained, they act **decisively**.

---

## Asymmetry of Influence

A key asymmetry:

- Retail dominates **trade count**
- Institutions dominate **capital impact**

Consequences:
- Price can move sharply with few institutional actions
- Retail often interprets institutional mechanics as “momentum”
- Liquidity is often thinner than it appears

Galactus explicitly normalizes signals by **capital-weighted impact**, not trade frequency.

---

## Initiation vs Amplification

Galactus distinguishes clearly:

- **Initiation**: When constrained capital must act
- **Amplification**: When reactive capital follows

Typically:
- Institutional or dealer-related capital initiates
- Retail capital amplifies late-stage movement

This sequencing is central to regime inference.

---

## Retail Crowding Effects

Retail behavior tends to cluster:
- Around round numbers
- Near recent highs/lows
- Around option strikes with asymmetric payoffs

Crowding creates:
- Fragile positioning
- Non-linear unwind risk
- Short-lived volatility bursts

Galactus uses retail signals only to measure **crowding intensity**, not direction.

---

## Institutional Absorption and Fading

Institutions often:
- Absorb retail-driven liquidity
- Fade overextended moves
- Rebalance gradually

However, when institutions themselves are constrained:
- They stop absorbing
- They become price insensitive
- Moves accelerate

This transition is critical and observable.

---

## Feedback Loops

Retail and institutional dynamics create feedback loops:

1. Forced institutional action moves price
2. Retail reacts to movement
3. Retail crowding increases volatility
4. Dealers hedge increased exposure
5. Further forced flows occur

Galactus is designed to detect **loop formation**, not just single-step effects.

---

## Why This Matters for Inference

Identical price movements can arise from:
- Retail chasing
- Institutional constraint resolution
- Dealer hedging
- Liquidity exhaustion

Only by modeling participant dynamics can Galactus distinguish between them.

---

## What Galactus Explicitly Avoids

Galactus does not:
- Assume retail is always wrong
- Assume institutions are always correct
- Assign moral or qualitative judgments

It models **mechanics, not intelligence**.

---

## Implications for Signal Design

Valid signals must:
- Identify which participant group is constrained
- Distinguish initiation from amplification
- Account for absorption capacity

Signals that only observe price or volume are insufficient.

---

## Final Statement

**Markets move fastest when those who cannot absorb are forced to react, and those who can absorb choose not to.**

Galactus exists to identify those moments.
