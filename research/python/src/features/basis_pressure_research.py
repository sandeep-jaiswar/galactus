"""
Basis Pressure Signal Research Implementation

This module implements the basis pressure signal for research and validation.

Hypothesis: Futures-spot basis divergence indicates forced arbitrage activity due to capital constraints.

Signal Logic:
- Measures divergence between futures price and fair value (cost of carry)
- Extreme basis suggests constrained arbitrage capacity or forced position closing
- Time-adjusted for expiry proximity
- Normalized to [-1, 1] range: positive = overpriced futures, negative = underpriced

Validation Requirements:
- Statistical significance testing
- Walk-forward validation across regimes
- Failure mode analysis
- Cross-validation with known market events
"""

from typing import Dict, List, Optional, Tuple, Any
import warnings
import numpy as np
from datetime import datetime, timedelta
from dataclasses import dataclass

from .isolation import mark_experimental


@dataclass
class BasisPressureResult:
    """Result of basis pressure computation."""

    pressure: float
    fair_basis: float
    actual_basis: float
    divergence_pct: float
    time_to_expiry_days: float
    confidence: float
    metadata: Dict[str, Any]


@dataclass
class BasisPressureConfig:
    """Configuration for basis pressure computation."""

    risk_free_rate: float = 0.05  # 5% annual rate
    min_time_to_expiry_days: float = 1.0
    max_time_to_expiry_days: float = 365.0
    divergence_threshold_pct: float = 0.5  # 0.5% threshold for significance
    confidence_threshold: float = 0.1


