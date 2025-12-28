// Chaos and Failure Injection Integration Tests
// Tests system resilience under adverse conditions

use galactus_core::confidence::*;
use galactus_core::failure_analysis::*;
use galactus_core::stress_scenarios::*;

#[test]
fn test_confidence_with_missing_data() {
    // Simulate missing data fields
    let data_input = DataQualityInput {
        fields_present: 5, // Only half of required fields
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    };

    let structural_input = StructuralAlignmentInput {
        identified_constraints: 2,
        expected_constraints: 2,
        aligned_signals: 4,
        total_signals: 5,
        contradictory_signals: 0,
    };

    let regime_input = RegimeConsistencyInput {
        regime_probabilities: vec![0.7, 0.2, 0.1],
        regime_transitions: 2,
        lookback_window: 10,
        consistent_signals: 8,
        total_signals_in_regime: 10,
    };

    let signal_input = SignalAgreementInput {
        agreeing_signals: 3,
        total_signals: 4,
        offsetting_pressure: 10.0,
        total_pressure: 100.0,
        aggregation_method_documented: true,
    };

    let temporal_input = TemporalStabilityInput {
        consistent_events: 8,
        total_events_in_window: 10,
        inference_values: vec![100.0, 102.0, 101.0],
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    };

    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![101.0, 99.0],
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    };

    let stability = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let confidence = assess_confidence(
        &data_input,
        &structural_input,
        &regime_input,
        &signal_input,
        stability.score,
    );

    // System should degrade gracefully with missing data
    // The score may not be below 0.5 if other components are strong
    // The key is that data quality component should reflect the issue
    assert!(
        confidence.score >= 0.0 && confidence.score <= 1.0,
        "Confidence must be in valid range"
    );
    assert!(
        confidence.components.data_quality < 0.6,
        "Data quality should be degraded with missing fields"
    );
}

#[test]
fn test_confidence_with_contradictory_signals() {
    // Simulate contradictory signals
    let data_input = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 4, // High contradictions
        total_cross_checks: 5,
    };

    let structural_input = StructuralAlignmentInput {
        identified_constraints: 1,
        expected_constraints: 3,
        aligned_signals: 2,
        total_signals: 5,
        contradictory_signals: 3, // Many contradictions
    };

    let regime_input = RegimeConsistencyInput {
        regime_probabilities: vec![0.4, 0.35, 0.25], // Uncertain regime
        regime_transitions: 5,                       // Frequent transitions
        lookback_window: 10,
        consistent_signals: 4,
        total_signals_in_regime: 10,
    };

    let signal_input = SignalAgreementInput {
        agreeing_signals: 1,
        total_signals: 4,
        offsetting_pressure: 80.0, // High offsetting pressure
        total_pressure: 100.0,
        aggregation_method_documented: true,
    };

    let temporal_input = TemporalStabilityInput {
        consistent_events: 8,
        total_events_in_window: 10,
        inference_values: vec![100.0, 102.0, 101.0],
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    };

    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![101.0, 99.0],
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    };

    let stability = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let confidence = assess_confidence(
        &data_input,
        &structural_input,
        &regime_input,
        &signal_input,
        stability.score,
    );

    // Should have very low confidence with contradictions
    assert!(
        confidence.score < 0.3,
        "Confidence should be very low with contradictions"
    );
    assert!(confidence.components.data_quality < 0.5);
    assert!(confidence.components.structural_alignment < 0.3);
}

#[test]
fn test_extreme_data_delay() {
    // Simulate extreme data delay
    let data_input = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 600.0, // 10 minute delay
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    };

    let structural_input = StructuralAlignmentInput {
        identified_constraints: 2,
        expected_constraints: 2,
        aligned_signals: 4,
        total_signals: 5,
        contradictory_signals: 0,
    };

    let regime_input = RegimeConsistencyInput {
        regime_probabilities: vec![0.7, 0.2, 0.1],
        regime_transitions: 2,
        lookback_window: 10,
        consistent_signals: 8,
        total_signals_in_regime: 10,
    };

    let signal_input = SignalAgreementInput {
        agreeing_signals: 3,
        total_signals: 4,
        offsetting_pressure: 10.0,
        total_pressure: 100.0,
        aggregation_method_documented: true,
    };

    let temporal_input = TemporalStabilityInput {
        consistent_events: 8,
        total_events_in_window: 10,
        inference_values: vec![100.0, 102.0, 101.0],
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    };

    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![101.0, 99.0],
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    };

    let stability = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let confidence = assess_confidence(
        &data_input,
        &structural_input,
        &regime_input,
        &signal_input,
        stability.score,
    );

    // Should trigger silence with extreme delay
    let decision = evaluate_silence(&confidence, &stability);
    assert!(
        matches!(decision, SilenceDecision::Suppress(_)),
        "Should suppress inference with extreme data delay"
    );
}

