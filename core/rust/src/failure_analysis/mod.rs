//! Galactus Failure Analysis Module
//! 
//! This module implements the failure analysis framework as defined in:
//! `docs/07-backtesting-and-validation/failure-analysis.md`
//!
//! ## Core Responsibilities
//!
//! - Categorize inference failures into structured types
//! - Document failures with complete metadata and context
//! - Enable learning from failures through pattern detection
//! - Trigger corrective actions when repeated failures occur
//!
//! ## Design Principles
//!
//! 1. **Structured Classification**: Every failure belongs to a specific category
//! 2. **Complete Documentation**: All failures are recorded with full context
//! 3. **Learning Loop**: Failures feed into research and signal refinement
//! 4. **Transparency**: Failures are not hidden or explained away
//! 5. **Determinism**: Failure categorization is consistent and reproducible
//!
//! ## Philosophy
//!
//! > "Every incorrect inference contains more information than a correct one."
//! 
//! Galactus treats failure as a signal, not an exception. Ignoring failure leads
//! to loss of structural understanding. This module ensures that every failure
//! is captured, categorized, and converted into learning.
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use galactus_core::failure_analysis::*;
//!
//! // Record a data failure
//! let failure = FailureRecord::new(
//!     FailureCategory::DataFailure,
//!     "Missing OI data for last 2 hours".to_string(),
//!     "Data provider outage".to_string(),
//!     vec!["oi_signal".to_string(), "derivatives_pressure".to_string()],
//! ).with_corrective_action("Implemented backup data source".to_string());
//!
//! // Analyze patterns
//! let analyzer = FailureAnalyzer::new();
//! analyzer.record_failure(failure);
//! let patterns = analyzer.detect_patterns();
//! ```

pub mod types;
pub mod recorder;
pub mod analyzer;

pub use types::*;
pub use recorder::*;
pub use analyzer::*;
