"""
Tests for Hedge Pressure Research Module

Tests the hedge pressure signal implementation and validation.
"""

from datetime import datetime

import pytest

from src.features.hedge_pressure_research import (
    HedgePressureConfig, HedgePressureResult, analyze_hedge_pressure_patterns,
    compute_hedge_pressure, generate_synthetic_hedge_data,
    validate_hedge_pressure_signal)


class TestHedgePressureComputation:
    """Test hedge pressure computation."""

    def test_basic_call_dominance(self):
        """Test detection of call dominance (positive hedge pressure)."""
        calls = {18000: 1500, 18500: 1200, 19000: 900}
        puts = {18000: 500, 18500: 400, 19000: 300}
        spot_price = 18500

        result = compute_hedge_pressure(calls, puts, spot_price)

        assert isinstance(result, HedgePressureResult)
        assert result.pressure > 0  # Positive pressure for call dominance
        assert result.call_oi_total > result.put_oi_total
        assert result.imbalance_ratio > 0
        assert result.strike_count == 3
        assert result.confidence > 0

    def test_basic_put_dominance(self):
        """Test detection of put dominance (negative hedge pressure)."""
        calls = {18000: 500, 18500: 400, 19000: 300}
        puts = {18000: 1500, 18500: 1200, 19000: 900}
        spot_price = 18500

        result = compute_hedge_pressure(calls, puts, spot_price)

        assert result.pressure < 0  # Negative pressure for put dominance
        assert result.put_oi_total > result.call_oi_total
        assert result.imbalance_ratio < 0

    def test_balanced_market(self):
        """Test behavior with balanced call/put OI."""
        calls = {18000: 1000, 18500: 800, 19000: 600}
        puts = {18000: 1000, 18500: 800, 19000: 600}
        spot_price = 18500

        result = compute_hedge_pressure(calls, puts, spot_price)

        assert abs(result.pressure) < 0.1  # Should be close to zero
        assert abs(result.imbalance_ratio) < 0.1
        assert result.call_oi_total == result.put_oi_total

    def test_insufficient_strikes(self):
        """Test handling of insufficient strike data."""
        calls = {18500: 100}
        puts = {18500: 100}
        spot_price = 18500

        result = compute_hedge_pressure(calls, puts, spot_price)

        assert result.pressure == 0.0
        assert result.confidence == 0.0
        assert "insufficient_strikes" in str(result.metadata)

    def test_insufficient_liquidity(self):
        """Test handling of very low OI totals."""
        calls = {18000: 10, 18500: 5, 19000: 2}
        puts = {18000: 10, 18500: 5, 19000: 2}
        spot_price = 18500

        result = compute_hedge_pressure(calls, puts, spot_price)

        assert result.pressure == 0.0
        assert result.confidence == 0.0
        assert "insufficient_liquidity" in str(result.metadata)

    def test_invalid_inputs(self):
        """Test error handling for invalid inputs."""
        with pytest.raises(TypeError, match="OI data must be dictionaries"):
            compute_hedge_pressure("invalid", {}, 18500)

        with pytest.raises(ValueError, match="Spot price must be a positive number"):
            compute_hedge_pressure({18500: 100}, {18500: 100}, -100)

    def test_extreme_imbalance(self):
        """Test behavior with extreme call/put imbalance."""
        calls = {18000: 10000, 18500: 5000, 19000: 1000}
        puts = {18000: 10, 18500: 5, 19000: 1}
        spot_price = 18500

        result = compute_hedge_pressure(calls, puts, spot_price)

        assert result.pressure <= 1.0 and result.pressure >= -1.0  # Should be clamped
        assert result.imbalance_ratio > 0.9  # Very high imbalance

    def test_spot_price_sensitivity(self):
        """Test that spot price affects the weighting."""
        calls = {18000: 1000, 18500: 800, 19000: 600}
        puts = {18000: 600, 18500: 800, 19000: 1000}

        # Test with different spot prices
        result_atm = compute_hedge_pressure(calls, puts, 18500)  # At-the-money
        result_otm = compute_hedge_pressure(calls, puts, 18200)  # OTM calls

        # Results should be different due to weighting
        # Note: This test uses imbalanced OI, so spot price weighting matters
        assert result_atm.pressure != result_otm.pressure


