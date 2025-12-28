"""
Tests for feature isolation mechanisms.

These tests validate that feature isolation works correctly and
prevents contamination of production logic.
"""

import os
import sys

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "../src"))

import warnings  # noqa: E402

import pytest  # noqa: E402

from features.isolation import (FeatureIsolationContext,  # noqa: E402
                                FeatureIsolationError, get_feature_metadata,
                                list_experimental_features, mark_experimental,
                                prevent_production_use,
                                validate_feature_isolation)


class TestFeatureIsolation:
    """Test feature isolation decorators and utilities."""

    def test_mark_experimental_creates_metadata(self):
        """Test that @mark_experimental creates proper metadata."""

        @mark_experimental(
            hypothesis="Test hypothesis",
            assumptions=["Assumption 1", "Assumption 2"],
            failure_modes=["Failure 1"],
        )
        def test_feature():
            return "result"

        # Check that feature is marked as experimental
        assert hasattr(test_feature, "__experimental__")
        assert test_feature.__experimental__ is True

        # Check metadata exists
        assert hasattr(test_feature, "__feature_metadata__")
        metadata = test_feature.__feature_metadata__

        assert metadata.name == "test_feature"
        assert metadata.hypothesis == "Test hypothesis"
        assert len(metadata.assumptions) == 2
        assert len(metadata.failure_modes) == 1

    def test_mark_experimental_emits_warning(self):
        """Test that using experimental features emits warnings."""

        @mark_experimental(
            hypothesis="Test hypothesis", assumptions=[], failure_modes=[]
        )
        def test_feature():
            return "result"

        # Using the feature should emit a warning
        with pytest.warns(UserWarning, match="experimental feature"):
            result = test_feature()

        assert result == "result"

    def test_prevent_production_use_allows_research(self):
        """Test that features work in research environment."""
        # Ensure we're not in production
        original_env = os.getenv("GALACTUS_ENV")
        if original_env == "production":
            os.environ.pop("GALACTUS_ENV")

        try:
            # Should not raise in research mode
            prevent_production_use("test_feature")
        finally:
            # Restore original environment
            if original_env:
                os.environ["GALACTUS_ENV"] = original_env

    def test_prevent_production_use_blocks_production(self):
        """Test that features are blocked in production environment."""
        # Set production environment
        original_env = os.getenv("GALACTUS_ENV")
        os.environ["GALACTUS_ENV"] = "production"

        try:
            with pytest.raises(
                FeatureIsolationError, match="cannot be used in production"
            ):
                prevent_production_use("test_feature")
        finally:
            # Restore original environment
            if original_env:
                os.environ["GALACTUS_ENV"] = original_env
            else:
                os.environ.pop("GALACTUS_ENV", None)

    def test_validate_feature_isolation(self):
        """Test validation of feature isolation."""

        @mark_experimental(hypothesis="Test", assumptions=[], failure_modes=[])
        def isolated_feature():
            pass

        def regular_function():
            pass

        # Isolated feature should validate
        assert validate_feature_isolation(isolated_feature) is True

        # Regular function should not validate
        assert validate_feature_isolation(regular_function) is False

    def test_get_feature_metadata(self):
        """Test retrieving feature metadata."""

        @mark_experimental(
            hypothesis="Test hypothesis", assumptions=["A1", "A2"], failure_modes=["F1"]
        )
        def test_feature():
            pass

        metadata = get_feature_metadata(test_feature)

        assert metadata is not None
        assert metadata.name == "test_feature"
        assert metadata.hypothesis == "Test hypothesis"
        assert len(metadata.assumptions) == 2
        assert len(metadata.failure_modes) == 1

    def test_list_experimental_features(self):
        """Test listing all experimental features in a module."""

        # Create a mock module with features
        class MockModule:
            @mark_experimental(hypothesis="H1", assumptions=[], failure_modes=[])
            def feature1(self):
                pass

            @mark_experimental(hypothesis="H2", assumptions=[], failure_modes=[])
            def feature2(self):
                pass

            def regular_function(self):
                pass

        module = MockModule()
        features = list_experimental_features(module)

        # Should find both experimental features
        assert len(features) == 2
        assert "feature1" in features
        assert "feature2" in features
        assert "regular_function" not in features


class TestFeatureIsolationContext:
    """Test the FeatureIsolationContext context manager."""

    def test_context_allows_research_mode(self):
        """Test that context works in research mode."""
        original_env = os.getenv("GALACTUS_ENV")
        if original_env == "production":
            os.environ.pop("GALACTUS_ENV")

        try:
            with FeatureIsolationContext() as ctx:
                assert ctx is not None
        finally:
            if original_env:
                os.environ["GALACTUS_ENV"] = original_env

    def test_context_blocks_production_mode(self):
        """Test that context blocks production environment."""
        original_env = os.getenv("GALACTUS_ENV")
        os.environ["GALACTUS_ENV"] = "production"

        try:
            with pytest.raises(FeatureIsolationError, match="production environment"):
                with FeatureIsolationContext():
                    pass
        finally:
            if original_env:
                os.environ["GALACTUS_ENV"] = original_env
            else:
                os.environ.pop("GALACTUS_ENV", None)


class TestFeatureMetadata:
    """Test FeatureMetadata class."""

    def test_metadata_to_dict(self):
        """Test converting metadata to dictionary."""
        from datetime import datetime

        from features.isolation import FeatureMetadata

        metadata = FeatureMetadata(
            name="test_feature",
            hypothesis="Test hypothesis",
            created=datetime(2024, 1, 1),
            assumptions=["A1"],
            failure_modes=["F1"],
        )

        result = metadata.to_dict()

        assert result["name"] == "test_feature"
        assert result["hypothesis"] == "Test hypothesis"
        assert result["assumptions"] == ["A1"]
        assert result["failure_modes"] == ["F1"]
        assert result["promoted"] is False
        assert result["production_ready"] is False


class TestExampleFeatures:
    """Test the example features work correctly."""

    def test_compute_oi_decay_pressure_basic(self):
        """Basic sanity check for OI decay pressure feature."""
        from features.examples import compute_oi_decay_pressure

        current_oi = {18000: 1000, 18500: 2000}
        previous_oi = {18000: 1500, 18500: 2500}

        with warnings.catch_warnings():
            warnings.simplefilter("ignore")  # Ignore experimental warnings for test
            result = compute_oi_decay_pressure(current_oi, previous_oi, 1.0)

        assert isinstance(result, float)
        assert -1.0 <= result <= 1.0

    def test_compute_hedge_pressure_basic(self):
        """Basic sanity check for hedge pressure feature."""
        from features.examples import compute_hedge_pressure

        calls_oi = {18000: 500, 18500: 1000, 19000: 800}
        puts_oi = {18000: 1200, 18500: 1500, 19000: 600}

        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            result = compute_hedge_pressure(calls_oi, puts_oi, 18500)

        assert isinstance(result, dict)
        assert "pressure" in result
        assert "confidence" in result
        assert -1.0 <= result["pressure"] <= 1.0
        assert 0.0 <= result["confidence"] <= 1.0

    def test_compute_basis_pressure_basic(self):
        """Basic sanity check for basis pressure feature."""
        from features.examples import compute_basis_pressure

        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            result = compute_basis_pressure(18550, 18500, 15)

        assert isinstance(result, dict)
        assert "pressure" in result
        assert "divergence_pct" in result
        assert "actual_basis" in result


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
