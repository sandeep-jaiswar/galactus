"""
Example: Walk-Forward Validation Usage

This example demonstrates how to use the walk-forward validation framework
to validate a signal using event-time honest evaluation with frozen logic.

Based on: docs/07-backtesting-and-validation/walk-forward-validation.md
"""

import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from datetime import datetime, timedelta
from validation import (
    WalkForwardValidator,
    ValidationConfig,
    EventTimeValidator,
    LogicFreezeValidator,
)


def main():
    """
    Example: Validating a capital pressure signal with walk-forward validation
    """
    
    # Step 1: Define validation configuration
    # All parameters are frozen - no mid-validation changes allowed
    config = ValidationConfig(
        training_window_days=90,      # 3 months for hypothesis discovery
        validation_window_days=30,     # 1 month per validation window
        step_size_days=30,             # Non-overlapping windows
        enforce_event_time=True,       # Mandatory - no look-ahead bias
        respect_disclosure_delays=True, # Respect reporting delays
        require_multiple_liquidity_regimes=True,
        require_multiple_volatility_regimes=True,
        require_expiry_period=True,
        require_regime_transition=True,
    )
    
    # Step 2: Create validator
    validator = WalkForwardValidator(config)
    
    # Step 3: Freeze logic before validation
    # This is the actual signal computation logic that will be tested
    signal_logic = """
def compute_pressure_signal(data, params):
    # Example: Simple pressure signal based on order flow imbalance
    threshold = params['pressure_threshold']
    imbalance = (data['buy_volume'] - data['sell_volume']) / data['total_volume']
    
    if abs(imbalance) > threshold:
        return {
            'pressure_detected': True,
            'direction': 'buy' if imbalance > 0 else 'sell',
            'magnitude': abs(imbalance),
            'confidence': min(abs(imbalance) / threshold, 1.0)
        }
    
    return {'pressure_detected': False, 'confidence': 0.0}
"""
    
    # Freeze parameters, thresholds, and regime definitions
    validator.freeze_logic(
        logic_code=signal_logic,
        parameters={
            'lookback_window': 20,
            'min_volume_threshold': 1000000,
        },
        thresholds={
            'pressure_threshold': 0.15,
            'confidence_min': 0.6,
        },
        regime_definitions={
            'high_liquidity': {'min_daily_volume': 5000000},
            'low_liquidity': {'max_daily_volume': 5000000},
            'high_volatility': {'min_daily_range_pct': 0.02},
            'low_volatility': {'max_daily_range_pct': 0.02},
        }
    )
    
    # Step 4: Define training window (hypothesis discovery)
    validator.set_training_window(
        start_time=datetime(2023, 1, 1),
        end_time=datetime(2023, 3, 31),
        justification=(
            "Q1 2023 training period covering post-COVID recovery phase. "
            "Used for hypothesis discovery and parameter selection only. "
            "Results from this window are not considered validation evidence."
        )
    )
    
    # Step 5: Define validation windows sequentially
    # Each window must cover different market regimes
    
    # Window 1: High liquidity, low volatility (calm period)
    validator.add_validation_window(
        start_time=datetime(2023, 4, 1),
        end_time=datetime(2023, 4, 30),
        regime_labels=["high_liquidity", "low_volatility"],
    )
    
    # Window 2: Low liquidity, high volatility (stress period)
    validator.add_validation_window(
        start_time=datetime(2023, 5, 1),
        end_time=datetime(2023, 5, 31),
        regime_labels=["low_liquidity", "high_volatility"],
    )
    
    # Window 3: Expiry-heavy period
    validator.add_validation_window(
        start_time=datetime(2023, 6, 22),
        end_time=datetime(2023, 6, 30),
        regime_labels=["expiry_heavy"],
    )
    
    # Window 4: Regime transition
    validator.add_validation_window(
        start_time=datetime(2023, 7, 1),
        end_time=datetime(2023, 7, 31),
        regime_labels=["regime_transition", "high_volatility"],
    )
    
    # Step 6: Evaluate each window
    # The inference function receives window boundaries and must respect event-time
    
    def evaluate_signal_for_window(start_time, end_time):
        """
        Evaluate signal for a specific window.
        
        This function MUST:
        - Use only data available up to the evaluation time
        - Respect disclosure delays
        - Not use any future information
        - Apply frozen logic without modifications
        """
        
        # In real usage, this would:
        # 1. Load data for the window from canonical data source
        # 2. Apply frozen logic to each event
        # 3. Track all activations and failures
        # 4. Return observations
        
        # Example observations (in real usage, computed from actual data):
        observations = {
            'total_events': 620,
            'pressure_activations': 45,
            'false_positives': 3,
            'missed_events': 7,
            'average_confidence': 0.72,
            'regime_stability': 0.85,
        }
        
        return observations
    
    # Evaluate all windows in chronological order
    print("Evaluating validation windows...")
    for i in range(len(validator._validation_windows)):
        print(f"  Window {i+1}: {validator._validation_windows[i].start_time} to {validator._validation_windows[i].end_time}")
        result = validator.evaluate_window(i, evaluate_signal_for_window)
        print(f"    Observations: {result.observations}")
        
        # Document any failures observed
        if result.observations.get('false_positives', 0) > 5:
            result.failures.append(
                f"High false positive rate: {result.observations['false_positives']} "
                "- likely due to regime classification uncertainty"
            )
    
    # Step 7: Finalize and get validation result
    documentation = """
    Walk-Forward Validation Results - Capital Pressure Signal
    
    Signal: Order flow imbalance pressure detection
    Period: April 2023 - July 2023
    
    Observations:
    - Signal activated consistently across all regime types
    - Confidence remained stable in high liquidity regimes
    - Expected degradation during regime transitions (acceptable)
    - False positive rate increased during expiry period (expected behavior)
    
    Failures:
    - Window 3 showed higher false positives during expiry volatility
    - This is acceptable as signal is designed to suppress during extreme events
    
    Conclusion:
    Signal demonstrates stable structural behavior across regimes.
    Failures are understood and align with design expectations.
    APPROVED for promotion to consideration.
    """
    
    result = validator.finalize(documentation=documentation)
    
    # Step 8: Review results
    print("\n" + "="*60)
    print("VALIDATION RESULTS")
    print("="*60)
    print(f"Passed: {result.passed}")
    print(f"Total validation days: {result.total_validation_days}")
    print(f"Regime coverage: {result.regime_coverage}")
    print(f"\nFailures: {len(result.failures)}")
    for failure in result.failures:
        print(f"  - {failure}")
    print(f"\nWarnings: {len(result.warnings)}")
    for warning in result.warnings:
        print(f"  - {warning}")
    
    if result.passed:
        print("\n✓ Validation PASSED - Signal is structurally sound")
        print("  Next step: Document in research log and prepare for promotion review")
    else:
        print("\n✗ Validation FAILED - Signal requires refinement")
        print("  Do NOT promote. Refine hypothesis and re-validate.")
    
    return result


