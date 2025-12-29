//! Snapshot Recorder
//!
//! Captures frozen inference snapshots at each evaluation tick.
//! Snapshots are immutable, append-only, and replayable.

use crate::backtesting::types::*;
use chrono::{DateTime, Utc};
use ordered_float::OrderedFloat;
use std::sync::Arc;
use std::sync::Mutex;

/// Records inference snapshots (immutable, append-only ledger)
pub struct SnapshotRecorder {
    /// Immutable ledger of snapshots
    snapshots: Arc<Mutex<Vec<InferenceSnapshot>>>,

    /// Current sequence number
    current_sequence: Arc<Mutex<EventSequenceNumber>>,

    /// Whether recording is active
    is_active: bool,
}

impl SnapshotRecorder {
    /// Create a new snapshot recorder
    pub fn new() -> Self {
        SnapshotRecorder {
            snapshots: Arc::new(Mutex::new(Vec::new())),
            current_sequence: Arc::new(Mutex::new(0)),
            is_active: true,
        }
    }

    /// Record a new snapshot (append-only)
    ///
    /// Panics if snapshot ID is not unique (prevents duplicate snapshots)
    pub fn record_snapshot(&self, mut snapshot: InferenceSnapshot) {
        if !self.is_active {
            return;
        }

        let mut snapshots = self.snapshots.lock().unwrap();

        // Verify uniqueness
        if snapshots.iter().any(|s| s.snapshot_id == snapshot.snapshot_id) {
            panic!(
                "Duplicate snapshot ID: {}. Snapshots must be immutable.",
                snapshot.snapshot_id
            );
        }

        // Update sequence if not set
        if snapshot.event_sequence == 0 {
            let mut seq = self.current_sequence.lock().unwrap();
            snapshot.event_sequence = *seq;
            *seq += 1;
        }

        // Record the timestamp
        snapshot.record_timestamp = Utc::now();

        snapshots.push(snapshot);
    }

    /// Get all recorded snapshots
    pub fn get_all_snapshots(&self) -> Vec<InferenceSnapshot> {
        self.snapshots.lock().unwrap().clone()
    }

    /// Get snapshots for a specific instrument
    pub fn get_instrument_snapshots(&self, instrument: &str) -> Vec<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.instrument == instrument)
            .cloned()
            .collect()
    }

    /// Get the most recent snapshot for an instrument
    pub fn get_latest_snapshot(&self, instrument: &str) -> Option<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.instrument == instrument)
            .last()
            .cloned()
    }

    /// Get snapshots in a time window
    pub fn get_snapshots_in_window(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.event_timestamp >= start && s.event_timestamp <= end)
            .cloned()
            .collect()
    }

    /// Get the number of recorded snapshots
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.lock().unwrap().len()
    }

    /// Freeze the recorder (no more snapshots can be added)
    pub fn freeze(&mut self) {
        self.is_active = false;
    }

    /// Is the recorder active?
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Get snapshots where Galactus was silenced
    pub fn get_silenced_snapshots(&self) -> Vec<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.silenced)
            .cloned()
            .collect()
    }

    /// Get snapshots where kill-switch was triggered
    pub fn get_kill_switch_snapshots(&self) -> Vec<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.kill_switch_status.is_triggered())
            .cloned()
            .collect()
    }

    /// Get snapshots with high confidence (> threshold)
    pub fn get_high_confidence_snapshots(&self, threshold: f64) -> Vec<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.overall_confidence > threshold)
            .cloned()
            .collect()
    }

    /// Get snapshot by ID
    pub fn get_snapshot_by_id(&self, snapshot_id: &str) -> Option<InferenceSnapshot> {
        self.snapshots
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.snapshot_id == snapshot_id)
            .cloned()
    }

    /// Export snapshots as JSON
    pub fn export_as_json(&self) -> Result<String, serde_json::Error> {
        let snapshots = self.snapshots.lock().unwrap();
        serde_json::to_string_pretty(&*snapshots)
    }

    /// Export snapshots as JSONL (one per line, for streaming)
    pub fn export_as_jsonl(&self) -> Result<String, serde_json::Error> {
        let snapshots = self.snapshots.lock().unwrap();
        let lines: Result<Vec<_>, _> = snapshots
            .iter()
            .map(serde_json::to_string)
            .collect();

        Ok(lines?.join("\n"))
    }

    /// Get statistics about the recorded snapshots
    pub fn get_statistics(&self) -> SnapshotStatistics {
        let snapshots = self.snapshots.lock().unwrap();

        let total = snapshots.len() as u64;
        let silenced = snapshots.iter().filter(|s| s.silenced).count() as u64;
        let kill_switch_triggered = snapshots
            .iter()
            .filter(|s| s.kill_switch_status.is_triggered())
            .count() as u64;

        let avg_confidence = if !snapshots.is_empty() {
            snapshots
                .iter()
                .map(|s| s.overall_confidence)
                .sum::<f64>()
                / snapshots.len() as f64
        } else {
            0.0
        };

        let high_confidence = snapshots
            .iter()
            .filter(|s| s.overall_confidence > 0.8)
            .count() as u64;

        SnapshotStatistics {
            total_snapshots: total,
            silenced_count: silenced,
            kill_switch_triggered_count: kill_switch_triggered,
            average_confidence: avg_confidence,
            high_confidence_count: high_confidence,
        }
    }

    /// Get regime transitions (changes in regime classification)
    pub fn get_regime_transitions(&self) -> Vec<RegimeTransition> {
        let snapshots = self.snapshots.lock().unwrap();
        let mut transitions = Vec::new();

        let mut last_regime: Option<String> = None;

        for snapshot in snapshots.iter() {
            let current_regime = &snapshot.regime.classification;

            if let Some(ref prev) = last_regime {
                if prev != current_regime {
                    transitions.push(RegimeTransition {
                        timestamp: snapshot.event_timestamp,
                        event_sequence: snapshot.event_sequence,
                        from_regime: prev.clone(),
                        to_regime: current_regime.clone(),
                        confidence: snapshot.regime.confidence,
                    });
                }
            }

            last_regime = Some(current_regime.clone());
        }

        transitions
    }
}

