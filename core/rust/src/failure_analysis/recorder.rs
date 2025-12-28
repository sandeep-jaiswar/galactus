//! Failure Recorder
//!
//! Records and persists failure information for later analysis.

use super::types::{DetectionMechanism, FailureRecord};
use std::collections::HashMap;

/// Failure recorder that maintains a log of all failures
///
/// This component is responsible for capturing and storing failure records.
/// In a production system, this would persist to a database or structured log system.
pub struct FailureRecorder {
    /// In-memory store of failures (indexed by ID)
    failures: HashMap<String, FailureRecord>,
}

impl FailureRecorder {
    /// Create a new failure recorder
    pub fn new() -> Self {
        FailureRecorder {
            failures: HashMap::new(),
        }
    }

    /// Record a failure
    ///
    /// This is the primary entry point for documenting failures.
    /// Each failure is assigned a unique ID and stored for analysis.
    pub fn record(&mut self, failure: FailureRecord) -> String {
        let id = failure.id.clone();
        self.failures.insert(id.clone(), failure);
        id
    }

    /// Record a failure with detection mechanism
    pub fn record_with_detection(
        &mut self,
        mut failure: FailureRecord,
        detection: DetectionMechanism,
    ) -> String {
        let detection_str = format!("{:?}", detection);
        failure = failure.with_metadata("detection_mechanism".to_string(), detection_str);
        self.record(failure)
    }

    /// Retrieve a specific failure by ID
    pub fn get(&self, id: &str) -> Option<&FailureRecord> {
        self.failures.get(id)
    }

    /// Get all recorded failures
    pub fn get_all(&self) -> Vec<&FailureRecord> {
        self.failures.values().collect()
    }

    /// Get failures within a time range
    pub fn get_by_time_range(&self, start: u64, end: u64) -> Vec<&FailureRecord> {
        self.failures
            .values()
            .filter(|f| f.timestamp >= start && f.timestamp <= end)
            .collect()
    }

    /// Get failures by category
    pub fn get_by_category(&self, category: &super::types::FailureCategory) -> Vec<&FailureRecord> {
        self.failures
            .values()
            .filter(|f| &f.category == category)
            .collect()
    }

    /// Get failures affecting a specific component
    pub fn get_by_component(&self, component: &str) -> Vec<&FailureRecord> {
        self.failures
            .values()
            .filter(|f| f.affected_components.iter().any(|c| c == component))
            .collect()
    }

    /// Get unaddressed failures (those without corrective actions)
    pub fn get_unaddressed(&self) -> Vec<&FailureRecord> {
        self.failures
            .values()
            .filter(|f| !f.is_addressed())
            .collect()
    }

    /// Count total failures
    pub fn count(&self) -> usize {
        self.failures.len()
    }

    /// Count failures by category
    pub fn count_by_category(&self, category: &super::types::FailureCategory) -> usize {
        self.get_by_category(category).len()
    }

    /// Mark a failure as addressed with a corrective action
    pub fn mark_addressed(&mut self, id: &str, corrective_action: String) -> bool {
        if let Some(failure) = self.failures.get_mut(id) {
            failure.corrective_action = Some(corrective_action);
            true
        } else {
            false
        }
    }

    /// Clear all recorded failures
    ///
    /// This should only be used in testing or when archiving to long-term storage
    pub fn clear(&mut self) {
        self.failures.clear();
    }
}

impl Default for FailureRecorder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::failure_analysis::types::FailureCategory;

    #[test]
    fn test_record_and_retrieve() {
        let mut recorder = FailureRecorder::new();

        let failure = FailureRecord::new(
            FailureCategory::DataFailure,
            "Test failure".to_string(),
            "Test cause".to_string(),
            vec!["component1".to_string()],
        );

        let id = recorder.record(failure.clone());

        let retrieved = recorder.get(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().description, "Test failure");
    }

    #[test]
    fn test_get_by_category() {
        let mut recorder = FailureRecorder::new();

        let failure1 = FailureRecord::new(
            FailureCategory::DataFailure,
            "Data failure 1".to_string(),
            "Cause 1".to_string(),
            vec![],
        );

        let failure2 = FailureRecord::new(
            FailureCategory::ModelAssumptionFailure,
            "Model failure 1".to_string(),
            "Cause 2".to_string(),
            vec![],
        );

        let failure3 = FailureRecord::new(
            FailureCategory::DataFailure,
            "Data failure 2".to_string(),
            "Cause 3".to_string(),
            vec![],
        );

        recorder.record(failure1);
        recorder.record(failure2);
        recorder.record(failure3);

        let data_failures = recorder.get_by_category(&FailureCategory::DataFailure);
        assert_eq!(data_failures.len(), 2);

        let model_failures = recorder.get_by_category(&FailureCategory::ModelAssumptionFailure);
        assert_eq!(model_failures.len(), 1);
    }

    #[test]
    fn test_get_by_component() {
        let mut recorder = FailureRecorder::new();

        let failure = FailureRecord::new(
            FailureCategory::SignalMisapplication,
            "Signal issue".to_string(),
            "Wrong context".to_string(),
            vec!["signal_a".to_string(), "signal_b".to_string()],
        );

        recorder.record(failure);

        let failures_a = recorder.get_by_component("signal_a");
        assert_eq!(failures_a.len(), 1);

        let failures_b = recorder.get_by_component("signal_b");
        assert_eq!(failures_b.len(), 1);

        let failures_c = recorder.get_by_component("signal_c");
        assert_eq!(failures_c.len(), 0);
    }

    #[test]
    fn test_unaddressed_failures() {
        let mut recorder = FailureRecorder::new();

        let failure1 = FailureRecord::new(
            FailureCategory::DataFailure,
            "Failure 1".to_string(),
            "Cause 1".to_string(),
            vec![],
        );

        let failure2 = FailureRecord::new(
            FailureCategory::DataFailure,
            "Failure 2".to_string(),
            "Cause 2".to_string(),
            vec![],
        )
        .with_corrective_action("Fixed".to_string());

        let id1 = recorder.record(failure1);
        recorder.record(failure2);

        let unaddressed = recorder.get_unaddressed();
        assert_eq!(unaddressed.len(), 1);

        recorder.mark_addressed(&id1, "Now fixed".to_string());

        let unaddressed = recorder.get_unaddressed();
        assert_eq!(unaddressed.len(), 0);
    }
}
