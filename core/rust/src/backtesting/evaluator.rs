//! Backtesting Evaluator
//!
//! Performs structural evaluation on recorded snapshots.
//! Focuses on:
//! - Confidence calibration
//! - Regime lag
//! - False pressure detection
//! - Silence correctness
//! - Kill-switch accuracy

use crate::backtesting::types::*;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

/// Evaluates the quality of inferences without outcome awareness
pub struct BacktestEvaluator;

impl BacktestEvaluator {
    /// Compute metrics from a set of snapshots
    pub fn compute_metrics(snapshots: &[InferenceSnapshot]) -> BacktestMetrics {
        let total_snapshots = snapshots.len() as u64;

        let mut failures_by_category: BTreeMap<String, u64> = BTreeMap::new();
        let mut regimes_observed = Vec::new();
        let mut stress_periods = 0;

        // Track regime transitions
        let mut current_regime: Option<String> = None;
        let mut regime_change_count = 0;

        for snapshot in snapshots {
            let regime = &snapshot.regime.classification;

            if let Some(ref prev) = current_regime {
                if prev != regime {
                    regime_change_count += 1;
                }
            }

            if !regimes_observed.contains(regime) {
                regimes_observed.push(regime.clone());
            }

            current_regime = Some(regime.clone());

            // Detect stress periods (kill-switch triggered)
            if snapshot.kill_switch_status.is_triggered() {
                stress_periods += 1;
            }
        }

        // Evaluate confidence calibration
        let (high_confidence_failure_rate, _) =
            Self::evaluate_confidence_calibration(snapshots);

        // Build failure ledger to get categorized failures
        let all_failures = Self::build_failure_ledger(snapshots);

        // Count failures by category
        for failure in &all_failures {
            *failures_by_category
                .entry(failure.category.as_str().to_string())
                .or_insert(0) += 1;
        }

        let total_failures = all_failures.len() as u64;

        // Evaluate false pressure
        let false_pressure_rate = Self::evaluate_false_pressure_rate(snapshots);

        // Evaluate regime lag
        let average_regime_lag_days = Self::evaluate_regime_lag(snapshots);

        // Evaluate silence correctness
        let silence_correctness_rate = Self::evaluate_silence_correctness(snapshots);

        // Evaluate kill-switch anticipation
        let kill_switch_anticipation = Self::evaluate_kill_switch_anticipation(snapshots);

        // Evaluate confidence calibration error
        let confidence_calibration_error =
            Self::evaluate_confidence_calibration_error(snapshots);

        BacktestMetrics {
            total_snapshots,
            total_failures,
            high_confidence_failure_rate,
            false_pressure_rate,
            average_regime_lag_days,
            silence_correctness_rate,
            kill_switch_anticipation,
            confidence_calibration_error,
            failures_by_category,
            regimes_observed,
            stress_periods_detected: stress_periods,
        }
    }

    /// Evaluate confidence calibration
    /// Returns (high_confidence_failure_rate, failure_details)
    fn evaluate_confidence_calibration(
        snapshots: &[InferenceSnapshot],
    ) -> (f64, Vec<ConfidenceFailure>) {
        let mut failures = Vec::new();
        let mut high_confidence_count = 0;
        let mut high_confidence_failures = 0;

        for (idx, snapshot) in snapshots.iter().enumerate() {
            let confidence = snapshot.overall_confidence;

            if confidence > 0.8 {
                high_confidence_count += 1;

                // Check for high-confidence failures
                // A failure occurs when:
                // 1. High confidence during regime transition (lag)
                // 2. High confidence when kill-switch is triggered
                // 3. High confidence when silenced (contradiction)

                let mut is_failure = false;

                if snapshot.kill_switch_status.is_triggered() {
                    is_failure = true;
                }

                if snapshot.silenced && !snapshot.kill_switch_status.is_triggered() {
                    // Silenced but high confidence is contradictory
                    is_failure = true;
                }

                // Check for regime instability nearby
                if idx > 0 && idx < snapshots.len() - 1 {
                    let prev_regime = &snapshots[idx - 1].regime.classification;
                    let next_regime = &snapshots[idx + 1].regime.classification;
                    let current_regime = &snapshot.regime.classification;

                    if prev_regime != current_regime || next_regime != current_regime {
                        is_failure = true;
                    }
                }

                if is_failure {
                    high_confidence_failures += 1;
                    failures.push(ConfidenceFailure {
                        timestamp: snapshot.event_timestamp,
                        event_sequence: snapshot.event_sequence,
                        confidence: confidence,
                        regime: snapshot.regime.classification.clone(),
                    });
                }
            }
        }

        let failure_rate = if high_confidence_count > 0 {
            high_confidence_failures as f64 / high_confidence_count as f64
        } else {
            0.0
        };

        (failure_rate, failures)
    }

