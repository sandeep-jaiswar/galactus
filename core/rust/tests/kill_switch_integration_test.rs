//! Integration tests for kill switch system
//!
//! These tests verify that the kill switch correctly prevents inference
//! under various failure conditions across the full inference pipeline.

#[cfg(test)]
mod kill_switch_integration_tests {
    use galactus_core::intent::{SafeIntentEngine, IntentConfig, SignalInput};
    use galactus_core::kill_switch::{
        KillSwitchConfig, DataQualityMetrics, RegimeAssessment, StructuralValidation,
    };
    use std::collections::HashMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_test_signal(name: &str, value: f64, confidence: f64) -> SignalInput {
        SignalInput {
            name: name.to_string(),
            value,
            confidence,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            metadata: HashMap::new(),
        }
    }

    fn create_valid_data_quality() -> DataQualityMetrics {
        DataQualityMetrics {
            data_completeness: 0.95,
            schema_valid: true,
            integrity_valid: true,
            max_time_ambiguity: 10,
            ordering_consistent: true,
        }
    }

    fn create_valid_structural() -> StructuralValidation {
        StructuralValidation {
            constraints_valid: true,
            forced_flow_consistent: true,
            violations: vec![],
        }
    }

    #[test]
    fn test_end_to_end_inference_with_valid_conditions() {
        // Create a safe engine with default configurations
        let engine = SafeIntentEngine::default();

        // Create test signals representing market conditions
        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
            create_test_signal("hedge_pressure", 0.4, 0.80),
            create_test_signal("basis_pressure", 0.2, 0.75),
        ];

        // Process with valid data quality and structural validation
        let result = engine.process_safe(
            signals,
            create_valid_data_quality(),
            create_valid_structural(),
        );

        // Should succeed
        assert!(result.is_ok());

        let result = result.unwrap();
        
        // Verify kill switch status is passed
        assert_eq!(
            result.metadata.get("kill_switch_status").unwrap(),
            "passed"
        );

        // Verify inference result is valid
        assert!(result.intent.pressure >= -1.0 && result.intent.pressure <= 1.0);
        assert!(result.intent.confidence >= 0.0 && result.intent.confidence <= 1.0);
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_corrupted_data() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        // Create data quality metrics with integrity failure
        let mut data_quality = create_valid_data_quality();
        data_quality.integrity_valid = false;

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        // Should fail with kill switch error
        assert!(result.is_err());
        
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Kill switch triggered"));
        assert!(error_msg.contains("Data integrity"));
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_schema_mismatch() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut data_quality = create_valid_data_quality();
        data_quality.schema_valid = false;

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("schema"));
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_insufficient_data() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut data_quality = create_valid_data_quality();
        data_quality.data_completeness = 0.60; // Below threshold of 0.80

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Missing core data"));
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_time_ambiguity() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut data_quality = create_valid_data_quality();
        data_quality.max_time_ambiguity = 120; // Exceeds threshold of 60s

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("time"));
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_event_ordering_issues() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut data_quality = create_valid_data_quality();
        data_quality.ordering_consistent = false;

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("ordering"));
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_constraint_violations() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut structural = create_valid_structural();
        structural.constraints_valid = false;
        structural.violations = vec![
            "Option chain discontinuity detected".to_string(),
            "Futures settlement mismatch".to_string(),
        ];

        let result = engine.process_safe(
            signals,
            create_valid_data_quality(),
            structural,
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Constraint"));
    }

    #[test]
    fn test_kill_switch_prevents_inference_with_forced_flow_contradiction() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut structural = create_valid_structural();
        structural.forced_flow_consistent = false;

        let result = engine.process_safe(
            signals,
            create_valid_data_quality(),
            structural,
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("flow"));
    }

    #[test]
    fn test_kill_switch_with_custom_thresholds() {
        // Create a more lenient kill switch configuration
        let mut kill_switch_config = KillSwitchConfig::default();
        kill_switch_config.min_data_completeness = 0.60; // Lower threshold

        let engine = SafeIntentEngine::new(
            IntentConfig::default(),
            kill_switch_config,
        );

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        let mut data_quality = create_valid_data_quality();
        data_quality.data_completeness = 0.70; // Would fail with default threshold

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        // Should succeed with lenient threshold
        assert!(result.is_ok());
    }

    #[test]
    fn test_kill_switch_with_multiple_violations() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
        ];

        // Create multiple violations
        let mut data_quality = create_valid_data_quality();
        data_quality.integrity_valid = false;
        data_quality.schema_valid = false;
        data_quality.data_completeness = 0.50;

        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );

        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        
        // Should indicate multiple conditions
        assert!(error_msg.contains("kill condition"));
    }

    #[test]
    fn test_kill_switch_metadata_in_successful_inference() {
        let engine = SafeIntentEngine::default();

        let signals = vec![
            create_test_signal("oi_decay", -0.3, 0.85),
            create_test_signal("hedge_pressure", 0.4, 0.80),
        ];

        let result = engine.process_safe(
            signals,
            create_valid_data_quality(),
            create_valid_structural(),
        );

        assert!(result.is_ok());
        let result = result.unwrap();

        // Verify kill switch status is recorded
        assert!(result.metadata.contains_key("kill_switch_status"));
        assert_eq!(result.metadata.get("kill_switch_status").unwrap(), "passed");
    }

    #[test]
    fn test_deterministic_kill_switch_evaluation() {
        let engine = SafeIntentEngine::default();

        let signals1 = vec![
            create_test_signal("signal1", 0.5, 0.8),
            create_test_signal("signal2", 0.3, 0.9),
        ];

        let signals2 = vec![
            create_test_signal("signal1", 0.5, 0.8),
            create_test_signal("signal2", 0.3, 0.9),
        ];

        let mut data_quality = create_valid_data_quality();
        data_quality.data_completeness = 0.70; // Will fail

        let result1 = engine.process_safe(
            signals1,
            data_quality.clone(),
            create_valid_structural(),
        );

        let result2 = engine.process_safe(
            signals2,
            data_quality,
            create_valid_structural(),
        );

        // Both should fail identically
        assert!(result1.is_err());
        assert!(result2.is_err());
        assert_eq!(
            format!("{}", result1.unwrap_err()),
            format!("{}", result2.unwrap_err())
        );
    }
}
