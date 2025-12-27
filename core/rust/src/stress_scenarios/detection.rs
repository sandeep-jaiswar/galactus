//! Stress scenario detection logic
//!
//! This module provides detection capabilities for identifying active stress scenarios
//! based on market data and system state.

use crate::regime::{LiquidityRegime, RegimeState, VolatilityRegime};
use crate::stress_scenarios::scenarios::*;
use crate::stress_scenarios::types::*;

/// Detects liquidity stress scenarios from regime state
pub fn detect_liquidity_stress(regime: &RegimeState) -> Option<StressScenario> {
    match regime.liquidity {
        LiquidityRegime::Illiquid => {
            let params = LiquidityStressParams::new(0.8, 5.0, true, true);
            Some(StressScenario::new(
                StressScenarioCategory::LiquidityStress,
                params.assess_severity(),
                "Illiquid market conditions detected".to_string(),
                regime.timestamp,
                false,
            ))
        }
        LiquidityRegime::Fragile => {
            let params = LiquidityStressParams::new(0.5, 3.0, true, false);
            Some(StressScenario::new(
                StressScenarioCategory::LiquidityStress,
                params.assess_severity(),
                "Fragile liquidity conditions detected".to_string(),
                regime.timestamp,
                false,
            ))
        }
        _ => None,
    }
}

/// Detects volatility shock scenarios from regime state
pub fn detect_volatility_shock(regime: &RegimeState) -> Option<StressScenario> {
    match regime.volatility {
        VolatilityRegime::Dislocated => {
            let params = VolatilityShockParams::new(100.0, true, false, 0.20);
            Some(StressScenario::new(
                StressScenarioCategory::VolatilityShock,
                params.assess_severity(),
                "Dislocated volatility regime detected".to_string(),
                regime.timestamp,
                false,
            ))
        }
        VolatilityRegime::Elevated => {
            let params = VolatilityShockParams::new(50.0, true, true, 0.10);
            Some(StressScenario::new(
                StressScenarioCategory::VolatilityShock,
                params.assess_severity(),
                "Elevated volatility regime detected".to_string(),
                regime.timestamp,
                false,
            ))
        }
        _ => None,
    }
}

/// Stress scenario detector that combines multiple detection methods
pub struct StressScenarioDetector {
    /// Whether to detect liquidity stress
    pub enable_liquidity_detection: bool,
    /// Whether to detect volatility shocks
    pub enable_volatility_detection: bool,
    /// Whether to detect expiry compression
    pub enable_expiry_detection: bool,
    /// Whether to detect data degradation
    pub enable_data_detection: bool,
}

impl StressScenarioDetector {
    /// Creates a new detector with all detection methods enabled
    pub fn new() -> Self {
        Self {
            enable_liquidity_detection: true,
            enable_volatility_detection: true,
            enable_expiry_detection: true,
            enable_data_detection: true,
        }
    }

    /// Creates a detector with custom configuration
    pub fn with_config(
        enable_liquidity: bool,
        enable_volatility: bool,
        enable_expiry: bool,
        enable_data: bool,
    ) -> Self {
        Self {
            enable_liquidity_detection: enable_liquidity,
            enable_volatility_detection: enable_volatility,
            enable_expiry_detection: enable_expiry,
            enable_data_detection: enable_data,
        }
    }

    /// Detects all active stress scenarios from regime state
    pub fn detect_from_regime(&self, regime: &RegimeState) -> Vec<StressScenario> {
        let mut scenarios = Vec::new();

        if self.enable_liquidity_detection {
            if let Some(scenario) = detect_liquidity_stress(regime) {
                scenarios.push(scenario);
            }
        }

        if self.enable_volatility_detection {
            if let Some(scenario) = detect_volatility_shock(regime) {
                scenarios.push(scenario);
            }
        }

        scenarios
    }

    /// Detects stress scenarios based on regime state and returns the most severe
    pub fn detect_most_severe(&self, regime: &RegimeState) -> Option<StressScenario> {
        let scenarios = self.detect_from_regime(regime);
        scenarios
            .into_iter()
            .max_by_key(|s| s.severity)
    }
}

