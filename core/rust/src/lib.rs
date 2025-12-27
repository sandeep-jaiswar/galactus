// Galactus Core Library
// Production Rust inference engine for capital pressure detection

pub mod data;
pub mod ingestion;
pub mod features;
pub mod intent;
pub mod regime;
pub mod confidence;
pub mod failure_analysis;
pub mod stress_scenarios;
pub mod api;
pub mod config;
pub mod persistence;

// Re-export main types for convenience
pub use data::*;
pub use features::*;
pub use intent::*;
pub use api::*;
pub use config::*;
pub use persistence::*;
