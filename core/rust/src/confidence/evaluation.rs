// Inference evaluation and silence decision logic
//
// Determines when inference must be suppressed based on confidence and stability scores

use crate::confidence::types::*;

/// Evaluate whether inference should be silenced
///
/// Implements the silence thresholds defined in:
/// docs/05-intent-engine/confidence-and-stability.md
pub fn evaluate_silence(
    confidence: &OverallConfidence,
    stability: &StabilityIndicator,
) -> SilenceDecision {
    // MANDATORY SILENCE CONDITIONS

    // 1. overall_confidence < 0.30
    if confidence.score < 0.30 {
        return SilenceDecision::Suppress(SuppressionReason::InsufficientConfidence);
    }

    // 2. data_quality_confidence < 0.50
    if confidence.components.data_quality < 0.50 {
        return SilenceDecision::Suppress(SuppressionReason::DataQualityBelowThreshold);
    }

    // 3. regime_confidence < 0.30
    if confidence.components.regime_consistency < 0.30 {
        return SilenceDecision::Suppress(SuppressionReason::RegimeIndeterminate);
    }

    // 4. structural_confidence < 0.40 && signal_confidence < 0.40
    if confidence.components.structural_alignment < 0.40
        && confidence.components.signal_agreement < 0.40
    {
        return SilenceDecision::Suppress(
            SuppressionReason::StructuralFoundationInsufficient,
        );
    }

    // 5. overall_stability < 0.20
    if stability.score < 0.20 {
        return SilenceDecision::Suppress(SuppressionReason::InferenceTooUnstable);
    }

    // PARTIAL SILENCE CONDITIONS (warnings)
    let mut warnings = Vec::new();

    // 1. 0.30 <= overall_confidence < 0.40
    if confidence.score >= 0.30 && confidence.score < 0.40 {
        warnings.push("LOW CONFIDENCE: Inference supported by limited evidence".to_string());
        warnings.push(format!(
            "Confidence components - Data: {:.2}, Structural: {:.2}, Regime: {:.2}, Signal: {:.2}",
            confidence.components.data_quality,
            confidence.components.structural_alignment,
            confidence.components.regime_consistency,
            confidence.components.signal_agreement
        ));
    }

    // 2. 0.30 <= overall_stability < 0.40
    if stability.score >= 0.30 && stability.score < 0.40 {
        warnings.push("FRAGILE INFERENCE: Inference may be sensitive to small changes".to_string());
        warnings.push(format!(
            "Stability components - Temporal: {:.2}, Sensitivity: {:.2}, Regime: {:.2}",
            stability.components.temporal,
            stability.components.sensitivity,
            stability.components.regime
        ));
    }

    // 3. signal_confidence < 0.40 (but other dimensions acceptable)
    if confidence.components.signal_agreement < 0.40
        && confidence.components.data_quality >= 0.50
        && confidence.components.regime_consistency >= 0.30
        && confidence.components.structural_alignment >= 0.40
    {
        warnings.push("LIMITED SIGNAL AGREEMENT: Few independent signals support this inference".to_string());
    }

    if !warnings.is_empty() {
        SilenceDecision::ProceedWithWarning(warnings)
    } else {
        SilenceDecision::Proceed
    }
}

/// Check for critical degradation triggers
///
/// Returns true if confidence or stability has critically degraded
pub fn has_critical_degradation(
    confidence: &OverallConfidence,
    stability: &StabilityIndicator,
) -> bool {
    // Any component below critical threshold
    // Note: These thresholds are different from silence thresholds
    // They indicate early warning signs of degradation
    confidence.components.data_quality < 0.30
        || confidence.components.structural_alignment < 0.30
        || confidence.components.regime_consistency < 0.30
        || confidence.components.signal_agreement < 0.30
        || stability.components.temporal < 0.20
        || stability.components.sensitivity < 0.20
        || stability.components.regime < 0.20
}

/// Determine if multiple components are below threshold
///
/// Used for additional degradation rules
pub fn count_components_below_threshold(components: &ConfidenceComponents, threshold: f64) -> usize {
    let mut count = 0;
    if components.data_quality < threshold {
        count += 1;
    }
    if components.structural_alignment < threshold {
        count += 1;
    }
    if components.regime_consistency < threshold {
        count += 1;
    }
    if components.signal_agreement < threshold {
        count += 1;
    }
    count
}

