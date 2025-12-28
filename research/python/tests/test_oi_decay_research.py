"""
Tests for OI Decay Research Module

Tests the OI decay pressure signal implementation and validation.
"""

from datetime import datetime

import pytest

from src.features.oi_decay_research import (OIDecayConfig, OIDecayResult,
                                            analyze_oi_decay_patterns,
                                            compute_oi_decay_pressure,
                                            generate_synthetic_oi_data,
                                            validate_oi_decay_signal)


class TestOIDecayPressure:
    """Test OI decay pressure computation."""

    def test_basic_decay_detection(self):
        """Test detection of OI decay."""
        current_oi = {18000: 500, 18500: 800, 19000: 600}
        previous_oi = {18000: 1000, 18500: 1500, 19000: 1200}
        time_delta = 1.0

        result = compute_oi_decay_pressure(current_oi, previous_oi, time_delta)

        assert isinstance(result, OIDecayResult)
        assert result.pressure < 0  # Negative pressure for decay
        assert result.total_decay > 0
        assert result.total_build == 0
        assert result.strike_count == 3
        assert result.confidence > 0

    def test_basic_building_detection(self):
        """Test detection of OI building."""
        current_oi = {18000: 1500, 18500: 2000, 19000: 1800}
        previous_oi = {18000: 1000, 18500: 1500, 19000: 1200}
        time_delta = 1.0

        result = compute_oi_decay_pressure(current_oi, previous_oi, time_delta)

        assert result.pressure > 0  # Positive pressure for building
        assert result.total_build > 0
        assert result.total_decay == 0

    def test_no_change_scenario(self):
        """Test behavior with no OI changes."""
        oi_data = {18000: 1000, 18500: 1500, 19000: 1200}
        time_delta = 1.0

        result = compute_oi_decay_pressure(oi_data, oi_data, time_delta)

        assert result.pressure == 0.0
        assert result.total_decay == 0
        assert result.total_build == 0
        assert result.confidence == 0.0

    def test_insufficient_data(self):
        """Test handling of insufficient strike data."""
        current_oi = {18000: 100}
        previous_oi = {18000: 200}
        time_delta = 1.0

        result = compute_oi_decay_pressure(current_oi, previous_oi, time_delta)

        assert result.pressure == 0.0
        assert result.confidence == 0.0
        assert "insufficient_data" in str(result.metadata)

    def test_invalid_time_delta(self):
        """Test error handling for invalid time delta."""
        current_oi = {18000: 1000, 18500: 1500}
        previous_oi = {18000: 1200, 18500: 1600}
        time_delta = -1.0

        with pytest.raises(ValueError, match="Time delta must be positive"):
            compute_oi_decay_pressure(current_oi, previous_oi, time_delta)

    def test_invalid_input_types(self):
        """Test error handling for invalid input types."""
        with pytest.raises(ValueError, match="OI data must be dictionaries"):
            compute_oi_decay_pressure("invalid", {}, 1.0)

    def test_time_factor_scaling(self):
        """Test that time factor affects pressure scaling."""
        current_oi = {18000: 500, 18500: 1000, 19000: 800}
        previous_oi = {18000: 1000, 18500: 1500, 19000: 1200}

        # Short time delta (higher pressure)
        result_short = compute_oi_decay_pressure(current_oi, previous_oi, 0.5)
        # Long time delta (lower pressure)
        result_long = compute_oi_decay_pressure(current_oi, previous_oi, 4.0)

        assert abs(result_short.pressure) > abs(result_long.pressure)
        assert result_short.time_factor > result_long.time_factor

    def test_extreme_values_clamping(self):
        """Test that pressure values are clamped to [-1, 1]."""
        # Extreme decay
        current_oi = {18000: 1, 18500: 1, 19000: 1}
        previous_oi = {18000: 10000, 18500: 10000, 19000: 10000}
        time_delta = 0.1  # Very short time for amplification

        result = compute_oi_decay_pressure(current_oi, previous_oi, time_delta)

        assert result.pressure >= -1.0 and result.pressure <= 1.0