class TestHedgePressureValidation:
    """Test hedge pressure signal validation."""

    def test_valid_signal_validation(self):
        """Test validation of a valid signal."""
        result = HedgePressureResult(
            pressure=0.4,
            call_oi_total=2000,
            put_oi_total=1000,
            imbalance_ratio=0.33,
            spot_distance_factor=3.5,
            strike_count=5,
            confidence=0.8,
            metadata={},
        )

        validation = validate_hedge_pressure_signal(result)

        assert validation["is_valid"] is True
        assert len(validation["warnings"]) == 0

    def test_low_confidence_warning(self):
        """Test validation warning for low confidence."""
        result = HedgePressureResult(
            pressure=0.05,
            call_oi_total=100,
            put_oi_total=90,
            imbalance_ratio=0.05,
            spot_distance_factor=2.0,
            strike_count=3,
            confidence=0.02,  # Below threshold
            metadata={},
        )

        validation = validate_hedge_pressure_signal(result)

        assert "Low confidence" in str(validation["warnings"])

    def test_weak_imbalance_warning(self):
        """Test validation warning for weak imbalance."""
        result = HedgePressureResult(
            pressure=0.02,
            call_oi_total=1100,
            put_oi_total=1000,
            imbalance_ratio=0.05,
            spot_distance_factor=3.0,
            strike_count=4,
            confidence=0.6,
            metadata={},
        )

        validation = validate_hedge_pressure_signal(result)

        assert "Weak imbalance" in str(validation["warnings"])

    def test_insufficient_strikes(self):
        """Test validation with insufficient strike coverage."""
        result = HedgePressureResult(
            pressure=0.3,
            call_oi_total=1000,
            put_oi_total=800,
            imbalance_ratio=0.11,
            spot_distance_factor=1.5,
            strike_count=1,  # Below minimum
            confidence=0.7,
            metadata={},
        )

        validation = validate_hedge_pressure_signal(result)

        assert validation["is_valid"] is False

    def test_regime_context_validation(self):
        """Test validation with regime context."""
        result = HedgePressureResult(
            pressure=0.6,
            call_oi_total=1500,
            put_oi_total=500,
            imbalance_ratio=0.5,
            spot_distance_factor=4.0,
            strike_count=5,
            confidence=0.9,
            metadata={},
        )

        # Test expiry regime
        expiry_context = {"regime": "expiry"}
        validation = validate_hedge_pressure_signal(result, expiry_context)

        assert validation["regime_compatibility"] == "poor"
        assert any("expiry" in warning.lower() for warning in validation["warnings"])

        # Test earnings regime
        earnings_context = {"regime": "earnings"}
        validation = validate_hedge_pressure_signal(result, earnings_context)

        assert validation["regime_compatibility"] == "moderate"


class TestSyntheticDataGeneration:
    """Test synthetic hedge data generation."""

    def test_synthetic_data_generation(self):
        """Test basic synthetic data generation."""
        base_calls = {18000: 1000, 18500: 1500, 19000: 1200}
        base_puts = {18000: 1000, 18500: 1500, 19000: 1200}
        spot_price = 18500
        hedge_pressure = 0.5  # Bullish pressure

        syn_calls, syn_puts = generate_synthetic_hedge_data(
            base_calls, base_puts, spot_price, hedge_pressure
        )

        assert len(syn_calls) == len(base_calls)
        assert len(syn_puts) == len(base_puts)
        assert all(k in syn_calls for k in base_calls.keys())
        assert all(k in syn_puts for k in base_puts.keys())

        # Check that calls increased relative to puts
        total_syn_calls = sum(syn_calls.values())
        total_syn_puts = sum(syn_puts.values())
        assert total_syn_calls > total_syn_puts

    def test_zero_pressure_generation(self):
        """Test synthetic data with zero hedge pressure."""
        base_calls = {18000: 1000, 18500: 1500}
        base_puts = {18000: 1000, 18500: 1500}
        spot_price = 18500
        hedge_pressure = 0.0

        syn_calls, syn_puts = generate_synthetic_hedge_data(
            base_calls, base_puts, spot_price, hedge_pressure
        )

        # Should be close to original (within noise tolerance)
        total_calls = sum(syn_calls.values())
        total_puts = sum(syn_puts.values())
        total_base_calls = sum(base_calls.values())
        total_base_puts = sum(base_puts.values())

        assert abs(total_calls - total_base_calls) < total_base_calls * 0.3
        assert abs(total_puts - total_base_puts) < total_base_puts * 0.3

    def test_negative_pressure_generation(self):
        """Test synthetic data with negative (put-dominant) pressure."""
        base_calls = {18000: 1000, 18500: 1500}
        base_puts = {18000: 1000, 18500: 1500}
        spot_price = 18500
        hedge_pressure = -0.5  # Put-dominant pressure

        syn_calls, syn_puts = generate_synthetic_hedge_data(
            base_calls, base_puts, spot_price, hedge_pressure
        )

        total_syn_calls = sum(syn_calls.values())
        total_syn_puts = sum(syn_puts.values())
        assert total_syn_puts > total_syn_calls


