//! Backtesting Harness Types

use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Uniquely identifies an event in the replay sequence
pub type EventSequenceNumber = u64;

/// Unique identifier for an inference snapshot
pub type SnapshotId = String;

/// Instrument identifier (e.g., "NIFTY", "BANKNIFTY")
pub type Instrument = String;

/// A frozen inference snapshot at a specific point in event-time
///
/// This is the ground truth of what Galactus believed at this moment.
/// It is immutable and append-only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceSnapshot {
    /// Unique identifier for this snapshot
    pub snapshot_id: SnapshotId,

    /// Event-time when this snapshot was created
    pub event_timestamp: DateTime<Utc>,

    /// Sequence number in the event replay
    pub event_sequence: EventSequenceNumber,

    /// Record-time (when snapshot was actually recorded)
    pub record_timestamp: DateTime<Utc>,

    /// Which instrument this inference is about
    pub instrument: Instrument,

    /// Regime classification
    pub regime: RegimeSnapshot,

    /// Capital pressure detection
    pub capital_pressure: PressureSnapshot,

    /// Forced flow estimation
    pub forced_flow: ForcedFlowSnapshot,

    /// Liquidity assessment
    pub liquidity: LiquiditySnapshot,

    /// Overall confidence across all signals (0.0 to 1.0)
    pub overall_confidence: f64,

    /// Stability indicator (0.0 = unstable, 1.0 = stable)
    pub stability_indicator: f64,

    /// Is Galactus silent on this instrument?
    pub silenced: bool,

    /// Kill-switch status
    pub kill_switch_status: KillSwitchStatus,

    /// Data quality metrics
    pub data_quality: DataQualitySnapshot,
}

/// Regime classification snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeSnapshot {
    /// Classified regime name
    pub classification: String,

    /// Confidence in this classification (0.0 to 1.0)
    pub confidence: f64,

    /// Signals that support this regime
    pub supporting_signals: Vec<String>,

    /// Signals that contradict this regime
    pub conflicting_signals: Vec<String>,

    /// Time since last regime transition (seconds)
    pub time_in_regime: u64,
}

/// Capital pressure detection snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureSnapshot {
    /// Is capital pressure detected?
    pub detected: bool,

    /// Estimated intensity (0.0 to 1.0)
    pub intensity: f64,

    /// Confidence in the detection (0.0 to 1.0)
    pub confidence: f64,

    /// Sources of pressure
    pub sources: Vec<String>,

    /// False positive rate for this instrument
    pub false_positive_rate: f64,
}

/// Forced flow estimation snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForcedFlowSnapshot {
    /// Estimated magnitude of forced flow
    pub estimated_magnitude: f64,

    /// Confidence in the estimate (0.0 to 1.0)
    pub confidence: f64,

    /// Primary driver (e.g., "expiry", "rebalance", "liquidation")
    pub primary_driver: Option<String>,

    /// Estimated number of minutes until completion
    pub estimated_duration_minutes: Option<u32>,
}

/// Liquidity assessment snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquiditySnapshot {
    /// Bid-ask spread in rupees
    pub bid_ask_spread: f64,

    /// Depth (approximate number of shares at best 5 levels)
    pub depth: f64,

    /// Qualitative assessment: "excellent", "normal", "degraded", "critical"
    pub quality: String,

    /// Estimated time to execute 10% of daily volume (minutes)
    pub market_impact_time: f64,
}

/// Kill-switch status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KillSwitchStatus {
    /// System is running normally
    Active,
    /// Kill-switch triggered due to data issues
    TriggeredData,
    /// Kill-switch triggered due to structural issues
    TriggeredStructural,
    /// Kill-switch triggered due to extreme volatility
    TriggeredVolatility,
}

impl KillSwitchStatus {
    pub fn is_triggered(&self) -> bool {
        !matches!(self, KillSwitchStatus::Active)
    }

    pub fn as_str(&self) -> &str {
        match self {
            KillSwitchStatus::Active => "active",
            KillSwitchStatus::TriggeredData => "triggered_data",
            KillSwitchStatus::TriggeredStructural => "triggered_structural",
            KillSwitchStatus::TriggeredVolatility => "triggered_volatility",
        }
    }
}

