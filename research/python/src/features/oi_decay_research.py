"""
OI Decay Pressure Signal Research Implementation

This module implements the OI decay pressure signal for research and validation.

Hypothesis: Derivatives OI decay rate indicates forced position unwinding due to capital constraints.

Signal Logic:
- Measures rate of open interest decrease across strike prices
- Rapid decay suggests forced position closure (margin pressure, risk limits)
- Normalized to [-1, 1] range: negative = unwinding, positive = building
- Time-adjusted for decay velocity

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
class OIDecayResult:
    """Result of OI decay pressure computation."""

    pressure: float
    total_decay: int
    total_build: int
    strike_count: int
    time_factor: float
    confidence: float
    metadata: Dict[str, Any]


@dataclass
class OIDecayConfig:
    """Configuration for OI decay computation."""

    min_strikes: int = 3
    max_time_factor: float = 1.0
    time_decay_exponent: float = 1.0
    confidence_threshold: float = 0.1


@mark_experimental(
    hypothesis="Derivatives OI decay rate indicates forced position unwinding due to capital constraints",
    assumptions=[
        "Clean derivatives market data available",
        "Sufficient market liquidity (>1000 contracts total OI)",
        "Normal market regime (not crisis or expiry)",
        "Data aligned to same timestamp",
        "OI changes reflect actual position adjustments",
        "Time delta accurately represents measurement interval",
    ],
    failure_modes=[
        "False signals during contract expiry weeks",
        "Sensitive to data quality issues (missing ticks)",
        "May miss gradual unwinding over long periods",
        "Fails in low volume periods",
        "Noise from normal position adjustments",
        "Time delta measurement errors",
        "Strike price discretization effects",
    ],
)
def compute_oi_decay_pressure(
    current_oi: Dict[float, int],
    previous_oi: Dict[float, int],
    time_delta_hours: float,
    config: Optional[OIDecayConfig] = None,
) -> OIDecayResult:
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
        config: Configuration parameters

    Returns:
        OIDecayResult with pressure metric and metadata

    Raises:
        ValueError: If input data is invalid
    """
    if config is None:
        config = OIDecayConfig()

    # Input validation
    if not isinstance(current_oi, dict) or not isinstance(previous_oi, dict):
        raise ValueError("OI data must be dictionaries mapping strike to quantity")

    if time_delta_hours <= 0:
        raise ValueError("Time delta must be positive")

    if len(current_oi) < config.min_strikes or len(previous_oi) < config.min_strikes:
        warnings.warn(
            f"Insufficient strike data: current={len(current_oi)}, previous={len(previous_oi)}, min={config.min_strikes}"
        )
        return OIDecayResult(
            pressure=0.0,
            total_decay=0,
            total_build=0,
            strike_count=max(len(current_oi), len(previous_oi)),
            time_factor=0.0,
            confidence=0.0,
            metadata={"error": "insufficient_data"},
        )

    # Compute OI changes across all strikes
    all_strikes = set(current_oi.keys()) | set(previous_oi.keys())
    total_decay = 0
    total_build = 0
    valid_strikes = 0

    for strike in all_strikes:
        curr = current_oi.get(strike, 0)
        prev = previous_oi.get(strike, 0)

        # Skip if both are zero (no activity)
        if curr == 0 and prev == 0:
            continue

        change = curr - prev
        valid_strikes += 1

        if change < 0:
            total_decay += abs(change)
        else:
            total_build += change

    # Check for sufficient activity
    total_change = total_decay + total_build
    if total_change == 0:
        return OIDecayResult(
            pressure=0.0,
            total_decay=total_decay,
            total_build=total_build,
            strike_count=valid_strikes,
            time_factor=0.0,
            confidence=0.0,
            metadata={"error": "no_change"},
        )

    # Compute base pressure: negative for decay (unwinding), positive for building
    pressure = (total_build - total_decay) / total_change

    # Apply time adjustment (faster decay = stronger signal)
    if time_delta_hours > 0:
        time_factor = min(
            config.max_time_factor, 1.0 / (time_delta_hours**config.time_decay_exponent)
        )
        pressure *= time_factor
    else:
        time_factor = 0.0

    # Compute confidence based on data quality and activity
    confidence = min(1.0, total_change / 1000.0)  # Scale confidence with activity level
    confidence *= min(1.0, valid_strikes / 5.0)  # Scale with strike coverage

    # Clamp to [-1, 1] range
    pressure = max(-1.0, min(1.0, pressure))

    return OIDecayResult(
        pressure=pressure,
        total_decay=total_decay,
        total_build=total_build,
        strike_count=valid_strikes,
        time_factor=time_factor,
        confidence=confidence,
        metadata={
            "total_change": total_change,
            "time_delta_hours": time_delta_hours,
            "computation_timestamp": datetime.now().isoformat(),
        },
    )


