//! Kill Switch Module
//!
//! Implements non-negotiable conditions under which inference must halt or suppress output.
//! These conditions are defined in: docs/08-risk-and-failure-modes/kill-switch-criteria.md
//!
//! # Purpose
//!
//! The kill switch exists to:
//! - Prevent false confidence
//! - Protect downstream consumers
//! - Preserve trust in inference
//!
//! Incorrect silence is preferable to confident error.
//!
//! # Kill Switch Philosophy
//!
//! A kill switch is an automatic mechanism that:
//! - Suppresses inference output when fundamental conditions are violated
//! - Freezes signal activation when data integrity fails
//! - Forces the system into a safe state (silence)
//!
//! # Design Principles
//!
//! 1. **Fail-safe**: Default to silence when uncertain
//! 2. **Transparent**: All kill conditions are logged with clear reasons
//! 3. **Deterministic**: Same inputs always trigger same kill decisions
//! 4. **Conservative**: Err on the side of caution

use std::collections::HashMap;
use crate::confidence::types::{OverallConfidence, StabilityIndicator};

/// Represents a kill switch condition that has been triggered
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KillCondition {
    /// The type of kill condition triggered
    pub condition_type: KillConditionType,
    
    /// Human-readable description of why this condition was triggered
    pub description: String,
    
    /// Severity level of this condition
    pub severity: KillSeverity,
    
    /// Additional context for debugging
    pub context: HashMap<String, String>,
}

/// Types of kill switch conditions as defined in the documentation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillConditionType {
    /// Data integrity failure - corrupted or inconsistent canonical events
    DataIntegrityFailure,
    
    /// Schema mismatch that cannot be resolved
    UnresolvableSchemaMismatch,
    
    /// Missing core data across required windows
    /// 
    /// Core data includes: canonical market events (trades, quotes, open interest),
    /// derivatives data (options chain, futures positions), and time-sensitive
    /// constraint data. Required windows depend on signal lookback periods.
    MissingCoreData,
    
    /// Event time cannot be reliably determined
    TimeSemanticViolation,
    
    /// Event ordering ambiguity exceeds tolerance
    EventOrderingAmbiguity,
    
    /// Regime classification confidence below minimum threshold
    RegimeIndeterminacy,
    
    /// Conflicting regime signals without resolution
    ConflictingRegimeSignals,
    
    /// Core constraint assumptions invalidated
    ConstraintMisidentification,
    
    /// Forced flow logic contradicted structurally
    ForcedFlowContradiction,
    
    /// Confidence calculation unavailable or corrupted
    ConfidenceSystemFailure,
    
    /// Confidence not degrading when required
    ConfidenceDegradationFailure,
}

/// Severity of a kill condition
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KillSeverity {
    /// Mandatory halt - inference must stop completely
    Mandatory,
    
    /// Partial suppression - selective output suppression
    Partial,
    
    /// Warning - proceed with extreme caution
    Warning,
}

/// Result of kill switch evaluation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KillSwitchDecision {
    /// Proceed with inference normally
    Proceed,
    
    /// Proceed with warnings attached
    ProceedWithWarnings(Vec<KillCondition>),
    
    /// Partial suppression required (e.g., signal-level, instrument-level)
    PartialSuppress(Vec<KillCondition>),
    
    /// Complete halt - inference must not proceed
    Halt(Vec<KillCondition>),
}

impl KillSwitchDecision {
    /// Check if this decision allows inference to proceed
    pub fn can_proceed(&self) -> bool {
        matches!(self, KillSwitchDecision::Proceed | KillSwitchDecision::ProceedWithWarnings(_))
    }
    
    /// Check if this decision requires complete halt
    pub fn must_halt(&self) -> bool {
        matches!(self, KillSwitchDecision::Halt(_))
    }
    
    /// Get all conditions associated with this decision
    pub fn conditions(&self) -> Vec<&KillCondition> {
        match self {
            KillSwitchDecision::Proceed => vec![],
            KillSwitchDecision::ProceedWithWarnings(c) => c.iter().collect(),
            KillSwitchDecision::PartialSuppress(c) => c.iter().collect(),
            KillSwitchDecision::Halt(c) => c.iter().collect(),
        }
    }
}