/// Apply additional degradation when multiple components are weak
///
/// If two or more components are below 0.5, reduce overall confidence by 20%
pub fn apply_degradation_penalty(
    base_confidence: f64,
    components: &ConfidenceComponents,
) -> f64 {
    let weak_components = count_components_below_threshold(components, 0.5);
    if weak_components >= 2 {
        base_confidence * 0.8 // 20% penalty
    } else {
        base_confidence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_confidence(
        data_quality: f64,
        structural: f64,
        regime: f64,
        signal: f64,
        stability_score: f64,
    ) -> OverallConfidence {
        let components = ConfidenceComponents {
            data_quality,
            structural_alignment: structural,
            regime_consistency: regime,
            signal_agreement: signal,
        };
        
        let score = (data_quality * structural * regime * signal).powf(0.25);
        let level = ConfidenceLevel::from_scores(score, stability_score);
        
        OverallConfidence {
            score,
            components,
            level,
        }
    }

    fn create_test_stability(temporal: f64, sensitivity: f64, regime: f64) -> StabilityIndicator {
        let components = StabilityComponents {
            temporal,
            sensitivity,
            regime,
        };
        
        let score = (temporal * sensitivity * regime).powf(1.0 / 3.0);
        let level = StabilityLevel::from_score(score);
        
        StabilityIndicator {
            score,
            components,
            level,
        }
    }

    #[test]
    fn test_silence_insufficient_confidence() {
        // Create confidence where overall is low but data quality is acceptable
        let confidence = create_test_confidence(0.55, 0.5, 0.4, 0.4, 0.8);
        let stability = create_test_stability(0.8, 0.8, 0.8);
        
        // Overall confidence should be (0.55 * 0.5 * 0.4 * 0.4)^0.25 = 0.047^0.25 ≈ 0.466
        // But we need it below 0.30, so let's adjust
        let mut confidence = confidence;
        confidence.score = 0.25; // Manually set for test
        
        let decision = evaluate_silence(&confidence, &stability);
        assert!(matches!(
            decision,
            SilenceDecision::Suppress(SuppressionReason::InsufficientConfidence)
        ));
    }

    #[test]
    fn test_silence_data_quality_below_threshold() {
        let confidence = create_test_confidence(0.45, 0.8, 0.7, 0.6, 0.8);
        let stability = create_test_stability(0.8, 0.8, 0.8);
        
        let decision = evaluate_silence(&confidence, &stability);
        assert!(matches!(
            decision,
            SilenceDecision::Suppress(SuppressionReason::DataQualityBelowThreshold)
        ));
    }

    #[test]
    fn test_silence_regime_indeterminate() {
        let confidence = create_test_confidence(0.9, 0.8, 0.25, 0.6, 0.8);
        let stability = create_test_stability(0.8, 0.8, 0.8);
        
        let decision = evaluate_silence(&confidence, &stability);
        assert!(matches!(
            decision,
            SilenceDecision::Suppress(SuppressionReason::RegimeIndeterminate)
        ));
    }

    #[test]
    fn test_silence_structural_foundation_insufficient() {
        let confidence = create_test_confidence(0.9, 0.35, 0.7, 0.35, 0.8);
        let stability = create_test_stability(0.8, 0.8, 0.8);
        
        let decision = evaluate_silence(&confidence, &stability);
        assert!(matches!(
            decision,
            SilenceDecision::Suppress(SuppressionReason::StructuralFoundationInsufficient)
        ));
    }

    #[test]
    fn test_silence_inference_too_unstable() {
        let confidence = create_test_confidence(0.9, 0.8, 0.7, 0.6, 0.15);
        let mut stability = create_test_stability(0.15, 0.8, 0.8);
        
        // Manually set stability score below 0.20 for test
        stability.score = 0.15;
        
        let decision = evaluate_silence(&confidence, &stability);
        assert!(matches!(
            decision,
            SilenceDecision::Suppress(SuppressionReason::InferenceTooUnstable)
        ));
    }

    #[test]
    fn test_proceed_with_warning_low_confidence() {
        let confidence = create_test_confidence(0.6, 0.6, 0.5, 0.5, 0.5);
        let stability = create_test_stability(0.5, 0.5, 0.5);
        
        // Manually set confidence to be in warning range
        let mut confidence = confidence;
        confidence.score = 0.35; // In range [0.30, 0.40)
        
        let decision = evaluate_silence(&confidence, &stability);
        match decision {
            SilenceDecision::ProceedWithWarning(warnings) => {
                assert!(!warnings.is_empty());
                assert!(warnings.iter().any(|w| w.contains("LOW CONFIDENCE")));
            }
            _ => panic!("Expected ProceedWithWarning"),
        }
    }

    #[test]
    fn test_proceed_with_warning_fragile_inference() {
        let confidence = create_test_confidence(0.7, 0.7, 0.6, 0.6, 0.35);
        let mut stability = create_test_stability(0.35, 0.5, 0.5);
        
        // Manually set stability to be in warning range
        stability.score = 0.35; // In range [0.30, 0.40)
        
        let decision = evaluate_silence(&confidence, &stability);
        match decision {
            SilenceDecision::ProceedWithWarning(warnings) => {
                assert!(!warnings.is_empty());
                assert!(warnings.iter().any(|w| w.contains("FRAGILE INFERENCE")));
            }
            _ => panic!("Expected ProceedWithWarning"),
        }
    }

    #[test]
    fn test_proceed_with_warning_limited_signal_agreement() {
        let confidence = create_test_confidence(0.9, 0.7, 0.6, 0.35, 0.8);
        let stability = create_test_stability(0.8, 0.8, 0.8);
        
        let decision = evaluate_silence(&confidence, &stability);
        match decision {
            SilenceDecision::ProceedWithWarning(warnings) => {
                assert!(!warnings.is_empty());
                assert!(warnings.iter().any(|w| w.contains("LIMITED SIGNAL AGREEMENT")));
            }
            _ => panic!("Expected ProceedWithWarning"),
        }
    }

    #[test]
    fn test_proceed_high_confidence() {
        let confidence = create_test_confidence(0.9, 0.85, 0.8, 0.75, 0.8);
        let stability = create_test_stability(0.85, 0.8, 0.9);
        
        let decision = evaluate_silence(&confidence, &stability);
        assert!(matches!(decision, SilenceDecision::Proceed));
    }

    #[test]
    fn test_has_critical_degradation() {
        let confidence = create_test_confidence(0.25, 0.8, 0.7, 0.6, 0.8);
        let stability = create_test_stability(0.8, 0.8, 0.8);
        
        assert!(has_critical_degradation(&confidence, &stability));
    }

    #[test]
    fn test_no_critical_degradation() {
        let confidence = create_test_confidence(0.7, 0.7, 0.6, 0.6, 0.8);
        let stability = create_test_stability(0.8, 0.7, 0.8);
        
        assert!(!has_critical_degradation(&confidence, &stability));
    }

    #[test]
    fn test_count_components_below_threshold() {
        let components = ConfidenceComponents {
            data_quality: 0.4,
            structural_alignment: 0.6,
            regime_consistency: 0.45,
            signal_agreement: 0.3,
        };
        
        let count = count_components_below_threshold(&components, 0.5);
        assert_eq!(count, 3); // data_quality, regime_consistency, signal_agreement
    }

    #[test]
    fn test_apply_degradation_penalty() {
        let components = ConfidenceComponents {
            data_quality: 0.4,
            structural_alignment: 0.6,
            regime_consistency: 0.45,
            signal_agreement: 0.3,
        };
        
        let base_confidence = 0.5;
        let penalized = apply_degradation_penalty(base_confidence, &components);
        
        assert!((penalized - 0.4).abs() < 0.001); // 0.5 * 0.8 = 0.4
    }

    #[test]
    fn test_no_degradation_penalty_when_strong() {
        let components = ConfidenceComponents {
            data_quality: 0.8,
            structural_alignment: 0.7,
            regime_consistency: 0.6,
            signal_agreement: 0.55,
        };
        
        let base_confidence = 0.5;
        let penalized = apply_degradation_penalty(base_confidence, &components);
        
        assert!((penalized - 0.5).abs() < 0.001); // No penalty
    }
}
