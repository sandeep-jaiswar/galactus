"""
Hedge Pressure Signal Research Implementation

This module implements the hedge pressure signal for research and validation.

Hypothesis: Call-put OI imbalance indicates forced directional positioning due to capital constraints.

Signal Logic:
- Measures imbalance between call and put open interest
- Normalized by total OI and distance from spot price
- Extreme imbalances may indicate forced hedging or positioning
- Time-adjusted for persistence of imbalance

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
class HedgePressureResult:
    """Result of hedge pressure computation."""

    pressure: float
    call_oi_total: int
    put_oi_total: int
    imbalance_ratio: float
    spot_distance_factor: float
    strike_count: int
    confidence: float
    metadata: Dict[str, Any]


@dataclass
class HedgePressureConfig:
    """Configuration for hedge pressure computation."""

    min_strikes: int = 3
    max_distance_factor: float = 2.0
    spot_sensitivity: float = 1.0
    confidence_threshold: float = 0.1
    imbalance_threshold: float = 0.3  # Minimum imbalance to be significant


@mark_experimental(
    hypothesis="Call-put OI imbalance indicates forced directional positioning due to capital constraints",
    assumptions=[
        "Clean derivatives market data available",
        "Sufficient market liquidity (>1000 contracts total OI)",
        "Normal market regime (not crisis or expiry)",
        "Data aligned to same timestamp",
        "OI reflects actual hedging activity",
        "Spot price accurately represents fair value",
        "Strike prices cover meaningful range around spot",
    ],
    failure_modes=[
        "False signals during earnings or news events",
        "Sensitive to spot price accuracy",
        "May miss complex hedging strategies",
        "Fails in low volume periods",
        "Noise from market maker positioning",
        "Strike discretization effects",
        "Expiry week distortions",
    ],
)
def compute_hedge_pressure(
    calls_oi: Dict[float, int],
    puts_oi: Dict[float, int],
    spot_price: float,
    config: Optional[HedgePressureConfig] = None,
) -> HedgePressureResult:
    """
    Compute hedge pressure from call-put OI imbalance.

    This feature attempts to detect forced directional positioning by measuring
    the imbalance between call and put open interest, normalized by proximity
    to spot price. Extreme imbalances may indicate forced hedging due to
    capital constraints or risk management.

    Args:
        calls_oi: Call open interest by strike price
        puts_oi: Put open interest by strike price
        spot_price: Current spot price
        config: Configuration parameters

    Returns:
        HedgePressureResult with pressure metric and metadata

    Raises:
        ValueError: If input data is invalid
    """
    if config is None:
        config = HedgePressureConfig()

    # Input validation
    if not isinstance(calls_oi, dict) or not isinstance(puts_oi, dict):
        raise ValueError("OI data must be dictionaries mapping strike to quantity")

    if not isinstance(spot_price, (int, float)) or spot_price <= 0:
        raise ValueError("Spot price must be a positive number")

    # Get all strikes and compute totals
    all_strikes = set(calls_oi.keys()) | set(puts_oi.keys())
    call_total = sum(calls_oi.values())
    put_total = sum(puts_oi.values())
    total_oi = call_total + put_total

    if len(all_strikes) < config.min_strikes:
        warnings.warn(
            f"Insufficient strike data: {len(all_strikes)} strikes, min={config.min_strikes}"
        )
        return HedgePressureResult(
            pressure=0.0,
            call_oi_total=call_total,
            put_oi_total=put_total,
            imbalance_ratio=0.0,
            spot_distance_factor=0.0,
            strike_count=len(all_strikes),
            confidence=0.0,
            metadata={"error": "insufficient_strikes"},
        )

    if total_oi < 100:  # Minimum liquidity threshold
        return HedgePressureResult(
            pressure=0.0,
            call_oi_total=call_total,
            put_oi_total=put_total,
            imbalance_ratio=0.0,
            spot_distance_factor=0.0,
            strike_count=len(all_strikes),
            confidence=0.0,
            metadata={"error": "insufficient_liquidity"},
        )

    # Compute raw imbalance ratio
    if total_oi > 0:
        imbalance_ratio = (call_total - put_total) / total_oi
    else:
        imbalance_ratio = 0.0

    # Compute spot distance factor - weight strikes by distance from spot
    spot_distance_factor = 0.0
    weighted_call_oi = 0.0
    weighted_put_oi = 0.0

    for strike in all_strikes:
        call_qty = calls_oi.get(strike, 0)
        put_qty = puts_oi.get(strike, 0)

        # Compute distance from spot (normalized)
        if spot_price > 0:
            distance = abs(strike - spot_price) / spot_price
            # Weight by inverse distance (closer strikes matter more)
            weight = max(0.1, 1.0 / (1.0 + distance * config.spot_sensitivity))
        else:
            weight = 1.0

        weighted_call_oi += call_qty * weight
        weighted_put_oi += put_qty * weight
        spot_distance_factor += weight

    # Compute weighted imbalance
    if spot_distance_factor > 0:
        weighted_total = weighted_call_oi + weighted_put_oi
        if weighted_total > 0:
            weighted_imbalance = (weighted_call_oi - weighted_put_oi) / weighted_total
        else:
            weighted_imbalance = 0.0
    else:
        weighted_imbalance = imbalance_ratio

    # Apply non-linear scaling for extreme imbalances
    # Use tanh to create S-curve response
    pressure = np.tanh(weighted_imbalance * 3.0)  # Scale factor makes it more sensitive

    # Only consider significant imbalances
    if abs(imbalance_ratio) < config.imbalance_threshold:
        pressure *= 0.1  # Dampen weak signals

    # Clamp to [-1, 1] range
    pressure = max(-1.0, min(1.0, pressure))

    # Compute confidence based on data quality and significance
    confidence = min(1.0, total_oi / 2000.0)  # Scale with total OI
    confidence *= min(1.0, len(all_strikes) / 8.0)  # Scale with strike coverage
    confidence *= min(
        1.0, abs(imbalance_ratio) / config.imbalance_threshold
    )  # Scale with signal strength

    return HedgePressureResult(
        pressure=pressure,
        call_oi_total=call_total,
        put_oi_total=put_total,
        imbalance_ratio=imbalance_ratio,
        spot_distance_factor=spot_distance_factor,
        strike_count=len(all_strikes),
        confidence=confidence,
        metadata={
            "weighted_imbalance": weighted_imbalance,
            "spot_price": spot_price,
            "total_oi": total_oi,
            "computation_timestamp": datetime.now().isoformat(),
        },
    )


def validate_hedge_pressure_signal(
    result: HedgePressureResult, regime_context: Optional[Dict[str, Any]] = None
) -> Dict[str, Any]:
    """
    Validate hedge pressure signal result for research purposes.

    Args:
        result: Hedge pressure computation result
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
            "Low confidence due to insufficient data or weak signal"
        )
        validation["confidence_assessment"] = "low"

    # Check for extreme values that might indicate data issues
    if abs(result.pressure) > 0.95:
        validation["warnings"].append("Extreme pressure value - check data quality")

    # Check imbalance significance
    if abs(result.imbalance_ratio) < 0.1:
        validation["warnings"].append("Weak imbalance signal")

    # Check strike coverage
    if result.strike_count < 3:
        validation["warnings"].append("Limited strike coverage")
        validation["is_valid"] = False

    # Regime-specific validation
    if regime_context:
        regime = regime_context.get("regime", "unknown")
        if regime == "expiry":
            validation["warnings"].append("Signal unreliable during expiry regime")
            validation["regime_compatibility"] = "poor"
        elif regime == "earnings":
            validation["warnings"].append("Signal may be noisy around earnings")
            validation["regime_compatibility"] = "moderate"
        elif regime == "normal":
            validation["regime_compatibility"] = "good"
        elif regime == "crisis":
            validation["warnings"].append(
                "Signal may be amplified during crisis regime"
            )
            validation["regime_compatibility"] = "moderate"

    # Overall validity assessment
    if len(validation["warnings"]) > 2:
        validation["is_valid"] = False

    return validation


