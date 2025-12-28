"""
Tests for Basis Pressure Research Module

Tests the basis pressure signal implementation and validation.
"""

from datetime import datetime

import numpy as np
import pytest

from src.features.basis_pressure_research import (
    BasisPressureConfig, BasisPressureResult, analyze_basis_pressure_patterns,
    compute_basis_pressure, generate_synthetic_basis_data,
    validate_basis_pressure_signal)


class TestBasisPressureComputation:
    """Test basis pressure computation."""

    def test_fair_value_scenario(self):
        """Test behavior at fair value (no pressure)."""
        spot_price = 18500
        time_to_expiry = 30
        risk_free_rate = 0.05

        # Calculate fair futures price
        time_fraction = time_to_expiry / 365.0
        fair_futures = spot_price * np.exp(risk_free_rate * time_fraction)

        result = compute_basis_pressure(
            fair_futures, spot_price, time_to_expiry, risk_free_rate
        )

        assert isinstance(result, BasisPressureResult)
        assert abs(result.pressure) < 0.01  # Should be very close to zero
        assert abs(result.divergence_pct) < 0.01  # Very small divergence
        assert result.fair_basis > 0
        assert result.actual_basis > 0
        assert result.confidence > 0

    def test_overpriced_futures(self):
        """Test overpriced futures (positive pressure)."""
        spot_price = 18500
        futures_price = 18600  # Overpriced
        time_to_expiry = 30
        risk_free_rate = 0.05

        result = compute_basis_pressure(
            futures_price, spot_price, time_to_expiry, risk_free_rate
        )

        assert result.pressure > 0  # Positive pressure for overpriced futures
        assert result.divergence_pct > 0
        assert result.actual_basis > result.fair_basis

    def test_underpriced_futures(self):
        """Test underpriced futures (negative pressure)."""
        spot_price = 18500
        futures_price = 18400  # Underpriced
        time_to_expiry = 30
        risk_free_rate = 0.05

        result = compute_basis_pressure(
            futures_price, spot_price, time_to_expiry, risk_free_rate
        )

        assert result.pressure < 0  # Negative pressure for underpriced futures
        assert result.divergence_pct < 0
        assert result.actual_basis < result.fair_basis

    def test_time_to_expiry_sensitivity(self):
        """Test that time to expiry affects the calculation."""
        spot_price = 18500
        futures_price = 18600
        risk_free_rate = 0.05

        result_short = compute_basis_pressure(
            futures_price, spot_price, 7, risk_free_rate
        )  # 1 week
        result_long = compute_basis_pressure(
            futures_price, spot_price, 90, risk_free_rate
        )  # 3 months

        # Should be different due to time weighting
        assert abs(result_short.pressure - result_long.pressure) > 0.001

    def test_invalid_inputs(self):
        """Test error handling for invalid inputs."""
        with pytest.raises(ValueError, match="Futures price must be a positive number"):
            compute_basis_pressure(-100, 18500, 30)

        with pytest.raises(ValueError, match="Spot price must be a positive number"):
            compute_basis_pressure(18550, 0, 30)

        with pytest.raises(ValueError, match="Time to expiry must be non-negative"):
            compute_basis_pressure(18550, 18500, -1)

    def test_extreme_values_clamping(self):
        """Test that pressure values are clamped to [-1, 1]."""
        spot_price = 18500
        time_to_expiry = 30

        # Extreme overpricing
        futures_price = 20000  # Very overpriced
        result = compute_basis_pressure(futures_price, spot_price, time_to_expiry)

        assert result.pressure <= 1.0 and result.pressure >= -1.0  # Should be clamped

    def test_too_close_to_expiry(self):
        """Test handling of very short time to expiry."""
        spot_price = 18500
        futures_price = 18550
        time_to_expiry = 0.5  # Half day

        result = compute_basis_pressure(futures_price, spot_price, time_to_expiry)

        assert result.pressure == 0.0
        assert result.confidence == 0.0
        assert "too_close_to_expiry" in str(result.metadata)

    def test_too_far_from_expiry(self):
        """Test handling of very long time to expiry."""
        spot_price = 18500
        futures_price = 18550
        time_to_expiry = 400  # Over a year

        result = compute_basis_pressure(futures_price, spot_price, time_to_expiry)

        assert result.pressure == 0.0
        assert result.confidence == 0.0
        assert "too_far_from_expiry" in str(result.metadata)


