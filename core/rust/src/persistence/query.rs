//! Query Builder for Intent History
//!
//! Provides a fluent API for querying stored intent vectors.

use super::{PersistenceError, TimestampedIntent};
use chrono::{DateTime, Utc, Duration};

/// Time range for queries
#[derive(Debug, Clone)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    /// Create a time range from start to end
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    /// Create a time range for the last N hours
    pub fn last_hours(hours: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::hours(hours);
        Self { start, end }
    }

    /// Create a time range for the last N days
    pub fn last_days(days: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::days(days);
        Self { start, end }
    }

    /// Create a time range for today
    pub fn today() -> Self {
        let now = Utc::now();
        let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end = now;
        Self { start, end }
    }
}

/// Intent query parameters
#[derive(Debug, Clone)]
pub struct IntentQuery {
    pub time_range: TimeRange,
    pub instruments: Option<Vec<String>>,
    pub min_confidence: Option<f64>,
    pub limit: Option<usize>,
}

impl IntentQuery {
    /// Create a new query for a time range
    pub fn new(time_range: TimeRange) -> Self {
        Self {
            time_range,
            instruments: None,
            min_confidence: None,
            limit: None,
        }
    }

    /// Filter by instruments
    pub fn instruments(mut self, instruments: Vec<String>) -> Self {
        self.instruments = Some(instruments);
        self
    }

    /// Filter by minimum confidence
    pub fn min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = Some(confidence);
        self
    }

    /// Limit number of results
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Apply filters to a list of intent vectors
    pub fn apply(&self, intents: Vec<TimestampedIntent>) -> Vec<TimestampedIntent> {
        let mut results: Vec<TimestampedIntent> = intents
            .into_iter()
            .filter(|intent| {
                // Check time range
                if intent.timestamp < self.time_range.start || intent.timestamp > self.time_range.end {
                    return false;
                }

                // Check instruments filter
                if let Some(ref instruments) = self.instruments {
                    if !instruments.contains(&intent.metadata.instrument) {
                        return false;
                    }
                }

                // Check confidence filter
                if let Some(min_conf) = self.min_confidence {
                    if intent.intent.confidence < min_conf {
                        return false;
                    }
                }

                true
            })
            .collect();

        // Apply limit
        if let Some(limit) = self.limit {
            results.truncate(limit);
        }

        results
    }
}

/// Query builder for fluent API
pub struct QueryBuilder {
    time_range: Option<TimeRange>,
    instruments: Option<Vec<String>>,
    min_confidence: Option<f64>,
    limit: Option<usize>,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            time_range: None,
            instruments: None,
            min_confidence: None,
            limit: None,
        }
    }

    /// Set the time range
    pub fn time_range(mut self, range: TimeRange) -> Self {
        self.time_range = Some(range);
        self
    }

    /// Set the time range to last N hours
    pub fn last_hours(mut self, hours: i64) -> Self {
        self.time_range = Some(TimeRange::last_hours(hours));
        self
    }

    /// Set the time range to last N days
    pub fn last_days(mut self, days: i64) -> Self {
        self.time_range = Some(TimeRange::last_days(days));
        self
    }

    /// Filter by instruments
    pub fn instruments(mut self, instruments: Vec<String>) -> Self {
        self.instruments = Some(instruments);
        self
    }

    /// Filter by minimum confidence
    pub fn min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = Some(confidence);
        self
    }

    /// Limit number of results
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Build the query
    pub fn build(self) -> Result<IntentQuery, PersistenceError> {
        let time_range = self.time_range.ok_or_else(|| {
            PersistenceError::QueryError("Time range is required".to_string())
        })?;

        Ok(IntentQuery {
            time_range,
            instruments: self.instruments,
            min_confidence: self.min_confidence,
            limit: self.limit,
        })
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::IntentVector;
    use crate::persistence::TimestampedIntent;

    #[test]
    fn test_time_range_last_hours() {
        let range = TimeRange::last_hours(24);
        assert!(range.start < range.end);
        let duration = range.end.signed_duration_since(range.start);
        assert_eq!(duration.num_hours(), 24);
    }

    #[test]
    fn test_time_range_last_days() {
        let range = TimeRange::last_days(7);
        let duration = range.end.signed_duration_since(range.start);
        assert_eq!(duration.num_days(), 7);
    }

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .last_hours(24)
            .instruments(vec!["NIFTY".to_string()])
            .min_confidence(0.7)
            .limit(100)
            .build();

        assert!(query.is_ok());
        let q = query.unwrap();
        assert!(q.instruments.is_some());
        assert_eq!(q.instruments.unwrap(), vec!["NIFTY".to_string()]);
        assert_eq!(q.min_confidence, Some(0.7));
        assert_eq!(q.limit, Some(100));
    }

    #[test]
    fn test_query_builder_missing_time_range() {
        let query = QueryBuilder::new()
            .instruments(vec!["NIFTY".to_string()])
            .build();

        assert!(query.is_err());
    }

    #[test]
    fn test_query_apply_filters() {
        let intent1 = TimestampedIntent::new(
            IntentVector {
                pressure: 0.5,
                confidence: 0.8,
                signals: std::collections::HashMap::new(),
                timestamp: 0,
                regime: "test".to_string(),
            },
            "NIFTY".to_string(),
            vec!["test".to_string()],
        );

        let intent2 = TimestampedIntent::new(
            IntentVector {
                pressure: -0.5,
                confidence: 0.5,
                signals: std::collections::HashMap::new(),
                timestamp: 0,
                regime: "test".to_string(),
            },
            "BANKNIFTY".to_string(),
            vec!["test".to_string()],
        );

        let query = QueryBuilder::new()
            .last_hours(24)
            .instruments(vec!["NIFTY".to_string()])
            .min_confidence(0.7)
            .build()
            .unwrap();

        let results = query.apply(vec![intent1.clone(), intent2]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].metadata.instrument, "NIFTY");
    }
}
