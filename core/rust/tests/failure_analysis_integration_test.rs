// Integration test for the Failure Analysis Framework
// Demonstrates end-to-end failure recording, analysis, and learning

use galactus_core::failure_analysis::*;

#[test]
fn test_end_to_end_failure_workflow() {
    let mut analyzer = FailureAnalyzer::new();
    
    // Scenario: System experiences multiple data failures
    let failure1 = FailureRecord::new(
        FailureCategory::DataFailure,
        "Missing OI data for NSE FO".to_string(),
        "Data provider API timeout".to_string(),
        vec!["oi_signal".to_string(), "derivatives_pressure".to_string()],
    );
    
    let failure2 = FailureRecord::new(
        FailureCategory::DataFailure,
        "Delayed index data by 5 minutes".to_string(),
        "Network latency spike".to_string(),
        vec!["index_tracker".to_string()],
    );
    
    let failure3 = FailureRecord::new(
        FailureCategory::DataFailure,
        "Missing OI data again".to_string(),
        "Data provider still down".to_string(),
        vec!["oi_signal".to_string(), "derivatives_pressure".to_string()],
    ).with_corrective_action("Switched to backup data source".to_string());
    
    // Record failures
    analyzer.record_failure(failure1);
    analyzer.record_failure(failure2);
    analyzer.record_failure(failure3);
    
    // Analyze patterns
    let patterns = analyzer.detect_patterns();
    
    // Should detect a data failure pattern
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].category, FailureCategory::DataFailure);
    assert_eq!(patterns[0].count, 3);
    assert!(patterns[0].requires_action);
    
    // Generate learning report
    let report = analyzer.generate_learning_report();
    assert_eq!(report.total_failures, 3);
    assert_eq!(report.unaddressed_failures, 2); // One was addressed
}

#[test]
fn test_repeated_signal_failures_trigger_deprecation() {
    let mut analyzer = FailureAnalyzer::new();
    
    // Record multiple failures for the same signal
    for i in 0..8 {
        let failure = FailureRecord::new(
            FailureCategory::SignalMisapplication,
            format!("Expiry signal fired {} days before expiry", 10 + i),
            "Signal applied outside valid temporal window".to_string(),
            vec!["expiry_pressure_signal".to_string()],
        );
        analyzer.record_failure(failure);
    }
    
    // Should recommend deprecation
    assert!(analyzer.should_deprecate_component("expiry_pressure_signal"));
    
    // Pattern should recommend signal deprecation
    let patterns = analyzer.detect_patterns();
    assert!(!patterns.is_empty());
    
    let pattern = &patterns[0];
    assert!(pattern.requires_action);
    assert!(matches!(
        pattern.recommended_trigger,
        Some(FailureTrigger::SignalDeprecation)
    ));
}

#[test]
fn test_mixed_category_failures() {
    let mut analyzer = FailureAnalyzer::new();
    
    // Record failures across different categories
    analyzer.record_failure(FailureRecord::new(
        FailureCategory::DataFailure,
        "Missing data".to_string(),
        "Provider issue".to_string(),
        vec!["data_source".to_string()],
    ));
    
    analyzer.record_failure(FailureRecord::new(
        FailureCategory::RegimeMisclassification,
        "Classified as normal when fragile".to_string(),
        "Regime detector missed liquidity stress".to_string(),
        vec!["regime_classifier".to_string()],
    ));
    
    analyzer.record_failure(FailureRecord::new(
        FailureCategory::OverconfidenceFailure,
        "High confidence with degraded data".to_string(),
        "Confidence degradation rules too lenient".to_string(),
        vec!["confidence_engine".to_string()],
    ));
    
    // Should detect patterns for each category
    let patterns = analyzer.detect_patterns();
    assert_eq!(patterns.len(), 3);
    
    // Each category should have different recommended actions
    let data_pattern = patterns.iter()
        .find(|p| p.category == FailureCategory::DataFailure)
        .unwrap();
    assert!(matches!(
        data_pattern.recommended_trigger,
        None // Single occurrence doesn't trigger action
    ));
}