class TestBasisPressureValidation:
    """Test basis pressure signal validation."""

    def test_valid_signal_validation(self):
        """Test validation of a valid signal."""
        result = BasisPressureResult(
            pressure=0.3,
            fair_basis=50.0,
            actual_basis=65.0,
            divergence_pct=1.5,
            time_to_expiry_days=30,
            confidence=0.8,
            metadata={},
        )

        validation = validate_basis_pressure_signal(result)

        assert validation["is_valid"] is True
        assert len(validation["warnings"]) == 0

    def test_low_confidence_warning(self):
        """Test validation warning for low confidence."""
        result = BasisPressureResult(
            pressure=0.02,
            fair_basis=50.0,
            actual_basis=51.0,
            divergence_pct=0.05,
            time_to_expiry_days=5,  # Very short time
            confidence=0.01,  # Below threshold
            metadata={},
        )

        validation = validate_basis_pressure_signal(result)

        assert "Low confidence" in str(validation["warnings"])

    def test_weak_divergence_warning(self):
        """Test validation warning for weak divergence."""
        result = BasisPressureResult(
            pressure=0.01,
            fair_basis=50.0,
            actual_basis=50.5,
            divergence_pct=0.05,
            time_to_expiry_days=30,
            confidence=0.6,
            metadata={},
        )

        validation = validate_basis_pressure_signal(result)

        assert "Weak basis divergence" in str(validation["warnings"])

    def test_close_to_expiry_warning(self):
        """Test validation warning for close to expiry."""
        result = BasisPressureResult(
            pressure=0.2,
            fair_basis=50.0,
            actual_basis=60.0,
            divergence_pct=1.0,
            time_to_expiry_days=1,  # Very close to expiry
            confidence=0.7,
            metadata={},
        )

        validation = validate_basis_pressure_signal(result)

        assert "Very close to expiry" in str(validation["warnings"])
        assert validation["regime_compatibility"] == "poor"

    def test_regime_context_validation(self):
        """Test validation with regime context."""
        result = BasisPressureResult(
            pressure=0.4,
            fair_basis=50.0,
            actual_basis=70.0,
            divergence_pct=2.0,
            time_to_expiry_days=30,
            confidence=0.9,
            metadata={},
        )

        # Test expiry regime
        expiry_context = {"regime": "expiry"}
        validation = validate_basis_pressure_signal(result, expiry_context)

        assert validation["regime_compatibility"] == "poor"
        assert any("expiry" in warning.lower() for warning in validation["warnings"])

        # Test illiquid regime
        illiquid_context = {"regime": "illiquid"}
        validation = validate_basis_pressure_signal(result, illiquid_context)

        assert validation["regime_compatibility"] == "poor"


