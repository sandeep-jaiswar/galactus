// Type definitions for confidence and stability metrics

/// Represents the overall confidence score and its components
#[derive(Debug, Clone, PartialEq)]
pub struct OverallConfidence {
    /// Overall confidence score [0.0, 1.0]
    pub score: f64,
    
    /// Component confidence scores
    pub components: ConfidenceComponents,
    
    /// Confidence level classification
    pub level: ConfidenceLevel,
}

/// Individual components contributing to overall confidence
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceComponents {
    pub data_quality: f64,
    pub structural_alignment: f64,
    pub regime_consistency: f64,
    pub signal_agreement: f64,
}

/// Confidence level classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceLevel {
    /// confidence >= 0.80 && stability >= 0.70
    High,
    /// confidence >= 0.60 && stability >= 0.50
    Medium,
    /// confidence >= 0.40 && stability >= 0.30
    Low,
    /// confidence < 0.40 || stability < 0.30
    Critical,
}

/// Represents the overall stability indicator and its components
#[derive(Debug, Clone, PartialEq)]
pub struct StabilityIndicator {
    /// Overall stability score [0.0, 1.0]
    pub score: f64,
    
    /// Component stability scores
    pub components: StabilityComponents,
    
    /// Stability level classification
    pub level: StabilityLevel,
}

/// Individual components contributing to overall stability
#[derive(Debug, Clone, PartialEq)]
pub struct StabilityComponents {
    pub temporal: f64,
    pub sensitivity: f64,
    pub regime: f64,
}

/// Stability level classifications
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StabilityLevel {
    /// stability >= 0.70
    Robust,
    /// stability >= 0.40
    Moderate,
    /// stability < 0.40
    Fragile,
}

/// Metadata about uncertainty in the inference
#[derive(Debug, Clone, PartialEq)]
pub struct UncertaintyMetadata {
    /// Explicit assumptions made in the inference
    pub assumptions: Vec<String>,
    
    /// Flags indicating known ambiguities
    pub ambiguity_flags: Vec<String>,
    
    /// Known limitations of the inference
    pub known_limitations: Vec<String>,
    
    /// Confidence bounds (lower, upper)
    pub confidence_bounds: (f64, f64),
}

/// Result of silence evaluation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SilenceDecision {
    /// Output inference normally
    Proceed,
    
    /// Output with explicit warnings
    ProceedWithWarning(Vec<String>),
    
    /// Suppress inference completely
    Suppress(SuppressionReason),
}

/// Reason for suppressing inference
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuppressionReason {
    /// overall_confidence < 0.30
    InsufficientConfidence,
    
    /// data_quality_confidence < 0.50
    DataQualityBelowThreshold,
    
    /// regime_confidence < 0.30
    RegimeIndeterminate,
    
    /// structural_confidence < 0.40 && signal_confidence < 0.40
    StructuralFoundationInsufficient,
    
    /// overall_stability < 0.20
    InferenceTooUnstable,
}

impl ConfidenceLevel {
    /// Determine confidence level from confidence and stability scores
    pub fn from_scores(confidence: f64, stability: f64) -> Self {
        if confidence >= 0.80 && stability >= 0.70 {
            ConfidenceLevel::High
        } else if confidence >= 0.60 && stability >= 0.50 {
            ConfidenceLevel::Medium
        } else if confidence >= 0.40 && stability >= 0.30 {
            ConfidenceLevel::Low
        } else {
            ConfidenceLevel::Critical
        }
    }
}

impl StabilityLevel {
    /// Determine stability level from stability score
    pub fn from_score(stability: f64) -> Self {
        if stability >= 0.70 {
            StabilityLevel::Robust
        } else if stability >= 0.40 {
            StabilityLevel::Moderate
        } else {
            StabilityLevel::Fragile
        }
    }
}

impl SuppressionReason {
    /// Get human-readable message for suppression reason
    pub fn message(&self) -> &'static str {
        match self {
            SuppressionReason::InsufficientConfidence => {
                "Insufficient confidence for inference"
            }
            SuppressionReason::DataQualityBelowThreshold => {
                "Data quality below acceptable threshold"
            }
            SuppressionReason::RegimeIndeterminate => {
                "Regime classification indeterminate"
            }
            SuppressionReason::StructuralFoundationInsufficient => {
                "Structural foundation insufficient"
            }
            SuppressionReason::InferenceTooUnstable => {
                "Inference too unstable for reliable interpretation"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_level_high() {
        assert_eq!(
            ConfidenceLevel::from_scores(0.85, 0.75),
            ConfidenceLevel::High
        );
    }

    #[test]
    fn test_confidence_level_medium() {
        assert_eq!(
            ConfidenceLevel::from_scores(0.65, 0.55),
            ConfidenceLevel::Medium
        );
    }

    #[test]
    fn test_confidence_level_low() {
        assert_eq!(
            ConfidenceLevel::from_scores(0.45, 0.35),
            ConfidenceLevel::Low
        );
    }

    #[test]
    fn test_confidence_level_critical() {
        assert_eq!(
            ConfidenceLevel::from_scores(0.35, 0.25),
            ConfidenceLevel::Critical
        );
        
        // Low confidence even with good stability
        assert_eq!(
            ConfidenceLevel::from_scores(0.35, 0.80),
            ConfidenceLevel::Critical
        );
        
        // Low stability even with good confidence
        assert_eq!(
            ConfidenceLevel::from_scores(0.85, 0.25),
            ConfidenceLevel::Critical
        );
    }

    #[test]
    fn test_stability_level_robust() {
        assert_eq!(StabilityLevel::from_score(0.75), StabilityLevel::Robust);
    }

    #[test]
    fn test_stability_level_moderate() {
        assert_eq!(StabilityLevel::from_score(0.50), StabilityLevel::Moderate);
    }

    #[test]
    fn test_stability_level_fragile() {
        assert_eq!(StabilityLevel::from_score(0.30), StabilityLevel::Fragile);
    }

    #[test]
    fn test_suppression_reason_messages() {
        assert!(!SuppressionReason::InsufficientConfidence.message().is_empty());
        assert!(!SuppressionReason::DataQualityBelowThreshold.message().is_empty());
        assert!(!SuppressionReason::RegimeIndeterminate.message().is_empty());
        assert!(!SuppressionReason::StructuralFoundationInsufficient.message().is_empty());
        assert!(!SuppressionReason::InferenceTooUnstable.message().is_empty());
    }
}
