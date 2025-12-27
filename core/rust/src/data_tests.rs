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