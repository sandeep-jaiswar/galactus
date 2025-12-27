"""
Tests for Validation Rules

Tests enforcement of:
- Event-time fidelity
- Logic freezing
- Regime coverage
- Forbidden practices detection
"""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..', 'src'))

import unittest
from datetime import datetime, timedelta
from validation.rules import (
    EventTimeValidator,
    LogicFreezeValidator,
    RegimeCoverageValidator,
    ForbiddenPracticesDetector,
    ViolationType,
)
from validation.walk_forward import FrozenLogic


class TestEventTimeValidator(unittest.TestCase):
    """Test event-time fidelity enforcement"""
    
    def test_valid_data_access(self):
        """Test that valid data access passes"""
        validator = EventTimeValidator()
        
        data_time = datetime(2023, 1, 1, 9, 0)
        access_time = datetime(2023, 1, 1, 10, 0)
        
        violation = validator.validate_data_access(
            data_timestamp=data_time,
            access_timestamp=access_time,
            disclosure_delay_hours=0
        )
        
        self.assertIsNone(violation)
    
    def test_look_ahead_bias_detection(self):
        """Test that look-ahead bias is detected"""
        validator = EventTimeValidator()
        
        data_time = datetime(2023, 1, 1, 10, 0)
        access_time = datetime(2023, 1, 1, 9, 0)  # Before data exists!
        
        violation = validator.validate_data_access(
            data_timestamp=data_time,
            access_timestamp=access_time,
            disclosure_delay_hours=0
        )
        
        self.assertIsNotNone(violation)
        self.assertEqual(violation.violation_type, ViolationType.LOOK_AHEAD_BIAS)
    
    def test_disclosure_delay_enforcement(self):
        """Test that disclosure delays are enforced"""
        validator = EventTimeValidator()
        
        data_time = datetime(2023, 1, 1, 9, 0)
        access_time = datetime(2023, 1, 1, 10, 0)  # Only 1 hour later
        
        # Require 2-hour delay
        violation = validator.validate_data_access(
            data_timestamp=data_time,
            access_timestamp=access_time,
            disclosure_delay_hours=2
        )
        
        self.assertIsNotNone(violation)
        self.assertEqual(violation.violation_type, ViolationType.LOOK_AHEAD_BIAS)
    
    def test_timeline_integrity(self):
        """Test timeline integrity checking"""
        validator = EventTimeValidator()
        
        # Register events in proper order
        validator.register_event(datetime(2023, 1, 1, 9, 0), "data_available")
        validator.register_event(datetime(2023, 1, 1, 10, 0), "inference_made")
        
        violations = validator.check_timeline_integrity()
        self.assertEqual(len(violations), 0)


class TestLogicFreezeValidator(unittest.TestCase):
    """Test logic freezing enforcement"""
    
    def setUp(self):
        """Set up frozen logic"""
        self.frozen = FrozenLogic.from_config(
            logic_code="def signal(x): return x > 0.5",
            parameters={"param1": 10, "param2": "value"},
            thresholds={"threshold1": 0.5, "threshold2": 0.8},
            regime_definitions={}
        )
        self.validator = LogicFreezeValidator(self.frozen)
    
    def test_parameter_unchanged(self):
        """Test that unchanged parameters pass"""
        violation = self.validator.check_parameter_change("param1", 10, 10)
        self.assertIsNone(violation)
    
    def test_parameter_change_detection(self):
        """Test that parameter changes are detected"""
        violation = self.validator.check_parameter_change("param1", 10, 20)
        
        self.assertIsNotNone(violation)
        self.assertEqual(violation.violation_type, ViolationType.PARAMETER_CHANGE)
        self.assertIn("param1", violation.description)
    
    def test_threshold_unchanged(self):
        """Test that unchanged thresholds pass"""
        violation = self.validator.check_threshold_change("threshold1", 0.5, 0.5)
        self.assertIsNone(violation)
    
    def test_threshold_change_detection(self):
        """Test that threshold changes are detected"""
        violation = self.validator.check_threshold_change("threshold1", 0.5, 0.6)
        
        self.assertIsNotNone(violation)
        self.assertEqual(violation.violation_type, ViolationType.THRESHOLD_CHANGE)
        self.assertIn("threshold1", violation.description)
    
    def test_threshold_tolerance(self):
        """Test numerical tolerance for thresholds"""
        # Exceeds tolerance - difference is 1e-9, tolerance is 1e-10
        violation = self.validator.check_threshold_change(
            "threshold1", 0.5, 0.5 + 1e-9, tolerance=1e-10
        )
        self.assertIsNotNone(violation)  # Exceeds tolerance
        
        # Within tolerance - difference is 1e-12, tolerance is 1e-6
        violation = self.validator.check_threshold_change(
            "threshold1", 0.5, 0.5 + 1e-12, tolerance=1e-6
        )
        self.assertIsNone(violation)
    
    def test_logic_hash_verification(self):
        """Test that logic code changes are detected"""
        original = "def signal(x): return x > 0.5"
        modified = "def signal(x): return x > 0.6"
        
        # Original should pass
        violation = self.validator.verify_logic_hash(original)
        self.assertIsNone(violation)
        
        # Modified should fail
        violation = self.validator.verify_logic_hash(modified)
        self.assertIsNotNone(violation)


