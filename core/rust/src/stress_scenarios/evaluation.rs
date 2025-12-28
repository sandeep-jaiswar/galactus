//! Stress scenario evaluation and reporting
//!
//! This module provides evaluation capabilities for assessing system behavior
//! under stress scenarios and determining acceptable vs unacceptable outcomes.

use crate::stress_scenarios::types::*;

/// Result of a stress scenario test
#[derive(Debug, Clone, PartialEq)]
pub struct StressTestResult {
    /// The stress scenario that was tested
    pub scenario: StressScenario,
    /// Observed confidence level during stress (0.0 to 1.0)
    pub observed_confidence: f64,
    /// Whether signal activation was suppressed
    pub was_signal_suppressed: bool,
    /// Whether inference was halted
    pub was_inference_halted: bool,
    /// Description of observed behavior
    pub observed_behavior: String,
    /// Whether the behavior was acceptable
    pub is_acceptable: bool,
    /// List of detected unacceptable behaviors (if any)
    pub unacceptable_behaviors: Vec<UnacceptableStressBehavior>,
    /// Lessons learned from this test
    pub lessons_learned: Vec<String>,
}

impl StressTestResult {
    /// Creates a new stress test result
    pub fn new(
        scenario: StressScenario,
        observed_confidence: f64,
        was_signal_suppressed: bool,
        was_inference_halted: bool,
        observed_behavior: String,
    ) -> Self {
        let mut result = Self {
            scenario: scenario.clone(),
            observed_confidence,
            was_signal_suppressed,
            was_inference_halted,
            observed_behavior,
            is_acceptable: true,
            unacceptable_behaviors: Vec::new(),
            lessons_learned: Vec::new(),
        };

        result.evaluate_acceptability();
        result
    }

    /// Evaluates whether the observed behavior is acceptable
    fn evaluate_acceptability(&mut self) {
        // Check for unacceptable behaviors
        if self.observed_confidence > 0.8 && self.scenario.is_critical() {
            self.unacceptable_behaviors
                .push(UnacceptableStressBehavior::HighConfidenceMaintained);
        }

        if !self.was_signal_suppressed
            && !self.was_inference_halted
            && self.scenario.should_suppress_inference()
        {
            self.unacceptable_behaviors
                .push(UnacceptableStressBehavior::ConfidentDirectionalInference);
        }

        // If any unacceptable behaviors detected, mark as unacceptable
        self.is_acceptable = self.unacceptable_behaviors.is_empty();
    }

    /// Adds a lesson learned
    pub fn add_lesson(&mut self, lesson: String) {
        self.lessons_learned.push(lesson);
    }

    /// Returns true if confidence degraded appropriately
    pub fn has_confidence_degraded(&self) -> bool {
        self.observed_confidence < 0.6
    }

    /// Returns true if system remained silent (no inference)
    pub fn remained_silent(&self) -> bool {
        self.was_inference_halted || self.was_signal_suppressed
    }

