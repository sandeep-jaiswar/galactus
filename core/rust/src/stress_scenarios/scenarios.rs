//! Specific stress scenario implementations
//!
//! This module provides concrete implementations of each stress scenario category
//! with their specific characteristics and detection criteria.

use crate::stress_scenarios::types::*;

/// Liquidity stress scenario parameters
#[derive(Debug, Clone, PartialEq)]
pub struct LiquidityStressParams {
    /// Volume drop percentage (0.0 to 1.0)
    pub volume_drop_pct: f64,
    /// Bid-ask spread widening factor
    pub spread_widening_factor: f64,
    /// Whether liquidity cliff exists around key strikes
    pub has_liquidity_cliff: bool,
    /// Whether order flow is one-sided
    pub is_one_sided_flow: bool,
}

impl LiquidityStressParams {
    /// Creates liquidity stress parameters
    pub fn new(
        volume_drop_pct: f64,
        spread_widening_factor: f64,
        has_liquidity_cliff: bool,
        is_one_sided_flow: bool,
    ) -> Self {
        Self {
            volume_drop_pct,
            spread_widening_factor,
            has_liquidity_cliff,
            is_one_sided_flow,
        }
    }

    /// Determines severity based on parameters
    pub fn assess_severity(&self) -> StressSeverity {
        if self.volume_drop_pct > 0.7 || self.spread_widening_factor > 5.0 {
            StressSeverity::Extreme
        } else if self.volume_drop_pct > 0.5 || self.spread_widening_factor > 3.0 {
            StressSeverity::Severe
        } else if self.volume_drop_pct > 0.3 || self.spread_widening_factor > 2.0 {
            StressSeverity::Moderate
        } else {
            StressSeverity::Mild
        }
    }
}

/// Expiry compression scenario parameters
#[derive(Debug, Clone, PartialEq)]
pub struct ExpiryCompressionParams {
    /// Hours until expiry
    pub hours_to_expiry: f64,
    /// Strike concentration factor (1.0 = normal, >2.0 = highly concentrated)
    pub strike_concentration: f64,
    /// Whether gamma regime is flipping rapidly
    pub is_gamma_flipping: bool,
    /// Open interest at near-the-money strikes
    pub atm_open_interest: u64,
}

impl ExpiryCompressionParams {
    /// Creates expiry compression parameters
    pub fn new(
        hours_to_expiry: f64,
        strike_concentration: f64,
        is_gamma_flipping: bool,
        atm_open_interest: u64,
    ) -> Self {
        Self {
            hours_to_expiry,
            strike_concentration,
            is_gamma_flipping,
            atm_open_interest,
        }
    }

    /// Determines severity based on parameters
    pub fn assess_severity(&self) -> StressSeverity {
        if self.hours_to_expiry < 1.0 || (self.is_gamma_flipping && self.strike_concentration > 3.0)
        {
            StressSeverity::Extreme
        } else if self.hours_to_expiry < 4.0 || self.strike_concentration > 2.5 {
            StressSeverity::Severe
        } else if self.hours_to_expiry < 24.0 || self.strike_concentration > 1.5 {
            StressSeverity::Moderate
        } else {
            StressSeverity::Mild
        }
    }

    /// Returns true if this is a last-day or last-hour scenario
    pub fn is_critical_expiry(&self) -> bool {
        self.hours_to_expiry < 4.0
    }
}

/// Volatility shock scenario parameters
#[derive(Debug, Clone, PartialEq)]
pub struct VolatilityShockParams {
    /// Volatility change percentage (can be positive or negative)
    pub volatility_change_pct: f64,
    /// Whether the shock was sudden (within single session)
    pub is_sudden: bool,
    /// Whether there's proportional liquidity to handle the volatility
    pub has_proportional_liquidity: bool,
    /// Implied vs realized volatility spread
    pub iv_rv_spread: f64,
}

