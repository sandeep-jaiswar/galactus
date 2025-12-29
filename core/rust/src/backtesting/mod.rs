//! Galactus Backtesting Harness
//!
//! Deterministic replay and structural evaluation of inference outputs.
//!
//! See: `docs/07-backtesting-and-validation/galactus-backtesting-harness.md`
//!
//! ## Core Components
//!
//! - **Event Replay Engine**: Replays historical events in strict event-time order
//! - **Snapshot Recorder**: Captures frozen inference snapshots at each tick
//! - **Evaluator**: Structural evaluation without outcome awareness
//! - **Failure Ledger**: Categorizes and logs structural failures
//!
//! ## Design Principles
//!
//! 1. **Event-time correctness** — No future leakage
//! 2. **Deterministic replay** — Same events, same order, same results
//! 3. **Frozen logic** — No parameter tuning
//! 4. **No outcome awareness** — Price is context, never a judge
//! 5. **Failure-first evaluation** — Study failures, not wins
//! 6. **Confidence honesty** — Did Galactus suppress when uncertain?

pub mod event_replay;
pub mod evaluator;
pub mod failure_ledger;
pub mod snapshot_recorder;
pub mod types;

pub use event_replay::*;
pub use evaluator::*;
pub use failure_ledger::*;
pub use snapshot_recorder::*;
pub use types::*;