class TestOIDecayValidation:
    """Test OI decay signal validation."""

    def test_valid_signal_validation(self):
        """Test validation of a valid signal."""
        result = OIDecayResult(
            pressure=-0.3,
            total_decay=500,
            total_build=200,
            strike_count=5,
            time_factor=0.8,
            confidence=0.7,
            metadata={},
        )

        validation = validate_oi_decay_signal(result)

        assert validation["is_valid"] is True
        assert len(validation["warnings"]) == 0

    def test_low_confidence_warning(self):
        """Test validation warning for low confidence."""
        result = OIDecayResult(
            pressure=-0.1,
            total_decay=50,
            total_build=0,
            strike_count=3,
            time_factor=1.0,
            confidence=0.05,  # Below threshold
            metadata={},
        )

        validation = validate_oi_decay_signal(result)

        assert "Low confidence" in str(validation["warnings"])

    def test_insufficient_strikes(self):
        """Test validation with insufficient strike coverage."""
        result = OIDecayResult(
            pressure=-0.5,
            total_decay=100,
            total_build=0,
            strike_count=1,  # Below minimum
            time_factor=1.0,
            confidence=0.8,
            metadata={},
        )

        validation = validate_oi_decay_signal(result)

        assert validation["is_valid"] is False

    def test_regime_context_validation(self):
        """Test validation with regime context."""
        result = OIDecayResult(
            pressure=-0.8,
            total_decay=1000,
            total_build=0,
            strike_count=5,
            time_factor=0.5,
            confidence=0.9,
            metadata={},
        )

        # Test expiry regime
        expiry_context = {"regime": "expiry"}
        validation = validate_oi_decay_signal(result, expiry_context)

        assert validation["regime_compatibility"] == "poor"
        assert any("expiry" in warning.lower() for warning in validation["warnings"])


class TestSyntheticDataGeneration:
    """Test synthetic OI data generation."""

    def test_synthetic_data_generation(self):
        """Test basic synthetic data generation."""
        base_oi = {18000: 1000, 18500: 1500, 19000: 1200}
        decay_rate = -0.5  # 50% decay over the period
        time_delta = 24.0  # Full day

        synthetic = generate_synthetic_oi_data(base_oi, decay_rate, time_delta)

        assert len(synthetic) == len(base_oi)
        assert all(k in synthetic for k in base_oi.keys())

        # Check that values are reduced (with some noise tolerance)
        total_original = sum(base_oi.values())
        total_synthetic = sum(synthetic.values())
        assert total_synthetic < total_original

    def test_zero_decay_generation(self):
        """Test synthetic data with zero decay."""
        base_oi = {18000: 1000, 18500: 1500}
        decay_rate = 0.0
        time_delta = 1.0

        synthetic = generate_synthetic_oi_data(base_oi, decay_rate, time_delta)

        # Should be close to original (within noise)
        for strike in base_oi:
            assert abs(synthetic[strike] - base_oi[strike]) < base_oi[strike] * 0.2

    def test_building_generation(self):
        """Test synthetic data with positive building."""
        base_oi = {18000: 1000, 18500: 1500}
        decay_rate = 0.5  # 50% building over the period
        time_delta = 24.0  # Full day

        synthetic = generate_synthetic_oi_data(base_oi, decay_rate, time_delta)

        total_original = sum(base_oi.values())
        total_synthetic = sum(synthetic.values())
        assert total_synthetic > total_original


class TestOIDecayAnalysis:
    """Test OI decay pattern analysis."""

    def test_pattern_analysis_basic(self):
        """Test basic pattern analysis."""
        # Create simple OI history
        timestamps = [
            datetime(2024, 1, 1, 10, 0),
            datetime(2024, 1, 1, 11, 0),
            datetime(2024, 1, 1, 12, 0),
        ]

        oi_history = [
            {18000: 1000, 18500: 1500},  # Initial
            {18000: 800, 18500: 1200},  # Some decay
            {18000: 600, 18500: 1000},  # More decay
        ]

        analysis = analyze_oi_decay_patterns(oi_history, timestamps)

        assert "total_periods" in analysis
        assert analysis["total_periods"] == 2  # Two transitions
        assert "pressure_stats" in analysis
        assert "results" in analysis
        assert len(analysis["results"]) == 2

    def test_insufficient_history(self):
        """Test error handling for insufficient history."""
        timestamps = [datetime(2024, 1, 1, 10, 0)]
        oi_history = [{18000: 1000}]

        with pytest.raises(ValueError, match="Need at least 2 OI snapshots"):
            analyze_oi_decay_patterns(oi_history, timestamps)

    def test_mismatched_lengths(self):
        """Test error handling for mismatched history lengths."""
        timestamps = [
            datetime(2024, 1, 1, 10, 0),
            datetime(2024, 1, 1, 11, 0),
        ]
        oi_history = [{18000: 1000}]  # Only one snapshot

        with pytest.raises(ValueError, match="matching timestamps"):
            analyze_oi_decay_patterns(oi_history, timestamps)


class TestOIDecayConfig:
    """Test OI decay configuration."""

    def test_default_config(self):
        """Test default configuration values."""
        config = OIDecayConfig()

        assert config.min_strikes == 3
        assert config.max_time_factor == 1.0
        assert config.time_decay_exponent == 1.0
        assert config.confidence_threshold == 0.1

    def test_custom_config(self):
        """Test custom configuration."""
        config = OIDecayConfig(
            min_strikes=5,
            max_time_factor=2.0,
            time_decay_exponent=0.5,
            confidence_threshold=0.2,
        )

        assert config.min_strikes == 5
        assert config.max_time_factor == 2.0
        assert config.time_decay_exponent == 0.5
        assert config.confidence_threshold == 0.2
