//! Failure Analyzer
//!
//! Analyzes failure patterns and triggers learning loops.

use super::recorder::FailureRecorder;
use super::types::{FailureCategory, FailurePattern, FailureRecord, FailureTrigger};
use std::collections::{HashMap, HashSet};

/// Threshold for repeated failures before triggering action
const REPEATED_FAILURE_THRESHOLD: usize = 3;

/// Time window for considering failures as repeated (in seconds)
/// Default: 30 days
const PATTERN_TIME_WINDOW: u64 = 30 * 24 * 60 * 60;

/// Failure analyzer that detects patterns and triggers learning loops
///
/// This component analyzes recorded failures to identify patterns,
/// repeated issues, and structural problems that require intervention.
pub struct FailureAnalyzer {
    recorder: FailureRecorder,
}

impl FailureAnalyzer {
    /// Create a new failure analyzer
    pub fn new() -> Self {
        FailureAnalyzer {
            recorder: FailureRecorder::new(),
        }
    }

    /// Create an analyzer with an existing recorder
    pub fn with_recorder(recorder: FailureRecorder) -> Self {
        FailureAnalyzer { recorder }
    }

    /// Record a failure
    pub fn record_failure(&mut self, failure: FailureRecord) -> String {
        self.recorder.record(failure)
    }

    /// Get the underlying recorder
    pub fn recorder(&self) -> &FailureRecorder {
        &self.recorder
    }

    /// Get mutable access to the recorder
    pub fn recorder_mut(&mut self) -> &mut FailureRecorder {
        &mut self.recorder
    }

    /// Detect patterns in recorded failures
    ///
    /// Analyzes all failures to identify:
    /// - Repeated failures of the same category
    /// - Common affected components
    /// - Patterns that require intervention
    pub fn detect_patterns(&self) -> Vec<FailurePattern> {
        let mut patterns = Vec::new();

        // Group failures by category
        let mut by_category: HashMap<String, Vec<&FailureRecord>> = HashMap::new();

        for failure in self.recorder.get_all() {
            let key = failure.category.as_str().to_string();
            by_category.entry(key).or_default().push(failure);
        }

        // Analyze each category
        for (_, failures) in by_category.iter() {
            if failures.is_empty() {
                continue;
            }

            let category = failures[0].category.clone();

            // Find common components
            let common_components = self.find_common_components(failures);

            // Calculate time window
            let time_window = if failures.len() > 1 {
                let timestamps: Vec<u64> = failures.iter().map(|f| f.timestamp).collect();
                timestamps.iter().max().unwrap() - timestamps.iter().min().unwrap()
            } else {
                0
            };

            // Determine if action is required
            let requires_action = failures.len() >= REPEATED_FAILURE_THRESHOLD;

            // Recommend trigger action
            let recommended_trigger = if requires_action {
                Some(self.recommend_trigger(&category, failures.len()))
            } else {
                None
            };

            patterns.push(FailurePattern {
                category,
                count: failures.len(),
                common_components,
                time_window,
                requires_action,
                recommended_trigger,
            });
        }

        patterns
    }

    /// Detect repeated failures for a specific component
    ///
    /// Returns true if the component has failed repeatedly within the time window
    pub fn has_repeated_failures(&self, component: &str) -> bool {
        let failures = self.recorder.get_by_component(component);

        if failures.len() < REPEATED_FAILURE_THRESHOLD {
            return false;
        }

        // Check if failures are within the time window
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let recent_failures: Vec<_> = failures
            .iter()
            .filter(|f| now - f.timestamp <= PATTERN_TIME_WINDOW)
            .collect();

        recent_failures.len() >= REPEATED_FAILURE_THRESHOLD
    }

    /// Check if a component should be deprecated due to repeated failures
    pub fn should_deprecate_component(&self, component: &str) -> bool {
        let failures = self.recorder.get_by_component(component);

        // Deprecate if there are many unaddressed failures
        let unaddressed = failures.iter().filter(|f| !f.is_addressed()).count();

        unaddressed >= REPEATED_FAILURE_THRESHOLD * 2
    }

    /// Analyze failures for a specific time period
    pub fn analyze_period(&self, start: u64, end: u64) -> PeriodAnalysis {
        let failures = self.recorder.get_by_time_range(start, end);

        let mut category_counts: HashMap<String, usize> = HashMap::new();
        let mut affected_components: HashSet<String> = HashSet::new();

        for failure in failures.iter() {
            let key = failure.category.as_str().to_string();
            *category_counts.entry(key).or_insert(0) += 1;

            for component in &failure.affected_components {
                affected_components.insert(component.clone());
            }
        }

        let addressed_count = failures.iter().filter(|f| f.is_addressed()).count();
        let unaddressed_count = failures.len() - addressed_count;

        PeriodAnalysis {
            total_failures: failures.len(),
            addressed_count,
            unaddressed_count,
            category_breakdown: category_counts,
            affected_components: affected_components.into_iter().collect(),
        }
    }

    /// Generate a learning report from failures
    ///
    /// This creates structured output that can feed into research hypotheses,
    /// signal refinement, and documentation updates.
    pub fn generate_learning_report(&self) -> LearningReport {
        let patterns = self.detect_patterns();
        let all_failures = self.recorder.get_all();

        let mut components_needing_attention = Vec::new();
        let mut deprecation_candidates = Vec::new();

        // Collect unique components
        let mut all_components = HashSet::new();
        for failure in all_failures.iter() {
            for component in &failure.affected_components {
                all_components.insert(component.as_str());
            }
        }

        for component in all_components {
            if self.should_deprecate_component(component) {
                deprecation_candidates.push(component.to_string());
            } else if self.has_repeated_failures(component) {
                components_needing_attention.push(component.to_string());
            }
        }

        LearningReport {
            patterns,
            components_needing_attention,
            deprecation_candidates,
            total_failures: all_failures.len(),
            unaddressed_failures: self.recorder.get_unaddressed().len(),
        }
    }