    /// Evaluate false pressure rate
    fn evaluate_false_pressure_rate(snapshots: &[InferenceSnapshot]) -> f64 {
        let mut false_positives = 0;
        let mut total_pressure_detections = 0;

        for snapshot in snapshots {
            if snapshot.capital_pressure.detected {
                total_pressure_detections += 1;

                // False positive if:
                // - Pressure detected but market was stable
                // - Pressure detected far from expiry/rebalance
                // - Pressure detected with low confidence in regime
                // Count as false positive if EITHER condition is met (not both)
                if snapshot.regime.confidence < 0.7 || snapshot.stability_indicator > 0.85 {
                    false_positives += 1;
                }
            }
        }

        if total_pressure_detections > 0 {
            false_positives as f64 / total_pressure_detections as f64
        } else {
            0.0
        }
    }

    /// Evaluate regime lag (average days between regime classification and structural break)
    fn evaluate_regime_lag(snapshots: &[InferenceSnapshot]) -> f64 {
        let mut lags = Vec::new();
        let mut current_regime: Option<String> = None;
        let mut regime_start_idx: usize = 0;

        for (idx, snapshot) in snapshots.iter().enumerate() {
            let regime = &snapshot.regime.classification;

            if let Some(ref prev) = current_regime {
                if prev != regime {
                    // Regime changed - estimate the lag
                    // Lag = time since last significant market move to regime change
                    if idx > regime_start_idx {
                        let time_diff = snapshot.event_timestamp
                            - snapshots[regime_start_idx].event_timestamp;
                        let days = time_diff.num_days() as f64;
                        lags.push(days);
                    }
                    regime_start_idx = idx;
                }
            }

            current_regime = Some(regime.clone());
        }

        if lags.is_empty() {
            0.0
        } else {
            lags.iter().sum::<f64>() / lags.len() as f64
        }
    }

    /// Evaluate silence correctness
    /// Returns percentage of times system silenced when it should have
    fn evaluate_silence_correctness(snapshots: &[InferenceSnapshot]) -> f64 {
        let mut correct_silences = 0;
        let mut total_should_silence = 0;

        for snapshot in snapshots {
            // Should silence if:
            // 1. Data quality is poor
            // 2. Kill-switch is triggered
            // 3. Regime conflict exists
            // 4. Confidence is very low

            let should_silence = snapshot.data_quality.quality_score < 0.7
                || snapshot.kill_switch_status.is_triggered()
                || !snapshot.regime.conflicting_signals.is_empty()
                || snapshot.overall_confidence < 0.3;

            if should_silence {
                total_should_silence += 1;

                if snapshot.silenced {
                    correct_silences += 1;
                }
            }
        }

        if total_should_silence > 0 {
            correct_silences as f64 / total_should_silence as f64
        } else {
            1.0
        }
    }