class TestHedgePressureAnalysis:
    """Test hedge pressure pattern analysis."""

    def test_pattern_analysis_basic(self):
        """Test basic pattern analysis."""
        # Create simple OI history
        timestamps = [
            datetime(2024, 1, 1, 10, 0),
            datetime(2024, 1, 1, 11, 0),
            datetime(2024, 1, 1, 12, 0),
        ]

        calls_history = [
            {18000: 1000, 18500: 1500},  # Initial
            {18000: 1200, 18500: 1800},  # More calls
            {18000: 800, 18500: 1200},  # Fewer calls
        ]

        puts_history = [
            {18000: 1000, 18500: 1500},  # Initial
            {18000: 800, 18500: 1200},  # Fewer puts
            {18000: 1200, 18500: 1800},  # More puts
        ]

        spot_prices = [18500, 18500, 18500]

        analysis = analyze_hedge_pressure_patterns(
            calls_history, puts_history, spot_prices, timestamps
        )

        assert "total_periods" in analysis
        assert analysis["total_periods"] == 3  # All periods
        assert "pressure_stats" in analysis
        assert "results" in analysis
        assert len(analysis["results"]) == 3

    def test_insufficient_history(self):
        """Test error handling for insufficient history."""
        timestamps = [datetime(2024, 1, 1, 10, 0)]
        calls_history = [{18000: 1000}]
        puts_history = [{18000: 1000}]
        spot_prices = [18500]

        with pytest.raises(ValueError, match="Need at least 2 OI snapshots"):
            analyze_hedge_pressure_patterns(
                calls_history, puts_history, spot_prices, timestamps
            )

    def test_mismatched_lengths(self):
        """Test error handling for mismatched history lengths."""
        timestamps = [
            datetime(2024, 1, 1, 10, 0),
            datetime(2024, 1, 1, 11, 0),
        ]
        calls_history = [{18000: 1000}]  # Only one snapshot
        puts_history = [{18000: 1000}, {18000: 1000}]
        spot_prices = [18500, 18500]

        with pytest.raises(ValueError, match="matching timestamps"):
            analyze_hedge_pressure_patterns(
                calls_history, puts_history, spot_prices, timestamps
            )


class TestHedgePressureConfig:
    """Test hedge pressure configuration."""

    def test_default_config(self):
        """Test default configuration values."""
        config = HedgePressureConfig()

        assert config.min_strikes == 3
        assert config.max_distance_factor == 2.0
        assert config.spot_sensitivity == 1.0
        assert config.confidence_threshold == 0.1
        assert config.imbalance_threshold == 0.3

    def test_custom_config(self):
        """Test custom configuration."""
        config = HedgePressureConfig(
            min_strikes=5,
            max_distance_factor=3.0,
            spot_sensitivity=0.5,
            confidence_threshold=0.2,
            imbalance_threshold=0.4,
        )

        assert config.min_strikes == 5
        assert config.max_distance_factor == 3.0
        assert config.spot_sensitivity == 0.5
        assert config.confidence_threshold == 0.2
        assert config.imbalance_threshold == 0.4
