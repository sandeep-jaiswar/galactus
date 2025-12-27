// Confidence computation functions
//
// Implements the confidence score computations as defined in:
// docs/05-intent-engine/confidence-and-stability.md

use crate::confidence::types::*;

/// Input data for computing data quality confidence
#[derive(Debug, Clone)]
pub struct DataQualityInput {
    pub fields_present: usize,
    pub fields_required: usize,
    pub delay_seconds: f64,
    pub acceptable_delay_threshold: f64,
    pub contradictions_detected: usize,
    pub total_cross_checks: usize,
}

/// Compute data quality confidence
///
/// Formula:
/// data_quality_confidence = min(completeness_score, timeliness_score, consistency_score)
///
/// where:
///   completeness_score = fields_present / fields_required
///   timeliness_score = exp(-delay_seconds / acceptable_delay_threshold)
///   consistency_score = 1.0 - (contradictions_detected / total_cross_checks)
pub fn compute_data_quality_confidence(input: &DataQualityInput) -> f64 {
    let completeness_score = if input.fields_required > 0 {
        input.fields_present as f64 / input.fields_required as f64
    } else {
        1.0
    };

    let timeliness_score = if input.acceptable_delay_threshold > 0.0 {
        (-input.delay_seconds / input.acceptable_delay_threshold).exp()
    } else {
        1.0
    };

    let consistency_score = if input.total_cross_checks > 0 {
        1.0 - (input.contradictions_detected as f64 / input.total_cross_checks as f64)
    } else {
        1.0
    };

    // Return minimum to ensure all dimensions are acceptable
    completeness_score.min(timeliness_score).min(consistency_score)
}

/// Input data for computing structural alignment confidence
#[derive(Debug, Clone)]
pub struct StructuralAlignmentInput {
    pub identified_constraints: usize,
    pub expected_constraints: usize,
    pub aligned_signals: usize,
    pub total_signals: usize,
    pub contradictory_signals: usize,
}

/// Compute structural alignment confidence
///
/// Formula:
/// structural_confidence = constraint_clarity * mechanic_alignment * (1.0 - contradiction_ratio)
///
/// where:
///   constraint_clarity = identified_constraints / expected_constraints
///   mechanic_alignment = aligned_signals / total_signals
///   contradiction_ratio = contradictory_signals / total_signals
pub fn compute_structural_confidence(input: &StructuralAlignmentInput) -> f64 {
    let constraint_clarity = if input.expected_constraints > 0 {
        input.identified_constraints as f64 / input.expected_constraints as f64
    } else {
        0.0
    };

    let mechanic_alignment = if input.total_signals > 0 {
        input.aligned_signals as f64 / input.total_signals as f64
    } else {
        0.0
    };

    let contradiction_ratio = if input.total_signals > 0 {
        input.contradictory_signals as f64 / input.total_signals as f64
    } else {
        0.0
    };

    constraint_clarity * mechanic_alignment * (1.0 - contradiction_ratio)
}

/// Input data for computing regime consistency confidence
#[derive(Debug, Clone)]
pub struct RegimeConsistencyInput {
    pub regime_probabilities: Vec<f64>,
    pub regime_transitions: usize,
    pub lookback_window: usize,
    pub consistent_signals: usize,
    pub total_signals_in_regime: usize,
}

