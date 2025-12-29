//! Output Validation Module
//!
//! Enforces output restrictions to ensure Galactus never emits forbidden content.
//!
//! # Purpose
//!
//! This module validates all outputs before delivery to ensure compliance with
//! output restrictions defined in `docs/09-compliance-and-language/output-restrictions.md`.
//!
//! # Forbidden Output Categories
//!
//! 1. Trading instructions (buy/sell, long/short, etc.)
//! 2. Price targets and predictions
//! 3. Position sizing and allocation
//! 4. Timing and urgency prompts
//! 5. Performance promises
//! 6. Personalized recommendations
//!
//! # Usage
//!
//! ```rust
//! use galactus_core::output_validation::validate_output;
//!
//! let output = "Capital pressure at 73rd percentile";
//! assert!(validate_output(output).is_ok());
//!
//! let forbidden = "BUY signal with target price 250";
//! assert!(validate_output(forbidden).is_err());
//! ```

use std::collections::HashSet;

/// Error types for output validation failures
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Trading instruction detected
    TradingInstruction { term: String, context: String },
    
    /// Price target or prediction detected
    PriceTarget { term: String, context: String },
    
    /// Position sizing recommendation detected
    PositionSizing { term: String, context: String },
    
    /// Timing/urgency language detected
    TimingUrgency { term: String, context: String },
    
    /// Performance promise detected
    PerformancePromise { term: String, context: String },
    
    /// Personalized recommendation detected
    PersonalizedAdvice { term: String, context: String },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TradingInstruction { term, context } => {
                write!(f, "Trading instruction detected: '{}' in '{}'", term, context)
            }
            ValidationError::PriceTarget { term, context } => {
                write!(f, "Price target detected: '{}' in '{}'", term, context)
            }
            ValidationError::PositionSizing { term, context } => {
                write!(f, "Position sizing detected: '{}' in '{}'", term, context)
            }
            ValidationError::TimingUrgency { term, context } => {
                write!(f, "Timing/urgency detected: '{}' in '{}'", term, context)
            }
            ValidationError::PerformancePromise { term, context } => {
                write!(f, "Performance promise detected: '{}' in '{}'", term, context)
            }
            ValidationError::PersonalizedAdvice { term, context } => {
                write!(f, "Personalized advice detected: '{}' in '{}'", term, context)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Forbidden terms for trading instructions
fn trading_instruction_terms() -> HashSet<&'static str> {
    [
        "buy signal", "sell signal", "strong buy", "strong sell",
        "buy now", "sell now",
        "enter long", "enter short", "exit position",
        "accumulate", "distribute", "scale in", "scale out",
        "take profit", "stop loss", "execute trade",
        "add to position", "reduce position", "open position", "close position",
        "buy call", "buy put", "sell call", "sell put",
    ]
    .iter()
    .copied()
    .collect()
}

/// Forbidden terms for price targets
fn price_target_terms() -> HashSet<&'static str> {
    [
        "target price", "price target", "price objective",
        "expected price", "fair value", "intrinsic value",
        "price range", "support level", "resistance level",
        "breakout target", "measured move", "upside target", "downside target",
        "will rise to", "will fall to", "expected to reach",
        "should hit", "likely to reach", "projected to move",
    ]
    .iter()
    .copied()
    .collect()
}

/// Forbidden terms for position sizing
fn position_sizing_terms() -> HashSet<&'static str> {
    [
        "position size", "allocate", "allocation", "capital allocation",
        "portfolio weight", "risk per trade", "use leverage",
        "margin", "exposure", "quantity", "lots",
        "% of portfolio", "% of capital", "recommended size",
        "optimal position", "maximum position", "position limit",
    ]
    .iter()
    .copied()
    .collect()
}