class TestRegimeCoverageValidator(unittest.TestCase):
    """Test regime coverage validation"""
    
    def test_sufficient_coverage(self):
        """Test that sufficient coverage passes"""
        validator = RegimeCoverageValidator()
        
        window_regimes = [
            ["high_liquidity", "low_volatility"],
            ["low_liquidity", "high_volatility"],
            ["expiry_heavy", "regime_transition"]
        ]
        
        violations = validator.validate_coverage(window_regimes)
        self.assertEqual(len(violations), 0)
    
    def test_insufficient_liquidity_coverage(self):
        """Test detection of insufficient liquidity regime coverage"""
        validator = RegimeCoverageValidator()
        
        window_regimes = [
            ["high_liquidity"],  # Only one liquidity regime
            ["high_liquidity"],
        ]
        
        violations = validator.validate_coverage(window_regimes)
        self.assertTrue(any(v.violation_type == ViolationType.WINDOW_EXCLUSION for v in violations))
        self.assertTrue(any("liquidity" in v.description.lower() for v in violations))
    
    def test_missing_expiry_period(self):
        """Test detection of missing expiry period"""
        validator = RegimeCoverageValidator()
        
        window_regimes = [
            ["high_liquidity", "low_volatility"],
            ["low_liquidity", "high_volatility"],
            # No expiry_heavy
        ]
        
        violations = validator.validate_coverage(window_regimes)
        self.assertTrue(any("expiry" in v.description.lower() for v in violations))
    
    def test_missing_regime_transition(self):
        """Test detection of missing regime transition"""
        validator = RegimeCoverageValidator()
        
        window_regimes = [
            ["high_liquidity", "low_volatility"],
            ["low_liquidity", "high_volatility"],
            ["expiry_heavy"],
            # No regime_transition
        ]
        
        violations = validator.validate_coverage(window_regimes)
        self.assertTrue(any("transition" in v.description.lower() for v in violations))
    
    def test_optional_requirements(self):
        """Test that requirements can be disabled"""
        validator = RegimeCoverageValidator(
            require_multiple_liquidity=False,
            require_multiple_volatility=False,
            require_expiry_period=False,
            require_regime_transition=False
        )
        
        # Even with minimal coverage, should pass if requirements disabled
        window_regimes = [["high_liquidity"]]
        violations = validator.validate_coverage(window_regimes)
        self.assertEqual(len(violations), 0)