def example_forbidden_practice_detection():
    """
    Example: How forbidden practices are detected and prevented
    """
    from validation.rules import ForbiddenPracticesDetector
    
    print("\n" + "="*60)
    print("FORBIDDEN PRACTICES DETECTION")
    print("="*60)
    
    detector = ForbiddenPracticesDetector()
    
    # Example 1: Detecting retroactive regime reclassification
    violation = detector.detect_retroactive_reclassification(
        window_index=3,
        original_regimes=["high_volatility", "low_liquidity"],
        new_regimes=["normal_volatility", "low_liquidity"]  # Changed!
    )
    
    if violation:
        print(f"\n✗ VIOLATION DETECTED: {violation.violation_type.value}")
        print(f"  {violation.description}")
        print("  This invalidates the entire validation!")
    
    # Example 2: Detecting result smoothing
    window_results = [0.8, 0.3, 0.9, 0.2, 0.7]  # Raw results
    reported_results = [0.6, 0.6, 0.6, 0.6, 0.6]  # Smoothed!
    
    violations = detector.detect_result_smoothing(window_results, reported_results)
    
    if violations:
        print(f"\n✗ RESULT SMOOTHING DETECTED: {len(violations)} violations")
        print("  Results were artificially smoothed across windows")
        print("  This is a form of hindsight bias and invalidates validation!")
    
    # Example 3: Detecting high exclusion rate
    violations = detector.detect_window_exclusion(
        total_windows=10,
        excluded_window_indices=[2, 4, 7, 8, 9],  # 50% excluded!
        justifications={i: "Bad performance" for i in [2, 4, 7, 8, 9]}
    )
    
    if violations:
        print(f"\n⚠ WARNING: High exclusion rate detected")
        for v in violations:
            if v.severity == "warning":
                print(f"  {v.description}")


if __name__ == "__main__":
    # Run main validation example
    result = main()
    
    # Show forbidden practices detection
    example_forbidden_practice_detection()
    
    print("\n" + "="*60)
    print("Example complete. See validation module documentation for details.")