/// Forbidden terms for timing/urgency
///
/// Note: "immediate action" is correctly in this list for user-facing outputs.
/// Internal code should use phrases like "immediate system response" to be more precise
/// and avoid false positives in validation.
fn timing_urgency_terms() -> HashSet<&'static str> {
    [
        "act now", "immediate action", "urgent", "don't miss",
        "last chance", "window closing", "time running out",
        "before it's too late", "get in now", "exit immediately",
        "move quickly", "right now", "asap", "hurry",
    ]
    .iter()
    .copied()
    .collect()
}

/// Forbidden terms for performance promises
fn performance_promise_terms() -> HashSet<&'static str> {
    [
        "guaranteed return", "expected return", "assured profit",
        "risk-free", "no downside", "safe bet", "can't lose",
        "will beat market", "outperform", "alpha generation",
        "expected gain", "projected returns", "guaranteed profit",
        "performance warranty", "promised outcome", "assured returns",
    ]
    .iter()
    .copied()
    .collect()
}

/// Forbidden terms for personalized advice
fn personalized_advice_terms() -> HashSet<&'static str> {
    [
        "you should", "we recommend", "best for you", "right for you",
        "ideal for you", "suited to you", "matches your",
        "your portfolio", "your risk profile", "your situation",
        "based on your", "for your", "your holdings",
    ]
    .iter()
    .copied()
    .collect()
}

/// Extract context around a term (up to 50 characters before and after)
fn extract_context(text: &str, position: usize, term_len: usize) -> String {
    let start = position.saturating_sub(50);
    let end = (position + term_len + 50).min(text.len());
    let context = &text[start..end];
    
    if start > 0 {
        format!("...{}", context)
    } else {
        context.to_string()
    }
}

/// Check for forbidden terms in text
fn check_forbidden_terms(
    text: &str,
    terms: &HashSet<&str>,
    error_constructor: impl Fn(String, String) -> ValidationError,
) -> Result<(), ValidationError> {
    let text_lower = text.to_lowercase();
    
    for term in terms {
        if let Some(pos) = text_lower.find(term) {
            let context = extract_context(&text_lower, pos, term.len());
            return Err(error_constructor(term.to_string(), context));
        }
    }
    
    Ok(())
}

/// Validate output text against all forbidden categories
///
/// # Arguments
///
/// * `output` - The text to validate
///
/// # Returns
///
/// * `Ok(())` if output passes validation
/// * `Err(ValidationError)` if forbidden content detected
///
/// # Examples
///
/// ```
/// use galactus_core::output_validation::validate_output;
///
/// // Allowed output
/// let allowed = "Capital pressure measured at 73rd percentile";
/// assert!(validate_output(allowed).is_ok());
///
/// // Forbidden output
/// let forbidden = "Strong BUY signal";
/// assert!(validate_output(forbidden).is_err());
/// ```
pub fn validate_output(output: &str) -> Result<(), ValidationError> {
    // Check for trading instructions
    check_forbidden_terms(
        output,
        &trading_instruction_terms(),
        |term, context| ValidationError::TradingInstruction { term, context },
    )?;
    
    // Check for price targets
    check_forbidden_terms(
        output,
        &price_target_terms(),
        |term, context| ValidationError::PriceTarget { term, context },
    )?;
    
    // Check for position sizing
    check_forbidden_terms(
        output,
        &position_sizing_terms(),
        |term, context| ValidationError::PositionSizing { term, context },
    )?;
    
    // Check for timing/urgency
    check_forbidden_terms(
        output,
        &timing_urgency_terms(),
        |term, context| ValidationError::TimingUrgency { term, context },
    )?;
    
    // Check for performance promises
    check_forbidden_terms(
        output,
        &performance_promise_terms(),
        |term, context| ValidationError::PerformancePromise { term, context },
    )?;
    
    // Check for personalized advice
    check_forbidden_terms(
        output,
        &personalized_advice_terms(),
        |term, context| ValidationError::PersonalizedAdvice { term, context },
    )?;
    
    Ok(())
}