    /// Generates a summary report
    pub fn summary(&self) -> String {
        format!(
            "Stress Test Result:\n\
             Scenario: {}\n\
             Severity: {}\n\
             Observed Confidence: {:.2}\n\
             Signal Suppressed: {}\n\
             Inference Halted: {}\n\
             Acceptable: {}\n\
             {}",
            self.scenario.category,
            self.scenario.severity,
            self.observed_confidence,
            self.was_signal_suppressed,
            self.was_inference_halted,
            self.is_acceptable,
            if !self.unacceptable_behaviors.is_empty() {
                format!(
                    "Unacceptable Behaviors: {}",
                    self.unacceptable_behaviors
                        .iter()
                        .map(|b| format!("{}", b))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            } else {
                "No unacceptable behaviors detected".to_string()
            }
        )
    }
}

/// Evaluates expected behavior for a given stress scenario
pub fn evaluate_expected_behavior(scenario: &StressScenario) -> Vec<ExpectedStressBehavior> {
    let mut expected = Vec::new();

    match scenario.category {
        StressScenarioCategory::LiquidityStress => {
            expected.push(ExpectedStressBehavior::ConfidenceDegradation);
            if scenario.severity >= StressSeverity::Severe {
                expected.push(ExpectedStressBehavior::SignalSuppression);
            }
        }
        StressScenarioCategory::ExpiryCompression => {
            expected.push(ExpectedStressBehavior::ConfidenceDegradation);
            // Signals must decay or resolve cleanly post-expiry
        }
        StressScenarioCategory::VolatilityShock => {
            expected.push(ExpectedStressBehavior::ConfidenceDegradation);
            if scenario.severity >= StressSeverity::Severe {
                expected.push(ExpectedStressBehavior::NoMeaningfulInference);
            }
        }
        StressScenarioCategory::RegimeTransition => {
            expected.push(ExpectedStressBehavior::ConfidenceDegradation);
            expected.push(ExpectedStressBehavior::NoMeaningfulInference);
        }
        StressScenarioCategory::ConstraintOverlap => {
            expected.push(ExpectedStressBehavior::ConfidenceDegradation);
            if scenario.severity >= StressSeverity::Severe {
                expected.push(ExpectedStressBehavior::ExplicitFailure);
            }
        }
        StressScenarioCategory::DataDegradation => {
            expected.push(ExpectedStressBehavior::ConfidenceDegradation);
            if scenario.should_suppress_inference() {
                expected.push(ExpectedStressBehavior::ExplicitFailure);
            }
        }
    }

    if scenario.severity == StressSeverity::Extreme {
        expected.push(ExpectedStressBehavior::ExplicitFailure);
    }

    expected
}

/// Stress test suite that runs multiple scenarios
#[derive(Debug, Clone)]
pub struct StressTestSuite {
    /// Name of the test suite
    pub name: String,
    /// Scenarios to test
    pub scenarios: Vec<StressScenario>,
    /// Test results
    pub results: Vec<StressTestResult>,
}

impl StressTestSuite {
    /// Creates a new test suite
    pub fn new(name: String) -> Self {
        Self {
            name,
            scenarios: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Adds a scenario to the suite
    pub fn add_scenario(&mut self, scenario: StressScenario) {
        self.scenarios.push(scenario);
    }

    /// Adds a test result
    pub fn add_result(&mut self, result: StressTestResult) {
        self.results.push(result);
    }

    /// Returns true if all tests passed
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.is_acceptable)
    }

