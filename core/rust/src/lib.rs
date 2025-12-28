// Galactus Core Library
// Production Rust inference engine for capital pressure detection

pub mod data;
pub mod ingestion;
pub mod features;
pub mod intent;
pub mod regime;
pub mod confidence;
pub mod kill_switch;
pub mod failure_analysis;
pub mod stress_scenarios;
pub mod api;
pub mod config;
pub mod persistence;

// Re-export main types for convenience - avoid ambiguous exports
pub use intent::{IntentEngine, IntentVector, SignalInput, IntentResult};
pub use features::registry::{FeatureResult, FeatureConfig};
pub use data::{CanonicalEvent, EventType, DataQuality};
pub use api::types;
pub use config::{GalactusConfig, ConfigManager, PersistenceConfig};
pub use persistence::PersistenceError;