@mark_experimental(
    hypothesis="Futures-spot basis divergence indicates forced arbitrage activity due to capital constraints",
    assumptions=[
        "Futures and spot prices are available and synchronized",
        "Markets are liquid with active arbitrage",
        "No circuit breakers or trading halts active",
        "Time to expiry is accurately known",
        "Risk-free rate is appropriate for the market",
        "Transaction costs are not prohibitive",
        "No significant market segmentation",
    ],
    failure_modes=[
        "Near expiry when basis naturally converges to zero",
        "Fails during illiquid periods or market closures",
        "Sensitive to risk-free rate assumptions",
        "May not reflect actual arbitrage constraints",
        "Transaction costs can prevent arbitrage",
        "Fails in segmented or restricted markets",
        "Time to expiry measurement errors",
    ],
)
def compute_basis_pressure(
    futures_price: float,
    spot_price: float,
    time_to_expiry_days: float,
    risk_free_rate: Optional[float] = None,
    config: Optional[BasisPressureConfig] = None,
) -> BasisPressureResult:
    """
    Compute basis pressure from futures-spot basis divergence.

    This feature attempts to detect forced arbitrage activity by measuring
    how much the futures-spot basis diverges from fair value. Extreme
    divergence may indicate constrained arbitrage capacity or forced
    position adjustments due to capital constraints.

    Args:
        futures_price: Current futures price
        spot_price: Current spot price
        time_to_expiry_days: Days until futures expiry
        risk_free_rate: Annual risk-free rate (optional, uses config default)
        config: Configuration parameters

    Returns:
        BasisPressureResult with pressure metric and metadata

    Raises:
        ValueError: If input data is invalid
    """
    if config is None:
        config = BasisPressureConfig()

    if risk_free_rate is None:
        risk_free_rate = config.risk_free_rate

    # Input validation
    if not isinstance(futures_price, (int, float)) or futures_price <= 0:
        raise ValueError("Futures price must be a positive number")

    if not isinstance(spot_price, (int, float)) or spot_price <= 0:
        raise ValueError("Spot price must be a positive number")

    if not isinstance(time_to_expiry_days, (int, float)) or time_to_expiry_days < 0:
        raise ValueError("Time to expiry must be non-negative")

    if time_to_expiry_days < config.min_time_to_expiry_days:
        warnings.warn(
            f"Very short time to expiry: {time_to_expiry_days} days, min={config.min_time_to_expiry_days}"
        )
        return BasisPressureResult(
            pressure=0.0,
            fair_basis=0.0,
            actual_basis=0.0,
            divergence_pct=0.0,
            time_to_expiry_days=time_to_expiry_days,
            confidence=0.0,
            metadata={"error": "too_close_to_expiry"},
        )

    if time_to_expiry_days > config.max_time_to_expiry_days:
        warnings.warn(
            f"Very long time to expiry: {time_to_expiry_days} days, max={config.max_time_to_expiry_days}"
        )
        return BasisPressureResult(
            pressure=0.0,
            fair_basis=0.0,
            actual_basis=0.0,
            divergence_pct=0.0,
            time_to_expiry_days=time_to_expiry_days,
            confidence=0.0,
            metadata={"error": "too_far_from_expiry"},
        )

    # Calculate fair basis using cost of carry model
    # Fair futures price = Spot * e^(r * t)
    # Fair basis = Fair futures - Spot = Spot * (e^(r * t) - 1)
    time_fraction = time_to_expiry_days / 365.0  # Convert to years

    try:
        # Use continuous compounding for precision
        fair_futures = spot_price * np.exp(risk_free_rate * time_fraction)
        fair_basis = fair_futures - spot_price
    except (OverflowError, ValueError):
        # Fallback to simple interest for very long periods
        fair_futures = spot_price * (1 + risk_free_rate * time_fraction)
        fair_basis = fair_futures - spot_price

    # Actual basis
    actual_basis = futures_price - spot_price

    # Compute divergence
    if abs(fair_basis) > 1e-6:  # Avoid division by very small numbers
        divergence_pct = ((actual_basis - fair_basis) / abs(fair_basis)) * 100.0
    else:
        # For very short times, use spot price as denominator
        divergence_pct = ((actual_basis - fair_basis) / spot_price) * 100.0

    # Compute pressure metric
    # Positive pressure = futures overpriced (buy spot, sell futures)
    # Negative pressure = futures underpriced (sell spot, buy futures)
    # Normalize by spot price and apply time weighting
    raw_pressure = (actual_basis - fair_basis) / spot_price

    # Apply time-based adjustment
    # Closer to expiry, smaller divergences are more significant
    time_factor = np.exp(-time_fraction * 2.0)  # Exponential decay with time
    pressure = raw_pressure * time_factor

    # Apply non-linear scaling for extreme divergences
    # Use tanh to create S-curve response
    pressure = np.tanh(pressure * 5.0)  # Scale factor makes it more sensitive

    # Only consider significant divergences
    if abs(divergence_pct) < config.divergence_threshold_pct:
        pressure *= 0.2  # Dampen weak signals

    # Clamp to [-1, 1] range
    pressure = max(-1.0, min(1.0, pressure))

    # Compute confidence based on data quality and significance
    # Confidence is independent of signal strength - it measures calculation reliability
    confidence = 1.0

    # Reduce confidence for very short time to expiry
    if time_to_expiry_days < 7.0:
        confidence *= 0.7

    # Reduce confidence for very long time to expiry
    if time_to_expiry_days > 180.0:
        confidence *= 0.8

    # Reduce confidence for very small spot prices (potential data issues)
    if spot_price < 100.0:
        confidence *= 0.5

    # Reduce confidence for extreme divergences (potential calculation issues)
    if abs(divergence_pct) > 20.0:  # More than 20% divergence
        confidence *= 0.6

    return BasisPressureResult(
        pressure=pressure,
        fair_basis=fair_basis,
        actual_basis=actual_basis,
        divergence_pct=divergence_pct,
        time_to_expiry_days=time_to_expiry_days,
        confidence=confidence,
        metadata={
            "futures_price": futures_price,
            "spot_price": spot_price,
            "risk_free_rate": risk_free_rate,
            "time_fraction": time_fraction,
            "raw_pressure": raw_pressure,
            "time_factor": time_factor,
            "computation_timestamp": datetime.now().isoformat(),
        },
    )


def validate_basis_pressure_signal(
    result: BasisPressureResult, regime_context: Optional[Dict[str, Any]] = None
) -> Dict[str, Any]:
    """
    Validate basis pressure signal result for research purposes.

    Args:
        result: Basis pressure computation result
        regime_context: Optional market regime information

    Returns:
        Validation metrics and flags
    """
    validation = {
        "is_valid": True,
        "warnings": [],
        "confidence_assessment": "low",
        "regime_compatibility": "unknown",
    }

    # Check confidence threshold
    if result.confidence < 0.1:
        validation["warnings"].append(
            "Low confidence due to weak divergence or short time to expiry"
        )
        validation["confidence_assessment"] = "low"

    # Check for extreme values that might indicate data issues
    if abs(result.pressure) > 0.95:
        validation["warnings"].append("Extreme pressure value - check data quality")

    # Check divergence significance
    if abs(result.divergence_pct) < 0.1:
        validation["warnings"].append("Weak basis divergence signal")

    # Check time to expiry
    if result.time_to_expiry_days < 2:
        validation["warnings"].append("Very close to expiry - basis may be unreliable")
        validation["regime_compatibility"] = "poor"
    elif result.time_to_expiry_days > 180:
        validation["warnings"].append("Very far from expiry - basis may be noisy")

    # Regime-specific validation
    if regime_context:
        regime = regime_context.get("regime", "unknown")
        if regime == "expiry":
            validation["warnings"].append("Signal unreliable during expiry regime")
            validation["regime_compatibility"] = "poor"
        elif regime == "crisis":
            validation["warnings"].append(
                "Signal may be amplified during crisis regime"
            )
            validation["regime_compatibility"] = "moderate"
        elif regime == "illiquid":
            validation["warnings"].append("Signal unreliable in illiquid conditions")
            validation["regime_compatibility"] = "poor"
        elif regime == "normal":
            validation["regime_compatibility"] = "good"

    # Overall validity assessment
    if len(validation["warnings"]) > 2:
        validation["is_valid"] = False

    return validation