    /// Evaluate kill-switch anticipation
    /// Returns percentage of kill-switch triggers that precede instability
    fn evaluate_kill_switch_anticipation(snapshots: &[InferenceSnapshot]) -> f64 {
        let mut anticipatory = 0;
        let mut total_triggers = 0;

        for (idx, snapshot) in snapshots.iter().enumerate() {
            if snapshot.kill_switch_status.is_triggered() {
                total_triggers += 1;

                // Check if instability followed in next 30 snapshots
                let mut instability_follows = false;

                for future_snapshot in snapshots.iter().skip(idx + 1).take(30) {
                    if future_snapshot.stability_indicator < 0.5 {
                        instability_follows = true;
                        break;
                    }
                }

                if instability_follows {
                    anticipatory += 1;
                }
            }
        }

        if total_triggers > 0 {
            anticipatory as f64 / total_triggers as f64
        } else {
            0.0
        }
    }

    /// Evaluate confidence calibration error
    /// Measures discrepancy between stated confidence and realized stability
    fn evaluate_confidence_calibration_error(snapshots: &[InferenceSnapshot]) -> f64 {
        let mut errors = Vec::new();

        for snapshot in snapshots {
            let stated_confidence = snapshot.overall_confidence;
            let realized_stability = snapshot.stability_indicator;

            let error = (stated_confidence - realized_stability).abs();
            errors.push(error);
        }

        if errors.is_empty() {
            0.0
        } else {
            errors.iter().sum::<f64>() / errors.len() as f64
        }
    }

    /// Build the failure ledger from snapshots
    pub fn build_failure_ledger(snapshots: &[InferenceSnapshot]) -> Vec<BacktestFailure> {
        let mut failures = Vec::new();

        for (idx, snapshot) in snapshots.iter().enumerate() {
            let mut failure_categories = Vec::new();

            // Check for regime lag
            if idx > 0 {
                let prev_regime = &snapshots[idx - 1].regime.classification;
                if prev_regime != &snapshot.regime.classification {
                    if snapshot.overall_confidence > 0.7 {
                        failure_categories.push(BacktestFailureCategory::RegimeLag);
                    }
                }
            }

            // Check for false pressure
            if snapshot.capital_pressure.detected {
                if snapshot.regime.confidence < 0.6 {
                    failure_categories.push(BacktestFailureCategory::FalsePressure);
                }
            }

            // Check for overconfidence
            if snapshot.kill_switch_status.is_triggered()
                && snapshot.overall_confidence > 0.7
            {
                failure_categories.push(BacktestFailureCategory::Overconfidence);
            }

            // Check for silence errors
            if snapshot.data_quality.quality_score < 0.5 && !snapshot.silenced {
                failure_categories.push(BacktestFailureCategory::IncorrectActivation);
            }

            // Check for kill-switch timing
            if snapshot.kill_switch_status.is_triggered() {
                if idx + 30 < snapshots.len() {
                    let mut instability_after = false;
                    for future in snapshots.iter().skip(idx + 1).take(30) {
                        if future.stability_indicator < 0.3 {
                            instability_after = true;
                            break;
                        }
                    }
                    if !instability_after {
                        failure_categories
                            .push(BacktestFailureCategory::KillSwitchEarly);
                    }
                }
            }

            // Create failure records
            for category in failure_categories {
                failures.push(BacktestFailure {
                    timestamp: snapshot.event_timestamp,
                    event_sequence: snapshot.event_sequence,
                    instrument: snapshot.instrument.clone(),
                    regime: snapshot.regime.classification.clone(),
                    confidence_at_failure: OrderedFloat(snapshot.overall_confidence),
                    was_silenced: snapshot.silenced,
                    category,
                    description: format!(
                        "{} in {} regime at seq {}",
                        category.as_str(),
                        snapshot.regime.classification,
                        snapshot.event_sequence
                    ),
                    root_cause: "To be determined".to_string(),
                    corrective_action: None,
                });
            }
        }

        failures
    }
}

/// A confidence failure for analysis
#[derive(Debug, Clone)]
struct ConfidenceFailure {
    timestamp: chrono::DateTime<chrono::Utc>,
    event_sequence: EventSequenceNumber,
    confidence: f64,
    regime: String,
}