class TestForbiddenPracticesDetector(unittest.TestCase):
    """Test forbidden practices detection"""
    
    def test_window_exclusion_detection(self):
        """Test detection of unjustified window exclusion"""
        detector = ForbiddenPracticesDetector()
        
        violations = detector.detect_window_exclusion(
            total_windows=10,
            excluded_window_indices=[3, 7],
            justifications={}  # No justifications
        )
        
        self.assertEqual(len(violations), 2)  # One per excluded window
        self.assertTrue(all(v.violation_type == ViolationType.WINDOW_EXCLUSION for v in violations))
    
    def test_window_exclusion_with_justification(self):
        """Test that justified exclusions are allowed"""
        detector = ForbiddenPracticesDetector()
        
        violations = detector.detect_window_exclusion(
            total_windows=10,
            excluded_window_indices=[3],
            justifications={3: "Data quality issue - exchange downtime"}
        )
        
        # Should still create violation but justification is recorded
        # High-level validation accepts justified exclusions
        self.assertEqual(len(violations), 0)
    
    def test_high_exclusion_rate_warning(self):
        """Test warning for high exclusion rate"""
        detector = ForbiddenPracticesDetector()
        
        violations = detector.detect_window_exclusion(
            total_windows=10,
            excluded_window_indices=[1, 2, 3, 4, 5],  # 50% excluded
            justifications={i: "reason" for i in [1, 2, 3, 4, 5]}
        )
        
        # Should have a warning about high exclusion rate
        self.assertTrue(any(v.severity == "warning" for v in violations))
        self.assertTrue(any("exclusion rate" in v.description.lower() for v in violations))
    
    def test_retroactive_reclassification_detection(self):
        """Test detection of retroactive regime reclassification"""
        detector = ForbiddenPracticesDetector()
        
        violation = detector.detect_retroactive_reclassification(
            window_index=5,
            original_regimes=["high_liquidity", "low_volatility"],
            new_regimes=["low_liquidity", "low_volatility"]  # Changed!
        )
        
        self.assertIsNotNone(violation)
        self.assertEqual(violation.violation_type, ViolationType.REGIME_RECLASSIFICATION)
    
    def test_no_reclassification(self):
        """Test that unchanged regimes don't trigger violation"""
        detector = ForbiddenPracticesDetector()
        
        violation = detector.detect_retroactive_reclassification(
            window_index=5,
            original_regimes=["high_liquidity", "low_volatility"],
            new_regimes=["high_liquidity", "low_volatility"]  # Same
        )
        
        self.assertIsNone(violation)
    
    def test_result_smoothing_detection(self):
        """Test detection of result smoothing"""
        detector = ForbiddenPracticesDetector()
        
        window_results = [0.5, 0.6, 0.3, 0.8]
        reported_results = [0.55, 0.55, 0.55, 0.55]  # Smoothed!
        
        violations = detector.detect_result_smoothing(window_results, reported_results)
        
        self.assertGreater(len(violations), 0)
        self.assertTrue(any(v.violation_type == ViolationType.RESULT_SMOOTHING for v in violations))
    
    def test_result_count_mismatch(self):
        """Test detection of result count mismatch"""
        detector = ForbiddenPracticesDetector()
        
        window_results = [0.5, 0.6, 0.3, 0.8]
        reported_results = [0.5, 0.6]  # Missing results!
        
        violations = detector.detect_result_smoothing(window_results, reported_results)
        
        self.assertGreater(len(violations), 0)
        self.assertTrue(any("count mismatch" in v.description.lower() for v in violations))
    
    def test_retroactive_fix_detection(self):
        """Test detection of retroactive fixes"""
        detector = ForbiddenPracticesDetector()
        
        validation_start = datetime(2023, 1, 1)
        fix_time = datetime(2023, 1, 5)  # After validation started
        
        violation = detector.detect_retroactive_fix(
            fix_description="Adjusted threshold based on observed failures",
            fix_timestamp=fix_time,
            validation_start=validation_start
        )
        
        self.assertIsNotNone(violation)
        self.assertEqual(violation.violation_type, ViolationType.RETROACTIVE_FIX)
    
    def test_fix_before_validation_allowed(self):
        """Test that fixes before validation are allowed"""
        detector = ForbiddenPracticesDetector()
        
        validation_start = datetime(2023, 1, 10)
        fix_time = datetime(2023, 1, 5)  # Before validation
        
        violation = detector.detect_retroactive_fix(
            fix_description="Pre-validation adjustment",
            fix_timestamp=fix_time,
            validation_start=validation_start
        )
        
        self.assertIsNone(violation)


if __name__ == '__main__':
    unittest.main()
