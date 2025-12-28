//! Failure Analysis Types
//!
//! Core types for the failure analysis framework.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Categories of failure as defined in the failure analysis framework
///
/// Each category represents a different root cause pattern and requires
/// a different response strategy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FailureCategory {
    /// Data Failures - caused by data issues
    ///
    /// Examples:
    /// - Missing or delayed events
    /// - Incorrect schema interpretation
    /// - Stale or corrupted inputs
    ///
    /// Response: Degrade confidence, improve data diagnostics, do not adjust inference logic
    DataFailure,

    /// Model Assumption Failures - caused by incorrect or outdated assumptions
    ///
    /// Examples:
    /// - Constraint no longer binding
    /// - Market structure change
    /// - Participant behavior shift
    ///
    /// Response: Re-examine assumptions, update documentation, consider model revision
    ModelAssumptionFailure,

    /// Regime Misclassification - incorrect regime inference
    ///
    /// Examples:
    /// - Treating fragile liquidity as normal
    /// - Missing regime transitions
    /// - Overstaying in an outdated regime
    ///
    /// Response: Improve regime detection, increase uncertainty during transitions
    RegimeMisclassification,

    /// Signal Misapplication - valid signal applied in invalid context
    ///
    /// Examples:
    /// - Using expiry-driven signals far from expiry
    /// - Applying derivatives pressure logic in cash-dominant regimes
    ///
    /// Response: Tighten applicability rules, narrow signal scope
    SignalMisapplication,

    /// Overconfidence Failures - inference more certain than evidence allows
    ///
    /// Examples:
    /// - High confidence under degraded data
    /// - Suppressed ambiguity flags
    /// - Ignored conflicting signals
    ///
    /// Response: Strengthen confidence degradation rules, add hard stop conditions
    OverconfidenceFailure,
}

impl FailureCategory {
    /// Returns the category name as a string
    pub fn as_str(&self) -> &str {
        match self {
            FailureCategory::DataFailure => "DataFailure",
            FailureCategory::ModelAssumptionFailure => "ModelAssumptionFailure",
            FailureCategory::RegimeMisclassification => "RegimeMisclassification",
            FailureCategory::SignalMisapplication => "SignalMisapplication",
            FailureCategory::OverconfidenceFailure => "OverconfidenceFailure",
        }
    }

    /// Returns the recommended response strategy for this failure category
    pub fn response_strategy(&self) -> &str {
        match self {
            FailureCategory::DataFailure =>
                "Degrade confidence, improve data diagnostics, do not adjust inference logic to compensate",
            FailureCategory::ModelAssumptionFailure =>
                "Re-examine assumptions, update documentation, consider model revision or deprecation",
            FailureCategory::RegimeMisclassification =>
                "Improve regime detection, increase uncertainty during transitions, add explicit transition states",
            FailureCategory::SignalMisapplication =>
                "Tighten applicability rules, narrow signal scope, improve documentation",
            FailureCategory::OverconfidenceFailure =>
                "Strengthen confidence degradation rules, add new hard stop conditions",
        }
    }
}

/// A complete record of an inference failure
///
/// This structure captures all information needed to understand,
/// reproduce, and learn from a failure.
#[derive(Debug, Clone)]
pub struct FailureRecord {
    /// Unique identifier for this failure
    pub id: String,

    /// When the failure occurred (Unix timestamp in seconds)
    pub timestamp: u64,

    /// Category of the failure
    pub category: FailureCategory,

    /// Human-readable description of what failed
    pub description: String,

    /// Root cause analysis - why the failure occurred
    pub root_cause: String,

    /// List of affected signals or components
    pub affected_components: Vec<String>,

    /// Corrective action taken (if any)
    pub corrective_action: Option<String>,

    /// Additional context or metadata
    pub metadata: Vec<(String, String)>,
}

// Global counter for unique IDs
static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl FailureRecord {
    /// Create a new failure record
    ///
    /// Automatically generates an ID and captures the current timestamp.
    pub fn new(
        category: FailureCategory,
        description: String,
        root_cause: String,
        affected_components: Vec<String>,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Generate unique ID from timestamp, category, and counter
        let counter = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        let id = format!("{}-{}-{}", category.as_str(), timestamp, counter);

        FailureRecord {
            id,
            timestamp,
            category,
            description,
            root_cause,
            affected_components,
            corrective_action: None,
            metadata: Vec::new(),
        }
    }

    /// Add a corrective action to this failure record
    pub fn with_corrective_action(mut self, action: String) -> Self {
        self.corrective_action = Some(action);
        self
    }

    /// Add metadata to this failure record
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.push((key, value));
        self
    }

    /// Check if this failure has been addressed (has a corrective action)
    pub fn is_addressed(&self) -> bool {
        self.corrective_action.is_some()
    }
}

/// Detection mechanism for failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectionMechanism {
    /// Detected through post-event analysis
    PostEventAnalysis,

    /// Detected through research review
    ResearchReview,

    /// Detected through monitoring alerts
    MonitoringAlert,

    /// Detected through consumer feedback (non-actionable)
    ConsumerFeedback,
}

/// Trigger conditions for corrective actions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FailureTrigger {
    /// Trigger signal deprecation
    SignalDeprecation,

    /// Trigger model revision
    ModelRevision,

    /// Trigger architectural review
    ArchitecturalReview,
}

/// Pattern detected from analyzing multiple failures
#[derive(Debug, Clone)]
pub struct FailurePattern {
    /// Category of failures in this pattern
    pub category: FailureCategory,

    /// Number of occurrences
    pub count: usize,

    /// Common affected components
    pub common_components: Vec<String>,

    /// Time window of occurrences (first to last, in seconds)
    pub time_window: u64,

    /// Whether this pattern requires immediate action
    pub requires_action: bool,

    /// Recommended trigger action
    pub recommended_trigger: Option<FailureTrigger>,
}
