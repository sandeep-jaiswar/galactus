//! State Persistence Layer
//!
//! Provides time-series storage and retrieval for intent vectors with efficient
//! storage format and query capabilities for historical analysis.
//!
//! # Design Principles
//! - Append-only writes for time-series data
//! - Efficient binary storage format
//! - Fast query capabilities for historical analysis
//! - Automatic data compression
//! - Configurable retention policies

pub mod storage;
pub mod query;
pub mod format;

pub use storage::{IntentStore, StorageBackend};
pub use query::{QueryBuilder, TimeRange, IntentQuery};
pub use format::{StorageFormat, CompressionType};

use crate::intent::IntentVector;
use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Persistence errors
#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceError {
    /// Storage I/O error
    IoError(String),
    /// Serialization error
    SerializationError(String),
    /// Query error
    QueryError(String),
    /// Invalid path
    InvalidPath(String),
    /// Storage full
    StorageFull,
    /// Item not found
    NotFound(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::IoError(msg) => write!(f, "Storage I/O error: {}", msg),
            PersistenceError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            PersistenceError::QueryError(msg) => write!(f, "Query error: {}", msg),
            PersistenceError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            PersistenceError::StorageFull => write!(f, "Storage is full"),
            PersistenceError::NotFound(id) => write!(f, "Item not found: {}", id),
        }
    }
}

impl std::error::Error for PersistenceError {}

/// Intent vector with timestamp for time-series storage
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimestampedIntent {
    pub timestamp: DateTime<Utc>,
    pub intent: IntentVector,
    pub metadata: IntentMetadata,
}

/// Metadata for stored intent vectors
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntentMetadata {
    pub id: String,
    pub instrument: String,
    pub source_signals: Vec<String>,
    pub computation_time_ms: u64,
}

impl TimestampedIntent {
    pub fn new(intent: IntentVector, instrument: String, source_signals: Vec<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            intent,
            metadata: IntentMetadata {
                id: uuid::Uuid::new_v4().to_string(),
                instrument,
                source_signals,
                computation_time_ms: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_timestamped_intent_creation() {
        let intent = IntentVector {
            pressure: 0.5,
            confidence: 0.8,
            signals: HashMap::new(),
            timestamp: 0,
            regime: "test".to_string(),
        };

        let timestamped = TimestampedIntent::new(
            intent,
            "NIFTY".to_string(),
            vec!["hedge_pressure".to_string()],
        );

        assert_eq!(timestamped.metadata.instrument, "NIFTY");
        assert!(!timestamped.metadata.id.is_empty());
    }
}
