// Galactus Regime Classification Framework
//
// This module implements the regime classification framework as defined in:
// docs/05-intent-engine/regime-classification.md
//
// Regimes describe how the market behaves, not what price does.
// All signals are interpreted conditional on regime.

use std::fmt;

/// Represents the liquidity state of the market
/// 
/// Liquidity regimes determine impact sensitivity - how much a given flow
/// will move the market based on available depth and absorption capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LiquidityRegime {
    /// High liquidity: Deep markets with strong absorption capacity
    High,
    /// Normal liquidity: Standard market conditions
    Normal,
    /// Fragile liquidity: Shallow depth, elevated sensitivity to flows
    Fragile,
    /// Illiquid: Very thin markets, high slippage risk
    Illiquid,
}

impl fmt::Display for LiquidityRegime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiquidityRegime::High => write!(f, "High Liquidity"),
            LiquidityRegime::Normal => write!(f, "Normal Liquidity"),
            LiquidityRegime::Fragile => write!(f, "Fragile Liquidity"),
            LiquidityRegime::Illiquid => write!(f, "Illiquid"),
        }
    }
}

/// Represents the volatility state of the market
/// 
/// Volatility regimes condition pressure amplification - how much a given
/// pressure will be amplified or dampened by current volatility conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VolatilityRegime {
    /// Compressed volatility: Low realized and implied volatility
    Compressed,
    /// Normal volatility: Standard volatility levels
    Normal,
    /// Elevated volatility: Higher than normal volatility
    Elevated,
    /// Dislocated volatility: Extreme volatility, potential market stress
    Dislocated,
}

impl fmt::Display for VolatilityRegime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolatilityRegime::Compressed => write!(f, "Compressed Volatility"),
            VolatilityRegime::Normal => write!(f, "Normal Volatility"),
            VolatilityRegime::Elevated => write!(f, "Elevated Volatility"),
            VolatilityRegime::Dislocated => write!(f, "Dislocated Volatility"),
        }
    }
}

/// Represents the relative dominance of derivatives vs cash markets
/// 
/// This regime determines where constraints originate - whether market
/// behavior is driven by derivatives positioning or cash flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DerivativesDominance {
    /// Derivatives-dominant: Options/futures positioning drives behavior
    DerivativesDominant,
    /// Mixed: Both derivatives and cash influence market
    Mixed,
    /// Cash-dominant: Spot market flows are primary driver
    CashDominant,
}

impl fmt::Display for DerivativesDominance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DerivativesDominance::DerivativesDominant => write!(f, "Derivatives-Dominant"),
            DerivativesDominance::Mixed => write!(f, "Mixed"),
            DerivativesDominance::CashDominant => write!(f, "Cash-Dominant"),
        }
    }
}

/// Represents proximity to time-sensitive events
/// 
/// Time constraint regimes control urgency - how much pressure exists
/// to act before optionality expires or deadlines are reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeConstraint {
    /// Far from deadlines: No immediate time pressure
    FarFromDeadlines,
    /// Approaching deadlines: Time pressure building
    ApproachingDeadlines,
    /// Immediate deadlines: At or near expiry/settlement
    ImmediateDeadlines,
}

impl fmt::Display for TimeConstraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeConstraint::FarFromDeadlines => write!(f, "Far from Deadlines"),
            TimeConstraint::ApproachingDeadlines => write!(f, "Approaching Deadlines"),
            TimeConstraint::ImmediateDeadlines => write!(f, "Immediate Deadlines"),
        }
    }
}

/// Represents the composition of market participants
/// 
/// Participation regimes influence feedback loops - whether flows are
/// likely to be amplified (retail-heavy) or absorbed (institutional-heavy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticipationRegime {
    /// Retail-dominant: High retail participation, amplification risk
    RetailDominant,
    /// Balanced: Mix of retail and institutional
    Balanced,
    /// Institutional-dominant: Institutional absorption, lower amplification
    InstitutionalDominant,
}

impl fmt::Display for ParticipationRegime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParticipationRegime::RetailDominant => write!(f, "Retail-Dominant"),
            ParticipationRegime::Balanced => write!(f, "Balanced"),
            ParticipationRegime::InstitutionalDominant => write!(f, "Institutional-Dominant"),
        }
    }
}

/// Confidence score for regime classification
/// 
/// Represents how certain we are about the current regime classification.
/// Uncertainty degrades downstream inference confidence.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RegimeConfidence {
    /// Confidence score between 0.0 and 1.0
    score: f64,
}