    /// Returns the number of failed tests
    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| !r.is_acceptable).count()
    }

    /// Returns scenarios grouped by category
    pub fn scenarios_by_category(&self) -> Vec<(StressScenarioCategory, usize)> {
        let mut counts = std::collections::HashMap::new();
        for scenario in &self.scenarios {
            *counts.entry(scenario.category).or_insert(0) += 1;
        }
        counts.into_iter().collect()
    }

    /// Validates that mandatory categories are covered
    pub fn validate_coverage(&self) -> Result<(), String> {
        let mandatory_categories = vec![
            StressScenarioCategory::LiquidityStress,
            StressScenarioCategory::ExpiryCompression,
            StressScenarioCategory::VolatilityShock,
            StressScenarioCategory::DataDegradation,
        ];

        let covered_categories: std::collections::HashSet<_> =
            self.scenarios.iter().map(|s| s.category).collect();

        let missing: Vec<_> = mandatory_categories
            .into_iter()
            .filter(|c| !covered_categories.contains(c))
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Missing mandatory stress categories: {}",
                missing
                    .iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        }
    }

    /// Generates a comprehensive report
    pub fn generate_report(&self) -> String {
        let mut report = format!("Stress Test Suite: {}\n", self.name);
        report.push_str(&format!("Total Scenarios: {}\n", self.scenarios.len()));
        report.push_str(&format!("Total Tests Run: {}\n", self.results.len()));
        report.push_str(&format!(
            "Tests Passed: {}\n",
            self.results.len() - self.failure_count()
        ));
        report.push_str(&format!("Tests Failed: {}\n", self.failure_count()));
        report.push_str(&format!(
            "Overall: {}\n\n",
            if self.all_passed() {
                "PASSED"
            } else {
                "FAILED"
            }
        ));

        // Category coverage
        report.push_str("Category Coverage:\n");
        for (category, count) in self.scenarios_by_category() {
            report.push_str(&format!("  {}: {} scenarios\n", category, count));
        }
        report.push('\n');

        // Coverage validation
        match self.validate_coverage() {
            Ok(_) => report.push_str("✓ All mandatory categories covered\n\n"),
            Err(msg) => report.push_str(&format!("✗ {}\n\n", msg)),
        }

        // Failed tests
        if self.failure_count() > 0 {
            report.push_str("Failed Tests:\n");
            for result in self.results.iter().filter(|r| !r.is_acceptable) {
                report.push_str(&format!("\n{}\n", result.summary()));
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable_stress_behavior() {
        let scenario = StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Severe,
            "Liquidity collapse".to_string(),
            1000000,
            false,
        );

        // Acceptable: low confidence, signal suppressed
        let result = StressTestResult::new(
            scenario.clone(),
            0.4,
            true,
            false,
            "Confidence degraded, signal suppressed".to_string(),
        );

        assert!(result.is_acceptable);
        assert!(result.has_confidence_degraded());
        assert!(result.remained_silent());
        assert!(result.unacceptable_behaviors.is_empty());
    }

    #[test]
    fn test_unacceptable_high_confidence() {
        let scenario = StressScenario::new(
            StressScenarioCategory::VolatilityShock,
            StressSeverity::Extreme,
            "Volatility shock".to_string(),
            1000000,
            false,
        );

        // Unacceptable: high confidence during extreme stress
        let result = StressTestResult::new(
            scenario.clone(),
            0.9,
            false,
            false,
            "High confidence maintained".to_string(),
        );

        assert!(!result.is_acceptable);
        assert!(!result.has_confidence_degraded());
        assert!(result
            .unacceptable_behaviors
            .contains(&UnacceptableStressBehavior::HighConfidenceMaintained));
    }

    #[test]
    fn test_unacceptable_no_suppression() {
        let scenario = StressScenario::new(
            StressScenarioCategory::DataDegradation,
            StressSeverity::Extreme,
            "Schema change".to_string(),
            1000000,
            false,
        );

        // Unacceptable: no suppression when it should be suppressed
        let result = StressTestResult::new(
            scenario.clone(),
            0.7,
            false,
            false,
            "Inference continued".to_string(),
        );

        assert!(!result.is_acceptable);
        assert!(result
            .unacceptable_behaviors
            .contains(&UnacceptableStressBehavior::ConfidentDirectionalInference));
    }

    #[test]
    fn test_expected_behavior_evaluation() {
        let liquidity_stress = StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Severe,
            "Liquidity collapse".to_string(),
            1000000,
            false,
        );

        let expected = evaluate_expected_behavior(&liquidity_stress);
        assert!(expected.contains(&ExpectedStressBehavior::ConfidenceDegradation));
        assert!(expected.contains(&ExpectedStressBehavior::SignalSuppression));
    }

    #[test]
    fn test_stress_test_suite() {
        let mut suite = StressTestSuite::new("Test Suite".to_string());

        let scenario1 = StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Severe,
            "Test 1".to_string(),
            1000000,
            false,
        );

        let scenario2 = StressScenario::new(
            StressScenarioCategory::VolatilityShock,
            StressSeverity::Extreme,
            "Test 2".to_string(),
            1000000,
            false,
        );

        suite.add_scenario(scenario1.clone());
        suite.add_scenario(scenario2.clone());

        // Add passing result
        let result1 = StressTestResult::new(scenario1, 0.3, true, false, "Passed".to_string());
        suite.add_result(result1);

        // Add failing result
        let result2 = StressTestResult::new(scenario2, 0.95, false, false, "Failed".to_string());
        suite.add_result(result2);

        assert!(!suite.all_passed());
        assert_eq!(suite.failure_count(), 1);
        assert_eq!(suite.scenarios.len(), 2);
    }

    #[test]
    fn test_suite_coverage_validation() {
        let mut suite = StressTestSuite::new("Coverage Test".to_string());

        // Add only one mandatory category
        suite.add_scenario(StressScenario::new(
            StressScenarioCategory::LiquidityStress,
            StressSeverity::Moderate,
            "Test".to_string(),
            1000000,
            false,
        ));

        // Should fail validation
        assert!(suite.validate_coverage().is_err());

        // Add remaining mandatory categories
        suite.add_scenario(StressScenario::new(
            StressScenarioCategory::ExpiryCompression,
            StressSeverity::Moderate,
            "Test".to_string(),
            1000000,
            false,
        ));
        suite.add_scenario(StressScenario::new(
            StressScenarioCategory::VolatilityShock,
            StressSeverity::Moderate,
            "Test".to_string(),
            1000000,
            false,
        ));
        suite.add_scenario(StressScenario::new(
            StressScenarioCategory::DataDegradation,
            StressSeverity::Moderate,
            "Test".to_string(),
            1000000,
            false,
        ));

        // Should pass validation
        assert!(suite.validate_coverage().is_ok());
    }

    #[test]
    fn test_lessons_learned() {
        let scenario = StressScenario::new(
            StressScenarioCategory::ExpiryCompression,
            StressSeverity::Moderate,
            "Test".to_string(),
            1000000,
            false,
        );

        let mut result = StressTestResult::new(scenario, 0.5, true, false, "Tested".to_string());

        result.add_lesson("Confidence degraded appropriately".to_string());
        result.add_lesson("Signal suppression worked as expected".to_string());

        assert_eq!(result.lessons_learned.len(), 2);
    }
}
