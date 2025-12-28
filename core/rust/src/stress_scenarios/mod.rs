//! Galactus Stress Scenario Framework
//!
//! This module implements the stress scenario framework as defined in:
//! `docs/07-backtesting-and-validation/stress-scenarios.md`
//!
//! ## Purpose
//!
//! Validates the robustness, stability, and honesty of signals, features, and inference logic
//! under extreme or abnormal market conditions.
//!
//! Stress testing is not about prediction - it is about **survivability of inference**.
//!
//! ## Core Stress Scenario Categories
//!
//! 1. **Liquidity Stress** - Market absorption capacity collapse
//! 2. **Expiry Compression** - Time constraints dominating behavior near expiries
//! 3. **Volatility Shocks** - Sudden volatility expansion or contraction
//! 4. **Regime Transitions** - Structural market regime shifts
//! 5. **Constraint Overlaps** - Multiple binding constraints acting simultaneously
//! 6. **Data Degradation** - Compromised data quality scenarios
//!
//! ## Design Principles
//!
//! - Stress scenarios are expected to **break some signals** - that is the point
//! - Acceptable outcomes: reduced confidence, suppressed signals, explicit failure
//! - Unacceptable outcomes: maintaining high confidence, producing confident directional inference
//! - **Silence, humility, and graceful degradation validate the system**

pub mod detection;
pub mod evaluation;
pub mod scenarios;
pub mod types;

pub use detection::*;
pub use evaluation::*;
pub use scenarios::*;
pub use types::*;
