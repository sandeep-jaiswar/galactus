//! Core types for stress scenario framework

use std::fmt;

/// Stress scenario category as defined in documentation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StressScenarioCategory {
    /// Liquidity collapse or uneven market absorption
    LiquidityStress,
    /// Time constraints near expiries dominating behavior
    ExpiryCompression,
    /// Sudden volatility expansion or contraction
    VolatilityShock,
    /// Market shifting from one structural regime to another
    RegimeTransition,
    /// Multiple binding constraints acting simultaneously
    ConstraintOverlap,
    /// Compromised data quality scenarios
    DataDegradation,
}

impl fmt::Display for StressScenarioCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StressScenarioCategory::LiquidityStress => write!(f, "Liquidity Stress"),
            StressScenarioCategory::ExpiryCompression => write!(f, "Expiry Compression"),
            StressScenarioCategory::VolatilityShock => write!(f, "Volatility Shock"),
            StressScenarioCategory::RegimeTransition => write!(f, "Regime Transition"),
            StressScenarioCategory::ConstraintOverlap => write!(f, "Constraint Overlap"),
            StressScenarioCategory::DataDegradation => write!(f, "Data Degradation"),
        }
    }
}

/// Severity level of a stress scenario
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StressSeverity {
    /// Mild stress - slight deviation from normal
    Mild,
    /// Moderate stress - noticeable structural pressure
    Moderate,
    /// Severe stress - significant structural disruption
    Severe,
    /// Extreme stress - critical market conditions
    Extreme,
}

impl fmt::Display for StressSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StressSeverity::Mild => write!(f, "Mild"),
            StressSeverity::Moderate => write!(f, "Moderate"),
            StressSeverity::Severe => write!(f, "Severe"),
            StressSeverity::Extreme => write!(f, "Extreme"),
        }
    }
}

/// Complete stress scenario descriptor
#[derive(Debug, Clone, PartialEq)]
pub struct StressScenario {
    /// Category of the stress scenario
    pub category: StressScenarioCategory,
    /// Severity level
    pub severity: StressSeverity,
    /// Human-readable description of the scenario
    pub description: String,
    /// Unix timestamp (seconds) when scenario was detected
    pub timestamp: i64,
    /// Whether this is a synthetic (constructed) or real-world scenario
    pub is_synthetic: bool,
    /// Optional tags for additional categorization
    pub tags: Vec<String>,
}

impl StressScenario {
    /// Creates a new stress scenario
    pub fn new(
        category: StressScenarioCategory,
        severity: StressSeverity,
        description: String,
        timestamp: i64,
        is_synthetic: bool,
    ) -> Self {
        Self {
            category,
            severity,
            description,
            timestamp,
            is_synthetic,
            tags: Vec::new(),
        }
    }

    /// Adds a tag to the scenario
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    /// Returns true if this is a critical stress scenario
    pub fn is_critical(&self) -> bool {
        matches!(
            self.severity,
            StressSeverity::Severe | StressSeverity::Extreme
        )
    }

    /// Returns true if inference should be suppressed under this scenario
    pub fn should_suppress_inference(&self) -> bool {
        // Extreme scenarios always suppress inference
        // Severe data degradation always suppresses inference
        self.severity == StressSeverity::Extreme
            || (self.category == StressScenarioCategory::DataDegradation
                && self.severity >= StressSeverity::Severe)
    }
}

impl fmt::Display for StressScenario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Stress Scenario:")?;
        writeln!(f, "  Category: {}", self.category)?;
        writeln!(f, "  Severity: {}", self.severity)?;
        writeln!(f, "  Description: {}", self.description)?;
        writeln!(f, "  Timestamp: {}", self.timestamp)?;
        writeln!(
            f,
            "  Type: {}",
            if self.is_synthetic {
                "Synthetic"
            } else {
                "Real-world"
            }
        )?;
        if !self.tags.is_empty() {
            write!(f, "  Tags: {}", self.tags.join(", "))?;
        }
        Ok(())
    }
}

/// Expected system behavior under stress
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedStressBehavior {
    /// Confidence should degrade sharply
    ConfidenceDegradation,
    /// Signal activation should be suppressed
    SignalSuppression,
    /// System should output "no meaningful inference"
    NoMeaningfulInference,
    /// System should explicitly fail closed
    ExplicitFailure,
}

