//! Failure Ledger for Backtesting
//!
//! Immutable, append-only ledger of failures identified during backtesting.
//! Enriches the failure_analysis module with backtesting-specific context.

use crate::backtesting::types::BacktestFailure;
use crate::failure_analysis::{FailureCategory, FailureRecord};
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use std::sync::Arc;
use std::sync::Mutex;

/// Immutable ledger of all backtesting failures
pub struct BacktestFailureLedger {
    /// All recorded failures (append-only)
    failures: Arc<Mutex<Vec<BacktestFailure>>>,

    /// Current ledger version
    version: u64,
}

impl BacktestFailureLedger {
    /// Create a new failure ledger
    pub fn new() -> Self {
        BacktestFailureLedger {
            failures: Arc::new(Mutex::new(Vec::new())),
            version: 1,
        }
    }

    /// Record a failure (append-only operation)
    pub fn record_failure(&self, failure: BacktestFailure) {
        let mut failures = self.failures.lock().unwrap();
        failures.push(failure);
    }

    /// Record multiple failures (batch operation)
    pub fn record_failures(&self, failures: Vec<BacktestFailure>) {
        let mut ledger = self.failures.lock().unwrap();
        ledger.extend(failures);
    }

    /// Get all failures
    pub fn get_all_failures(&self) -> Vec<BacktestFailure> {
        self.failures.lock().unwrap().clone()
    }

    /// Get failures for a specific instrument
    pub fn get_instrument_failures(&self, instrument: &str) -> Vec<BacktestFailure> {
        self.failures
            .lock()
            .unwrap()
            .iter()
            .filter(|f| f.instrument == instrument)
            .cloned()
            .collect()
    }

    /// Get failures by category
    pub fn get_failures_by_category(
        &self,
        category: &super::types::BacktestFailureCategory,
    ) -> Vec<BacktestFailure> {
        self.failures
            .lock()
            .unwrap()
            .iter()
            .filter(|f| f.category == *category)
            .cloned()
            .collect()
    }