/// Configuration for kill switch thresholds
#[derive(Debug, Clone)]
pub struct KillSwitchConfig {
    /// Minimum confidence threshold for regime classification
    pub min_regime_confidence: f64,
    
    /// Minimum data quality threshold
    pub min_data_quality: f64,
    
    /// Minimum overall confidence threshold
    pub min_overall_confidence: f64,
    
    /// Maximum allowed event ordering ambiguity (in seconds)
    pub max_time_ambiguity_seconds: i64,
    
    /// Minimum required data completeness ratio (0.0 to 1.0)
    pub min_data_completeness: f64,
    
    /// Whether to enforce strict confidence degradation checks
    pub enforce_confidence_degradation: bool,
}

impl Default for KillSwitchConfig {
    fn default() -> Self {
        Self {
            min_regime_confidence: 0.30,
            min_data_quality: 0.50,
            min_overall_confidence: 0.30,
            max_time_ambiguity_seconds: 60,
            min_data_completeness: 0.80,
            enforce_confidence_degradation: true,
        }
    }
}

/// Data quality metrics for kill switch evaluation
#[derive(Debug, Clone)]
pub struct DataQualityMetrics {
    /// Ratio of valid events to total events (0.0 to 1.0)
    pub data_completeness: f64,
    
    /// Whether schema validation passed
    pub schema_valid: bool,
    
    /// Whether data integrity checks passed
    pub integrity_valid: bool,
    
    /// Maximum time ambiguity detected (in seconds)
    pub max_time_ambiguity: i64,
    
    /// Whether event ordering is consistent
    pub ordering_consistent: bool,
}

/// Regime assessment for kill switch evaluation
#[derive(Debug, Clone)]
pub struct RegimeAssessment {
    /// Confidence in regime classification (0.0 to 1.0)
    pub regime_confidence: f64,
    
    /// Whether multiple conflicting regime signals exist
    pub has_conflicts: bool,
    
    /// List of detected regime conflicts
    pub conflicts: Vec<String>,
}

/// Structural validation for kill switch evaluation
#[derive(Debug, Clone)]
pub struct StructuralValidation {
    /// Whether constraint assumptions are valid
    pub constraints_valid: bool,
    
    /// Whether forced flow logic is consistent
    pub forced_flow_consistent: bool,
    
    /// List of structural violations detected
    pub violations: Vec<String>,
}

/// Main kill switch evaluator
pub struct KillSwitchEvaluator {
    config: KillSwitchConfig,
}

impl KillSwitchEvaluator {
    /// Create a new kill switch evaluator with the given configuration
    pub fn new(config: KillSwitchConfig) -> Self {
        Self { config }
    }
    
    /// Create a new evaluator with default configuration
    pub fn default() -> Self {
        Self::new(KillSwitchConfig::default())
    }
    
    /// Evaluate all kill switch conditions and return a decision
    ///
    /// This is the main entry point for kill switch evaluation.
    /// It checks all mandatory conditions and returns a decision.
    pub fn evaluate(
        &self,
        data_quality: &DataQualityMetrics,
        regime: &RegimeAssessment,
        structural: &StructuralValidation,
        confidence: &OverallConfidence,
        stability: &StabilityIndicator,
    ) -> KillSwitchDecision {
        let mut mandatory_conditions = Vec::new();
        let partial_conditions = Vec::new();
        let warning_conditions = Vec::new();
        
        // Cache frequently accessed config values
        let _min_data_completeness = self.config.min_data_completeness;
        let _max_time_ambiguity = self.config.max_time_ambiguity_seconds;
        let _min_regime_conf = self.config.min_regime_confidence;
        let _min_overall_conf = self.config.min_overall_confidence;
        let _min_data_quality = self.config.min_data_quality;
        
        // 1. Check data integrity
        self.check_data_integrity(data_quality, &mut mandatory_conditions);
        
        // 2. Check time semantics
        self.check_time_semantics(data_quality, &mut mandatory_conditions);
        
        // 3. Check regime indeterminacy
        self.check_regime_validity(regime, &mut mandatory_conditions);
        
        // 4. Check constraint validity
        self.check_structural_validity(structural, &mut mandatory_conditions);
        
        // 5. Check confidence system
        self.check_confidence_system(confidence, stability, &mut mandatory_conditions);
        
        // Determine final decision based on collected conditions
        if !mandatory_conditions.is_empty() {
            KillSwitchDecision::Halt(mandatory_conditions)
        } else if !partial_conditions.is_empty() {
            KillSwitchDecision::PartialSuppress(partial_conditions)
        } else if !warning_conditions.is_empty() {
            KillSwitchDecision::ProceedWithWarnings(warning_conditions)
        } else {
            KillSwitchDecision::Proceed
        }
    }
    
