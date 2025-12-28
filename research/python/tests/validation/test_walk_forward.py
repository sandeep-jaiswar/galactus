"""
Tests for Walk-Forward Validation Framework

Tests enforce the principles defined in:
docs/07-backtesting-and-validation/walk-forward-validation.md
"""

import os
import sys

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "..", "src"))

import unittest  # noqa: E402
from datetime import datetime  # noqa: E402

from validation import (ValidationConfig, ValidationError,  # noqa: E402
                        ValidationWindow, WalkForwardValidator, WindowType)
from validation.walk_forward import FrozenLogic  # noqa: E402


class TestValidationConfig(unittest.TestCase):
    """Test validation configuration"""

    def test_valid_config(self):
        """Test creating valid configuration"""
        config = ValidationConfig(
            training_window_days=90, validation_window_days=30, step_size_days=30
        )
        self.assertEqual(config.training_window_days, 90)
        self.assertTrue(config.enforce_event_time)

    def test_config_immutability(self):
        """Test that config is frozen (immutable)"""
        config = ValidationConfig(
            training_window_days=90, validation_window_days=30, step_size_days=30
        )

        # Frozen dataclass should not allow attribute changes
        with self.assertRaises(Exception):
            config.training_window_days = 100

    def test_invalid_window_days(self):
        """Test that invalid window days are rejected"""
        with self.assertRaises(ValueError):
            ValidationConfig(
                training_window_days=-1, validation_window_days=30, step_size_days=30
            )

    def test_event_time_enforcement_mandatory(self):
        """Test that event-time enforcement cannot be disabled"""
        with self.assertRaises(ValueError):
            ValidationConfig(
                training_window_days=90,
                validation_window_days=30,
                step_size_days=30,
                enforce_event_time=False,
            )


class TestValidationWindow(unittest.TestCase):
    """Test validation window management"""

    def test_create_valid_window(self):
        """Test creating a valid window"""
        start = datetime(2023, 1, 1)
        end = datetime(2023, 2, 1)

        window = ValidationWindow(
            window_type=WindowType.VALIDATION,
            start_time=start,
            end_time=end,
            regime_labels=["high_liquidity", "low_volatility"],
        )

        self.assertEqual(window.start_time, start)
        self.assertEqual(window.end_time, end)
        self.assertEqual(window.duration_days, 31)

    def test_invalid_time_ordering(self):
        """Test that end must be after start"""
        with self.assertRaises(ValueError):
            ValidationWindow(
                window_type=WindowType.VALIDATION,
                start_time=datetime(2023, 2, 1),
                end_time=datetime(2023, 1, 1),
                regime_labels=[],
            )

    def test_training_window_requires_justification(self):
        """Test that training windows require justification"""
        with self.assertRaises(ValueError):
            ValidationWindow(
                window_type=WindowType.TRAINING,
                start_time=datetime(2023, 1, 1),
                end_time=datetime(2023, 2, 1),
                regime_labels=[],
                justification=None,
            )

    def test_window_overlap_detection(self):
        """Test overlap detection between windows"""
        window1 = ValidationWindow(
            window_type=WindowType.VALIDATION,
            start_time=datetime(2023, 1, 1),
            end_time=datetime(2023, 2, 1),
            regime_labels=[],
        )

        # Overlapping window
        window2 = ValidationWindow(
            window_type=WindowType.VALIDATION,
            start_time=datetime(2023, 1, 15),
            end_time=datetime(2023, 2, 15),
            regime_labels=[],
        )

        # Non-overlapping window
        window3 = ValidationWindow(
            window_type=WindowType.VALIDATION,
            start_time=datetime(2023, 2, 1),
            end_time=datetime(2023, 3, 1),
            regime_labels=[],
        )

        self.assertTrue(window1.overlaps_with(window2))
        self.assertFalse(window1.overlaps_with(window3))