# Research utility functions


def generate_synthetic_basis_data(
    spot_price: float,
    time_to_expiry_days: float,
    basis_pressure: float,
    risk_free_rate: float = 0.05,
    noise_factor: float = 0.001,
) -> Tuple[float, float]:
    """
    Generate synthetic futures and spot prices for testing basis pressure.

    Args:
        spot_price: Base spot price
        time_to_expiry_days: Days to expiry
        basis_pressure: Target basis pressure (-1 to 1)
        risk_free_rate: Risk-free rate
        noise_factor: Random noise to add

    Returns:
        Tuple of (futures_price, spot_price) - spot_price may be adjusted slightly
    """
    # Start with fair value
    time_fraction = time_to_expiry_days / 365.0
    fair_futures = spot_price * np.exp(risk_free_rate * time_fraction)
    fair_basis = fair_futures - spot_price

    # Apply basis pressure to create divergence
    # Positive pressure = overpriced futures, negative = underpriced
    # Scale the divergence based on fair basis magnitude
    max_divergence_pct = 5.0  # 5% max divergence for testing
    target_divergence_pct = basis_pressure * max_divergence_pct

    # Apply divergence to fair basis
    target_basis = fair_basis * (1 + target_divergence_pct / 100.0)

    futures_price = spot_price + target_basis

    # Add noise (much smaller than the intended divergence)
    noise_factor_adjusted = (
        max_divergence_pct / 100.0 * 0.1
    )  # 10% of max divergence as noise
    futures_noise = np.random.normal(0, noise_factor_adjusted * abs(fair_basis))
    spot_noise = np.random.normal(
        0, noise_factor_adjusted * abs(fair_basis) * 0.1
    )  # Less spot noise

    futures_price += futures_noise
    adjusted_spot = spot_price + spot_noise

    # Ensure positive prices
    futures_price = max(0.01, futures_price)
    adjusted_spot = max(0.01, adjusted_spot)

    return futures_price, adjusted_spot


def analyze_basis_pressure_patterns(
    futures_prices: List[float],
    spot_prices: List[float],
    time_to_expiry_days: float,
    timestamps: List[datetime],
    risk_free_rate: float = 0.05,
) -> Dict[str, Any]:
    """
    Analyze basis pressure patterns over time for research.

    Args:
        futures_prices: List of futures prices over time
        spot_prices: List of spot prices over time
        time_to_expiry_days: Days to expiry (constant for this analysis)
        timestamps: Corresponding timestamps
        risk_free_rate: Risk-free rate

    Returns:
        Analysis results
    """
    if len(futures_prices) != len(spot_prices) or len(timestamps) != len(
        futures_prices
    ):
        raise ValueError("All input lists must have matching lengths")

    if len(futures_prices) < 2:
        raise ValueError("Need at least 2 price observations")

    results = []
    config = BasisPressureConfig()

    for i in range(len(futures_prices)):
        result = compute_basis_pressure(
            futures_prices[i],
            spot_prices[i],
            time_to_expiry_days,
            risk_free_rate,
            config,
        )

        results.append(
            {
                "timestamp": timestamps[i],
                "futures_price": futures_prices[i],
                "spot_price": spot_prices[i],
                "result": result,
            }
        )

    # Aggregate analysis
    pressures = [r["result"].pressure for r in results]
    confidences = [r["result"].confidence for r in results]
    divergences = [r["result"].divergence_pct for r in results]

    analysis = {
        "total_periods": len(results),
        "pressure_stats": {
            "mean": float(np.mean(pressures)),
            "std": float(np.std(pressures)),
            "min": float(np.min(pressures)),
            "max": float(np.max(pressures)),
        },
        "confidence_stats": {
            "mean": float(np.mean(confidences)),
            "std": float(np.std(confidences)),
        },
        "divergence_stats": {
            "mean": float(np.mean(divergences)),
            "std": float(np.std(divergences)),
        },
        "results": results,
    }

    return analysis