    /// Check data integrity conditions (mandatory kill condition #1)
    fn check_data_integrity(
        &self,
        data_quality: &DataQualityMetrics,
        conditions: &mut Vec<KillCondition>,
    ) {
        // Check for corrupted or inconsistent canonical events
        if !data_quality.integrity_valid {
            let mut context = HashMap::new();
            context.insert("data_completeness".to_string(), data_quality.data_completeness.to_string());
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::DataIntegrityFailure,
                description: "Data integrity validation failed - corrupted or inconsistent canonical events detected".to_string(),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
        
        // Check for unresolvable schema mismatch
        if !data_quality.schema_valid {
            conditions.push(KillCondition {
                condition_type: KillConditionType::UnresolvableSchemaMismatch,
                description: "Schema validation failed - unresolvable mismatch detected".to_string(),
                severity: KillSeverity::Mandatory,
                context: HashMap::new(),
            });
        }
        
        // Check for missing core data
        if data_quality.data_completeness < self.config.min_data_completeness {
            let mut context = HashMap::new();
            context.insert("actual_completeness".to_string(), data_quality.data_completeness.to_string());
            context.insert("required_completeness".to_string(), self.config.min_data_completeness.to_string());
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::MissingCoreData,
                description: format!(
                    "Missing core data across required windows - completeness {:.2}% below required {:.2}%",
                    data_quality.data_completeness * 100.0,
                    self.config.min_data_completeness * 100.0
                ),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
    }
    
    /// Check time semantics conditions (mandatory kill condition #2)
    fn check_time_semantics(
        &self,
        data_quality: &DataQualityMetrics,
        conditions: &mut Vec<KillCondition>,
    ) {
        // Check for event time reliability
        if data_quality.max_time_ambiguity > self.config.max_time_ambiguity_seconds {
            let mut context = HashMap::new();
            context.insert("max_ambiguity".to_string(), data_quality.max_time_ambiguity.to_string());
            context.insert("allowed_ambiguity".to_string(), self.config.max_time_ambiguity_seconds.to_string());
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::TimeSemanticViolation,
                description: format!(
                    "Event time cannot be reliably determined - ambiguity {}s exceeds allowed {}s",
                    data_quality.max_time_ambiguity,
                    self.config.max_time_ambiguity_seconds
                ),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
        
        // Check for event ordering consistency
        if !data_quality.ordering_consistent {
            conditions.push(KillCondition {
                condition_type: KillConditionType::EventOrderingAmbiguity,
                description: "Event ordering ambiguity exceeds tolerance - cannot establish reliable event sequence".to_string(),
                severity: KillSeverity::Mandatory,
                context: HashMap::new(),
            });
        }
    }
    
    /// Check regime validity conditions (mandatory kill condition #3)
    fn check_regime_validity(
        &self,
        regime: &RegimeAssessment,
        conditions: &mut Vec<KillCondition>,
    ) {
        // Check regime classification confidence
        if regime.regime_confidence < self.config.min_regime_confidence {
            let mut context = HashMap::new();
            context.insert("actual_confidence".to_string(), regime.regime_confidence.to_string());
            context.insert("required_confidence".to_string(), self.config.min_regime_confidence.to_string());
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::RegimeIndeterminacy,
                description: format!(
                    "Regime classification confidence {:.2} below minimum threshold {:.2}",
                    regime.regime_confidence,
                    self.config.min_regime_confidence
                ),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
        
        // Check for conflicting regime signals
        if regime.has_conflicts {
            let mut context = HashMap::new();
            context.insert("conflict_count".to_string(), regime.conflicts.len().to_string());
            context.insert("conflicts".to_string(), regime.conflicts.join("; "));
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::ConflictingRegimeSignals,
                description: format!(
                    "Conflicting regime signals detected without resolution: {}",
                    regime.conflicts.join(", ")
                ),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
    }
    
    /// Check structural validity conditions (mandatory kill condition #4)
    fn check_structural_validity(
        &self,
        structural: &StructuralValidation,
        conditions: &mut Vec<KillCondition>,
    ) {
        // Check constraint assumptions
        if !structural.constraints_valid {
            let mut context = HashMap::new();
            if !structural.violations.is_empty() {
                context.insert("violations".to_string(), structural.violations.join("; "));
            }
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::ConstraintMisidentification,
                description: "Core constraint assumptions invalidated - structural model inconsistent".to_string(),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
        
        // Check forced flow logic consistency
        if !structural.forced_flow_consistent {
            conditions.push(KillCondition {
                condition_type: KillConditionType::ForcedFlowContradiction,
                description: "Forced flow logic contradicted structurally - cannot maintain consistency".to_string(),
                severity: KillSeverity::Mandatory,
                context: HashMap::new(),
            });
        }
    }
    
    /// Check confidence system conditions (mandatory kill condition #5)
    fn check_confidence_system(
        &self,
        confidence: &OverallConfidence,
        stability: &StabilityIndicator,
        conditions: &mut Vec<KillCondition>,
    ) {
        // Check if confidence system is functional
        // If confidence or stability scores are NaN or infinite, the system has failed
        if !confidence.score.is_finite() || !stability.score.is_finite() {
            conditions.push(KillCondition {
                condition_type: KillConditionType::ConfidenceSystemFailure,
                description: "Confidence calculation unavailable or corrupted - non-finite values detected".to_string(),
                severity: KillSeverity::Mandatory,
                context: HashMap::new(),
            });
            return;
        }
        
        // Check if confidence is below absolute minimum
        if confidence.score < self.config.min_overall_confidence {
            let mut context = HashMap::new();
            context.insert("actual_confidence".to_string(), confidence.score.to_string());
            context.insert("required_confidence".to_string(), self.config.min_overall_confidence.to_string());
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::ConfidenceSystemFailure,
                description: format!(
                    "Overall confidence {:.2} below minimum threshold {:.2}",
                    confidence.score,
                    self.config.min_overall_confidence
                ),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
        
        // Check data quality component specifically
        if confidence.components.data_quality < self.config.min_data_quality {
            let mut context = HashMap::new();
            context.insert("actual_quality".to_string(), confidence.components.data_quality.to_string());
            context.insert("required_quality".to_string(), self.config.min_data_quality.to_string());
            
            conditions.push(KillCondition {
                condition_type: KillConditionType::ConfidenceSystemFailure,
                description: format!(
                    "Data quality confidence {:.2} below minimum threshold {:.2}",
                    confidence.components.data_quality,
                    self.config.min_data_quality
                ),
                severity: KillSeverity::Mandatory,
                context,
            });
        }
    }
    
    /// Get the current configuration
    pub fn config(&self) -> &KillSwitchConfig {
        &self.config
    }
}

impl KillConditionType {
    /// Get a human-readable description of this condition type
    pub fn description(&self) -> &'static str {
        match self {
            KillConditionType::DataIntegrityFailure => "Data integrity failure",
            KillConditionType::UnresolvableSchemaMismatch => "Unresolvable schema mismatch",
            KillConditionType::MissingCoreData => "Missing core data",
            KillConditionType::TimeSemanticViolation => "Time semantic violation",
            KillConditionType::EventOrderingAmbiguity => "Event ordering ambiguity",
            KillConditionType::RegimeIndeterminacy => "Regime indeterminacy",
            KillConditionType::ConflictingRegimeSignals => "Conflicting regime signals",
            KillConditionType::ConstraintMisidentification => "Constraint misidentification",
            KillConditionType::ForcedFlowContradiction => "Forced flow contradiction",
            KillConditionType::ConfidenceSystemFailure => "Confidence system failure",
            KillConditionType::ConfidenceDegradationFailure => "Confidence degradation failure",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::confidence::types::{ConfidenceComponents, ConfidenceLevel, StabilityComponents, StabilityLevel};
    
    fn create_valid_data_quality() -> DataQualityMetrics {
        DataQualityMetrics {
            data_completeness: 0.95,
            schema_valid: true,
            integrity_valid: true,
            max_time_ambiguity: 10,
            ordering_consistent: true,
        }
    }
    
    fn create_valid_regime() -> RegimeAssessment {
        RegimeAssessment {
            regime_confidence: 0.8,
            has_conflicts: false,
            conflicts: vec![],
        }
    }
    
    fn create_valid_structural() -> StructuralValidation {
        StructuralValidation {
            constraints_valid: true,
            forced_flow_consistent: true,
            violations: vec![],
        }
    }
    
    fn create_valid_confidence() -> OverallConfidence {
        OverallConfidence {
            score: 0.75,
            components: ConfidenceComponents {
                data_quality: 0.8,
                structural_alignment: 0.75,
                regime_consistency: 0.7,
                signal_agreement: 0.75,
            },
            level: ConfidenceLevel::Medium,
        }
    }
    
    fn create_valid_stability() -> StabilityIndicator {
        StabilityIndicator {
            score: 0.7,
            components: StabilityComponents {
                temporal: 0.7,
                sensitivity: 0.7,
                regime: 0.7,
            },
            level: StabilityLevel::Robust,
        }
    }
    
    #[test]
    fn test_proceed_with_valid_inputs() {
        let evaluator = KillSwitchEvaluator::default();
        let data_quality = create_valid_data_quality();
        let regime = create_valid_regime();
        let structural = create_valid_structural();
        let confidence = create_valid_confidence();
        let stability = create_valid_stability();
        
        let decision = evaluator.evaluate(&data_quality, &regime, &structural, &confidence, &stability);
        
        assert_eq!(decision, KillSwitchDecision::Proceed);
    }
    
    #[test]
    fn test_halt_on_data_integrity_failure() {
        let evaluator = KillSwitchEvaluator::default();
        let mut data_quality = create_valid_data_quality();
        data_quality.integrity_valid = false;
        
        let decision = evaluator.evaluate(
            &data_quality,
            &create_valid_regime(),
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::DataIntegrityFailure));
    }
    
    #[test]
    fn test_halt_on_schema_mismatch() {
        let evaluator = KillSwitchEvaluator::default();
        let mut data_quality = create_valid_data_quality();
        data_quality.schema_valid = false;
        
        let decision = evaluator.evaluate(
            &data_quality,
            &create_valid_regime(),
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::UnresolvableSchemaMismatch));
    }
    
    #[test]
    fn test_halt_on_missing_core_data() {
        let evaluator = KillSwitchEvaluator::default();
        let mut data_quality = create_valid_data_quality();
        data_quality.data_completeness = 0.5; // Below default threshold of 0.8
        
        let decision = evaluator.evaluate(
            &data_quality,
            &create_valid_regime(),
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::MissingCoreData));
    }
    
    #[test]
    fn test_halt_on_time_ambiguity() {
        let evaluator = KillSwitchEvaluator::default();
        let mut data_quality = create_valid_data_quality();
        data_quality.max_time_ambiguity = 120; // Exceeds default threshold of 60s
        
        let decision = evaluator.evaluate(
            &data_quality,
            &create_valid_regime(),
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::TimeSemanticViolation));
    }
    
    #[test]
    fn test_halt_on_event_ordering_ambiguity() {
        let evaluator = KillSwitchEvaluator::default();
        let mut data_quality = create_valid_data_quality();
        data_quality.ordering_consistent = false;
        
        let decision = evaluator.evaluate(
            &data_quality,
            &create_valid_regime(),
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::EventOrderingAmbiguity));
    }
    
    #[test]
    fn test_halt_on_regime_indeterminacy() {
        let evaluator = KillSwitchEvaluator::default();
        let mut regime = create_valid_regime();
        regime.regime_confidence = 0.2; // Below default threshold of 0.3
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &regime,
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::RegimeIndeterminacy));
    }
    
    #[test]
    fn test_halt_on_conflicting_regime_signals() {
        let evaluator = KillSwitchEvaluator::default();
        let mut regime = create_valid_regime();
        regime.has_conflicts = true;
        regime.conflicts = vec!["Bull vs Bear".to_string(), "High vol vs Low vol".to_string()];
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &regime,
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::ConflictingRegimeSignals));
    }
    
    #[test]
    fn test_halt_on_invalid_constraints() {
        let evaluator = KillSwitchEvaluator::default();
        let mut structural = create_valid_structural();
        structural.constraints_valid = false;
        structural.violations = vec!["Option chain broken".to_string()];
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &create_valid_regime(),
            &structural,
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::ConstraintMisidentification));
    }
    
    #[test]
    fn test_halt_on_forced_flow_contradiction() {
        let evaluator = KillSwitchEvaluator::default();
        let mut structural = create_valid_structural();
        structural.forced_flow_consistent = false;
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &create_valid_regime(),
            &structural,
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::ForcedFlowContradiction));
    }
    
    #[test]
    fn test_halt_on_low_confidence() {
        let evaluator = KillSwitchEvaluator::default();
        let mut confidence = create_valid_confidence();
        confidence.score = 0.2; // Below default threshold of 0.3
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &create_valid_regime(),
            &create_valid_structural(),
            &confidence,
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::ConfidenceSystemFailure));
    }
    
    #[test]
    fn test_halt_on_low_data_quality_confidence() {
        let evaluator = KillSwitchEvaluator::default();
        let mut confidence = create_valid_confidence();
        confidence.components.data_quality = 0.4; // Below default threshold of 0.5
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &create_valid_regime(),
            &create_valid_structural(),
            &confidence,
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::ConfidenceSystemFailure));
    }
    
    #[test]
    fn test_halt_on_nan_confidence() {
        let evaluator = KillSwitchEvaluator::default();
        let mut confidence = create_valid_confidence();
        confidence.score = f64::NAN;
        
        let decision = evaluator.evaluate(
            &create_valid_data_quality(),
            &create_valid_regime(),
            &create_valid_structural(),
            &confidence,
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.iter().any(|c| c.condition_type == KillConditionType::ConfidenceSystemFailure));
    }
    
    #[test]
    fn test_multiple_kill_conditions() {
        let evaluator = KillSwitchEvaluator::default();
        let mut data_quality = create_valid_data_quality();
        data_quality.integrity_valid = false;
        data_quality.schema_valid = false;
        
        let decision = evaluator.evaluate(
            &data_quality,
            &create_valid_regime(),
            &create_valid_structural(),
            &create_valid_confidence(),
            &create_valid_stability(),
        );
        
        assert!(decision.must_halt());
        let conditions = decision.conditions();
        assert!(conditions.len() >= 2);
    }
    
    #[test]
    fn test_kill_switch_decision_methods() {
        let proceed = KillSwitchDecision::Proceed;
        assert!(proceed.can_proceed());
        assert!(!proceed.must_halt());
        
        let halt = KillSwitchDecision::Halt(vec![]);
        assert!(!halt.can_proceed());
        assert!(halt.must_halt());
    }
}
