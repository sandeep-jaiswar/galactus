//! Safe Intent Engine with Kill Switch Integration
//!
//! This module wraps the core intent engine with kill switch evaluation
//! to ensure inference only proceeds when fundamental conditions are met.
//!
//! # Safety Philosophy
//!
//! The safe engine enforces the principle: "Incorrect silence is preferable to confident error"
//! by evaluating kill switch conditions before allowing inference to proceed.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use super::{IntentEngine, IntentResult, IntentError, IntentConfig, SignalInput};
use crate::confidence::types::{OverallConfidence, StabilityIndicator, ConfidenceComponents, StabilityComponents, ConfidenceLevel, StabilityLevel};
use crate::kill_switch::{
    KillSwitchEvaluator, KillSwitchConfig, KillSwitchDecision,
    DataQualityMetrics, RegimeAssessment, StructuralValidation,
};

/// Safe intent engine that integrates kill switch evaluation
pub struct SafeIntentEngine {
    /// Core intent engine
    engine: IntentEngine,
    
    /// Kill switch evaluator
    kill_switch: KillSwitchEvaluator,
    
    /// Whether to enforce kill switch (can be disabled for testing)
    enforce_kill_switch: bool,
}

impl SafeIntentEngine {
    /// Create a new safe intent engine with given configurations
    pub fn new(
        intent_config: IntentConfig,
        kill_switch_config: KillSwitchConfig,
    ) -> Self {
        Self {
            engine: IntentEngine::new(intent_config),
            kill_switch: KillSwitchEvaluator::new(kill_switch_config),
            enforce_kill_switch: true,
        }
    }
    
    /// Create a new safe engine with default configurations
    pub fn default() -> Self {
        Self::new(IntentConfig::default(), KillSwitchConfig::default())
    }
    
    /// Process signals with kill switch evaluation
    ///
    /// This is the main entry point that:
    /// 1. Validates data quality and structural conditions
    /// 2. Evaluates kill switch conditions
    /// 3. Either halts, warns, or proceeds with inference
    ///
    /// # Arguments
    /// * `signals` - Vector of signal inputs
    /// * `data_quality` - Data quality metrics
    /// * `structural` - Structural validation results
    ///
    /// # Returns
    /// Intent result if safe to proceed, or error if kill switch triggered
    pub fn process_safe(
        &self,
        signals: Vec<SignalInput>,
        data_quality: DataQualityMetrics,
        structural: StructuralValidation,
    ) -> Result<IntentResult, IntentError> {
        // Step 1: Perform core inference (this gives us confidence/stability)
        // We do this first to get confidence metrics, then evaluate kill switch
        let inference_result = self.engine.process(signals)?;
        
        // Step 2: Extract or compute confidence and stability
        // For now, we'll create placeholder values - in production these would come
        // from the inference process
        let confidence = self.extract_confidence(&inference_result);
        let stability = self.extract_stability(&inference_result);
        
        // Step 3: Assess regime from the result
        let regime = self.assess_regime(&inference_result);
        
        // Step 4: Evaluate kill switch conditions
        if self.enforce_kill_switch {
            let kill_decision = self.kill_switch.evaluate(
                &data_quality,
                &regime,
                &structural,
                &confidence,
                &stability,
            );
            
            match kill_decision {
                KillSwitchDecision::Halt(conditions) => {
                    // Mandatory halt - return error with details
                    let reasons: Vec<String> = conditions.iter()
                        .map(|c| format!("{}: {}", c.condition_type.description(), c.description))
                        .collect();
                    return Err(IntentError::KillSwitchTriggered(
                        format!("Inference halted due to {} kill condition(s): {}", 
                            conditions.len(),
                            reasons.join("; ")
                        )
                    ));
                }
                KillSwitchDecision::PartialSuppress(conditions) => {
                    // Partial suppression - could implement signal filtering here
                    // For now, return error to be conservative
                    let reasons: Vec<String> = conditions.iter()
                        .map(|c| format!("{}: {}", c.condition_type.description(), c.description))
                        .collect();
                    return Err(IntentError::KillSwitchTriggered(
                        format!("Inference partially suppressed due to {} condition(s): {}", 
                            conditions.len(),
                            reasons.join("; ")
                        )
                    ));
                }
                KillSwitchDecision::ProceedWithWarnings(warnings) => {
                    // Attach warnings to result metadata
                    let mut result = inference_result;
                    for (i, warning) in warnings.iter().enumerate() {
                        result.metadata.insert(
                            format!("kill_switch_warning_{}", i),
                            format!("{}: {}", warning.condition_type.description(), warning.description),
                        );
                    }
                    result.metadata.insert("kill_switch_status".to_string(), "warning".to_string());
                    return Ok(result);
                }
                KillSwitchDecision::Proceed => {
                    // Safe to proceed - add status to metadata
                    let mut result = inference_result;
                    result.metadata.insert("kill_switch_status".to_string(), "passed".to_string());
                    return Ok(result);
                }
            }
        } else {
            // Kill switch disabled - return result with note
            let mut result = inference_result;
            result.metadata.insert("kill_switch_status".to_string(), "disabled".to_string());
            Ok(result)
        }
    }
    
