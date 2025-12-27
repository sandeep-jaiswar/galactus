// Integration test to verify determinism of confidence and stability computations

use galactus_core::confidence::*;

#[test]
fn test_determinism_same_inputs_same_outputs() {
    // Run the same computation twice
    let data_input = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 5.0,
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
        inference_values: vec![100.0, 102.0, 101.0, 99.0, 100.5],
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    };

    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![101.0, 99.0, 100.5],
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    };

    // First run
    let stability1 = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let confidence1 = assess_confidence(
        &data_input,
        &structural_input,
        &regime_input,
        &signal_input,
        stability1.score,
    );

    // Second run with same inputs
    let stability2 = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let confidence2 = assess_confidence(
        &data_input,
        &structural_input,
        &regime_input,
        &signal_input,
        stability2.score,
    );

    // Verify determinism
    assert_eq!(confidence1.score, confidence2.score);
    assert_eq!(confidence1.components, confidence2.components);
    assert_eq!(confidence1.level, confidence2.level);
    assert_eq!(stability1.score, stability2.score);
    assert_eq!(stability1.components, stability2.components);
    assert_eq!(stability1.level, stability2.level);
}

#[test]
fn test_geometric_mean_prevents_false_confidence() {
    // Test that one very low component significantly degrades overall confidence
    let data_input = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    };

    // Very low structural confidence
    let structural_input = StructuralAlignmentInput {
        identified_constraints: 1,
        expected_constraints: 5,
        aligned_signals: 2,
        total_signals: 5,
        contradictory_signals: 2,
    };

    let regime_input = RegimeConsistencyInput {
        regime_probabilities: vec![0.8, 0.2],
        regime_transitions: 1,
        lookback_window: 10,
        consistent_signals: 9,
        total_signals_in_regime: 10,
    };

    let signal_input = SignalAgreementInput {
        agreeing_signals: 3,
        total_signals: 3,
        offsetting_pressure: 0.0,
        total_pressure: 100.0,
        aggregation_method_documented: true,
    };

    let temporal_input = TemporalStabilityInput {
        consistent_events: 9,
        total_events_in_window: 10,
        inference_values: vec![100.0, 101.0, 100.0],
        time_since_last_confirmation: 0.0,
        half_life: 300.0,
    };

    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![100.5, 99.5],
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 0,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 2,
    };

    let stability = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let confidence = assess_confidence(
        &data_input,
        &structural_input,
        &regime_input,
        &signal_input,
        stability.score,
    );

    // Verify that despite having 3 high components, the low structural confidence degrades overall
    assert!(confidence.components.data_quality > 0.9, 
        "Data quality should be high: {}", confidence.components.data_quality);
    assert!(confidence.components.signal_agreement > 0.9,
        "Signal agreement should be high: {}", confidence.components.signal_agreement);
    assert!(confidence.components.structural_alignment < 0.2,
        "Structural alignment should be low: {}", confidence.components.structural_alignment);
    
    // The geometric mean should prevent averaging from hiding the weak component
    // With one component < 0.2 and others > 0.9, geometric mean should be < 0.6
    assert!(confidence.score < 0.6,
        "Overall confidence should be significantly degraded by weak structural component: {}", confidence.score);
    
    // Verify geometric mean is working (if we used arithmetic mean, we'd get ~0.625)
    let arithmetic_mean = (confidence.components.data_quality 
        + confidence.components.structural_alignment 
        + confidence.components.regime_consistency 
        + confidence.components.signal_agreement) / 4.0;
    assert!(confidence.score < arithmetic_mean,
        "Geometric mean ({}) should be lower than arithmetic mean ({})", 
        confidence.score, arithmetic_mean);
}

#[test]
fn test_silence_decision_consistency() {
    // Test that silence decisions are consistent and correct
    
    // Scenario 1: Good confidence and stability -> Proceed
    let data_input1 = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    };

    let structural_input1 = StructuralAlignmentInput {
        identified_constraints: 3,
        expected_constraints: 3,
        aligned_signals: 5,
        total_signals: 5,
        contradictory_signals: 0,
    };

    let regime_input1 = RegimeConsistencyInput {
        regime_probabilities: vec![0.8, 0.2],
        regime_transitions: 1,
        lookback_window: 10,
        consistent_signals: 9,
        total_signals_in_regime: 10,
    };

    let signal_input1 = SignalAgreementInput {
        agreeing_signals: 4,
        total_signals: 4,
        offsetting_pressure: 0.0,
        total_pressure: 100.0,
        aggregation_method_documented: true,
    };

    let temporal_input1 = TemporalStabilityInput {
        consistent_events: 9,
        total_events_in_window: 10,
        inference_values: vec![100.0, 101.0, 100.0, 99.0, 100.5],
        time_since_last_confirmation: 0.0,
        half_life: 300.0,
    };

    let sensitivity_input1 = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![100.5, 99.5, 100.2],
    };

    let regime_stability_input1 = RegimeStabilityInput {
        incoherent_inferences: 0,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 2,
    };

    let stability1 = assess_stability(&temporal_input1, &sensitivity_input1, &regime_stability_input1);
    let confidence1 = assess_confidence(
        &data_input1,
        &structural_input1,
        &regime_input1,
        &signal_input1,
        stability1.score,
    );

    let decision1 = evaluate_silence(&confidence1, &stability1);
    assert!(matches!(decision1, SilenceDecision::Proceed));

    // Scenario 2: Poor data quality -> Suppress
    let data_input2 = DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 4,  // High contradictions
        total_cross_checks: 5,
    };

    let stability2 = assess_stability(&temporal_input1, &sensitivity_input1, &regime_stability_input1);
    let confidence2 = assess_confidence(
        &data_input2,
        &structural_input1,
        &regime_input1,
        &signal_input1,
        stability2.score,
    );

    let decision2 = evaluate_silence(&confidence2, &stability2);
    assert!(matches!(
        decision2,
        SilenceDecision::Suppress(SuppressionReason::DataQualityBelowThreshold)
    ));
}

#[test]
fn test_example_scenarios_from_documentation() {
    // Example 1 from docs: High Confidence Scenario
    let data_quality1 = compute_data_quality_confidence(&DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 0.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    });
    assert!((data_quality1 - 1.0).abs() < 0.01);

    let structural1 = compute_structural_confidence(&StructuralAlignmentInput {
        identified_constraints: 3,
        expected_constraints: 3,
        aligned_signals: 5,
        total_signals: 5,
        contradictory_signals: 0,
    });
    assert!((structural1 - 1.0).abs() < 0.01);

    // Example 3 from docs: Critical - Silence Required
    let data_quality3 = compute_data_quality_confidence(&DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 300.0,  // 5 minutes delay
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    });
    // exp(-300/60) = exp(-5) ≈ 0.0067
    assert!(data_quality3 < 0.01);
}