/// Compute regime consistency confidence
///
/// Formula:
/// regime_confidence = regime_clarity * regime_stability * signal_regime_alignment
///
/// where:
///   regime_clarity = max(regime_probabilities) - second_max(regime_probabilities)
///   regime_stability = 1.0 - (regime_transitions / lookback_window)
///   signal_regime_alignment = consistent_signals / total_signals_in_regime
pub fn compute_regime_confidence(input: &RegimeConsistencyInput) -> f64 {
    let regime_clarity = if input.regime_probabilities.len() >= 2 {
        let mut sorted = input.regime_probabilities.clone();
        // Sort in descending order, handling NaN by treating as 0.0
        sorted.sort_by(|a, b| {
            b.partial_cmp(a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted[0] - sorted[1]
    } else if input.regime_probabilities.len() == 1 {
        input.regime_probabilities[0]
    } else {
        0.0
    };

    let regime_stability = if input.lookback_window > 0 {
        1.0 - (input.regime_transitions as f64 / input.lookback_window as f64)
    } else {
        1.0
    };

    let signal_regime_alignment = if input.total_signals_in_regime > 0 {
        input.consistent_signals as f64 / input.total_signals_in_regime as f64
    } else {
        1.0
    };

    regime_clarity * regime_stability * signal_regime_alignment
}

/// Input data for computing signal agreement confidence
#[derive(Debug, Clone)]
pub struct SignalAgreementInput {
    pub agreeing_signals: usize,
    pub total_signals: usize,
    pub offsetting_pressure: f64,
    pub total_pressure: f64,
    pub aggregation_method_documented: bool,
}

/// Compute signal agreement confidence
///
/// Formula:
/// signal_confidence = confluence_factor * (1.0 - offset_ratio) * transparency_score
///
/// where:
///   confluence_factor = sqrt(agreeing_signals / total_signals)
///   offset_ratio = offsetting_pressure / total_pressure
///   transparency_score = 1.0 if aggregation_method_documented else 0.5
pub fn compute_signal_confidence(input: &SignalAgreementInput) -> f64 {
    let confluence_factor = if input.total_signals > 0 {
        (input.agreeing_signals as f64 / input.total_signals as f64).sqrt()
    } else {
        0.0
    };

    let offset_ratio = if input.total_pressure > 0.0 {
        input.offsetting_pressure / input.total_pressure
    } else {
        0.0
    };

    let transparency_score = if input.aggregation_method_documented {
        1.0
    } else {
        0.5
    };

    confluence_factor * (1.0 - offset_ratio) * transparency_score
}

/// Number of confidence components
const CONFIDENCE_COMPONENTS: usize = 4;

/// Compute overall confidence using geometric mean
///
/// Formula:
/// overall_confidence = (product of component confidences)^(1/n)
///
/// Rationale: Geometric mean ensures that a single critically low component
/// significantly degrades overall scores, preventing false confidence from averaging.
pub fn compute_overall_confidence(components: &ConfidenceComponents) -> f64 {
    let product = components.data_quality
        * components.structural_alignment
        * components.regime_consistency
        * components.signal_agreement;
    
    product.powf(1.0 / CONFIDENCE_COMPONENTS as f64)
}

/// Build complete confidence assessment
pub fn assess_confidence(
    data_quality_input: &DataQualityInput,
    structural_input: &StructuralAlignmentInput,
    regime_input: &RegimeConsistencyInput,
    signal_input: &SignalAgreementInput,
    stability_score: f64,
) -> OverallConfidence {
    let components = ConfidenceComponents {
        data_quality: compute_data_quality_confidence(data_quality_input),
        structural_alignment: compute_structural_confidence(structural_input),
        regime_consistency: compute_regime_confidence(regime_input),
        signal_agreement: compute_signal_confidence(signal_input),
    };

    let score = compute_overall_confidence(&components);
    let level = ConfidenceLevel::from_scores(score, stability_score);

    OverallConfidence {
        score,
        components,
        level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_quality_perfect() {
        let input = DataQualityInput {
            fields_present: 10,
            fields_required: 10,
            delay_seconds: 0.0,
            acceptable_delay_threshold: 60.0,
            contradictions_detected: 0,
            total_cross_checks: 5,
        };
        
        let confidence = compute_data_quality_confidence(&input);
        assert!((confidence - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_data_quality_degraded_by_delay() {
        let input = DataQualityInput {
            fields_present: 10,
            fields_required: 10,
            delay_seconds: 300.0,
            acceptable_delay_threshold: 60.0,
            contradictions_detected: 0,
            total_cross_checks: 5,
        };
        
        let confidence = compute_data_quality_confidence(&input);
        assert!(confidence < 0.01); // exp(-300/60) ≈ 0.0067
    }

    #[test]
    fn test_structural_confidence_perfect() {
        let input = StructuralAlignmentInput {
            identified_constraints: 3,
            expected_constraints: 3,
            aligned_signals: 5,
            total_signals: 5,
            contradictory_signals: 0,
        };
        
        let confidence = compute_structural_confidence(&input);
        assert!((confidence - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_structural_confidence_with_contradictions() {
        let input = StructuralAlignmentInput {
            identified_constraints: 2,
            expected_constraints: 3,
            aligned_signals: 4,
            total_signals: 5,
            contradictory_signals: 1,
        };
        
        let confidence = compute_structural_confidence(&input);
        // (2/3) * (4/5) * (1 - 1/5) = 0.667 * 0.8 * 0.8 = 0.427
        assert!((confidence - 0.427).abs() < 0.01);
    }

    #[test]
    fn test_regime_confidence_clear_regime() {
        let input = RegimeConsistencyInput {
            regime_probabilities: vec![0.8, 0.1, 0.1],
            regime_transitions: 1,
            lookback_window: 10,
            consistent_signals: 9,
            total_signals_in_regime: 10,
        };
        
        let confidence = compute_regime_confidence(&input);
        // (0.8 - 0.1) * (1 - 1/10) * (9/10) = 0.7 * 0.9 * 0.9 = 0.567
        assert!((confidence - 0.567).abs() < 0.01);
    }

    #[test]
    fn test_signal_confidence_high_agreement() {
        let input = SignalAgreementInput {
            agreeing_signals: 3,
            total_signals: 3,
            offsetting_pressure: 0.0,
            total_pressure: 100.0,
            aggregation_method_documented: true,
        };
        
        let confidence = compute_signal_confidence(&input);
        assert!((confidence - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_signal_confidence_with_offset() {
        let input = SignalAgreementInput {
            agreeing_signals: 2,
            total_signals: 3,
            offsetting_pressure: 25.0,
            total_pressure: 100.0,
            aggregation_method_documented: true,
        };
        
        let confidence = compute_signal_confidence(&input);
        // sqrt(2/3) * (1 - 0.25) * 1.0 = 0.816 * 0.75 = 0.612
        assert!((confidence - 0.612).abs() < 0.01);
    }

    #[test]
    fn test_overall_confidence_geometric_mean() {
        let components = ConfidenceComponents {
            data_quality: 0.9,
            structural_alignment: 0.8,
            regime_consistency: 0.7,
            signal_agreement: 0.6,
        };
        
        let confidence = compute_overall_confidence(&components);
        // (0.9 * 0.8 * 0.7 * 0.6)^0.25 = 0.3024^0.25 = 0.742
        assert!((confidence - 0.742).abs() < 0.01);
    }

    #[test]
    fn test_geometric_mean_degrades_with_low_component() {
        let components = ConfidenceComponents {
            data_quality: 0.1, // Very low
            structural_alignment: 0.9,
            regime_consistency: 0.9,
            signal_agreement: 0.9,
        };
        
        let confidence = compute_overall_confidence(&components);
        // (0.1 * 0.9 * 0.9 * 0.9)^0.25 = 0.0729^0.25 = 0.513
        assert!(confidence < 0.6); // Significantly degraded
    }
}