impl VolatilityShockParams {
    /// Creates volatility shock parameters
    pub fn new(
        volatility_change_pct: f64,
        is_sudden: bool,
        has_proportional_liquidity: bool,
        iv_rv_spread: f64,
    ) -> Self {
        Self {
            volatility_change_pct,
            is_sudden,
            has_proportional_liquidity,
            iv_rv_spread,
        }
    }

    /// Determines severity based on parameters
    pub fn assess_severity(&self) -> StressSeverity {
        let vol_abs = self.volatility_change_pct.abs();

        if vol_abs > 100.0 || (vol_abs > 50.0 && !self.has_proportional_liquidity) {
            StressSeverity::Extreme
        } else if vol_abs > 50.0 || (vol_abs > 30.0 && self.is_sudden) {
            StressSeverity::Severe
        } else if vol_abs > 30.0 || (vol_abs > 15.0 && self.is_sudden) {
            StressSeverity::Moderate
        } else {
            StressSeverity::Mild
        }
    }

    /// Returns true if this is a volatility expansion shock
    pub fn is_expansion(&self) -> bool {
        self.volatility_change_pct > 0.0
    }
}

/// Data degradation scenario parameters
#[derive(Debug, Clone, PartialEq)]
pub struct DataDegradationParams {
    /// Whether derivatives data is delayed
    pub has_delayed_derivatives: bool,
    /// Whether bhavcopy entries are missing
    pub has_missing_bhavcopy: bool,
    /// Whether schema has changed mid-period
    pub has_schema_change: bool,
    /// Data staleness in minutes
    pub staleness_minutes: u32,
    /// Percentage of missing data points (0.0 to 1.0)
    pub missing_data_pct: f64,
}

impl DataDegradationParams {
    /// Creates data degradation parameters
    pub fn new(
        has_delayed_derivatives: bool,
        has_missing_bhavcopy: bool,
        has_schema_change: bool,
        staleness_minutes: u32,
        missing_data_pct: f64,
    ) -> Self {
        Self {
            has_delayed_derivatives,
            has_missing_bhavcopy,
            has_schema_change,
            staleness_minutes,
            missing_data_pct,
        }
    }

    /// Determines severity based on parameters
    pub fn assess_severity(&self) -> StressSeverity {
        // Schema changes or critical missing data = extreme
        if self.has_schema_change || self.has_missing_bhavcopy {
            StressSeverity::Extreme
        } else if self.staleness_minutes > 60 || self.missing_data_pct > 0.5 {
            StressSeverity::Severe
        } else if self.staleness_minutes > 30 || self.missing_data_pct > 0.2 {
            StressSeverity::Moderate
        } else {
            StressSeverity::Mild
        }
    }

    /// Returns true if inference must halt immediately
    pub fn must_halt_inference(&self) -> bool {
        self.has_schema_change || self.has_missing_bhavcopy || self.missing_data_pct > 0.5
    }
}

/// Constraint overlap scenario parameters
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintOverlapParams {
    /// Number of simultaneous binding constraints
    pub constraint_count: usize,
    /// Whether constraints are offsetting or reinforcing
    pub are_constraints_offsetting: bool,
    /// List of active constraint types
    pub active_constraints: Vec<String>,
}

impl ConstraintOverlapParams {
    /// Creates constraint overlap parameters
    pub fn new(
        constraint_count: usize,
        are_constraints_offsetting: bool,
        active_constraints: Vec<String>,
    ) -> Self {
        Self {
            constraint_count,
            are_constraints_offsetting,
            active_constraints,
        }
    }

    /// Determines severity based on parameters
    pub fn assess_severity(&self) -> StressSeverity {
        if self.constraint_count >= 4 {
            StressSeverity::Extreme
        } else if self.constraint_count >= 3 {
            StressSeverity::Severe
        } else if self.constraint_count >= 2 {
            StressSeverity::Moderate
        } else {
            StressSeverity::Mild
        }
    }

