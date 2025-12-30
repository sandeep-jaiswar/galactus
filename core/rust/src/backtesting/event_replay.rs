//! Event Replay Engine
//!
//! Replays historical market events in strict event-time order.
//! Guarantees:
//! - No future leakage
//! - Deterministic sequencing
//! - Late-event handling (re-computation triggers)

use crate::data::CanonicalEvent;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// Controls the replay clock
#[derive(Debug, Clone, Copy)]
pub enum ReplaySpeed {
    /// Real-time (1x)
    RealTime,
    /// Accelerated (10x)
    Fast10x,
    /// Accelerated (100x)
    Fast100x,
}

impl ReplaySpeed {
    pub fn multiplier(&self) -> f64 {
        match self {
            ReplaySpeed::RealTime => 1.0,
            ReplaySpeed::Fast10x => 10.0,
            ReplaySpeed::Fast100x => 100.0,
        }
    }
}

/// Manages event replay with strict ordering guarantees
pub struct EventReplayEngine {
    /// All historical events in event-time order
    events: Vec<CanonicalEvent>,

    /// Current position in the event stream
    current_index: usize,

    /// Current event-time
    current_time: DateTime<Utc>,

    /// Configuration
    speed: ReplaySpeed,

    /// Events that arrived late (after current_time)
    late_events: VecDeque<CanonicalEvent>,

    /// Whether late events should trigger re-computation
    trigger_recompute_on_late_events: bool,
}

impl EventReplayEngine {
    /// Create a new replay engine
    pub fn new(mut events: Vec<CanonicalEvent>, speed: ReplaySpeed) -> Self {
        // Events must be sorted by event-time
        events.sort_by(|a, b| a.event_time.cmp(&b.event_time));

        let current_time = events
            .first()
            .map(|e| e.event_time)
            .unwrap_or_else(Utc::now);

        EventReplayEngine {
            events,
            current_index: 0,
            current_time,
            speed,
            late_events: VecDeque::new(),
            trigger_recompute_on_late_events: true,
        }
    }

    /// Get the next batch of events that should be processed at this time
    ///
    /// Returns:
    /// - Events that arrived on-time (event_time <= current_time)
    /// - Any late events that have now become available
    pub fn get_next_events(&mut self) -> Vec<CanonicalEvent> {
        let mut result = Vec::new();

        // Add any queued late events first
        while let Some(late_event) = self.late_events.pop_front() {
            if late_event.event_time <= self.current_time {
                result.push(late_event);
            } else {
                self.late_events.push_front(late_event);
                break;
            }
        }

        // Add on-time events
        while self.current_index < self.events.len() {
            let event = &self.events[self.current_index];
            if event.event_time <= self.current_time {
                result.push(event.clone());
                self.current_index += 1;
            } else {
                break;
            }
        }

        result
    }

    /// Advance the replay clock by one interval
    /// Returns the new current time
    pub fn advance_time(&mut self, interval_seconds: u64) -> DateTime<Utc> {
        let multiplied_seconds = (interval_seconds as f64 * self.speed.multiplier()) as u64;
        self.current_time += chrono::Duration::seconds(multiplied_seconds as i64);
        self.current_time
    }

    /// Manually inject a late event (e.g., from a data source with latency)
    /// Queues it for processing when the clock advances to its timestamp
    pub fn inject_late_event(&mut self, event: CanonicalEvent) {
        // Queue late event for processing when clock advances to its timestamp
        self.late_events.push_back(event);
    }

    /// Check if there are late events pending recomputation
    pub fn has_pending_recompute(&self) -> bool {
        !self.late_events.is_empty() && self.trigger_recompute_on_late_events
    }

    /// Get the current event-time
    pub fn current_time(&self) -> DateTime<Utc> {
        self.current_time
    }

