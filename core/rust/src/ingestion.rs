// Galactus Data Ingestion Module
//
// This module implements data ingestion and normalization as defined in:
// docs/03-data-and-schemas/data-sources.md
// docs/03-data-and-schemas/data-quality-rules.md
//
// Responsibilities:
// - Validate incoming data against canonical schemas
// - Normalize data formats and units
// - Assess data quality and completeness
// - Reject invalid or low-quality data

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::data::*;
use crate::failure_analysis::{FailureCategory, FailureRecord};

/// Data ingestion result
#[derive(Debug, Clone, PartialEq)]
pub enum IngestionResult<T> {
    Success(T),
    Rejected(String),
    FailedValidation(Vec<String>),
}

/// Data source validator
pub struct DataValidator {
    approved_sources: HashMap<String, SourceMetadata>,
}

#[derive(Debug, Clone)]
struct SourceMetadata {
    name: String,
    data_types: Vec<EventType>,
    quality_thresholds: QualityThresholds,
}

#[derive(Debug, Clone)]
struct QualityThresholds {
    min_completeness: f64,
    max_delay_seconds: f64,
    max_contradictions: usize,
}

impl Default for QualityThresholds {
    fn default() -> Self {
        Self {
            min_completeness: 0.8,
            max_delay_seconds: 300.0, // 5 minutes
            max_contradictions: 0,
        }
    }
}

impl DataValidator {
    pub fn new() -> Self {
        let mut approved_sources = HashMap::new();

        // NSE/BSE cash market data
        approved_sources.insert(
            "NSE".to_string(),
            SourceMetadata {
                name: "National Stock Exchange".to_string(),
                data_types: vec![EventType::Liquidity, EventType::Information],
                quality_thresholds: QualityThresholds::default(),
            },
        );

        approved_sources.insert(
            "BSE".to_string(),
            SourceMetadata {
                name: "Bombay Stock Exchange".to_string(),
                data_types: vec![EventType::Liquidity, EventType::Information],
                quality_thresholds: QualityThresholds::default(),
            },
        );

        // Derivatives data
        approved_sources.insert(
            "NSE_FO".to_string(),
            SourceMetadata {
                name: "NSE Futures & Options".to_string(),
                data_types: vec![EventType::Positioning, EventType::MarketStructure],
                quality_thresholds: QualityThresholds::default(),
            },
        );

        Self { approved_sources }
    }

    pub fn validate_event(&self, event: &CanonicalEvent) -> IngestionResult<CanonicalEvent> {
        let mut validation_errors = Vec::new();

        // Check if source is approved
        if !self.approved_sources.contains_key(&event.source.0) {
            return IngestionResult::Rejected(format!("Unapproved data source: {}", event.source.0));
        }

        let source_meta = &self.approved_sources[&event.source.0];

        // Check if event type is allowed for this source
        if !source_meta.data_types.contains(&event.event_type) {
            validation_errors.push(format!(
                "Event type {:?} not allowed for source {}",
                event.event_type, event.source.0
            ));
        }

        // Validate schema version
        if event.schema_version.0.is_empty() {
            validation_errors.push("Schema version cannot be empty".to_string());
        }

        // Validate instruments
        if event.instruments.is_empty() {
            validation_errors.push("Event must specify at least one instrument".to_string());
        }

        // Validate payload matches event type
        if let Err(payload_error) = self.validate_payload(&event.event_type, &event.payload) {
            validation_errors.push(payload_error);
        }

        // Check data quality if we have quality metadata
        // Note: In real implementation, this would be computed from the event data

        if !validation_errors.is_empty() {
            return IngestionResult::FailedValidation(validation_errors);
        }

        IngestionResult::Success(event.clone())
    }

    fn validate_payload(&self, event_type: &EventType, payload: &EventPayload) -> Result<(), String> {
        match (event_type, payload) {
            (EventType::MarketStructure, EventPayload::MarketStructure(_)) => Ok(()),
            (EventType::Positioning, EventPayload::Positioning(_)) => Ok(()),
            (EventType::Liquidity, EventPayload::Liquidity(_)) => Ok(()),
            (EventType::Information, EventPayload::Information(_)) => Ok(()),
            (EventType::System, EventPayload::System(_)) => Ok(()),
            _ => Err(format!("Payload type does not match event type {:?}", event_type)),
        }
    }
}

/// Data normalizer for unit conversions and format standardization
pub struct DataNormalizer;

impl DataNormalizer {
    pub fn new() -> Self {
        Self
    }

    /// Normalize volume values to standard units
    pub fn normalize_volume(&self, volume: u64, source: &DataSource) -> u64 {
        // Different sources may report in different units
        // For now, assume all sources use consistent units
        // In production, this would handle conversions
        match source.0.as_str() {
            "NSE" | "BSE" | "NSE_FO" => volume,
            _ => volume, // Default passthrough
        }
    }

    /// Normalize price values to standard precision
    pub fn normalize_price(&self, price: f64) -> f64 {
        // Round to 2 decimal places for currency precision
        (price * 100.0).round() / 100.0
    }