    /// Returns true if constraint aggregation is interpretable
    pub fn is_aggregation_interpretable(&self) -> bool {
        // With offsetting constraints, interpretation becomes harder
        self.constraint_count <= 2 || !self.are_constraints_offsetting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidity_stress_severity() {
        let extreme = LiquidityStressParams::new(0.8, 6.0, true, true);
        assert_eq!(extreme.assess_severity(), StressSeverity::Extreme);

        let severe = LiquidityStressParams::new(0.6, 3.5, false, true);
        assert_eq!(severe.assess_severity(), StressSeverity::Severe);

        let moderate = LiquidityStressParams::new(0.4, 2.5, false, false);
        assert_eq!(moderate.assess_severity(), StressSeverity::Moderate);

        let mild = LiquidityStressParams::new(0.2, 1.5, false, false);
        assert_eq!(mild.assess_severity(), StressSeverity::Mild);
    }

    #[test]
    fn test_expiry_compression_severity() {
        let extreme = ExpiryCompressionParams::new(0.5, 3.5, true, 100000);
        assert_eq!(extreme.assess_severity(), StressSeverity::Extreme);
        assert!(extreme.is_critical_expiry());

        let severe = ExpiryCompressionParams::new(2.0, 2.8, false, 80000);
        assert_eq!(severe.assess_severity(), StressSeverity::Severe);
        assert!(severe.is_critical_expiry());

        let moderate = ExpiryCompressionParams::new(12.0, 1.8, false, 50000);
        assert_eq!(moderate.assess_severity(), StressSeverity::Moderate);
        assert!(!moderate.is_critical_expiry());
    }

    #[test]
    fn test_volatility_shock_severity() {
        let extreme = VolatilityShockParams::new(120.0, true, false, 0.15);
        assert_eq!(extreme.assess_severity(), StressSeverity::Extreme);
        assert!(extreme.is_expansion());

        let severe = VolatilityShockParams::new(-60.0, true, true, 0.10);
        assert_eq!(severe.assess_severity(), StressSeverity::Severe);
        assert!(!severe.is_expansion());

        let moderate = VolatilityShockParams::new(35.0, false, true, 0.05);
        assert_eq!(moderate.assess_severity(), StressSeverity::Moderate);
    }

    #[test]
    fn test_data_degradation_severity() {
        let extreme_schema = DataDegradationParams::new(false, false, true, 10, 0.1);
        assert_eq!(extreme_schema.assess_severity(), StressSeverity::Extreme);
        assert!(extreme_schema.must_halt_inference());

        let extreme_bhavcopy = DataDegradationParams::new(false, true, false, 20, 0.2);
        assert_eq!(extreme_bhavcopy.assess_severity(), StressSeverity::Extreme);
        assert!(extreme_bhavcopy.must_halt_inference());

        let severe = DataDegradationParams::new(true, false, false, 90, 0.6);
        assert_eq!(severe.assess_severity(), StressSeverity::Severe);
        assert!(severe.must_halt_inference());

        let moderate = DataDegradationParams::new(true, false, false, 45, 0.3);
        assert_eq!(moderate.assess_severity(), StressSeverity::Moderate);
        assert!(!moderate.must_halt_inference());
    }

    #[test]
    fn test_constraint_overlap_severity() {
        let extreme = ConstraintOverlapParams::new(
            4,
            true,
            vec![
                "expiry".to_string(),
                "rebalance".to_string(),
                "margin_stress".to_string(),
                "settlement".to_string(),
            ],
        );
        assert_eq!(extreme.assess_severity(), StressSeverity::Extreme);
        assert!(!extreme.is_aggregation_interpretable());

        let severe = ConstraintOverlapParams::new(
            3,
            false,
            vec![
                "expiry".to_string(),
                "rebalance".to_string(),
                "volatility_spike".to_string(),
            ],
        );
        assert_eq!(severe.assess_severity(), StressSeverity::Severe);
        assert!(severe.is_aggregation_interpretable());

        let moderate = ConstraintOverlapParams::new(
            2,
            false,
            vec!["expiry".to_string(), "settlement".to_string()],
        );
        assert_eq!(moderate.assess_severity(), StressSeverity::Moderate);
        assert!(moderate.is_aggregation_interpretable());
    }
}
