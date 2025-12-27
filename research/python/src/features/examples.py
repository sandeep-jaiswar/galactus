"""
Example Experimental Features

This module demonstrates proper feature isolation and documentation.

These are REFERENCE EXAMPLES only. Real features should be in separate files.
"""

from typing import List, Dict, Any
import warnings
from .isolation import mark_experimental


@mark_experimental(
    hypothesis="Derivatives OI decay rate indicates forced position unwinding",
    assumptions=[
        "Clean derivatives market data available",
        "Sufficient market liquidity (>1000 contracts)",
        "Normal market regime (not crisis or expiry)",
        "Data aligned to same timestamp"
    ],
    failure_modes=[
        "False signals during contract expiry weeks",
        "Sensitive to data quality issues (missing ticks)",
        "May miss gradual unwinding over long periods",
        "Fails in low volume periods"
    ]
)
def compute_oi_decay_pressure(
    current_oi: Dict[float, int],
    previous_oi: Dict[float, int],
    time_delta_hours: float
) -> float:
    """
    Compute capital pressure from derivatives OI decay patterns.
    
    This feature attempts to detect forced unwinding by measuring
    the rate at which open interest decreases across strike prices.
    Rapid OI decay may indicate forced position closure due to
    margin pressure or risk limits.
    
    Args:
        current_oi: Current open interest by strike price
        previous_oi: Previous open interest by strike price
        time_delta_hours: Time elapsed between measurements
    
    Returns:
        Pressure metric: -1 (strong unwinding) to +1 (strong building)
        Returns 0.0 if insufficient data
    
    Example:
        >>> current = {18000: 1000, 18500: 2000, 19000: 1500}
        >>> previous = {18000: 1500, 18500: 2500, 19000: 2000}
        >>> pressure = compute_oi_decay_pressure(current, previous, 1.0)
        >>> print(f"Decay pressure: {pressure:.3f}")
    
    Notes:
        - This is experimental and unvalidated
        - Requires promotion before production use
        - Must pass promotion checklist
    """
    # Check for empty data
    if not current_oi or not previous_oi:
        warnings.warn("Insufficient OI data for pressure computation")
        return 0.0
    
    # Compute total OI change
    all_strikes = set(current_oi.keys()) | set(previous_oi.keys())
    
    total_decay = 0
    total_build = 0
    
    for strike in all_strikes:
        curr = current_oi.get(strike, 0)
        prev = previous_oi.get(strike, 0)
        change = curr - prev
        
        if change < 0:
            total_decay += abs(change)
        else:
            total_build += change
    
    # Normalize to [-1, 1] range
    total_change = total_decay + total_build
    if total_change == 0:
        return 0.0
    
    # Negative for decay (unwinding), positive for building
    pressure = (total_build - total_decay) / total_change
    
    # Adjust for time (faster decay = stronger signal)
    if time_delta_hours > 0:
        time_factor = min(1.0, 1.0 / time_delta_hours)
        pressure *= time_factor
    
    return pressure


@mark_experimental(
    hypothesis="Call-put OI imbalance at key strikes indicates forced delta hedging",
    assumptions=[
        "Options market is liquid",
        "Market makers are actively hedging",
        "Spot price is near key strikes",
        "Not during major events or expiry"
    ],
    failure_modes=[
        "Misleading during low gamma periods",
        "Fails when market makers are passive",
        "Sensitive to strike selection",
        "May not work for illiquid underlyings"
    ]
)
def compute_hedge_pressure(
    calls_oi_by_strike: Dict[float, int],
    puts_oi_by_strike: Dict[float, int],
    spot_price: float,
    strike_range_pct: float = 5.0
) -> Dict[str, Any]:
    """
    Compute forced hedging pressure from call-put OI imbalance.
    
    Market makers must delta hedge their option positions. Large
    imbalances in call vs put OI near the current spot price can
    create forced hedging flows as price moves.
    
    Args:
        calls_oi_by_strike: Call open interest by strike
        puts_oi_by_strike: Put open interest by strike
        spot_price: Current spot price
        strike_range_pct: Consider strikes within this % of spot
    
    Returns:
        Dictionary with:
            - pressure: Float from -1 (put heavy) to +1 (call heavy)
            - confidence: Float from 0 (low) to 1 (high)
            - strikes_analyzed: Number of strikes in range
    
    Example:
        >>> calls = {18000: 500, 18500: 1000, 19000: 800}
        >>> puts = {18000: 1200, 18500: 1500, 19000: 600}
        >>> result = compute_hedge_pressure(calls, puts, 18500)
        >>> print(f"Pressure: {result['pressure']:.2f}")
    """
    # Define strike range around spot
    lower_bound = spot_price * (1 - strike_range_pct / 100)
    upper_bound = spot_price * (1 + strike_range_pct / 100)
    
    # Filter strikes in range
    relevant_strikes = [
        s for s in set(calls_oi_by_strike.keys()) | set(puts_oi_by_strike.keys())
        if lower_bound <= s <= upper_bound
    ]
    
    if not relevant_strikes:
        return {
            "pressure": 0.0,
            "confidence": 0.0,
            "strikes_analyzed": 0,
            "reason": "No strikes in specified range"
        }
    
    # Sum OI in range
    total_calls = sum(calls_oi_by_strike.get(s, 0) for s in relevant_strikes)
    total_puts = sum(puts_oi_by_strike.get(s, 0) for s in relevant_strikes)
    
    total_oi = total_calls + total_puts
    
    if total_oi == 0:
        return {
            "pressure": 0.0,
            "confidence": 0.0,
            "strikes_analyzed": len(relevant_strikes),
            "reason": "Zero OI in range"
        }
    
    # Compute imbalance: positive = call heavy, negative = put heavy
    imbalance = (total_calls - total_puts) / total_oi
    
    # Confidence based on absolute OI (higher OI = more confident)
    # This is a simple heuristic - real implementation would be more sophisticated
    min_oi_threshold = 1000  # Arbitrary threshold for this example
    confidence = min(1.0, total_oi / (min_oi_threshold * 10))
    
    return {
        "pressure": imbalance,
        "confidence": confidence,
        "strikes_analyzed": len(relevant_strikes),
        "total_calls": total_calls,
        "total_puts": total_puts,
        "reason": "normal"
    }