def validate_oi_decay_signal(
    result: OIDecayResult, regime_context: Optional[Dict[str, Any]] = None
) -> Dict[str, Any]:
    """
    Validate OI decay signal result for research purposes.

    Args:
        result: OI decay computation result
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
        validation["warnings"].append("Low confidence due to insufficient activity")
        validation["confidence_assessment"] = "low"

    # Check for extreme values that might indicate data issues
    if abs(result.pressure) > 0.95:
        validation["warnings"].append("Extreme pressure value - check data quality")

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
        elif regime == "normal":
            validation["regime_compatibility"] = "good"
        elif regime == "crisis":
            validation["warnings"].append("Signal may be noisy during crisis regime")
            validation["regime_compatibility"] = "moderate"

    # Overall validity assessment
    if len(validation["warnings"]) > 2:
        validation["is_valid"] = False

    return validation


# Research utility functions


def generate_synthetic_oi_data(
    base_oi: Dict[float, int],
    decay_rate: float,
    time_delta_hours: float,
    noise_factor: float = 0.05,
) -> Dict[float, int]:
    """
    Generate synthetic OI data for testing.

    Args:
        base_oi: Base OI levels by strike
        decay_rate: Rate of OI decay (-1 to 1, negative = decay)
        time_delta_hours: Time delta for scaling
        noise_factor: Random noise to add

    Returns:
        Synthetic OI data
    """
    synthetic_oi = {}

    for strike, base_qty in base_oi.items():
        # Apply decay/building
        change_factor = decay_rate * (time_delta_hours / 24.0)  # Scale to daily
        new_qty = int(base_qty * (1 + change_factor))

        # Add noise
        noise = np.random.normal(0, noise_factor * base_qty)
        new_qty = max(0, int(new_qty + noise))

        synthetic_oi[strike] = new_qty

    return synthetic_oi


def analyze_oi_decay_patterns(
    oi_history: List[Dict[float, int]], timestamps: List[datetime]
) -> Dict[str, Any]:
    """
    Analyze OI decay patterns over time for research.

    Args:
        oi_history: List of OI snapshots over time
        timestamps: Corresponding timestamps

    Returns:
        Analysis results
    """
    if len(oi_history) < 2 or len(timestamps) != len(oi_history):
        raise ValueError("Need at least 2 OI snapshots with matching timestamps")

    results = []
    config = OIDecayConfig()

    for i in range(1, len(oi_history)):
        time_delta = (timestamps[i] - timestamps[i - 1]).total_seconds() / 3600.0

        result = compute_oi_decay_pressure(
            oi_history[i], oi_history[i - 1], time_delta, config
        )

        results.append(
            {
                "timestamp": timestamps[i],
                "result": result,
                "time_delta_hours": time_delta,
            }
        )

    # Aggregate analysis
    pressures = [r["result"].pressure for r in results]
    confidences = [r["result"].confidence for r in results]

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
        "results": results,
    }

    return analysis
