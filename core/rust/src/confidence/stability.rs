// Stability computation functions
//
// Implements the stability score computations as defined in:
// docs/05-intent-engine/confidence-and-stability.md

use crate::confidence::types::*;

/// Input data for computing temporal stability
#[derive(Debug, Clone)]
pub struct TemporalStabilityInput {
    pub consistent_events: usize,
    pub total_events_in_window: usize,
    pub inference_values: Vec<f64>,
    pub time_since_last_confirmation: f64,
    pub half_life: f64,
}

/// Compute temporal stability
///
/// Formula:
/// temporal_stability = persistence_score * noise_resistance * decay_factor
///
/// where:
///   persistence_score = consistent_events / total_events_in_window
///   noise_resistance = 1.0 - (variance / mean) for inference values
///   decay_factor = exp(-time_since_last_confirmation / half_life)
pub fn compute_temporal_stability(input: &TemporalStabilityInput) -> f64 {
    let persistence_score = if input.total_events_in_window > 0 {
        input.consistent_events as f64 / input.total_events_in_window as f64
    } else {
        0.0
    };

    let noise_resistance = if !input.inference_values.is_empty() {
        let mean = input.inference_values.iter().sum::<f64>() / input.inference_values.len() as f64;
        
        if mean.abs() < 1e-10 {
            0.0 // Avoid division by zero
        } else {
            let variance = input.inference_values.iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>() / input.inference_values.len() as f64;
            
            (1.0 - (variance.sqrt() / mean.abs())).max(0.0)
        }
    } else {
        0.0
    };

    let decay_factor = if input.half_life > 0.0 {
        (-input.time_since_last_confirmation / input.half_life).exp()
    } else {
        1.0
    };

    persistence_score * noise_resistance * decay_factor
}

/// Input data for computing sensitivity stability
#[derive(Debug, Clone)]
pub struct SensitivityStabilityInput {
    pub inference_baseline: f64,
    pub inference_perturbed: Vec<f64>,
}

/// Compute sensitivity stability
///
/// Formula:
/// sensitivity_stability = 1.0 - max(perturbation_impact)
///
/// where:
///   perturbation_impact = |inference_perturbed - inference_baseline| / inference_baseline
pub fn compute_sensitivity_stability(input: &SensitivityStabilityInput) -> f64 {
    if input.inference_perturbed.is_empty() || input.inference_baseline.abs() < 1e-10 {
        return 0.0;
    }

    let max_impact = input.inference_perturbed.iter()
        .map(|&perturbed| {
            (perturbed - input.inference_baseline).abs() / input.inference_baseline.abs()
        })
        .fold(0.0f64, |a, b| a.max(b));

    (1.0 - max_impact).max(0.0)
}

/// Input data for computing regime stability
#[derive(Debug, Clone)]
pub struct RegimeStabilityInput {
    pub incoherent_inferences: usize,
    pub total_inferences_in_regime: usize,
    pub surprise_transitions: usize,
    pub total_transitions: usize,
}

/// Compute regime stability
///
/// Formula:
/// regime_stability = regime_coherence * boundary_predictability
///
/// where:
///   regime_coherence = 1.0 - (incoherent_inferences / total_inferences_in_regime)
///   boundary_predictability = 1.0 - (surprise_transitions / total_transitions)
pub fn compute_regime_stability(input: &RegimeStabilityInput) -> f64 {
    let regime_coherence = if input.total_inferences_in_regime > 0 {
        1.0 - (input.incoherent_inferences as f64 / input.total_inferences_in_regime as f64)
    } else {
        1.0
    };

    let boundary_predictability = if input.total_transitions > 0 {
        1.0 - (input.surprise_transitions as f64 / input.total_transitions as f64)
    } else {
        1.0
    };

    regime_coherence * boundary_predictability
}

/// Compute overall stability using geometric mean
///
/// Formula:
/// overall_stability = (product of component stabilities)^(1/n)
///
/// Rationale: Geometric mean ensures that a single critically low component
/// significantly degrades overall scores.
pub fn compute_overall_stability(components: &StabilityComponents) -> f64 {
    let product = components.temporal * components.sensitivity * components.regime;
    product.powf(1.0 / 3.0) // Cube root for 3 components
}

