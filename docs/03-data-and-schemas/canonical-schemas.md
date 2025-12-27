# Galactus — Canonical Schemas

## Purpose of This Document

This document defines the **canonical schemas** used throughout Project Galactus.

It exists to:
- Ensure consistency across components
- Guarantee deterministic inference
- Enable replayability and auditability
- Prevent schema drift and implicit assumptions

All data exchanged between components must conform to these schemas.

---

## Core Schema Principles

All canonical schemas must be:

1. **Explicit**
   - No implicit fields or inferred meaning
2. **Stable**
   - Changes are versioned, never silent
3. **Minimal**
   - Only fields required for inference
4. **Composable**
   - Schemas can be combined without ambiguity
5. **Time-aware**
   - Event time is always explicit

---

## Canonical Event Schema

All market events conform to the following high-level structure:

Event {
    event_id: string
    event_type: enum
    event_time: timestamp
    source: string
    instruments: list<string>
    payload: object
    completeness: enum
    schema_version: string
}


### Field Definitions

- `event_id`  
  Globally unique identifier for the event.

- `event_type`  
  One of the defined event taxonomy categories.

- `event_time`  
  Timestamp representing when the event occurred in the market.

- `source`  
  Origin of the data (exchange, disclosure system, index provider).

- `instruments`  
  Affected symbols or identifiers.

- `payload`  
  Event-specific data, defined by subtype schemas.

- `completeness`  
  Indicator of data completeness (complete, partial, delayed).

- `schema_version`  
  Explicit schema version identifier.

---

## Canonical Positioning Payload (Example)

For derivatives positioning events:

PositioningPayload {
    instrument: string
    expiry: date
    strike: number
    option_type: enum
    open_interest: number
    oi_change: number
    volume: number
}


All numeric fields must have documented units and normalization rules.

---

## Canonical Liquidity Payload (Example)

LiquidityPayload {
    instrument: string
    traded_volume: number
    delivery_volume: number
    delivery_ratio: number
    avg_daily_value: number
}


Derived fields must reference their computation source.

---

## Canonical Intent Output Schema

Outputs from the Intent Engine follow:

IntentVector {
    instrument: string
    event_time: timestamp
    capital_pressure: number
    pressure_direction: enum
    regime_state: enum
    confidence: number
    assumptions: list<string>
    model_version: string
}


Intent vectors are **descriptive**, not actionable.

---

## Schema Versioning Rules

- All schemas include an explicit `schema_version`
- Breaking changes require:
  - New version
  - Migration documentation
  - Parallel support where necessary

- Backward compatibility is preferred but not assumed

No schema change is allowed without documentation updates.

---

## Nullability and Missing Data

- Missing fields must be explicit
- Nulls must be intentional, not default
- Default values are forbidden unless documented

Missing data must degrade confidence, not inference silently.

---

## Numeric Precision and Types

- Numeric precision must be explicit
- Floating-point usage must be justified
- Rounding rules must be documented

Silent type coercion is forbidden.

---

## Time Semantics

All schemas must:
- Use explicit timezones
- Distinguish event time from ingestion time
- Avoid implicit ordering assumptions

Time ambiguity invalidates inference.

---

## Schema Governance

Schemas are governed by:
- Documentation
- Version control
- Review against design principles

Schema ownership is collective, not individual.

---

## Final Statement

**Canonical schemas are the language Galactus uses to think.**

Ambiguous data produces ambiguous inference.