# Research utility functions


def generate_synthetic_hedge_data(
    base_calls: Dict[float, int],
    base_puts: Dict[float, int],
    spot_price: float,
    hedge_pressure: float,
    noise_factor: float = 0.05,
) -> Tuple[Dict[float, int], Dict[float, int]]:
    """
    Generate synthetic call/put OI data for testing hedge pressure.

    Args:
        base_calls: Base call OI levels by strike
        base_puts: Base put OI levels by strike
        spot_price: Current spot price
        hedge_pressure: Target hedge pressure (-1 to 1)
        noise_factor: Random noise to add

    Returns:
        Tuple of (synthetic_calls, synthetic_puts)
    """
    synthetic_calls = {}
    synthetic_puts = {}

    all_strikes = set(base_calls.keys()) | set(base_puts.keys())

    for strike in all_strikes:
        base_call = base_calls.get(strike, 0)
        base_put = base_puts.get(strike, 0)

        # Compute distance-based adjustment
        if spot_price > 0:
            distance = (strike - spot_price) / spot_price
        else:
            distance = 0.0

        # Apply hedge pressure bias
        # Positive pressure = more calls (call-dominant bias)
        # Negative pressure = more puts (put-dominant bias)
        call_adjustment = hedge_pressure * (1.0 - abs(distance) * 0.5)
        put_adjustment = -hedge_pressure * (1.0 - abs(distance) * 0.5)

        # Apply adjustments
        new_call = int(base_call * (1 + call_adjustment))
        new_put = int(base_put * (1 + put_adjustment))

        # Add noise
        call_noise = np.random.normal(0, noise_factor * max(1, base_call))
        put_noise = np.random.normal(0, noise_factor * max(1, base_put))

        new_call = max(0, int(new_call + call_noise))
        new_put = max(0, int(new_put + put_noise))

        synthetic_calls[strike] = new_call
        synthetic_puts[strike] = new_put

    return synthetic_calls, synthetic_puts


def analyze_hedge_pressure_patterns(
    calls_history: List[Dict[float, int]],
    puts_history: List[Dict[float, int]],
    spot_prices: List[float],
    timestamps: List[datetime],
) -> Dict[str, Any]:
    """
    Analyze hedge pressure patterns over time for research.

    Args:
        calls_history: List of call OI snapshots over time
        puts_history: List of put OI snapshots over time
        spot_prices: Corresponding spot prices
        timestamps: Corresponding timestamps

    Returns:
        Analysis results
    """
    if len(calls_history) < 2 or len(timestamps) != len(calls_history):
        raise ValueError("Need at least 2 OI snapshots with matching timestamps")

    if len(spot_prices) != len(calls_history):
        raise ValueError("Spot prices must match OI history length")

    results = []
    config = HedgePressureConfig()

    for i in range(len(calls_history)):
        result = compute_hedge_pressure(
            calls_history[i], puts_history[i], spot_prices[i], config
        )

        results.append(
            {
                "timestamp": timestamps[i],
                "spot_price": spot_prices[i],
                "result": result,
            }
        )

    # Aggregate analysis
    pressures = [r["result"].pressure for r in results]
    confidences = [r["result"].confidence for r in results]
    imbalances = [r["result"].imbalance_ratio for r in results]

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
        "imbalance_stats": {
            "mean": float(np.mean(imbalances)),
            "std": float(np.std(imbalances)),
        },
        "results": results,
    }

    return analysis
