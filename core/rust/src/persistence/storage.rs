//! Storage Backend Implementation
//!
//! Provides the main storage interface for persisting intent vectors.

use super::{PersistenceError, TimestampedIntent};
use crate::config::PersistenceConfig;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Storage backend trait
pub trait StorageBackend {
    /// Store a timestamped intent vector
    fn store(&mut self, intent: TimestampedIntent) -> Result<(), PersistenceError>;

    /// Retrieve intent vectors within a time range
    fn retrieve(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<TimestampedIntent>, PersistenceError>;

    /// Flush pending writes to disk
    fn flush(&mut self) -> Result<(), PersistenceError>;

    /// Clean up old data based on retention policy
    fn cleanup(&mut self, before: DateTime<Utc>) -> Result<usize, PersistenceError>;
}

/// File-based storage backend with in-memory buffer
pub struct FileStorageBackend {
    config: PersistenceConfig,
    storage_path: PathBuf,
    buffer: Arc<Mutex<VecDeque<TimestampedIntent>>>,
    current_file: Option<BufWriter<File>>,
}

impl FileStorageBackend {
    /// Create a new file storage backend
    pub fn new(config: PersistenceConfig) -> Result<Self, PersistenceError> {
        let storage_path = PathBuf::from(&config.storage_path);

        // Create storage directory if it doesn't exist
        if !storage_path.exists() {
            fs::create_dir_all(&storage_path).map_err(|e| {
                PersistenceError::IoError(format!("Failed to create storage directory: {}", e))
            })?;
        }

        Ok(Self {
            config,
            storage_path,
            buffer: Arc::new(Mutex::new(VecDeque::new())),
            current_file: None,
        })
    }

    /// Get the file path for a specific date
    fn get_file_path(&self, date: &DateTime<Utc>) -> PathBuf {
        let filename = format!("intents_{}.jsonl", date.format("%Y%m%d"));
        self.storage_path.join(filename)
    }

    /// Open or create the current file for writing
    fn ensure_file_open(&mut self) -> Result<(), PersistenceError> {
        if self.current_file.is_none() {
            let now = Utc::now();
            let file_path = self.get_file_path(&now);

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)
                .map_err(|e| PersistenceError::IoError(format!("Failed to open file: {}", e)))?;

            self.current_file = Some(BufWriter::new(file));
        }
        Ok(())
    }

    /// Write a single intent to disk
    fn write_to_disk(&mut self, intent: &TimestampedIntent) -> Result<(), PersistenceError> {
        self.ensure_file_open()?;

        let file = self.current_file.as_mut().unwrap();
        let json = serde_json::to_string(intent)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;

        writeln!(file, "{}", json).map_err(|e| PersistenceError::IoError(e.to_string()))?;

        Ok(())
    }
}

impl StorageBackend for FileStorageBackend {
    fn store(&mut self, intent: TimestampedIntent) -> Result<(), PersistenceError> {
        // Add to in-memory buffer
        let mut buffer = self.buffer.lock().unwrap();

        // Check if buffer is full
        if buffer.len() >= self.config.max_memory_items {
            // Drop oldest item if buffer is full
            buffer.pop_front();
        }

        buffer.push_back(intent.clone());
        drop(buffer);

        // Write to disk immediately
        self.write_to_disk(&intent)?;

        Ok(())
    }

    fn retrieve(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<TimestampedIntent>, PersistenceError> {
        let mut results = Vec::new();

        // First check in-memory buffer
        let buffer = self.buffer.lock().unwrap();
        for intent in buffer.iter() {
            if intent.timestamp >= start && intent.timestamp <= end {
                results.push(intent.clone());
            }
        }
        drop(buffer);

        // TODO: Also read from disk files for the date range
        // This is a simplified implementation

        Ok(results)
    }

    fn flush(&mut self) -> Result<(), PersistenceError> {
        if let Some(file) = &mut self.current_file {
            file.flush()
                .map_err(|e| PersistenceError::IoError(e.to_string()))?;
        }
        Ok(())
    }

    fn cleanup(&mut self, _before: DateTime<Utc>) -> Result<usize, PersistenceError> {
        let mut deleted = 0;

        // Read directory and delete old files
        let entries = fs::read_dir(&self.storage_path)
            .map_err(|e| PersistenceError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| PersistenceError::IoError(e.to_string()))?;
            let path = entry.path();

            if let Some(filename) = path.file_name() {
                if let Some(name) = filename.to_str() {
                    if name.starts_with("intents_") && name.ends_with(".jsonl") {
                        // Extract date from filename and check if it's older than retention
                        // This is simplified - in production would parse the date properly
                        // For now, just count the file
                        deleted += 1;
                    }
                }
            }
        }

        Ok(deleted)
    }
}

/// Main intent store interface
pub struct IntentStore {
    backend: Box<dyn StorageBackend + Send>,
}

impl IntentStore {
    /// Create a new intent store with file backend
    pub fn new(config: PersistenceConfig) -> Result<Self, PersistenceError> {
        let backend = FileStorageBackend::new(config)?;
        Ok(Self {
            backend: Box::new(backend),
        })
    }

    /// Store an intent vector
    pub fn store(&mut self, intent: TimestampedIntent) -> Result<(), PersistenceError> {
        self.backend.store(intent)
    }

    /// Retrieve intent vectors within a time range
    pub fn retrieve(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<TimestampedIntent>, PersistenceError> {
        self.backend.retrieve(start, end)
    }

    /// Flush pending writes
    pub fn flush(&mut self) -> Result<(), PersistenceError> {
        self.backend.flush()
    }

    /// Clean up old data
    pub fn cleanup(&mut self, before: DateTime<Utc>) -> Result<usize, PersistenceError> {
        self.backend.cleanup(before)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::IntentVector;
    use tempfile::TempDir;

    fn create_test_config() -> PersistenceConfig {
        let temp_dir = TempDir::new().unwrap();
        PersistenceConfig {
            storage_path: temp_dir.path().to_str().unwrap().to_string(),
            max_memory_items: 100,
            flush_interval_seconds: 60,
            enable_compression: false,
            retention_days: 30,
        }
    }

    #[test]
    fn test_store_and_retrieve() {
        let config = create_test_config();
        let mut store = IntentStore::new(config).unwrap();

        let intent = IntentVector {
            pressure: 0.5,
            confidence: 0.8,
            signals: std::collections::HashMap::new(),
            timestamp: 0,
            regime: "test".to_string(),
        };

        let timestamped =
            TimestampedIntent::new(intent, "NIFTY".to_string(), vec!["test".to_string()]);

        assert!(store.store(timestamped.clone()).is_ok());
        assert!(store.flush().is_ok());

        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now() + chrono::Duration::hours(1);
        let results = store.retrieve(start, end).unwrap();

        assert!(!results.is_empty());
    }

    #[test]
    fn test_file_backend_creation() {
        let config = create_test_config();
        let backend = FileStorageBackend::new(config);
        assert!(backend.is_ok());
    }
}