class TestFrozenLogic(unittest.TestCase):
    """Test logic freezing mechanism"""

    def test_create_frozen_logic(self):
        """Test creating frozen logic"""
        logic_code = "def compute_signal(data): return data * 2"
        params = {"threshold": 0.5, "window": 20}
        thresholds = {"confidence_min": 0.7}
        regimes = {"high_vol": {"threshold": 0.3}}

        frozen = FrozenLogic.from_config(
            logic_code=logic_code,
            parameters=params,
            thresholds=thresholds,
            regime_definitions=regimes,
        )

        self.assertIsNotNone(frozen.logic_hash)
        self.assertEqual(frozen.parameters, params)
        self.assertTrue(frozen.verify_unchanged(logic_code))

    def test_detect_logic_change(self):
        """Test that logic changes are detected"""
        original_code = "def compute_signal(data): return data * 2"
        modified_code = "def compute_signal(data): return data * 3"

        frozen = FrozenLogic.from_config(
            logic_code=original_code,
            parameters={},
            thresholds={},
            regime_definitions={},
        )

        self.assertTrue(frozen.verify_unchanged(original_code))
        self.assertFalse(frozen.verify_unchanged(modified_code))


class TestWalkForwardValidator(unittest.TestCase):
    """Test walk-forward validator orchestration"""

    def setUp(self):
        """Set up test fixtures"""
        self.config = ValidationConfig(
            training_window_days=90, validation_window_days=30, step_size_days=30
        )
        self.validator = WalkForwardValidator(self.config)

    def test_create_validator(self):
        """Test validator creation"""
        self.assertIsNotNone(self.validator)

    def test_freeze_logic(self):
        """Test logic freezing"""
        logic_code = "def signal(x): return x > 0.5"

        self.validator.freeze_logic(
            logic_code=logic_code,
            parameters={"param1": 10},
            thresholds={"threshold1": 0.5},
            regime_definitions={"regime1": {}},
        )

        # Should not be able to freeze again
        with self.assertRaises(ValidationError):
            self.validator.freeze_logic(
                logic_code=logic_code,
                parameters={},
                thresholds={},
                regime_definitions={},
            )

    def test_set_training_window(self):
        """Test setting training window"""
        start = datetime(2023, 1, 1)
        end = datetime(2023, 3, 31)

        self.validator.set_training_window(
            start_time=start,
            end_time=end,
            justification="Initial hypothesis discovery period covering Q1 2023",
        )

        # Cannot set twice
        with self.assertRaises(ValidationError):
            self.validator.set_training_window(
                start_time=start, end_time=end, justification="Another period"
            )

    def test_training_window_requires_justification(self):
        """Test that training window requires justification"""
        with self.assertRaises(ValidationError):
            self.validator.set_training_window(
                start_time=datetime(2023, 1, 1),
                end_time=datetime(2023, 3, 31),
                justification="",
            )

    def test_add_validation_windows(self):
        """Test adding validation windows"""
        # Set training window first
        self.validator.set_training_window(
            start_time=datetime(2023, 1, 1),
            end_time=datetime(2023, 3, 31),
            justification="Training period",
        )

        # Add validation windows
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity", "low_volatility"],
        )

        self.validator.add_validation_window(
            start_time=datetime(2023, 5, 1),
            end_time=datetime(2023, 5, 31),
            regime_labels=["low_liquidity", "high_volatility"],
        )

    def test_validation_window_after_training(self):
        """Test that validation windows must come after training"""
        self.validator.set_training_window(
            start_time=datetime(2023, 1, 1),
            end_time=datetime(2023, 3, 31),
            justification="Training period",
        )

        # This should fail - starts before training ends
        with self.assertRaises(ValidationError):
            self.validator.add_validation_window(
                start_time=datetime(2023, 3, 1),  # Overlaps with training
                end_time=datetime(2023, 4, 1),
                regime_labels=["high_liquidity"],
            )

    def test_chronological_ordering(self):
        """Test that windows must be added in chronological order"""
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity"],
        )

        # This should fail - earlier than previous window
        with self.assertRaises(ValidationError):
            self.validator.add_validation_window(
                start_time=datetime(2023, 3, 1),
                end_time=datetime(2023, 3, 31),
                regime_labels=["low_liquidity"],
            )

    def test_overlapping_windows_require_justification(self):
        """Test that overlapping validation windows require justification"""
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity"],
        )

        # Overlapping without justification should fail
        with self.assertRaises(ValidationError):
            self.validator.add_validation_window(
                start_time=datetime(2023, 4, 15),  # Overlaps
                end_time=datetime(2023, 5, 15),
                regime_labels=["low_liquidity"],
            )

        # With justification should succeed
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 15),
            end_time=datetime(2023, 5, 15),
            regime_labels=["low_liquidity"],
            justification="Testing regime transition at month boundary",
        )

    def test_evaluate_window_requires_frozen_logic(self):
        """Test that evaluation requires frozen logic"""
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity"],
        )

        def dummy_inference(start, end):
            return {"signal": 0.5}

        # Should fail - logic not frozen
        with self.assertRaises(ValidationError):
            self.validator.evaluate_window(0, dummy_inference)

    def test_full_validation_workflow(self):
        """Test complete validation workflow"""
        # 1. Freeze logic
        self.validator.freeze_logic(
            logic_code="def signal(x): return x > 0.5",
            parameters={"param1": 10},
            thresholds={"threshold1": 0.5},
            regime_definitions={},
        )

        # 2. Set training window
        self.validator.set_training_window(
            start_time=datetime(2023, 1, 1),
            end_time=datetime(2023, 3, 31),
            justification="Q1 2023 training period",
        )

        # 3. Add validation windows with diverse regimes
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity", "low_volatility"],
        )

        self.validator.add_validation_window(
            start_time=datetime(2023, 5, 1),
            end_time=datetime(2023, 5, 31),
            regime_labels=["low_liquidity", "high_volatility"],
        )

        self.validator.add_validation_window(
            start_time=datetime(2023, 6, 1),
            end_time=datetime(2023, 6, 30),
            regime_labels=["expiry_heavy", "regime_transition"],
        )

        # 4. Evaluate windows
        def dummy_inference(start, end):
            return {"pressure_detected": True, "confidence": 0.7}

        self.validator.evaluate_window(0, dummy_inference)
        self.validator.evaluate_window(1, dummy_inference)
        self.validator.evaluate_window(2, dummy_inference)

        # 5. Finalize
        result = self.validator.finalize(
            documentation="Validation shows stable behavior across regimes"
        )

        self.assertTrue(result.passed)
        self.assertEqual(len(result.validation_windows), 3)
        # Date differences: Apr 1-30 (29), May 1-31 (30), Jun 1-30 (29) = 88 days
        self.assertEqual(result.total_validation_days, 88)

    def test_regime_coverage_validation(self):
        """Test that regime coverage is validated"""
        self.validator.freeze_logic(
            logic_code="def signal(x): return x",
            parameters={},
            thresholds={},
            regime_definitions={},
        )

        # Add only one liquidity regime - should fail
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity"],
        )

        result = self.validator.finalize(documentation="Test")

        self.assertFalse(result.passed)
        self.assertTrue(any("liquidity" in f.lower() for f in result.failures))

    def test_no_failures_warning(self):
        """Test that validation warns if no failures are documented"""
        self.validator.freeze_logic(
            logic_code="def signal(x): return x",
            parameters={},
            thresholds={},
            regime_definitions={},
        )

        # Add windows with good coverage
        self.validator.add_validation_window(
            start_time=datetime(2023, 4, 1),
            end_time=datetime(2023, 4, 30),
            regime_labels=["high_liquidity", "low_volatility"],
        )

        self.validator.add_validation_window(
            start_time=datetime(2023, 5, 1),
            end_time=datetime(2023, 5, 31),
            regime_labels=[
                "low_liquidity",
                "high_volatility",
                "expiry_heavy",
                "regime_transition",
            ],
        )

        def perfect_inference(start, end):
            return {"signal": 1.0}  # No failures

        self.validator.evaluate_window(0, perfect_inference)
        self.validator.evaluate_window(1, perfect_inference)

        result = self.validator.finalize(documentation="Test")

        # Should have warning about no failures
        self.assertTrue(any("never fail" in w.lower() for w in result.warnings))


if __name__ == "__main__":
    unittest.main()