#[test]
fn test_regime_failures_escalate_appropriately() {
    let mut analyzer = FailureAnalyzer::new();
    
    // Record moderate regime failures
    for i in 0..5 {
        analyzer.record_failure(FailureRecord::new(
            FailureCategory::RegimeMisclassification,
            format!("Regime misclassification {}", i),
            "Missed transition signal".to_string(),
            vec!["regime_detector".to_string()],
        ));
    }
    
    let patterns = analyzer.detect_patterns();
    let regime_pattern = patterns.iter()
        .find(|p| p.category == FailureCategory::RegimeMisclassification)
        .unwrap();
    
    // Should recommend model revision for moderate failures
    assert!(regime_pattern.requires_action);
    assert!(matches!(
        regime_pattern.recommended_trigger,
        Some(FailureTrigger::ModelRevision)
    ));
    
    // Add more failures to trigger architectural review
    for i in 5..12 {
        analyzer.record_failure(FailureRecord::new(
            FailureCategory::RegimeMisclassification,
            format!("Regime misclassification {}", i),
            "Structural regime detection issue".to_string(),
            vec!["regime_detector".to_string()],
        ));
    }
    
    let new_patterns = analyzer.detect_patterns();
    let severe_pattern = new_patterns.iter()
        .find(|p| p.category == FailureCategory::RegimeMisclassification)
        .unwrap();
    
    // Should escalate to architectural review for severe failures
    assert!(matches!(
        severe_pattern.recommended_trigger,
        Some(FailureTrigger::ArchitecturalReview)
    ));
}

#[test]
fn test_learning_report_identifies_problem_components() {
    let mut analyzer = FailureAnalyzer::new();
    
    // Create failures for a good component
    for _ in 0..2 {
        analyzer.record_failure(FailureRecord::new(
            FailureCategory::DataFailure,
            "Occasional data hiccup".to_string(),
            "Transient issue".to_string(),
            vec!["stable_component".to_string()],
        ).with_corrective_action("Fixed".to_string()));
    }
    
    // Create failures for a problematic component
    for i in 0..5 {
        analyzer.record_failure(FailureRecord::new(
            FailureCategory::SignalMisapplication,
            format!("Problem {}", i),
            "Persistent issue".to_string(),
            vec!["problematic_signal".to_string()],
        ));
    }
    
    // Create failures for a very bad component
    for i in 0..10 {
        analyzer.record_failure(FailureRecord::new(
            FailureCategory::ModelAssumptionFailure,
            format!("Critical issue {}", i),
            "Fundamental problem".to_string(),
            vec!["broken_model".to_string()],
        ));
    }
    
    let report = analyzer.generate_learning_report();
    
    // Should identify problematic component (repeated but not critical)
    assert!(report.components_needing_attention.contains(&"problematic_signal".to_string()));
    
    // Should identify broken component for deprecation
    assert!(report.deprecation_candidates.contains(&"broken_model".to_string()));
    
    // Stable component should not appear in either list
    assert!(!report.components_needing_attention.contains(&"stable_component".to_string()));
    assert!(!report.deprecation_candidates.contains(&"stable_component".to_string()));
}

#[test]
fn test_failure_categories_have_correct_response_strategies() {
    // Verify each category has an appropriate response strategy
    assert!(FailureCategory::DataFailure.response_strategy()
        .contains("do not adjust inference logic"));
    
    assert!(FailureCategory::ModelAssumptionFailure.response_strategy()
        .contains("model revision"));
    
    assert!(FailureCategory::RegimeMisclassification.response_strategy()
        .contains("regime detection"));
    
    assert!(FailureCategory::SignalMisapplication.response_strategy()
        .contains("applicability rules"));
    
    assert!(FailureCategory::OverconfidenceFailure.response_strategy()
        .contains("confidence degradation"));
}

#[test]
fn test_deterministic_pattern_detection() {
    // Verify that pattern detection is deterministic
    let mut analyzer1 = FailureAnalyzer::new();
    let mut analyzer2 = FailureAnalyzer::new();
    
    // Record same failures in both
    for i in 0..4 {
        let failure1 = FailureRecord::new(
            FailureCategory::DataFailure,
            format!("Failure {}", i),
            "Cause".to_string(),
            vec!["component".to_string()],
        );
        
        let failure2 = FailureRecord::new(
            FailureCategory::DataFailure,
            format!("Failure {}", i),
            "Cause".to_string(),
            vec!["component".to_string()],
        );
        
        analyzer1.record_failure(failure1);
        analyzer2.record_failure(failure2);
    }
    
    let patterns1 = analyzer1.detect_patterns();
    let patterns2 = analyzer2.detect_patterns();
    
    assert_eq!(patterns1.len(), patterns2.len());
    assert_eq!(patterns1[0].count, patterns2[0].count);
    assert_eq!(patterns1[0].requires_action, patterns2[0].requires_action);
}