impl fmt::Display for ExpectedStressBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpectedStressBehavior::ConfidenceDegradation => {
                write!(f, "Confidence Degradation Expected")
            }
            ExpectedStressBehavior::SignalSuppression => write!(f, "Signal Suppression Expected"),
            ExpectedStressBehavior::NoMeaningfulInference => {
                write!(f, "No Meaningful Inference Expected")
            }
            ExpectedStressBehavior::ExplicitFailure => write!(f, "Explicit Failure Expected"),
        }
    }
}

/// Unacceptable behavior under stress (failure modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnacceptableStressBehavior {
    /// Maintaining high confidence during stress
    HighConfidenceMaintained,
    /// Producing confident directional inference during stress
    ConfidentDirectionalInference,
    /// Masking uncertainty with smoothing
    UncertaintyMasking,
    /// Silent regime transitions without detection
    SilentRegimeTransition,
}

impl fmt::Display for UnacceptableStressBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnacceptableStressBehavior::HighConfidenceMaintained => {
                write!(f, "High Confidence Maintained (UNACCEPTABLE)")
            }
            UnacceptableStressBehavior::ConfidentDirectionalInference => {
                write!(f, "Confident Directional Inference (UNACCEPTABLE)")
            }
            UnacceptableStressBehavior::UncertaintyMasking => {
                write!(f, "Uncertainty Masking (UNACCEPTABLE)")
            }
            UnacceptableStressBehavior::SilentRegimeTransition => {
                write!(f, "Silent Regime Transition (UNACCEPTABLE)")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_category_display() {
        assert_eq!(
            StressScenarioCategory::LiquidityStress.to_string(),
            "Liquidity Stress"
        );
        assert_eq!(
            StressScenarioCategory::ExpiryCompression.to_string(),
            "Expiry Compression"
        );
        assert_eq!(
            StressScenarioCategory::VolatilityShock.to_string(),
            "Volatility Shock"
        );
    }

    #[test]
    fn test_stress_severity_ordering() {
        assert!(StressSeverity::Mild < StressSeverity::Moderate);
        assert!(StressSeverity::Moderate < StressSeverity::Severe);
        assert!(StressSeverity::Severe < StressSeverity::Extreme);
    }

    #[test]
    fn test_stress_scenario_creation() {
        let scenario = StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Severe,
            "Sudden volume drop in mid-cap stocks".to_string(),
            1000000,
            false,
        );

        assert_eq!(scenario.category, StressScenarioCategory::LiquidityStress);
        assert_eq!(scenario.severity, StressSeverity::Severe);
        assert!(scenario.is_critical());
        assert!(!scenario.is_synthetic);
    }

    #[test]
    fn test_stress_scenario_with_tags() {
        let scenario = StressScenario::new(
            StressScenarioCategory::ExpiryCompression,
            StressSeverity::Moderate,
            "Last hour expiry".to_string(),
            1000000,
            false,
        )
        .with_tag("weekly_expiry".to_string())
        .with_tag("high_oi".to_string());

        assert_eq!(scenario.tags.len(), 2);
        assert!(scenario.tags.contains(&"weekly_expiry".to_string()));
    }

    #[test]
    fn test_scenario_inference_suppression() {
        // Extreme scenarios suppress inference
        let extreme = StressScenario::new(
            StressScenarioCategory::VolatilityShock,
            StressSeverity::Extreme,
            "Market crash".to_string(),
            1000000,
            false,
        );
        assert!(extreme.should_suppress_inference());

        // Severe data degradation suppresses inference
        let data_severe = StressScenario::new(
            StressScenarioCategory::DataDegradation,
            StressSeverity::Severe,
            "Missing bhavcopy".to_string(),
            1000000,
            false,
        );
        assert!(data_severe.should_suppress_inference());

        // Moderate scenarios don't necessarily suppress
        let moderate = StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Moderate,
            "Reduced volume".to_string(),
            1000000,
            false,
        );
        assert!(!moderate.should_suppress_inference());
    }

    #[test]
    fn test_critical_detection() {
        let severe = StressScenario::new(
            StressScenarioCategory::ConstraintOverlap,
            StressSeverity::Severe,
            "Expiry + rebalance".to_string(),
            1000000,
            false,
        );
        assert!(severe.is_critical());

        let mild = StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Mild,
            "Slight volume drop".to_string(),
            1000000,
            false,
        );
        assert!(!mild.is_critical());
    }
}