    /// Process signals using default data quality and structural validation
    ///
    /// This is a convenience method that assumes data is valid.
    /// Use `process_safe` for production use with real validation.
    pub fn process(&self, signals: Vec<SignalInput>) -> Result<IntentResult, IntentError> {
        let data_quality = DataQualityMetrics {
            data_completeness: 1.0,
            schema_valid: true,
            integrity_valid: true,
            max_time_ambiguity: 0,
            ordering_consistent: true,
        };
        
        let structural = StructuralValidation {
            constraints_valid: true,
            forced_flow_consistent: true,
            violations: vec![],
        };
        
        self.process_safe(signals, data_quality, structural)
    }
    
    /// Extract confidence from intent result
    ///
    /// In production, this would use actual confidence computation from the engine.
    /// For now, we create a placeholder based on the intent confidence.
    fn extract_confidence(&self, result: &IntentResult) -> OverallConfidence {
        let base_confidence = result.intent.confidence;
        
        // Create reasonable component values
        let components = ConfidenceComponents {
            data_quality: (base_confidence * 0.95).min(1.0),
            structural_alignment: (base_confidence * 0.9).min(1.0),
            regime_consistency: (base_confidence * 0.85).min(1.0),
            signal_agreement: base_confidence,
        };
        
        let level = ConfidenceLevel::from_scores(base_confidence, 0.7);
        
        OverallConfidence {
            score: base_confidence,
            components,
            level,
        }
    }
    
    /// Extract stability from intent result
    ///
    /// In production, this would use actual stability computation.
    /// For now, we create a reasonable placeholder.
    fn extract_stability(&self, result: &IntentResult) -> StabilityIndicator {
        let base_stability = result.intent.confidence * 0.9;
        
        let components = StabilityComponents {
            temporal: base_stability,
            sensitivity: base_stability * 0.95,
            regime: base_stability * 0.9,
        };
        
        let level = StabilityLevel::from_score(base_stability);
        
        StabilityIndicator {
            score: base_stability,
            components,
            level,
        }
    }
    
    /// Assess regime from intent result
    fn assess_regime(&self, result: &IntentResult) -> RegimeAssessment {
        // Extract regime confidence from metadata if available
        let regime_confidence = result.metadata
            .get("regime_confidence")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.8);
        