/// Data quality assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualitySnapshot {
    /// Missing data sources
    pub missing_sources: Vec<String>,

    /// Sources with staleness warnings
    pub staleness_warnings: Vec<String>,

    /// Validation failures
    pub validation_failures: Vec<String>,

    /// Overall data quality score (0.0 to 1.0)
    pub quality_score: f64,
}

/// Categories for backtest failures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BacktestFailureCategory {
    /// Regime detection failed (lag, misclassification, conflict)
    RegimeLag,

    /// False positive on capital pressure detection
    FalsePressure,

    /// High confidence during ambiguity or data issues
    Overconfidence,

    /// Incorrect silence (should have spoken)
    IncorrectSilence,

    /// Incorrect activation (should have silenced)
    IncorrectActivation,

    /// Kill-switch triggered too late
    KillSwitchLate,

    /// Kill-switch triggered too early (false positive)
    KillSwitchEarly,

    /// Other structural failure
    Other,
}

impl BacktestFailureCategory {
    pub fn as_str(&self) -> &str {
        match self {
            BacktestFailureCategory::RegimeLag => "regime_lag",
            BacktestFailureCategory::FalsePressure => "false_pressure",
            BacktestFailureCategory::Overconfidence => "overconfidence",
            BacktestFailureCategory::IncorrectSilence => "incorrect_silence",
            BacktestFailureCategory::IncorrectActivation => "incorrect_activation",
            BacktestFailureCategory::KillSwitchLate => "kill_switch_late",
            BacktestFailureCategory::KillSwitchEarly => "kill_switch_early",
            BacktestFailureCategory::Other => "other",
        }
    }
}

/// A failure identified during backtesting evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestFailure {
    /// Timestamp of the failure
    pub timestamp: DateTime<Utc>,

    /// Event sequence number
    pub event_sequence: EventSequenceNumber,

    /// Instrument affected
    pub instrument: Instrument,

    /// Regime at failure time
    pub regime: String,

    /// Overall confidence at time of failure
    pub confidence_at_failure: OrderedFloat<f64>,

    /// Was the system silenced at failure time?
    pub was_silenced: bool,

    /// Category of failure
    pub category: BacktestFailureCategory,

    /// Detailed description
    pub description: String,

    /// Root cause analysis
    pub root_cause: String,

    /// Corrective action (if any)
    pub corrective_action: Option<String>,
}

/// Summary statistics from a backtest run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestMetrics {
    /// Total number of snapshots
    pub total_snapshots: u64,

    /// Total number of failures identified
    pub total_failures: u64,

    /// High-confidence failure rate (failures with confidence > 0.8)
    pub high_confidence_failure_rate: f64,

    /// False pressure rate
    pub false_pressure_rate: f64,

    /// Regime lag (average days)
    pub average_regime_lag_days: f64,

    /// Silence correctness rate
    pub silence_correctness_rate: f64,

    /// Kill-switch anticipation (% triggers before instability)
    pub kill_switch_anticipation: f64,

    /// Confidence calibration error
    pub confidence_calibration_error: f64,

    /// Failures by category
    pub failures_by_category: BTreeMap<String, u64>,

    /// Regimes observed
    pub regimes_observed: Vec<String>,

    /// Stress periods detected
    pub stress_periods_detected: u64,
}

/// Complete backtest run report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestReport {
    /// Name of the backtest run
    pub run_name: String,

    /// Start time of the period
    pub period_start: DateTime<Utc>,

    /// End time of the period
    pub period_end: DateTime<Utc>,

    /// Galactus version tested
    pub galactus_version: String,

    /// All snapshots (append-only ledger)
    pub snapshots: Vec<InferenceSnapshot>,

    /// All failures (failure ledger)
    pub failures: Vec<BacktestFailure>,

    /// Summary metrics
    pub metrics: BacktestMetrics,

    /// When the report was generated
    pub generated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_switch_status_is_triggered() {
        assert!(!KillSwitchStatus::Active.is_triggered());
        assert!(KillSwitchStatus::TriggeredData.is_triggered());
        assert!(KillSwitchStatus::TriggeredStructural.is_triggered());
        assert!(KillSwitchStatus::TriggeredVolatility.is_triggered());
    }

    #[test]
    fn test_backtest_failure_category_as_str() {
        assert_eq!(BacktestFailureCategory::RegimeLag.as_str(), "regime_lag");
        assert_eq!(
            BacktestFailureCategory::Overconfidence.as_str(),
            "overconfidence"
        );
    }
}
