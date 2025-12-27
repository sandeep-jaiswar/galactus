// Galactus Confidence and Stability Module
// 
// This module implements the confidence and stability model as defined in:
// docs/05-intent-engine/confidence-and-stability.md
//
// Core Responsibilities:
// - Compute confidence scores from multiple dimensions
// - Compute stability indicators
// - Determine when inference must be silenced
// - Ensure conservative degradation under uncertainty

pub mod types;
pub mod confidence;
pub mod stability;
pub mod evaluation;

pub use types::*;
pub use confidence::*;
pub use stability::*;
pub use evaluation::*;