        RegimeAssessment {
            regime_confidence,
            has_conflicts: false,
            conflicts: vec![],
        }
    }
    
    /// Enable or disable kill switch enforcement
    ///
    /// Use this carefully - disabling kill switch should only be done
    /// for testing or debugging purposes.
    pub fn set_enforce_kill_switch(&mut self, enforce: bool) {
        self.enforce_kill_switch = enforce;
    }
    
    /// Get reference to the underlying intent engine
    pub fn engine(&self) -> &IntentEngine {
        &self.engine
    }
    
    /// Get reference to the kill switch evaluator
    pub fn kill_switch(&self) -> &KillSwitchEvaluator {
        &self.kill_switch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_signal(name: &str, value: f64, confidence: f64) -> SignalInput {
        SignalInput {
            name: name.to_string(),
            value,
            confidence,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            metadata: HashMap::new(),
        }
    }
    
    fn create_valid_data_quality() -> DataQualityMetrics {
        DataQualityMetrics {
            data_completeness: 0.95,
            schema_valid: true,
            integrity_valid: true,
            max_time_ambiguity: 10,
            ordering_consistent: true,
        }
    }
    
    fn create_valid_structural() -> StructuralValidation {
        StructuralValidation {
            constraints_valid: true,
            forced_flow_consistent: true,
            violations: vec![],
        }
    }
    
    #[test]
    fn test_safe_engine_with_valid_conditions() {
        let engine = SafeIntentEngine::default();
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
            create_test_signal("signal2", 0.3, 0.9),
        ];
        
        let result = engine.process_safe(
            signals,
            create_valid_data_quality(),
            create_valid_structural(),
        );
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.metadata.get("kill_switch_status").unwrap(), "passed");
    }
    
    #[test]
    fn test_safe_engine_halts_on_data_integrity_failure() {
        let engine = SafeIntentEngine::default();
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
        ];
        
        let mut data_quality = create_valid_data_quality();
        data_quality.integrity_valid = false;
        
        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );
        
        assert!(result.is_err());
        match result {
            Err(IntentError::KillSwitchTriggered(msg)) => {
                assert!(msg.contains("Data integrity"));
            }
            _ => panic!("Expected KillSwitchTriggered error"),
        }
    }
    
    #[test]
    fn test_safe_engine_halts_on_missing_data() {
        let engine = SafeIntentEngine::default();
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
        ];
        
        let mut data_quality = create_valid_data_quality();
        data_quality.data_completeness = 0.5; // Below threshold
        
        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );
        
        assert!(result.is_err());
        match result {
            Err(IntentError::KillSwitchTriggered(msg)) => {
                assert!(msg.contains("Missing core data"));
            }
            _ => panic!("Expected KillSwitchTriggered error"),
        }
    }
    
    #[test]
    fn test_safe_engine_halts_on_structural_violation() {
        let engine = SafeIntentEngine::default();
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
        ];
        
        let mut structural = create_valid_structural();
        structural.constraints_valid = false;
        structural.violations = vec!["Constraint violation".to_string()];
        
        let result = engine.process_safe(
            signals,
            create_valid_data_quality(),
            structural,
        );
        
        assert!(result.is_err());
        match result {
            Err(IntentError::KillSwitchTriggered(msg)) => {
                assert!(msg.contains("Constraint"));
            }
            _ => panic!("Expected KillSwitchTriggered error"),
        }
    }
    
    #[test]
    fn test_safe_engine_with_kill_switch_disabled() {
        let mut engine = SafeIntentEngine::default();
        engine.set_enforce_kill_switch(false);
        
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
        ];
        
        let mut data_quality = create_valid_data_quality();
        data_quality.integrity_valid = false; // Would normally halt
        
        let result = engine.process_safe(
            signals,
            data_quality,
            create_valid_structural(),
        );
        
        // Should succeed because kill switch is disabled
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.metadata.get("kill_switch_status").unwrap(), "disabled");
    }
    
    #[test]
    fn test_safe_engine_convenience_method() {
        let engine = SafeIntentEngine::default();
        let signals = vec![
            create_test_signal("signal1", 0.5, 0.8),
            create_test_signal("signal2", 0.3, 0.9),
        ];
        
        let result = engine.process(signals);
        assert!(result.is_ok());
    }
}