impl Default for SnapshotRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SnapshotRecorder {
    fn clone(&self) -> Self {
        SnapshotRecorder {
            snapshots: Arc::clone(&self.snapshots),
            current_sequence: Arc::clone(&self.current_sequence),
            is_active: self.is_active,
        }
    }
}

/// Statistics about recorded snapshots
#[derive(Debug, Clone)]
pub struct SnapshotStatistics {
    pub total_snapshots: u64,
    pub silenced_count: u64,
    pub kill_switch_triggered_count: u64,
    pub average_confidence: f64,
    pub high_confidence_count: u64,
}

/// A regime transition event
#[derive(Debug, Clone)]
pub struct RegimeTransition {
    pub timestamp: DateTime<Utc>,
    pub event_sequence: EventSequenceNumber,
    pub from_regime: String,
    pub to_regime: String,
    pub confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    

    fn create_test_snapshot(id: &str, instrument: &str, regime: &str) -> InferenceSnapshot {
        InferenceSnapshot {
            snapshot_id: id.to_string(),
            event_timestamp: Utc::now(),
            event_sequence: 0,
            record_timestamp: Utc::now(),
            instrument: instrument.to_string(),
            regime: RegimeSnapshot {
                classification: regime.to_string(),
                confidence: OrderedFloat(0.9),
                supporting_signals: vec![],
                conflicting_signals: vec![],
                time_in_regime: 0,
            },
            capital_pressure: PressureSnapshot {
                detected: false,
                intensity: OrderedFloat(0.0),
                confidence: OrderedFloat(0.8),
                sources: vec![],
                false_positive_rate: OrderedFloat(0.05),
            },
            forced_flow: ForcedFlowSnapshot {
                estimated_magnitude: OrderedFloat(0.0),
                confidence: OrderedFloat(0.7),
                primary_driver: None,
                estimated_duration_minutes: None,
            },
            liquidity: LiquiditySnapshot {
                bid_ask_spread: OrderedFloat(10.0),
                depth: OrderedFloat(500.0),
                quality: "normal".to_string(),
                market_impact_time: OrderedFloat(5.0),
            },
            overall_confidence: OrderedFloat(0.85),
            stability_indicator: OrderedFloat(0.9),
            silenced: false,
            kill_switch_status: KillSwitchStatus::Active,
            data_quality: DataQualitySnapshot {
                missing_sources: vec![],
                staleness_warnings: vec![],
                validation_failures: vec![],
                quality_score: OrderedFloat(1.0),
            },
        }
    }

    #[test]
    fn test_snapshot_uniqueness() {
        let recorder = SnapshotRecorder::new();
        let snapshot = create_test_snapshot("snap1", "NIFTY", "Normal");
        recorder.record_snapshot(snapshot.clone());

        // Attempting to record duplicate should panic
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            recorder.record_snapshot(snapshot);
        }));

        assert!(result.is_err());
    }

    #[test]
    fn test_filter_by_instrument() {
        let recorder = SnapshotRecorder::new();
        recorder.record_snapshot(create_test_snapshot("snap1", "NIFTY", "Normal"));
        recorder.record_snapshot(create_test_snapshot("snap2", "BANKNIFTY", "Normal"));
        recorder.record_snapshot(create_test_snapshot("snap3", "NIFTY", "Stress"));

        let nifty = recorder.get_instrument_snapshots("NIFTY");
        assert_eq!(nifty.len(), 2);
    }

    #[test]
    fn test_regime_transitions() {
        let recorder = SnapshotRecorder::new();
        recorder.record_snapshot(create_test_snapshot("snap1", "NIFTY", "Normal"));
        recorder.record_snapshot(create_test_snapshot("snap2", "NIFTY", "Normal"));
        recorder.record_snapshot(create_test_snapshot("snap3", "NIFTY", "Stress"));

        let transitions = recorder.get_regime_transitions();
        assert_eq!(transitions.len(), 1);
        assert_eq!(transitions[0].from_regime, "Normal");
        assert_eq!(transitions[0].to_regime, "Stress");
    }
}
