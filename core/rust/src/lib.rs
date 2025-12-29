// Galactus Core Library
// Production Rust inference engine for capital pressure detection

pub mod api;
pub mod confidence;
pub mod config;
pub mod data;
pub mod failure_analysis;
pub mod features;
pub mod ingestion;
pub mod intent;
pub mod kill_switch;
pub mod output_validation;
pub mod persistence;
pub mod regime;
pub mod stress_scenarios;

// Re-export main types for convenience - avoid ambiguous exports
pub use api::types;
pub use config::{ConfigManager, GalactusConfig, PersistenceConfig};
pub use data::{CanonicalEvent, DataQuality, EventType};
pub use features::registry::{FeatureConfig, FeatureResult};
pub use intent::{IntentEngine, IntentResult, IntentVector, SignalInput};
pub use persistence::PersistenceError;
