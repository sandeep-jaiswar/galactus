# Galactus — Event-Driven Architecture

## Purpose of This Document

This document defines the **event-driven architectural model** used by Project Galactus.

It explains:
- What constitutes an event
- Why Galactus is event-driven instead of tick-driven
- How time, ordering, and causality are handled
- How replayability is guaranteed

This document governs all decisions related to data flow and processing semantics.

---

## Core Principle

Galactus models **structural change**, not continuous motion.

Structural change occurs at **events**, not at every price tick.

---

## Why Event-Driven (Not Tick-Driven)

Tick-driven systems assume:
- Continuous price discovery
- Meaningful information in every micro-movement
- Latency as a competitive advantage

These assumptions do not hold for Galactus.

Galactus prioritizes:
- Capital constraints
- Forced flows
- Discrete market mechanics

These manifest at **specific moments**, not continuously.

---

## Definition: Event

In Galactus, an **event** is:

> A discrete, timestamped occurrence that materially alters the state of market constraints or capital behavior.

Events are not limited to price changes.

---

## Types of Events

### 1. Market Structure Events

Examples:
- Options expiry
- Index rebalancing
- Settlement cycles
- Regulatory changes

These events impose **hard constraints** on capital.

---

### 2. Positioning Events

Examples:
- Significant open interest changes
- Strike concentration shifts
- Gamma exposure transitions

These events alter **hedging requirements**.

---

### 3. Liquidity Events

Examples:
- Sudden volume exhaustion
- Delivery spikes
- Liquidity cliffs

These events change **absorption capacity**.

---

### 4. Information Events

Examples:
- Corporate disclosures
- Scheduled announcements

Galactus observes **capital reaction**, not narrative content.

---

## Event Time vs Processing Time

Galactus distinguishes strictly between:

- **Event Time**: When the market event actually occurred
- **Processing Time**: When the system processed the event

All inference is aligned to **event time**.

Processing delays must not alter inference results.

---

## Event Ordering Guarantees

Galactus enforces:

- Total ordering within event streams
- Explicit handling of late or out-of-order events
- Deterministic reprocessing behavior

Ordering ambiguity must be resolved explicitly, never implicitly.

---

## Idempotency and Reprocessing

All event handling must be:
- Idempotent
- Replay-safe
- Deterministic

Reprocessing the same event stream must:
- Produce identical inference outputs
- Not depend on system state outside the stream

---

## Windowing and Time Horizons

Galactus uses **explicit windows**, not implicit rolling assumptions.

Examples:
- Time-to-expiry windows
- Liquidity lookback windows
- Regime evaluation windows

All windows are:
- Explicitly defined
- Versioned
- Justified in documentation

---

## Why This Matters for Inference

Event-driven processing allows Galactus to:

- Attribute cause before effect
- Separate structural pressure from noise
- Replay and audit inference decisions
- Align backtests with live behavior

Tick-driven systems obscure causality.

---

## Failure Modes and Safeguards

Galactus explicitly handles:
- Missing events
- Delayed events
- Event bursts near expiries
- Data source outages

When event integrity is compromised:
- Inference confidence must degrade
- Silence is acceptable

---

## Implications for System Design

Because Galactus is event-driven:
- Storage must be append-only
- Replay must be first-class
- Stateful shortcuts are forbidden
- Time must be modeled, not assumed

---

## What Galactus Explicitly Avoids

Galactus does not:
- React to every tick
- Optimize for minimal latency
- Infer meaning from microstructure noise

---

## Final Statement

**Markets change state at events, not at every moment.**

Galactus is designed to observe those state changes clearly.