    /// Get the current position in the event stream
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Is the replay complete?
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.events.len() && self.late_events.is_empty()
    }

    /// Get progress as a percentage (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.events.is_empty() {
            return 1.0;
        }
        (self.current_index as f64 / self.events.len() as f64).min(1.0)
    }

    /// Get the number of events that have been replayed so far
    pub fn events_replayed(&self) -> usize {
        self.current_index
    }

    /// Get the total number of events
    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    /// Check if we're in a stress period by analyzing event statistics
    ///
    /// Returns true if:
    /// - Volatility spike detected
    /// - Volume spike detected
    /// - OI decay anomaly
    pub fn is_stress_period(&self) -> bool {
        // This would need to analyze recent events
        // For now, placeholder implementation
        false
    }

    /// Get a window of recent events (up to N most recent)
    pub fn recent_events(&self, count: usize) -> Vec<&CanonicalEvent> {
        let start_index = self.current_index.saturating_sub(count);
        self.events[start_index..self.current_index]
            .iter()
            .collect()
    }

    /// Get the number of events between two timestamps
    pub fn event_count_between(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> usize {
        self.events
            .iter()
            .filter(|e| e.event_time > start && e.event_time <= end)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_ordering() {
        let t1 = Utc::now();
        let t2 = t1 + chrono::Duration::seconds(10);
        let t3 = t2 + chrono::Duration::seconds(10);

        // Create events in reverse order
        let mut events = vec![
            CanonicalEvent {
                event_id: crate::data::EventId("e3".to_string()),
                event_type: crate::data::EventType::System,
                event_time: t3,
                source: crate::data::DataSource("Realtime".to_string()),
                instruments: vec![crate::data::Instrument("NIFTY".to_string())],
                payload: crate::data::EventPayload::NoOp,
                completeness: crate::data::Completeness::Complete,
                schema_version: crate::data::SchemaVersion("1".to_string()),
            },
            CanonicalEvent {
                event_id: crate::data::EventId("e1".to_string()),
                event_type: crate::data::EventType::System,
                event_time: t1,
                source: crate::data::DataSource("Realtime".to_string()),
                instruments: vec![crate::data::Instrument("NIFTY".to_string())],
                payload: crate::data::EventPayload::NoOp,
                completeness: crate::data::Completeness::Complete,
                schema_version: crate::data::SchemaVersion("1".to_string()),
            },
            CanonicalEvent {
                event_id: crate::data::EventId("e2".to_string()),
                event_type: crate::data::EventType::System,
                event_time: t2,
                source: crate::data::DataSource("Realtime".to_string()),
                instruments: vec![crate::data::Instrument("NIFTY".to_string())],
                payload: crate::data::EventPayload::NoOp,
                completeness: crate::data::Completeness::Complete,
                schema_version: crate::data::SchemaVersion("1".to_string()),
            },
        ];

        let engine = EventReplayEngine::new(events, ReplaySpeed::RealTime);

        // Events should be sorted
        for i in 0..engine.events.len() - 1 {
            assert!(engine.events[i].event_time <= engine.events[i + 1].event_time);
        }
    }

    #[test]
    fn test_replay_progress() {
        let t1 = Utc::now();
        let events = vec![
            CanonicalEvent {
                event_id: crate::data::EventId("e1".to_string()),
                event_type: crate::data::EventType::System,
                event_time: t1,
                source: crate::data::DataSource("Realtime".to_string()),
                instruments: vec![crate::data::Instrument("NIFTY".to_string())],
                payload: crate::data::EventPayload::NoOp,
                completeness: crate::data::Completeness::Complete,
                schema_version: crate::data::SchemaVersion("1".to_string()),
            },
            CanonicalEvent {
                event_id: crate::data::EventId("e2".to_string()),
                event_type: crate::data::EventType::System,
                event_time: t1 + chrono::Duration::seconds(10),
                source: crate::data::DataSource("Realtime".to_string()),
                instruments: vec![crate::data::Instrument("NIFTY".to_string())],
                payload: crate::data::EventPayload::NoOp,
                completeness: crate::data::Completeness::Complete,
                schema_version: crate::data::SchemaVersion("1".to_string()),
            },
        ];

        let mut engine = EventReplayEngine::new(events, ReplaySpeed::RealTime);

        assert_eq!(engine.progress(), 0.0);
        engine.advance_time(1);
        let next = engine.get_next_events();
        assert!(!next.is_empty());
        assert!(engine.progress() > 0.0);
    }
}