class TestSyntheticDataGeneration:
    """Test synthetic basis data generation."""

    def test_synthetic_data_generation(self):
        """Test basic synthetic data generation."""
        spot_price = 18500
        time_to_expiry = 30
        basis_pressure = 0.5  # Positive pressure
        risk_free_rate = 0.05

        futures_price, spot_price_out = generate_synthetic_basis_data(
            spot_price, time_to_expiry, basis_pressure, risk_free_rate
        )

        assert futures_price > 0
        assert spot_price_out > 0

        # Check that the generated data produces expected pressure
        result = compute_basis_pressure(
            futures_price, spot_price_out, time_to_expiry, risk_free_rate
        )
        assert result.pressure > 0  # Should be positive pressure

    def test_zero_pressure_generation(self):
        """Test synthetic data with zero basis pressure."""
        spot_price = 18500
        time_to_expiry = 30
        basis_pressure = 0.0
        risk_free_rate = 0.05

        futures_price, spot_price_out = generate_synthetic_basis_data(
            spot_price, time_to_expiry, basis_pressure, risk_free_rate
        )

        # Should be close to fair value
        result = compute_basis_pressure(
            futures_price, spot_price_out, time_to_expiry, risk_free_rate
        )
        assert abs(result.pressure) < 0.1  # Should be close to zero

    def test_negative_pressure_generation(self):
        """Test synthetic data with negative basis pressure."""
        spot_price = 18500
        time_to_expiry = 30
        basis_pressure = -0.5  # Negative pressure
        risk_free_rate = 0.05

        futures_price, spot_price_out = generate_synthetic_basis_data(
            spot_price, time_to_expiry, basis_pressure, risk_free_rate
        )

        result = compute_basis_pressure(
            futures_price, spot_price_out, time_to_expiry, risk_free_rate
        )
        assert result.pressure < 0  # Should be negative pressure


class TestBasisPressureAnalysis:
    """Test basis pressure pattern analysis."""

    def test_pattern_analysis_basic(self):
        """Test basic pattern analysis."""
        # Create simple price history
        timestamps = [
            datetime(2024, 1, 1, 10, 0),
            datetime(2024, 1, 1, 11, 0),
            datetime(2024, 1, 1, 12, 0),
        ]

        futures_prices = [18550, 18570, 18540]  # Varying futures prices
        spot_prices = [18500, 18510, 18505]  # Varying spot prices
        time_to_expiry = 30
        risk_free_rate = 0.05

        analysis = analyze_basis_pressure_patterns(
            futures_prices, spot_prices, time_to_expiry, timestamps, risk_free_rate
        )

        assert "total_periods" in analysis
        assert analysis["total_periods"] == 3
        assert "pressure_stats" in analysis
        assert "results" in analysis
        assert len(analysis["results"]) == 3

    def test_insufficient_history(self):
        """Test error handling for insufficient history."""
        timestamps = [datetime(2024, 1, 1, 10, 0)]
        futures_prices = [18550]
        spot_prices = [18500]
        time_to_expiry = 30

        with pytest.raises(ValueError, match="Need at least 2"):
            analyze_basis_pressure_patterns(
                futures_prices, spot_prices, time_to_expiry, timestamps
            )

    def test_mismatched_lengths(self):
        """Test error handling for mismatched history lengths."""
        timestamps = [
            datetime(2024, 1, 1, 10, 0),
            datetime(2024, 1, 1, 11, 0),
        ]
        futures_prices = [18550]  # Only one price
        spot_prices = [18500, 18510]
        time_to_expiry = 30

        with pytest.raises(ValueError, match="matching lengths"):
            analyze_basis_pressure_patterns(
                futures_prices, spot_prices, time_to_expiry, timestamps
            )


class TestBasisPressureConfig:
    """Test basis pressure configuration."""

    def test_default_config(self):
        """Test default configuration values."""
        config = BasisPressureConfig()

        assert config.risk_free_rate == 0.05
        assert config.min_time_to_expiry_days == 1.0
        assert config.max_time_to_expiry_days == 365.0
        assert config.divergence_threshold_pct == 0.5
        assert config.confidence_threshold == 0.1

    def test_custom_config(self):
        """Test custom configuration."""
        config = BasisPressureConfig(
            risk_free_rate=0.03,
            min_time_to_expiry_days=2.0,
            max_time_to_expiry_days=180.0,
            divergence_threshold_pct=1.0,
            confidence_threshold=0.2,
        )

        assert config.risk_free_rate == 0.03
        assert config.min_time_to_expiry_days == 2.0
        assert config.max_time_to_expiry_days == 180.0
        assert config.divergence_threshold_pct == 1.0
        assert config.confidence_threshold == 0.2