    // Private helper methods

    fn find_common_components(&self, failures: &[&FailureRecord]) -> Vec<String> {
        if failures.is_empty() {
            return Vec::new();
        }

        // Count component occurrences
        let mut component_counts: HashMap<String, usize> = HashMap::new();

        for failure in failures {
            for component in &failure.affected_components {
                *component_counts.entry(component.clone()).or_insert(0) += 1;
            }
        }

        // Return components that appear in more than half of failures
        let threshold = failures.len().div_ceil(2);

        component_counts
            .into_iter()
            .filter(|(_, count)| *count >= threshold)
            .map(|(component, _)| component)
            .collect()
    }

    fn recommend_trigger(&self, category: &FailureCategory, count: usize) -> FailureTrigger {
        // Recommend based on category and severity
        match category {
            FailureCategory::SignalMisapplication => {
                if count >= REPEATED_FAILURE_THRESHOLD * 2 {
                    FailureTrigger::SignalDeprecation
                } else {
                    FailureTrigger::ModelRevision
                }
            }
            FailureCategory::ModelAssumptionFailure => FailureTrigger::ModelRevision,
            FailureCategory::RegimeMisclassification => {
                if count >= REPEATED_FAILURE_THRESHOLD * 3 {
                    FailureTrigger::ArchitecturalReview
                } else {
                    FailureTrigger::ModelRevision
                }
            }
            FailureCategory::DataFailure => {
                // Data failures shouldn't trigger model changes
                FailureTrigger::ArchitecturalReview
            }
            FailureCategory::OverconfidenceFailure => FailureTrigger::ModelRevision,
        }
    }
}

impl Default for FailureAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis of failures within a specific time period
#[derive(Debug, Clone)]
pub struct PeriodAnalysis {
    pub total_failures: usize,
    pub addressed_count: usize,
    pub unaddressed_count: usize,
    pub category_breakdown: HashMap<String, usize>,
    pub affected_components: Vec<String>,
}

/// Learning report generated from failure analysis
///
/// This report feeds into the learning loop, informing:
/// - Research hypotheses
/// - Signal refinement
/// - Regime evolution
/// - Documentation updates
#[derive(Debug, Clone)]
pub struct LearningReport {
    /// Detected failure patterns
    pub patterns: Vec<FailurePattern>,

    /// Components that have repeated failures but not enough to deprecate
    pub components_needing_attention: Vec<String>,

    /// Components that should be deprecated due to persistent failures
    pub deprecation_candidates: Vec<String>,

    /// Total number of failures recorded
    pub total_failures: usize,

    /// Number of failures without corrective actions
    pub unaddressed_failures: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection() {
        let mut analyzer = FailureAnalyzer::new();

        // Record multiple failures of the same category
        for i in 0..5 {
            let failure = FailureRecord::new(
                FailureCategory::DataFailure,
                format!("Data failure {}", i),
                "Missing data".to_string(),
                vec!["data_source".to_string()],
            );
            analyzer.record_failure(failure);
        }

        let patterns = analyzer.detect_patterns();

        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].count, 5);
        assert_eq!(patterns[0].category, FailureCategory::DataFailure);
        assert!(patterns[0].requires_action);
    }

    #[test]
    fn test_repeated_failures_detection() {
        let mut analyzer = FailureAnalyzer::new();

        // Record failures for a specific component
        for i in 0..4 {
            let failure = FailureRecord::new(
                FailureCategory::SignalMisapplication,
                format!("Signal issue {}", i),
                "Wrong context".to_string(),
                vec!["signal_x".to_string()],
            );
            analyzer.record_failure(failure);
        }

        assert!(analyzer.has_repeated_failures("signal_x"));
        assert!(!analyzer.has_repeated_failures("signal_y"));
    }

    #[test]
    fn test_deprecation_recommendation() {
        let mut analyzer = FailureAnalyzer::new();

        // Record many unaddressed failures
        for i in 0..10 {
            let failure = FailureRecord::new(
                FailureCategory::SignalMisapplication,
                format!("Failure {}", i),
                "Persistent issue".to_string(),
                vec!["bad_signal".to_string()],
            );
            analyzer.record_failure(failure);
        }

        assert!(analyzer.should_deprecate_component("bad_signal"));
    }

    #[test]
    fn test_learning_report_generation() {
        let mut analyzer = FailureAnalyzer::new();

        // Record various failures
        for i in 0..4 {
            let failure = FailureRecord::new(
                FailureCategory::DataFailure,
                format!("Data failure {}", i),
                "Data issues".to_string(),
                vec!["data_component".to_string()],
            );
            analyzer.record_failure(failure);
        }

        let report = analyzer.generate_learning_report();

        assert_eq!(report.total_failures, 4);
        assert_eq!(report.unaddressed_failures, 4);
        assert!(!report.patterns.is_empty());
    }

    #[test]
    fn test_common_components_detection() {
        let mut analyzer = FailureAnalyzer::new();

        // Record failures with common components
        for i in 0..3 {
            let failure = FailureRecord::new(
                FailureCategory::RegimeMisclassification,
                format!("Regime issue {}", i),
                "Misclassification".to_string(),
                vec!["regime_detector".to_string(), "signal_a".to_string()],
            );
            analyzer.record_failure(failure);
        }

        let patterns = analyzer.detect_patterns();
        assert!(!patterns.is_empty());

        let pattern = &patterns[0];
        assert!(pattern
            .common_components
            .contains(&"regime_detector".to_string()));
    }
}