impl Default for StressScenarioDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects expiry compression from time-to-expiry and market parameters
pub fn detect_expiry_compression(
    hours_to_expiry: f64,
    strike_concentration: f64,
    is_gamma_flipping: bool,
    atm_oi: u64,
    timestamp: i64,
) -> Option<StressScenario> {
    if hours_to_expiry < 48.0 {
        let params = ExpiryCompressionParams::new(
            hours_to_expiry,
            strike_concentration,
            is_gamma_flipping,
            atm_oi,
        );

        let description = if params.is_critical_expiry() {
            format!(
                "Critical expiry compression: {:.1}h to expiry, concentration {:.2}x",
                hours_to_expiry, strike_concentration
            )
        } else {
            format!(
                "Expiry compression: {:.1}h to expiry, concentration {:.2}x",
                hours_to_expiry, strike_concentration
            )
        };

        Some(
            StressScenario::new(
                StressScenarioCategory::ExpiryCompression,
                params.assess_severity(),
                description,
                timestamp,
                false,
            )
            .with_tag(if params.is_critical_expiry() {
                "critical_expiry".to_string()
            } else {
                "approaching_expiry".to_string()
            }),
        )
    } else {
        None
    }
}

/// Detects data degradation from data quality metrics
pub fn detect_data_degradation(
    has_delayed_derivatives: bool,
    has_missing_bhavcopy: bool,
    has_schema_change: bool,
    staleness_minutes: u32,
    missing_data_pct: f64,
    timestamp: i64,
) -> Option<StressScenario> {
    if has_schema_change
        || has_missing_bhavcopy
        || has_delayed_derivatives
        || staleness_minutes > 10
        || missing_data_pct > 0.05
    {
        let params = DataDegradationParams::new(
            has_delayed_derivatives,
            has_missing_bhavcopy,
            has_schema_change,
            staleness_minutes,
            missing_data_pct,
        );

        let mut description_parts: Vec<String> = Vec::new();
        if has_schema_change {
            description_parts.push("schema change".to_string());
        }
        if has_missing_bhavcopy {
            description_parts.push("missing bhavcopy".to_string());
        }
        if has_delayed_derivatives {
            description_parts.push("delayed derivatives data".to_string());
        }
        if staleness_minutes > 10 {
            description_parts.push(format!("data stale by {}m", staleness_minutes));
        }
        if missing_data_pct > 0.05 {
            description_parts.push(format!("missing {:.1}% data", missing_data_pct * 100.0));
        }

        let description = format!("Data degradation: {}", description_parts.join(", "));

        Some(
            StressScenario::new(
                StressScenarioCategory::DataDegradation,
                params.assess_severity(),
                description,
                timestamp,
                false,
            )
            .with_tag(if params.must_halt_inference() {
                "halt_inference".to_string()
            } else {
                "degrade_confidence".to_string()
            }),
        )
    } else {
        None
    }
}

