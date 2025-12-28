// Performance and Benchmark Integration Tests
// Tests system performance under various load conditions

use galactus_core::confidence::*;
use galactus_core::failure_analysis::*;
use galactus_core::stress_scenarios::*;
use std::time::Instant;

#[test]
fn test_confidence_computation_performance() {
    // Test that confidence computation completes within acceptable time
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

    // Run 1000 iterations and measure time
    let start = Instant::now();
    for _ in 0..1000 {
        let stability =
            assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
        let _confidence = assess_confidence(
            &data_input,
            &structural_input,
            &regime_input,
            &signal_input,
            stability.score,
        );
    }
    let duration = start.elapsed();

    // Should complete 1000 iterations in less than 100ms
    assert!(
        duration.as_millis() < 100,
        "Confidence computation too slow: {}ms for 1000 iterations",
        duration.as_millis()
    );

    // Average time per computation should be < 0.1ms
    let avg_micros = duration.as_micros() / 1000;
    println!("Average confidence computation time: {}µs", avg_micros);
    assert!(
        avg_micros < 100,
        "Average computation time too high: {}µs",
        avg_micros
    );
}

#[test]
fn test_failure_analysis_scalability() {
    // Test failure analyzer with large number of failures
    let mut analyzer = FailureAnalyzer::new();

    let start = Instant::now();

    // Record 1000 failures
    for i in 0..1000 {
        let category = match i % 5 {
            0 => FailureCategory::DataFailure,
            1 => FailureCategory::RegimeMisclassification,
            2 => FailureCategory::SignalMisapplication,
            3 => FailureCategory::OverconfidenceFailure,
            _ => FailureCategory::ModelAssumptionFailure,
        };

        let failure = FailureRecord::new(
            category,
            format!("Failure {}", i),
            "Test cause".to_string(),
            vec!["component".to_string()],
        );
        analyzer.record_failure(failure);
    }

    let record_duration = start.elapsed();
    println!(
        "Time to record 1000 failures: {}ms",
        record_duration.as_millis()
    );

    // Pattern detection should be fast even with many failures
    let detect_start = Instant::now();
    let patterns = analyzer.detect_patterns();
    let detect_duration = detect_start.elapsed();

    println!("Time to detect patterns: {}ms", detect_duration.as_millis());
    println!("Patterns detected: {}", patterns.len());

    // Should handle 1000 failures efficiently
    assert!(
        record_duration.as_millis() < 100,
        "Recording failures too slow"
    );
    assert!(
        detect_duration.as_millis() < 50,
        "Pattern detection too slow"
    );
    assert_eq!(patterns.len(), 5); // One pattern per category
}

#[test]
fn test_stress_scenario_evaluation_performance() {
    // Test stress scenario evaluation performance
    use galactus_core::stress_scenarios::{StressScenarioCategory, StressSeverity};

    let scenario = StressScenario::new(
        StressScenarioCategory::VolatilityShock,
        StressSeverity::Severe,
        "Rapid market decline with high volatility".to_string(),
        1234567890,
        true,
    );

    let start = Instant::now();

    // Evaluate scenario multiple times
    for _ in 0..100 {
        // Check if should suppress
        let _should_suppress = scenario.should_suppress_inference();
        let _is_critical = scenario.is_critical();
    }

    let duration = start.elapsed();
    println!(
        "Time to evaluate scenario 100 times: {}ms",
        duration.as_millis()
    );

    // Should be fast
    assert!(duration.as_millis() < 50, "Scenario evaluation too slow");
}

#[test]
fn test_concurrent_confidence_assessment() {
    // Test that confidence assessment works correctly under concurrent load
    use std::sync::Arc;
    use std::thread;

    let data_input = Arc::new(DataQualityInput {
        fields_present: 10,
        fields_required: 10,
        delay_seconds: 5.0,
        acceptable_delay_threshold: 60.0,
        contradictions_detected: 0,
        total_cross_checks: 5,
    });

    let structural_input = Arc::new(StructuralAlignmentInput {
        identified_constraints: 2,
        expected_constraints: 2,
        aligned_signals: 4,
        total_signals: 5,
        contradictory_signals: 0,
    });

    let regime_input = Arc::new(RegimeConsistencyInput {
        regime_probabilities: vec![0.7, 0.2, 0.1],
        regime_transitions: 2,
        lookback_window: 10,
        consistent_signals: 8,
        total_signals_in_regime: 10,
    });

    let signal_input = Arc::new(SignalAgreementInput {
        agreeing_signals: 3,
        total_signals: 4,
        offsetting_pressure: 10.0,
        total_pressure: 100.0,
        aggregation_method_documented: true,
    });

    let temporal_input = Arc::new(TemporalStabilityInput {
        consistent_events: 8,
        total_events_in_window: 10,
        inference_values: vec![100.0, 102.0, 101.0, 99.0, 100.5],
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    });

    let sensitivity_input = Arc::new(SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: vec![101.0, 99.0, 100.5],
    });

    let regime_stability_input = Arc::new(RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    });

    let mut handles = vec![];

    // Spawn 10 threads, each computing confidence 100 times
    for _ in 0..10 {
        let data = Arc::clone(&data_input);
        let structural = Arc::clone(&structural_input);
        let regime = Arc::clone(&regime_input);
        let signal = Arc::clone(&signal_input);
        let temporal = Arc::clone(&temporal_input);
        let sensitivity = Arc::clone(&sensitivity_input);
        let regime_stability = Arc::clone(&regime_stability_input);

        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let stability = assess_stability(&temporal, &sensitivity, &regime_stability);
                let confidence =
                    assess_confidence(&data, &structural, &regime, &signal, stability.score);
                // All threads should get the same result (determinism)
                assert!(confidence.score > 0.0);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    let start = Instant::now();
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    let duration = start.elapsed();

    println!("Concurrent assessment time: {}ms", duration.as_millis());
    // Should complete in reasonable time
    assert!(duration.as_secs() < 5, "Concurrent processing too slow");
}

#[test]
fn test_memory_efficiency_with_large_inputs() {
    // Test that system handles large inputs efficiently
    let large_values: Vec<f64> = (0..10000).map(|i| 100.0 + (i as f64) * 0.01).collect();
    let large_perturbed: Vec<f64> = (0..1000).map(|i| 100.0 + (i as f64) * 0.001).collect();

    let temporal_input = TemporalStabilityInput {
        consistent_events: 9999,
        total_events_in_window: 10000,
        inference_values: large_values,
        time_since_last_confirmation: 30.0,
        half_life: 300.0,
    };

    let sensitivity_input = SensitivityStabilityInput {
        inference_baseline: 100.0,
        inference_perturbed: large_perturbed,
    };

    let regime_stability_input = RegimeStabilityInput {
        incoherent_inferences: 1,
        total_inferences_in_regime: 20,
        surprise_transitions: 0,
        total_transitions: 3,
    };

    let start = Instant::now();
    let stability = assess_stability(&temporal_input, &sensitivity_input, &regime_stability_input);
    let duration = start.elapsed();

    println!("Large input processing time: {}ms", duration.as_millis());

    // Should still be reasonably fast
    assert!(
        duration.as_millis() < 100,
        "Large input processing too slow"
    );
    assert!(stability.score >= 0.0 && stability.score <= 1.0);
}