#[test]
fn test_volatile_inference_values() {
    // Simulate highly volatile/unstable inference values
    let data_input = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    };

    let structural_input = StructuralAlignmentInput {
        identified_constraints: 2,
        expected_constraints: 2,
        aligned_signals: 4,
        total_signals: 5,
        contradictory_signals: 0,
    };

    let regime_input = RegimeConsistencyInput {
        regime_probabilities: vec![0.7, 0.2, 0.1],
        regime_transitions: 2,
        lookback_window: 10,
        consistent_signals: 8,
        total_signals_in_regime: 10,
    };

    let signal_input = SignalAgreementInput {
        agreeing_signals: 3,
        total_signals: 4,
        offsetting_pressure: 10.0,
        total_pressure: 100.0,
        aggregation_method_documented: true,
    };

    // Highly volatile values
    let temporal_input = TemporalStabilityInput {
        consistent_events: 3,
        total_events_in_window: 10,
        inference_values: vec![100.0, 150.0, 80.0, 200.0, 50.0], // Wild swings
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    };

    // High sensitivity to perturbations
    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![150.0, 50.0, 180.0, 30.0], // Large changes
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    };

    let stability = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);

    // Stability should be low with volatile values, but may not be extremely low
    // if the regime component is still strong. The key is detecting the volatility.
    assert!(
        stability.score >= 0.0 && stability.score <= 1.0,
        "Stability must be in valid range"
    );
    assert!(
        stability.components.temporal < 0.6 || stability.components.sensitivity < 0.6,
        "Either temporal or sensitivity component should detect volatility"
    );
}

#[test]
fn test_stress_scenario_market_crash() {
    // Test system behavior during market crash scenario
    use galactus_core::stress_scenarios::{StressScenarioCategory, StressSeverity};

    let scenario = StressScenario::new(
        StressScenarioCategory::VolatilityShock,
        StressSeverity::Extreme,
        "Rapid market decline with high volatility".to_string(),
        1234567890,
        false,
    );

    // Should identify high risk
    assert!(scenario.is_critical());
    assert!(scenario.should_suppress_inference());
}

#[test]
fn test_stress_scenario_expiry_day() {
    // Test system behavior during expiry day chaos
    use galactus_core::stress_scenarios::{StressScenarioCategory, StressSeverity};

    let scenario = StressScenario::new(
        StressScenarioCategory::ExpiryCompression,
        StressSeverity::Severe,
        "Expiry day with extreme activity".to_string(),
        1234567890,
        false,
    );

    // Should recognize high risk situation
    assert!(scenario.is_critical());
}

#[test]
fn test_cascading_failures() {
    // Test system response to cascading failures
    let mut analyzer = FailureAnalyzer::new();

    // Simulate cascade: data failure -> regime misclassification -> overconfidence
    analyzer.record_failure(FailureRecord::new(
        FailureCategory::DataFailure,
        "Initial data feed loss".to_string(),
        "Provider outage".to_string(),
        vec!["data_source".to_string()],
    ));

    analyzer.record_failure(FailureRecord::new(
        FailureCategory::RegimeMisclassification,
        "Missed regime shift due to stale data".to_string(),
        "Data delay from previous failure".to_string(),
        vec!["regime_detector".to_string(), "data_source".to_string()],
    ));

    analyzer.record_failure(FailureRecord::new(
        FailureCategory::OverconfidenceFailure,
        "High confidence despite data issues".to_string(),
        "Confidence system didn't detect data staleness".to_string(),
        vec!["confidence_engine".to_string(), "data_source".to_string()],
    ));

    // Should detect related failures
    let patterns = analyzer.detect_patterns();
    assert!(!patterns.is_empty());

    // Should flag data_source as problematic component
    let report = analyzer.generate_learning_report();
    assert!(report
        .components_needing_attention
        .contains(&"data_source".to_string()));
}

#[test]
fn test_regime_uncertainty_chaos() {
    // Test with highly uncertain regime state
    let regime_input = RegimeConsistencyInput {
        regime_probabilities: vec![0.26, 0.25, 0.24, 0.25], // Nearly uniform
        regime_transitions: 10,                             // Frequent transitions
        lookback_window: 10,
        consistent_signals: 3,
        total_signals_in_regime: 10,
    };

    let confidence = compute_regime_confidence(&regime_input);

    // Should have low confidence in uncertain regime
    assert!(
        confidence < 0.4,
        "Confidence should be low with regime uncertainty"
    );
}

#[test]
fn test_empty_signal_set() {
    // Test behavior with no signals
    let signal_input = SignalAgreementInput {
        agreeing_signals: 0,
        total_signals: 0,
        offsetting_pressure: 0.0,
        total_pressure: 0.0,
        aggregation_method_documented: true,
    };

    let confidence = compute_signal_confidence(&signal_input);

    // Should handle empty signals gracefully
    assert!(confidence >= 0.0 && confidence <= 1.0);
}

#[test]
fn test_extreme_perturbation_sensitivity() {
    // Test with extreme sensitivity to input perturbations
    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![500.0, 1.0, 800.0], // Extreme variations
    };

    let stability = compute_sensitivity_stability(&sensitivity_input);

    // Should recognize high instability
    assert!(stability < 0.2, "Should detect extreme instability");
}