    /// Get failures in a time window
    pub fn get_failures_in_window(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<BacktestFailure> {
        self.failures
            .lock()
            .unwrap()
            .iter()
            .filter(|f| f.timestamp >= start && f.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Get high-confidence failures (worst case)
    pub fn get_high_confidence_failures(&self, threshold: f64) -> Vec<BacktestFailure> {
        let threshold_ordered = OrderedFloat(threshold);
        self.failures
            .lock()
            .unwrap()
            .iter()
            .filter(|f| f.confidence_at_failure > threshold_ordered)
            .cloned()
            .collect()
    }

    /// Get the total number of failures
    pub fn failure_count(&self) -> usize {
        self.failures.lock().unwrap().len()
    }

    /// Export failures as JSON
    pub fn export_as_json(&self) -> Result<String, serde_json::Error> {
        let failures = self.failures.lock().unwrap();
        serde_json::to_string_pretty(&*failures)
    }

    /// Export failures as CSV (for analysis in spreadsheets)
    pub fn export_as_csv(&self) -> String {
        use std::fmt::Write;

        let failures = self.failures.lock().unwrap();

        let mut csv = String::new();

        // Header
        writeln!(
            csv,
            "Timestamp,Event Seq,Instrument,Regime,Confidence,Was Silenced,Category,Description,Root Cause"
        )
        .unwrap();

        // Rows
        for failure in failures.iter() {
            writeln!(
                csv,
                "{},{},{},{},{},{},{},\"{}\",\"{}\"",
                failure.timestamp.to_rfc3339(),
                failure.event_sequence,
                failure.instrument,
                failure.regime,
                failure.confidence_at_failure,
                failure.was_silenced,
                failure.category.as_str(),
                failure.description.replace("\"", "\"\""),
                failure.root_cause.replace("\"", "\"\""),
            )
            .unwrap();
        }

        csv
    }

    /// Get failure summary statistics
    pub fn get_summary(&self) -> FailureLedgerSummary {
        let failures = self.failures.lock().unwrap();

        let total = failures.len();
        let by_category = count_by_category(&failures);
        let by_instrument = count_by_instrument(&failures);

        let high_confidence = failures
            .iter()
            .filter(|f| f.confidence_at_failure > OrderedFloat(0.8))
            .count();

        let while_silenced = failures.iter().filter(|f| f.was_silenced).count();

        FailureLedgerSummary {
            total_failures: total,
            high_confidence_failures: high_confidence,
            failures_while_silenced: while_silenced,
            failures_by_category: by_category,
            failures_by_instrument: by_instrument,
        }
    }

    /// Convert backtesting failures to general failure records (for cross-system analysis)
    pub fn to_failure_records(&self) -> Vec<FailureRecord> {
        self.failures
            .lock()
            .unwrap()
            .iter()
            .map(|bf| {
                let category = match bf.category {
                    super::types::BacktestFailureCategory::RegimeLag => {
                        FailureCategory::RegimeMisclassification
                    }
                    super::types::BacktestFailureCategory::FalsePressure => {
                        FailureCategory::ModelAssumptionFailure
                    }
                    super::types::BacktestFailureCategory::Overconfidence => {
                        FailureCategory::OverconfidenceFailure
                    }
                    super::types::BacktestFailureCategory::IncorrectSilence => {
                        FailureCategory::SignalMisapplication
                    }
                    super::types::BacktestFailureCategory::IncorrectActivation => {
                        FailureCategory::SignalMisapplication
                    }
                    super::types::BacktestFailureCategory::KillSwitchLate
                    | super::types::BacktestFailureCategory::KillSwitchEarly => {
                        FailureCategory::DataFailure
                    }
                    super::types::BacktestFailureCategory::Other => {
                        FailureCategory::ModelAssumptionFailure
                    }
                };

                FailureRecord::new(
                    category,
                    bf.description.clone(),
                    bf.root_cause.clone(),
                    vec![bf.instrument.clone()],
                )
            })
            .collect()
    }

    /// Get pattern analysis (which failures co-occur?)
    pub fn analyze_failure_patterns(&self) -> FailurePatterns {
        let failures = self.failures.lock().unwrap();

        let mut patterns = FailurePatterns {
            regime_failure_correlation: BTreeMap::new(),
            instrument_failure_correlation: BTreeMap::new(),
            time_between_failures: Vec::new(),
        };

        // Analyze regime correlation
        for failure in failures.iter() {
            let entry = patterns
                .regime_failure_correlation
                .entry(failure.regime.clone())
                .or_insert(0);
            *entry += 1;
        }

        // Analyze instrument correlation
        for failure in failures.iter() {
            let entry = patterns
                .instrument_failure_correlation
                .entry(failure.instrument.clone())
                .or_insert(0);
            *entry += 1;
        }

        // Analyze time between failures
        let mut prev_time: Option<DateTime<Utc>> = None;
        for failure in failures.iter() {
            if let Some(pt) = prev_time {
                let duration = failure.timestamp.signed_duration_since(pt);
                patterns
                    .time_between_failures
                    .push(duration.num_seconds() as f64 / 60.0); // Minutes
            }
            prev_time = Some(failure.timestamp);
        }

        patterns
    }
}

impl Default for BacktestFailureLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for BacktestFailureLedger {
    fn clone(&self) -> Self {
        BacktestFailureLedger {
            failures: Arc::clone(&self.failures),
            version: self.version,
        }
    }
}

/// Summary of failure ledger
#[derive(Debug, Clone)]
pub struct FailureLedgerSummary {
    pub total_failures: usize,
    pub high_confidence_failures: usize,
    pub failures_while_silenced: usize,
    pub failures_by_category: std::collections::BTreeMap<String, usize>,
    pub failures_by_instrument: std::collections::BTreeMap<String, usize>,
}

/// Failure patterns and correlations
#[derive(Debug, Clone)]
pub struct FailurePatterns {
    pub regime_failure_correlation: std::collections::BTreeMap<String, usize>,
    pub instrument_failure_correlation: std::collections::BTreeMap<String, usize>,
    pub time_between_failures: Vec<f64>, // In minutes
}

// Helper functions
use std::collections::BTreeMap;

fn count_by_category(failures: &[BacktestFailure]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for failure in failures {
        let entry = counts
            .entry(failure.category.as_str().to_string())
            .or_insert(0);
        *entry += 1;
    }
    counts
}

fn count_by_instrument(failures: &[BacktestFailure]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for failure in failures {
        let entry = counts.entry(failure.instrument.clone()).or_insert(0);
        *entry += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_failure(
        timestamp: DateTime<Utc>,
        category: super::super::types::BacktestFailureCategory,
    ) -> BacktestFailure {
        BacktestFailure {
            timestamp,
            event_sequence: 0,
            instrument: "NIFTY".to_string(),
            regime: "Normal".to_string(),
            confidence_at_failure: OrderedFloat(0.85),
            was_silenced: false,
            category,
            description: "Test failure".to_string(),
            root_cause: "Testing".to_string(),
            corrective_action: None,
        }
    }

    #[test]
    fn test_failure_ledger_append_only() {
        let ledger = BacktestFailureLedger::new();
        let now = Utc::now();

        ledger.record_failure(create_test_failure(
            now,
            super::super::types::BacktestFailureCategory::RegimeLag,
        ));
        assert_eq!(ledger.failure_count(), 1);

        ledger.record_failure(create_test_failure(
            now + chrono::Duration::seconds(10),
            super::super::types::BacktestFailureCategory::Overconfidence,
        ));
        assert_eq!(ledger.failure_count(), 2);
    }

    #[test]
    fn test_failure_ledger_filtering() {
        let ledger = BacktestFailureLedger::new();
        let now = Utc::now();

        ledger.record_failure(create_test_failure(
            now,
            super::super::types::BacktestFailureCategory::RegimeLag,
        ));

        let filtered = ledger
            .get_failures_by_category(&super::super::types::BacktestFailureCategory::RegimeLag);
        assert_eq!(filtered.len(), 1);
    }
}