/// Detects constraint overlap scenarios
pub fn detect_constraint_overlap(
    active_constraints: Vec<String>,
    are_constraints_offsetting: bool,
    timestamp: i64,
) -> Option<StressScenario> {
    let constraint_count = active_constraints.len();

    if constraint_count >= 2 {
        let params =
            ConstraintOverlapParams::new(constraint_count, are_constraints_offsetting, active_constraints.clone());

        let description = format!(
            "Multiple constraints active: {} ({})",
            active_constraints.join(", "),
            if are_constraints_offsetting {
                "offsetting"
            } else {
                "reinforcing"
            }
        );

        Some(
            StressScenario::new(
                StressScenarioCategory::ConstraintOverlap,
                params.assess_severity(),
                description,
                timestamp,
                false,
            )
            .with_tag(format!("constraint_count_{}", constraint_count)),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regime::{DerivativesDominance, ParticipationRegime, RegimeConfidence, TimeConstraint};

    #[test]
    fn test_detect_liquidity_stress() {
        let illiquid_regime = RegimeState::new(
            LiquidityRegime::Illiquid,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );

        let scenario = detect_liquidity_stress(&illiquid_regime);
        assert!(scenario.is_some());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.category, StressScenarioCategory::LiquidityStress);
        assert!(scenario.is_critical());
    }

    #[test]
    fn test_detect_volatility_shock() {
        let dislocated_regime = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Dislocated,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );

        let scenario = detect_volatility_shock(&dislocated_regime);
        assert!(scenario.is_some());
        let scenario = scenario.unwrap();
        assert_eq!(scenario.category, StressScenarioCategory::VolatilityShock);
        assert!(scenario.is_critical());
    }

    #[test]
    fn test_stress_detector_multiple_scenarios() {
        let stressed_regime = RegimeState::new(
            LiquidityRegime::Fragile,
            VolatilityRegime::Elevated,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.6),
            1000000,
        );

        let detector = StressScenarioDetector::new();
        let scenarios = detector.detect_from_regime(&stressed_regime);

        assert_eq!(scenarios.len(), 2);
        assert!(scenarios
            .iter()
            .any(|s| s.category == StressScenarioCategory::LiquidityStress));
        assert!(scenarios
            .iter()
            .any(|s| s.category == StressScenarioCategory::VolatilityShock));
    }

    #[test]
    fn test_detect_expiry_compression() {
        let critical = detect_expiry_compression(0.5, 3.5, true, 100000, 1000000);
        assert!(critical.is_some());
        let scenario = critical.unwrap();
        assert_eq!(
            scenario.category,
            StressScenarioCategory::ExpiryCompression
        );
        assert!(scenario.tags.contains(&"critical_expiry".to_string()));

        let approaching = detect_expiry_compression(12.0, 1.8, false, 50000, 1000000);
        assert!(approaching.is_some());
        let scenario = approaching.unwrap();
        assert!(scenario.tags.contains(&"approaching_expiry".to_string()));

        let no_stress = detect_expiry_compression(72.0, 1.0, false, 30000, 1000000);
        assert!(no_stress.is_none());
    }

    #[test]
    fn test_detect_data_degradation() {
        let critical = detect_data_degradation(false, true, false, 20, 0.2, 1000000);
        assert!(critical.is_some());
        let scenario = critical.unwrap();
        assert_eq!(
            scenario.category,
            StressScenarioCategory::DataDegradation
        );
        assert!(scenario.tags.contains(&"halt_inference".to_string()));
        assert!(scenario.should_suppress_inference());

        let moderate = detect_data_degradation(true, false, false, 45, 0.3, 1000000);
        assert!(moderate.is_some());

        let no_stress = detect_data_degradation(false, false, false, 5, 0.01, 1000000);
        assert!(no_stress.is_none());
    }

    #[test]
    fn test_detect_constraint_overlap() {
        let severe = detect_constraint_overlap(
            vec![
                "expiry".to_string(),
                "rebalance".to_string(),
                "margin_stress".to_string(),
            ],
            true,
            1000000,
        );
        assert!(severe.is_some());
        let scenario = severe.unwrap();
        assert_eq!(
            scenario.category,
            StressScenarioCategory::ConstraintOverlap
        );
        assert!(scenario.is_critical());

        let no_overlap = detect_constraint_overlap(vec!["expiry".to_string()], false, 1000000);
        assert!(no_overlap.is_none());
    }

    #[test]
    fn test_detector_configuration() {
        let detector = StressScenarioDetector::with_config(true, false, false, false);
        assert!(detector.enable_liquidity_detection);
        assert!(!detector.enable_volatility_detection);

        let regime = RegimeState::new(
            LiquidityRegime::Fragile,
            VolatilityRegime::Elevated,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );

        let scenarios = detector.detect_from_regime(&regime);
        // Only liquidity should be detected
        assert_eq!(scenarios.len(), 1);
        assert_eq!(
            scenarios[0].category,
            StressScenarioCategory::LiquidityStress
        );
    }

    #[test]
    fn test_detect_most_severe() {
        let stressed_regime = RegimeState::new(
            LiquidityRegime::Fragile,
            VolatilityRegime::Dislocated,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.6),
            1000000,
        );

        let detector = StressScenarioDetector::new();
        let most_severe = detector.detect_most_severe(&stressed_regime);

        assert!(most_severe.is_some());
        let scenario = most_severe.unwrap();
        // Dislocated volatility should be more severe than fragile liquidity
        assert_eq!(scenario.category, StressScenarioCategory::VolatilityShock);
        assert_eq!(scenario.severity, StressSeverity::Extreme);
    }
}