impl RegimeConfidence {
    /// Creates a new confidence score
    /// 
    /// # Panics
    /// Panics if score is not in [0.0, 1.0]
    pub fn new(score: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&score),
            "Confidence score must be between 0.0 and 1.0"
        );
        Self { score }
    }

    /// Returns the confidence score
    pub fn score(&self) -> f64 {
        self.score
    }

    /// Returns true if confidence is high (>= 0.8)
    pub fn is_high(&self) -> bool {
        self.score >= 0.8
    }

    /// Returns true if confidence is uncertain (< 0.4)
    pub fn is_uncertain(&self) -> bool {
        self.score < 0.4
    }

    /// Returns true if confidence is ambiguous (0.4-0.7 range)
    /// Ambiguous confidence should be treated cautiously but not rejected
    pub fn is_ambiguous(&self) -> bool {
        self.score >= 0.4 && self.score < 0.7
    }
}

impl fmt::Display for RegimeConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.score * 100.0)
    }
}

/// Complete regime state at a point in time
/// 
/// Represents the full market regime configuration across all dimensions.
/// All signals are interpreted conditional on this state.
#[derive(Debug, Clone, PartialEq)]
pub struct RegimeState {
    /// Current liquidity regime
    pub liquidity: LiquidityRegime,
    /// Current volatility regime
    pub volatility: VolatilityRegime,
    /// Current derivatives dominance
    pub derivatives: DerivativesDominance,
    /// Current time constraint
    pub time_constraint: TimeConstraint,
    /// Current participation regime
    pub participation: ParticipationRegime,
    /// Confidence in this classification
    pub confidence: RegimeConfidence,
    /// Unix timestamp (seconds) when this state was determined
    pub timestamp: i64,
}

impl RegimeState {
    /// Creates a new regime state with all dimensions specified
    pub fn new(
        liquidity: LiquidityRegime,
        volatility: VolatilityRegime,
        derivatives: DerivativesDominance,
        time_constraint: TimeConstraint,
        participation: ParticipationRegime,
        confidence: RegimeConfidence,
        timestamp: i64,
    ) -> Self {
        Self {
            liquidity,
            volatility,
            derivatives,
            time_constraint,
            participation,
            confidence,
            timestamp,
        }
    }

    /// Returns true if any regime dimension indicates stress conditions
    pub fn is_stressed(&self) -> bool {
        matches!(self.liquidity, LiquidityRegime::Fragile | LiquidityRegime::Illiquid)
            || matches!(
                self.volatility,
                VolatilityRegime::Elevated | VolatilityRegime::Dislocated
            )
            || matches!(self.time_constraint, TimeConstraint::ImmediateDeadlines)
    }

    /// Returns true if confidence is insufficient for reliable inference
    pub fn is_confidence_degraded(&self) -> bool {
        self.confidence.is_uncertain()
    }
}

impl fmt::Display for RegimeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Regime State:")?;
        writeln!(f, "  Liquidity: {}", self.liquidity)?;
        writeln!(f, "  Volatility: {}", self.volatility)?;
        writeln!(f, "  Derivatives: {}", self.derivatives)?;
        writeln!(f, "  Time Constraint: {}", self.time_constraint)?;
        writeln!(f, "  Participation: {}", self.participation)?;
        writeln!(f, "  Confidence: {}", self.confidence)?;
        write!(f, "  Timestamp: {}", self.timestamp)
    }
}

/// Represents a transition from one regime to another
/// 
/// Regime transitions are critical events that must be:
/// - Explicitly detected
/// - Logged
/// - Reflected in confidence
/// 
/// Silent regime shifts are a critical failure mode.
#[derive(Debug, Clone, PartialEq)]
pub struct RegimeTransition {
    /// The previous regime state
    pub from: RegimeState,
    /// The new regime state
    pub to: RegimeState,
    /// Type of transition (gradual or abrupt)
    pub transition_type: TransitionType,
    /// Description of what triggered the transition
    pub trigger: String,
}

impl RegimeTransition {
    /// Creates a new regime transition
    pub fn new(
        from: RegimeState,
        to: RegimeState,
        transition_type: TransitionType,
        trigger: String,
    ) -> Self {
        Self {
            from,
            to,
            transition_type,
            trigger,
        }
    }

    /// Returns true if this was a major regime shift (multiple dimensions changed)
    pub fn is_major_shift(&self) -> bool {
        let mut changed_count = 0;

        if self.from.liquidity != self.to.liquidity {
            changed_count += 1;
        }
        if self.from.volatility != self.to.volatility {
            changed_count += 1;
        }
        if self.from.derivatives != self.to.derivatives {
            changed_count += 1;
        }
        if self.from.time_constraint != self.to.time_constraint {
            changed_count += 1;
        }
        if self.from.participation != self.to.participation {
            changed_count += 1;
        }

        changed_count >= 2
    }
}

impl fmt::Display for RegimeTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Regime Transition ({:?}):", self.transition_type)?;
        writeln!(f, "  Trigger: {}", self.trigger)?;
        writeln!(f, "  From: {} @ {}", self.format_regime_summary(&self.from), self.from.timestamp)?;
        write!(f, "  To:   {} @ {}", self.format_regime_summary(&self.to), self.to.timestamp)
    }
}