    /// Normalize ratio values to [0, 1] range
    pub fn normalize_ratio(&self, ratio: f64) -> f64 {
        ratio.max(0.0).min(1.0)
    }
}

/// Data quality assessor
pub struct QualityAssessor;

impl QualityAssessor {
    pub fn new() -> Self {
        Self
    }

    /// Assess overall data quality
    pub fn assess_quality(&self, event: &CanonicalEvent) -> DataQuality {
        let mut fields_present = 0;
        let mut fields_required = 0;
        let mut contradictions = 0;

        // Count required vs present fields based on payload type
        match &event.payload {
            EventPayload::Positioning(payload) => {
                fields_required = 5; // instrument, expiry, oi, oi_change, volume
                if !payload.instrument.0.is_empty() { fields_present += 1; }
                if payload.open_interest > 0 { fields_present += 1; }
                // Add more field checks as needed
            },
            EventPayload::Liquidity(payload) => {
                fields_required = 4; // instrument, volumes, ratio, value
                if !payload.instrument.0.is_empty() { fields_present += 1; }
                if payload.traded_volume > 0 { fields_present += 1; }
                if payload.delivery_volume > 0 { fields_present += 1; }
                if payload.delivery_ratio >= 0.0 && payload.delivery_ratio <= 1.0 { fields_present += 1; }
                if payload.avg_daily_value > 0.0 { fields_present += 1; }
            },
            _ => {
                // Default assessment
                fields_required = 3;
                fields_present = 3; // Assume basic fields are present
            }
        }

        // Calculate delay (simplified - in real implementation would compare to current time)
        let delay_seconds = 0.0; // Placeholder

        DataQuality {
            fields_present,
            fields_required,
            delay_seconds,
            acceptable_delay_threshold: 300.0,
            contradictions_detected: contradictions,
            total_cross_checks: fields_required,
        }
    }

    /// Check if data quality meets minimum thresholds
    pub fn meets_thresholds(&self, quality: &DataQuality, thresholds: &QualityThresholds) -> bool {
        quality.completeness_ratio() >= thresholds.min_completeness
            && !quality.is_delayed()
            && quality.contradictions_detected <= thresholds.max_contradictions
    }
}

/// Main ingestion processor
pub struct DataIngestion {
    validator: DataValidator,
    normalizer: DataNormalizer,
    quality_assessor: QualityAssessor,
}

impl DataIngestion {
    pub fn new() -> Self {
        Self {
            validator: DataValidator::new(),
            normalizer: DataNormalizer::new(),
            quality_assessor: QualityAssessor::new(),
        }
    }

    /// Process incoming event data
    pub fn process_event(&self, raw_event: CanonicalEvent) -> IngestionResult<CanonicalEvent> {
        // First validate the event structure
        let validation_result = self.validator.validate_event(&raw_event);
        if let IngestionResult::FailedValidation(errors) = &validation_result {
            return IngestionResult::FailedValidation(errors.clone());
        }

        // Assess data quality
        let quality = self.quality_assessor.assess_quality(&raw_event);
        let thresholds = QualityThresholds::default();

        if !self.quality_assessor.meets_thresholds(&quality, &thresholds) {
            return IngestionResult::Rejected(format!(
                "Data quality below threshold: completeness={:.2}, delayed={}, contradictions={}",
                quality.completeness_ratio(),
                quality.is_delayed(),
                quality.contradictions_detected
            ));
        }

        // Normalize data
        let normalized_event = self.normalize_event(raw_event);

        IngestionResult::Success(normalized_event)
    }

    fn normalize_event(&self, mut event: CanonicalEvent) -> CanonicalEvent {
        // Apply normalization based on payload type
        match &mut event.payload {
            EventPayload::Positioning(payload) => {
                payload.open_interest = self.normalizer.normalize_volume(payload.open_interest, &event.source);
                payload.volume = self.normalizer.normalize_volume(payload.volume, &event.source);
                if let Some(strike) = payload.strike {
                    payload.strike = Some(self.normalizer.normalize_price(strike));
                }
            },
            EventPayload::Liquidity(payload) => {
                payload.traded_volume = self.normalizer.normalize_volume(payload.traded_volume, &event.source);
                payload.delivery_volume = self.normalizer.normalize_volume(payload.delivery_volume, &event.source);
                payload.delivery_ratio = self.normalizer.normalize_ratio(payload.delivery_ratio);
                payload.avg_daily_value = self.normalizer.normalize_price(payload.avg_daily_value);
            },
            _ => {} // Other payload types don't need normalization yet
        }

        event
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_valid_positioning_event() {
        let ingestion = DataIngestion::new();

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

        let result = ingestion.process_event(event);
        assert!(matches!(result, IngestionResult::Success(_)));
    }

    #[test]
    fn test_reject_unapproved_source() {
        let ingestion = DataIngestion::new();

        let event = CanonicalEvent::new(
            EventId("test-event-2".to_string()),
            EventType::Positioning,
            Utc::now(),
            DataSource("UNKNOWN_SOURCE".to_string()),
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

        let result = ingestion.process_event(event);
        assert!(matches!(result, IngestionResult::Rejected(_)));
    }
}