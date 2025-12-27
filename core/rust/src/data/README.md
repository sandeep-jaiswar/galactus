# Galactus Data & Schema Implementation

This document describes the data structures and schemas implemented for Galactus based on the canonical schemas defined in [`docs/03-data-and-schemas/canonical-schemas.md`](../../../../docs/03-data-and-schemas/canonical-schemas.md).

## Overview

The data layer provides:
- **Canonical event schemas** for all market data
- **Data ingestion and validation** pipeline
- **Schema versioning** and compatibility
- **Data quality assessment** and normalization

## Core Data Structures

### CanonicalEvent
The fundamental event structure that all market data conforms to:

```rust
pub struct CanonicalEvent {
    pub event_id: EventId,
    pub event_type: EventType,
    pub event_time: DateTime<Utc>,
    pub source: DataSource,
    pub instruments: Vec<Instrument>,
    pub payload: EventPayload,
    pub completeness: Completeness,
    pub schema_version: SchemaVersion,
}
```

### Event Types
Based on the event taxonomy in [`docs/03-data-and-schemas/event-taxonomy.md`](../../../../docs/03-data-and-schemas/event-taxonomy.md):

- `MarketStructure` - Hard constraints (expiries, rebalances)
- `Positioning` - Derivatives exposure changes
- `Liquidity` - Market absorption capacity
- `Information` - Public disclosures
- `System` - Technical events

### Payload Types
Each event type has a specific payload structure:

- **PositioningPayload** - Derivatives OI, strikes, expiry data
- **LiquidityPayload** - Volume, delivery ratios, market depth
- **MarketStructurePayload** - Constraints and deadlines
- **InformationPayload** - Disclosure metadata
- **SystemPayload** - Technical event details

## Data Ingestion Pipeline

### DataValidator
Validates incoming events against:
- Approved data sources (NSE, BSE, NSE_FO)
- Event type compatibility
- Schema version requirements
- Required field presence

### DataNormalizer
Standardizes data formats:
- Volume units (contracts, shares)
- Price precision (2 decimal places)
- Ratio bounds [0, 1]
- Source-specific conversions

### QualityAssessor
Evaluates data quality metrics:
- Field completeness ratios
- Data freshness (delay thresholds)
- Internal consistency checks
- Contradiction detection

## Approved Data Sources

Based on [`docs/03-data-and-schemas/data-sources.md`](../../../../docs/03-data-and-schemas/data-sources.md):

| Source | Data Types | Purpose |
|--------|------------|---------|
| NSE | Liquidity, Information | Cash market data |
| BSE | Liquidity, Information | Cash market data |
| NSE_FO | Positioning, MarketStructure | Derivatives data |

## Schema Versioning

All schemas include explicit version identifiers. Changes follow:
1. Version bump for breaking changes
2. Migration documentation
3. Parallel support during transitions
4. Explicit deprecation policies

## Data Quality Rules

Implemented from [`docs/03-data-and-schemas/data-quality-rules.md`](../../../../docs/03-data-and-schemas/data-quality-rules.md):

- **Completeness**: Minimum 80% field presence
- **Freshness**: Maximum 5-minute delay
- **Consistency**: Zero contradictions allowed
- **Traceability**: All data source-documented

## Usage Examples

### Creating a Positioning Event
```rust
use galactus_core::data::*;
use chrono::{Utc, NaiveDate};

let event = CanonicalEvent::new(
    EventId("nse-fo-oi-20251201".to_string()),
    EventType::Positioning,
    Utc::now(),
    DataSource("NSE_FO".to_string()),
    vec![Instrument("NIFTY".to_string())],
    EventPayload::Positioning(PositioningPayload {
        instrument: Instrument("NIFTY".to_string()),
        expiry: NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        strike: Some(22000.0),
        option_type: Some(OptionType::Call),
        open_interest: 10000,
        oi_change: 500,
        volume: 2500,
    }),
    Completeness::Complete,
    SchemaVersion("1.0".to_string()),
);
```

### Processing Incoming Data
```rust
use galactus_core::ingestion::DataIngestion;

let ingestion = DataIngestion::new();
match ingestion.process_event(event) {
    IngestionResult::Success(processed_event) => {
        // Event accepted and normalized
    },
    IngestionResult::Rejected(reason) => {
        // Event rejected with reason
    },
    IngestionResult::FailedValidation(errors) => {
        // Validation errors to fix
    }
}
```

## Testing

Comprehensive tests cover:
- Event creation and validation
- Data quality assessment
- Normalization logic
- Source approval checks
- Schema compatibility

Run tests with: `cargo test --package galactus-core`

## Future Extensions

- Additional data source integrations
- Real-time data quality monitoring
- Automated schema migration
- Enhanced normalization rules
- Performance optimizations for high-throughput ingestion