impl RegimeTransition {
    fn format_regime_summary(&self, state: &RegimeState) -> String {
        format!(
            "L:{:?} V:{:?} D:{:?} T:{:?} P:{:?}",
            state.liquidity, state.volatility, state.derivatives, 
            state.time_constraint, state.participation
        )
    }
}

/// Type of regime transition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    /// Gradual transition over time
    Gradual,
    /// Abrupt transition (event-driven)
    Abrupt,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liquidity_regime_display() {
        assert_eq!(LiquidityRegime::High.to_string(), "High Liquidity");
        assert_eq!(LiquidityRegime::Normal.to_string(), "Normal Liquidity");
        assert_eq!(LiquidityRegime::Fragile.to_string(), "Fragile Liquidity");
        assert_eq!(LiquidityRegime::Illiquid.to_string(), "Illiquid");
    }

    #[test]
    fn test_volatility_regime_display() {
        assert_eq!(VolatilityRegime::Compressed.to_string(), "Compressed Volatility");
        assert_eq!(VolatilityRegime::Normal.to_string(), "Normal Volatility");
        assert_eq!(VolatilityRegime::Elevated.to_string(), "Elevated Volatility");
        assert_eq!(VolatilityRegime::Dislocated.to_string(), "Dislocated Volatility");
    }

    #[test]
    fn test_regime_confidence() {
        let high = RegimeConfidence::new(0.9);
        assert!(high.is_high());
        assert!(!high.is_uncertain());
        assert!(!high.is_ambiguous());

        let uncertain = RegimeConfidence::new(0.3);
        assert!(!uncertain.is_high());
        assert!(uncertain.is_uncertain());
        assert!(!uncertain.is_ambiguous());

        let ambiguous = RegimeConfidence::new(0.5);
        assert!(!ambiguous.is_high());
        assert!(!ambiguous.is_uncertain());
        assert!(ambiguous.is_ambiguous());

        // Edge cases
        let boundary_low = RegimeConfidence::new(0.4);
        assert!(!boundary_low.is_uncertain());
        assert!(boundary_low.is_ambiguous());

        let boundary_high = RegimeConfidence::new(0.7);
        assert!(!boundary_high.is_ambiguous());
        assert!(!boundary_high.is_high());
    }

    #[test]
    #[should_panic(expected = "Confidence score must be between 0.0 and 1.0")]
    fn test_confidence_out_of_range() {
        RegimeConfidence::new(1.5);
    }

    #[test]
    fn test_regime_state_stress_detection() {
        let stressed_liquidity = RegimeState::new(
            LiquidityRegime::Fragile,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );
        assert!(stressed_liquidity.is_stressed());

        let stressed_volatility = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Dislocated,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );
        assert!(stressed_volatility.is_stressed());

        let stressed_time = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::ImmediateDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );
        assert!(stressed_time.is_stressed());

        let normal = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );
        assert!(!normal.is_stressed());
    }

    #[test]
    fn test_regime_state_confidence_degradation() {
        let degraded = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.3),
            1000000,
        );
        assert!(degraded.is_confidence_degraded());

        let confident = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.9),
            1000000,
        );
        assert!(!confident.is_confidence_degraded());
    }

    #[test]
    fn test_regime_transition_major_shift_detection() {
        let from = RegimeState::new(
            LiquidityRegime::High,
            VolatilityRegime::Compressed,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );

        // Single dimension change - not major
        let to_minor = RegimeState::new(
            LiquidityRegime::Normal,
            VolatilityRegime::Compressed,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000100,
        );

        let minor_transition = RegimeTransition::new(
            from.clone(),
            to_minor,
            TransitionType::Gradual,
            "Minor adjustment".to_string(),
        );
        assert!(!minor_transition.is_major_shift());

        // Multiple dimension change - major
        let to_major = RegimeState::new(
            LiquidityRegime::Fragile,
            VolatilityRegime::Elevated,
            DerivativesDominance::DerivativesDominant,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.6),
            1000100,
        );

        let major_transition = RegimeTransition::new(
            from,
            to_major,
            TransitionType::Abrupt,
            "Market stress event".to_string(),
        );
        assert!(major_transition.is_major_shift());
    }

    #[test]
    fn test_regime_equality() {
        let regime1 = RegimeState::new(
            LiquidityRegime::High,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );

        let regime2 = RegimeState::new(
            LiquidityRegime::High,
            VolatilityRegime::Normal,
            DerivativesDominance::Mixed,
            TimeConstraint::FarFromDeadlines,
            ParticipationRegime::Balanced,
            RegimeConfidence::new(0.8),
            1000000,
        );

        assert_eq!(regime1, regime2);
    }

    #[test]
    fn test_transition_types() {
        assert_eq!(TransitionType::Gradual, TransitionType::Gradual);
        assert_eq!(TransitionType::Abrupt, TransitionType::Abrupt);
        assert_ne!(TransitionType::Gradual, TransitionType::Abrupt);
    }
}