/// Validate structured metadata fields
///
/// Validates HashMap string values for forbidden content
pub fn validate_metadata(metadata: &std::collections::HashMap<String, String>) -> Result<(), ValidationError> {
    for (key, value) in metadata {
        // Validate both key and value
        validate_output(key)?;
        validate_output(value)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_structural_output() {
        let output = "Capital pressure measured at 73rd percentile. \
                      Rollover obligation estimated at 2.1σ above normal. \
                      High-pressure regime with elevated structural constraints.";
        assert!(validate_output(output).is_ok());
    }

    #[test]
    fn test_allowed_mechanistic_explanation() {
        let output = "Settlement mechanics force basis convergence. \
                      Delta hedging propagates pressure to spot markets.";
        assert!(validate_output(output).is_ok());
    }

    #[test]
    fn test_allowed_uncertainty() {
        let output = "Confidence degraded due to data quality. \
                      Insufficient evidence for structural inference. \
                      Cannot determine with available data.";
        assert!(validate_output(output).is_ok());
    }

    #[test]
    fn test_forbidden_buy_signal() {
        let output = "Strong BUY SIGNAL detected";
        let result = validate_output(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TradingInstruction { .. }));
    }

    #[test]
    fn test_forbidden_sell_instruction() {
        let output = "Strong SELL SIGNAL at current levels";
        let result = validate_output(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TradingInstruction { .. }));
    }

    #[test]
    fn test_forbidden_price_target() {
        let output = "Target price of 250 rupees";
        let result = validate_output(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::PriceTarget { .. }));
    }

    #[test]
    fn test_forbidden_position_sizing() {
        let output = "Allocate 10% of portfolio to this position";
        let result = validate_output(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::PositionSizing { .. }));
    }

    #[test]
    fn test_forbidden_urgency() {
        let output = "Act now before the window closes";
        let result = validate_output(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::TimingUrgency { .. }));
    }

    #[test]
    fn test_forbidden_performance_promise() {
        let output = "Guaranteed return of 25%";
        let result = validate_output(output);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::PerformancePromise { .. }));
    }

    #[test]
    fn test_forbidden_personalized_advice() {
        let output = "You should consider this based on your risk profile";
        let result = validate_output(output);
        assert!(result.is_err());
        // This will catch "you should" which is personalized advice
        assert!(matches!(result.unwrap_err(), ValidationError::PersonalizedAdvice { .. }));
    }

    #[test]
    fn test_case_insensitive_detection() {
        let outputs = vec![
            "BUY SIGNAL detected",
            "Buy Signal detected",
            "buy signal detected",
            "BuY sIgNaL detected",
        ];
        
        for output in outputs {
            assert!(validate_output(output).is_err());
        }
    }

    #[test]
    fn test_context_extraction() {
        let output = "This is a long sentence with a BUY SIGNAL instruction hidden in the middle of it";
        let result = validate_output(output);
        assert!(result.is_err());
        
        if let Err(ValidationError::TradingInstruction { term, context }) = result {
            assert_eq!(term, "buy signal");
            assert!(context.contains("buy signal"));
        }
    }

    #[test]
    fn test_metadata_validation() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("signal_type".to_string(), "structural_pressure".to_string());
        metadata.insert("regime".to_string(), "high_pressure".to_string());
        
        assert!(validate_metadata(&metadata).is_ok());
    }

    #[test]
    fn test_forbidden_metadata() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("recommendation".to_string(), "buy signal detected".to_string());
        
        assert!(validate_metadata(&metadata).is_err());
    }

    #[test]
    fn test_pressure_terminology_allowed() {
        // "buying pressure" and "selling pressure" are allowed structural terms
        // referring to capital flows, not trading instructions
        let output = "Negative pressure indicates selling pressure from forced unwinding. \
                      Positive values indicate buying pressure from position building.";
        assert!(validate_output(output).is_ok());
    }

    #[test]
    fn test_multiple_violations() {
        // First violation should be caught
        let output = "BUY SIGNAL now with target price 250 and allocate 10% of portfolio";
        assert!(validate_output(output).is_err());
    }
}