/// Build complete stability assessment
pub fn assess_stability(
    temporal_input: &TemporalStabilityInput,
    sensitivity_input: &SensitivityStabilityInput,
    regime_input: &RegimeStabilityInput,
) -> StabilityIndicator {
    let components = StabilityComponents {
        temporal: compute_temporal_stability(temporal_input),
        sensitivity: compute_sensitivity_stability(sensitivity_input),
        regime: compute_regime_stability(regime_input),
    };

    let score = compute_overall_stability(&components);
    let level = StabilityLevel::from_score(score);

    StabilityIndicator {
        score,
        components,
        level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_stability_perfect() {
        let input = TemporalStabilityInput {
            consistent_events: 10,
            total_events_in_window: 10,
            inference_values: vec![100.0, 102.0, 101.0, 99.0, 100.5],
            time_since_last_confirmation: 0.0,
            half_life: 300.0,
        };
        
        let stability = compute_temporal_stability(&input);
        assert!(stability > 0.9); // High persistence, low noise, no decay
    }

    #[test]
    fn test_temporal_stability_with_decay() {
        let input = TemporalStabilityInput {
            consistent_events: 10,
            total_events_in_window: 10,
            inference_values: vec![100.0; 5],
            time_since_last_confirmation: 300.0, // One half-life
            half_life: 300.0,
        };
        
        let stability = compute_temporal_stability(&input);
        // persistence=1.0, noise_resistance=1.0, decay=exp(-1)≈0.368
        assert!((stability - 0.368).abs() < 0.01);
    }

    #[test]
    fn test_temporal_stability_noisy() {
        let input = TemporalStabilityInput {
            consistent_events: 5,
            total_events_in_window: 10,
            inference_values: vec![100.0, 50.0, 150.0, 75.0, 125.0],
            time_since_last_confirmation: 0.0,
            half_life: 300.0,
        };
        
        let stability = compute_temporal_stability(&input);
        assert!(stability < 0.5); // Low persistence and high noise
    }

    #[test]
    fn test_sensitivity_stability_robust() {
        let input = SensitivityStabilityInput {
            inference_baseline: 100.0,
            inference_perturbed: vec![101.0, 99.0, 100.5, 99.5],
        };
        
        let stability = compute_sensitivity_stability(&input);
        // Max impact = 1.0/100 = 0.01, so stability = 1 - 0.01 = 0.99
        assert!((stability - 0.99).abs() < 0.01);
    }

    #[test]
    fn test_sensitivity_stability_sensitive() {
        let input = SensitivityStabilityInput {
            inference_baseline: 100.0,
            inference_perturbed: vec![120.0, 80.0, 110.0],
        };
        
        let stability = compute_sensitivity_stability(&input);
        // Max impact = 20.0/100 = 0.20, so stability = 1 - 0.20 = 0.80
        assert!((stability - 0.80).abs() < 0.01);
    }

    #[test]
    fn test_sensitivity_stability_very_sensitive() {
        let input = SensitivityStabilityInput {
            inference_baseline: 100.0,
            inference_perturbed: vec![150.0, 50.0],
        };
        
        let stability = compute_sensitivity_stability(&input);
        // Max impact = 50.0/100 = 0.50, so stability = 1 - 0.50 = 0.50
        assert!((stability - 0.50).abs() < 0.01);
    }

    #[test]
    fn test_regime_stability_perfect() {
        let input = RegimeStabilityInput {
            incoherent_inferences: 0,
            total_inferences_in_regime: 20,
            surprise_transitions: 0,
            total_transitions: 5,
        };
        
        let stability = compute_regime_stability(&input);
        assert!((stability - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_regime_stability_with_surprises() {
        let input = RegimeStabilityInput {
            incoherent_inferences: 2,
            total_inferences_in_regime: 20,
            surprise_transitions: 1,
            total_transitions: 5,
        };
        
        let stability = compute_regime_stability(&input);
        // (1 - 2/20) * (1 - 1/5) = 0.9 * 0.8 = 0.72
        assert!((stability - 0.72).abs() < 0.01);
    }

    #[test]
    fn test_overall_stability_geometric_mean() {
        let components = StabilityComponents {
            temporal: 0.9,
            sensitivity: 0.8,
            regime: 0.7,
        };
        
        let stability = compute_overall_stability(&components);
        // (0.9 * 0.8 * 0.7)^(1/3) = 0.504^(1/3) = 0.796
        assert!((stability - 0.796).abs() < 0.01);
    }

    #[test]
    fn test_overall_stability_degrades_with_low_component() {
        let components = StabilityComponents {
            temporal: 0.2, // Very low
            sensitivity: 0.9,
            regime: 0.9,
        };
        
        let stability = compute_overall_stability(&components);
        // (0.2 * 0.9 * 0.9)^(1/3) = 0.162^(1/3) = 0.545
        assert!(stability < 0.6); // Significantly degraded
    }
}