@mark_experimental(
    hypothesis="Futures basis divergence indicates forced arbitrage activity",
    assumptions=[
        "Futures and spot prices are available",
        "Markets are liquid and accessible",
        "No circuit breakers or trading halts",
        "Time to expiry is known and significant"
    ],
    failure_modes=[
        "Near expiry, basis naturally converges",
        "Fails during illiquid periods",
        "Sensitive to transaction costs",
        "May not reflect actual arbitrage constraints"
    ]
)
def compute_basis_pressure(
    futures_price: float,
    spot_price: float,
    time_to_expiry_days: float,
    risk_free_rate: float = 0.0
) -> Dict[str, Any]:
    """
    Compute forced arbitrage pressure from futures-spot basis.
    
    When basis diverges from fair value, arbitrageurs are forced
    to act. Extreme basis suggests constrained arbitrage capacity
    or forced position closing.
    
    Args:
        futures_price: Current futures price
        spot_price: Current spot price
        time_to_expiry_days: Days until futures expiry
        risk_free_rate: Annual risk-free rate (default 0)
    
    Returns:
        Dictionary with:
            - pressure: Float indicating basis divergence
            - fair_basis: Calculated fair basis
            - actual_basis: Observed basis
            - divergence_pct: Percentage divergence
    
    Example:
        >>> result = compute_basis_pressure(18550, 18500, 15)
        >>> print(f"Basis divergence: {result['divergence_pct']:.2f}%")
    """
    if spot_price <= 0 or time_to_expiry_days <= 0:
        return {
            "pressure": 0.0,
            "fair_basis": 0.0,
            "actual_basis": 0.0,
            "divergence_pct": 0.0,
            "reason": "Invalid inputs"
        }
    
    # Calculate fair basis (cost of carry)
    # Fair = Spot * (1 + r * t)
    # Simplified for this example
    time_fraction = time_to_expiry_days / 365.0
    fair_futures = spot_price * (1 + risk_free_rate * time_fraction)
    
    # Actual basis
    actual_basis = futures_price - spot_price
    fair_basis = fair_futures - spot_price
    
    # Divergence
    if fair_basis != 0:
        divergence_pct = ((actual_basis - fair_basis) / abs(fair_basis)) * 100
    else:
        divergence_pct = (actual_basis / spot_price) * 100
    
    # Pressure metric: large positive = overpriced futures, negative = underpriced
    # Normalized by spot price
    pressure = (actual_basis - fair_basis) / spot_price
    
    return {
        "pressure": pressure,
        "fair_basis": fair_basis,
        "actual_basis": actual_basis,
        "divergence_pct": divergence_pct,
        "futures_price": futures_price,
        "spot_price": spot_price,
        "reason": "normal"
    }


# Module-level documentation of all features
FEATURE_CATALOG = {
    "compute_oi_decay_pressure": {
        "category": "derivatives",
        "status": "experimental",
        "created": "2024-Q4",
        "promoted": False
    },
    "compute_hedge_pressure": {
        "category": "derivatives",
        "status": "experimental",
        "created": "2024-Q4",
        "promoted": False
    },
    "compute_basis_pressure": {
        "category": "arbitrage",
        "status": "experimental",
        "created": "2024-Q4",
        "promoted": False
    }
}


def get_feature_catalog() -> Dict[str, Dict[str, Any]]:
    """
    Get catalog of all features in this module.
    
    Returns:
        Dictionary mapping feature names to their metadata
    """
    return FEATURE_CATALOG.copy()
