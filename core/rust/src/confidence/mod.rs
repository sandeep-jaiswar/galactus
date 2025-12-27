//! Galactus Confidence and Stability Module
//! 
//! This module implements the confidence and stability model as defined in:
//! `docs/05-intent-engine/confidence-and-stability.md`
//!
//! ## Core Responsibilities
//!
//! - Compute confidence scores from multiple dimensions
//! - Compute stability indicators
//! - Determine when inference must be silenced
//! - Ensure conservative degradation under uncertainty
//!
//! ## Design Principles
//!
//! 1. **Determinism**: All computations are pure functions with no hidden state
//! 2. **Conservative**: Geometric mean ensures single weak component degrades overall score
//! 3. **Explicit**: All thresholds and formulas are documented and non-negotiable
//! 4. **Silent over Wrong**: System prefers silence to false confidence
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use galactus_core::confidence::*;
//!
//! // Assess confidence from multiple dimensions
//! let confidence = assess_confidence(
//!     &data_quality_input,
//!     &structural_input,
//!     &regime_input,
//!     &signal_input,
//!     stability_score,
//! );
//!
//! // Assess stability
//! let stability = assess_stability(
//!     &temporal_input,
//!     &sensitivity_input,
//!     &regime_stability_input,
//! );
//!
//! // Evaluate whether to silence inference
//! match evaluate_silence(&confidence, &stability) {
//!     SilenceDecision::Proceed => { /* emit inference */ },
//!     SilenceDecision::ProceedWithWarning(warnings) => { /* emit with warnings */ },
//!     SilenceDecision::Suppress(reason) => { /* stay silent */ },
//! }
//! ```

pub mod types;
pub mod confidence;
pub mod stability;
pub mod evaluation;

pub use types::*;
pub use confidence::*;
pub use stability::*;
pub use evaluation::*;
