// Galactus Data Types and Schemas
//
// This module implements the canonical schemas defined in:
// docs/03-data-and-schemas/canonical-schemas.md
//
// All data structures are designed for:
// - Deterministic processing
// - Schema versioning
// - Auditability and replayability

use chrono::{DateTime, Utc};
use std::fmt;

/// Globally unique identifier for events
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventId(pub String);

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Schema version identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaVersion(pub String);

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Data source identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataSource(pub String);

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Financial instrument identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument(pub String);

impl fmt::Display for Instrument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Event type classification based on taxonomy
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    MarketStructure,
    Positioning,
    Liquidity,
    Information,
    System,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::MarketStructure => write!(f, "MarketStructure"),
            EventType::Positioning => write!(f, "Positioning"),
            EventType::Liquidity => write!(f, "Liquidity"),
            EventType::Information => write!(f, "Information"),
            EventType::System => write!(f, "System"),
        }
    }
}

/// Data completeness indicator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Completeness {
    Complete,
    Partial,
    Delayed,
}

impl fmt::Display for Completeness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Completeness::Complete => write!(f, "Complete"),
            Completeness::Partial => write!(f, "Partial"),
            Completeness::Delayed => write!(f, "Delayed"),
        }
    }
}

/// Canonical event structure - all market events conform to this
#[derive(Debug, Clone, PartialEq)]
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

impl CanonicalEvent {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        event_id: EventId,
        event_type: EventType,
        event_time: DateTime<Utc>,
        source: DataSource,
        instruments: Vec<Instrument>,
        payload: EventPayload,
        completeness: Completeness,
        schema_version: SchemaVersion,
    ) -> Self {
        Self {
            event_id,
            event_type,
            event_time,
            source,
            instruments,
            payload,
            completeness,
            schema_version,
        }
    }
}

/// Event payload variants based on event type
#[derive(Debug, Clone, PartialEq)]
pub enum EventPayload {
    MarketStructure(MarketStructurePayload),
    Positioning(PositioningPayload),
    Liquidity(LiquidityPayload),
    Information(InformationPayload),
    System(SystemPayload),
}

/// Market structure event payload
#[derive(Debug, Clone, PartialEq)]
pub struct MarketStructurePayload {
    pub instrument: Instrument,
    pub event_subtype: MarketStructureSubtype,
    pub effective_time: DateTime<Utc>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MarketStructureSubtype {
    OptionsExpiry,
    FuturesSettlement,
    IndexRebalance,
    TradingHalt,
    RegulatoryChange,
}

/// Positioning event payload for derivatives
#[derive(Debug, Clone, PartialEq)]
pub struct PositioningPayload {
    pub instrument: Instrument,
    pub expiry: chrono::NaiveDate,
    pub strike: Option<f64>,
    pub option_type: Option<OptionType>,
    pub open_interest: u64,
    pub oi_change: i64,
    pub volume: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OptionType {
    Call,
    Put,
}

/// Liquidity event payload
#[derive(Debug, Clone, PartialEq)]
pub struct LiquidityPayload {
    pub instrument: Instrument,
    pub traded_volume: u64,
    pub delivery_volume: u64,
    pub delivery_ratio: f64,
    pub avg_daily_value: f64,
}

/// Information event payload
#[derive(Debug, Clone, PartialEq)]
pub struct InformationPayload {
    pub instrument: Instrument,
    pub disclosure_type: InformationSubtype,
    pub announced_time: DateTime<Utc>,
    pub effective_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InformationSubtype {
    FinancialResults,
    PledgeDisclosure,
    BulkDeal,
    CorporateAction,
}

/// System event payload
#[derive(Debug, Clone, PartialEq)]
pub struct SystemPayload {
    pub system_event_type: SystemSubtype,
    pub description: String,
    pub affected_instruments: Vec<Instrument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemSubtype {
    DataFeedInterruption,
    ProcessingDelay,
    QualityCheckFailure,
}

/// Constraint representation for market structure events
#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub severity: ConstraintSeverity,
    pub description: String,
    pub affected_participants: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstraintType {
    TimeDeadline,
    PositionLimit,
    CapitalRequirement,
    TradingRestriction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstraintSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Intent vector output schema
#[derive(Debug, Clone, PartialEq)]
pub struct IntentVector {
    pub instrument: Instrument,
    pub event_time: DateTime<Utc>,
    pub capital_pressure: f64,
    pub pressure_direction: PressureDirection,
    pub regime_state: crate::regime::RegimeState,
    pub confidence: crate::confidence::OverallConfidence,
    pub assumptions: Vec<String>,
    pub model_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PressureDirection {
    Buying,
    Selling,
    Neutral,
}

impl fmt::Display for PressureDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PressureDirection::Buying => write!(f, "Buying"),
            PressureDirection::Selling => write!(f, "Selling"),
            PressureDirection::Neutral => write!(f, "Neutral"),
        }
    }
}

/// Data quality metadata
#[derive(Debug, Clone, PartialEq)]
pub struct DataQuality {
    pub fields_present: usize,
    pub fields_required: usize,
    pub delay_seconds: f64,
    pub acceptable_delay_threshold: f64,
    pub contradictions_detected: usize,
    pub total_cross_checks: usize,
}

impl DataQuality {
    pub fn completeness_ratio(&self) -> f64 {
        if self.fields_required == 0 {
            0.0
        } else {
            self.fields_present as f64 / self.fields_required as f64
        }
    }

    pub fn is_delayed(&self) -> bool {
        self.delay_seconds > self.acceptable_delay_threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_canonical_event_creation() {
        let event = CanonicalEvent::new(
            EventId("test-event-1".to_string()),
            EventType::Positioning,
            Utc::now(),
            DataSource("NSE_FO".to_string()),
            vec![Instrument("NIFTY".to_string())],
            EventPayload::Positioning(PositioningPayload {
                instrument: Instrument("NIFTY".to_string()),
                expiry: chrono::NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
                strike: Some(22000.0),
                option_type: Some(OptionType::Call),
                open_interest: 1000,
                oi_change: 100,
                volume: 500,
            }),
            Completeness::Complete,
            SchemaVersion("1.0".to_string()),
        );

        assert_eq!(event.event_id.0, "test-event-1");
        assert_eq!(event.event_type, EventType::Positioning);
        assert_eq!(event.source.0, "NSE_FO");
        assert_eq!(event.instruments.len(), 1);
        assert_eq!(event.instruments[0].0, "NIFTY");
        assert_eq!(event.completeness, Completeness::Complete);
        assert_eq!(event.schema_version.0, "1.0");
    }

    #[test]
    fn test_data_quality_assessment() {
        let quality = DataQuality {
            fields_present: 4,
            fields_required: 5,
            delay_seconds: 60.0,
            acceptable_delay_threshold: 300.0,
            contradictions_detected: 0,
            total_cross_checks: 5,
        };

        assert_eq!(quality.completeness_ratio(), 0.8);
        assert!(!quality.is_delayed());
        assert_eq!(quality.contradictions_detected, 0);
    }

    #[test]
    fn test_pressure_direction_display() {
        assert_eq!(format!("{}", PressureDirection::Buying), "Buying");
        assert_eq!(format!("{}", PressureDirection::Selling), "Selling");
        assert_eq!(format!("{}", PressureDirection::Neutral), "Neutral");
    }
}
