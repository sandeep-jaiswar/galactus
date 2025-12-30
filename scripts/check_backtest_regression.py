#!/usr/bin/env python3
"""
Regression Check for Backtest Metrics

Compares current backtest metrics against baseline to detect regressions.
Fails CI if regression thresholds are exceeded.
"""

import json
import sys
from pathlib import Path
from typing import Dict, Optional


def load_metrics(path: Optional[str]) -> Optional[Dict]:
    """Load metrics from JSON file"""
    if not path or not Path(path).exists():
        return None

    with open(path, "r") as f:
        return json.load(f)


def check_regressions(baseline: Dict, current: Dict) -> bool:
    """
    Check for regressions.

    Returns True if no regressions, False if regressions detected.
    """
    issues = []

    baseline_metrics = baseline.get("metrics", {})
    current_metrics = current.get("metrics", {})

    # Check high-confidence failure rate (should not increase > 0.5%)
    baseline_hc_fail = baseline_metrics.get("high_confidence_failure_rate", 0)
    current_hc_fail = current_metrics.get("high_confidence_failure_rate", 0)
    delta_hc = current_hc_fail - baseline_hc_fail

    if delta_hc > 0.005:  # > 0.5% increase
        issues.append(
            f"❌ HIGH-CONFIDENCE FAILURE RATE REGRESSION: "
            f"{baseline_hc_fail:.2%} → {current_hc_fail:.2%} "
            f"(+{delta_hc:.2%})"
        )

    # Check false pressure rate (should not increase > 2%)
    baseline_fp = baseline_metrics.get("false_pressure_rate", 0)
    current_fp = current_metrics.get("false_pressure_rate", 0)
    delta_fp = current_fp - baseline_fp

    if delta_fp > 0.02:  # > 2% increase
        issues.append(
            f"❌ FALSE PRESSURE RATE REGRESSION: "
            f"{baseline_fp:.2%} → {current_fp:.2%} "
            f"(+{delta_fp:.2%})"
        )

    # Check regime lag (should not increase > 12 hours = 0.5 days)
    baseline_lag = baseline_metrics.get("average_regime_lag_days", 0)
    current_lag = current_metrics.get("average_regime_lag_days", 0)
    delta_lag = current_lag - baseline_lag

    if delta_lag > 0.5:  # > 12 hours increase
        issues.append(
            f"❌ REGIME LAG REGRESSION: "
            f"{baseline_lag:.2f} days → {current_lag:.2f} days "
            f"(+{delta_lag:.2f} days)"
        )

    # Check kill-switch anticipation (should not decrease > 10%)
    baseline_ks = baseline_metrics.get("kill_switch_anticipation", 0)
    current_ks = current_metrics.get("kill_switch_anticipation", 0)
    delta_ks = baseline_ks - current_ks

    if delta_ks > 0.1:  # > 10% decrease
        issues.append(
            f"❌ KILL-SWITCH ANTICIPATION REGRESSION: "
            f"{baseline_ks:.2%} → {current_ks:.2%} "
            f"(-{delta_ks:.2%})"
        )

    # Check silence correctness (should not decrease > 5%)
    baseline_sc = baseline_metrics.get("silence_correctness_rate", 0)
    current_sc = current_metrics.get("silence_correctness_rate", 0)
    delta_sc = baseline_sc - current_sc

    if delta_sc > 0.05:  # > 5% decrease
        issues.append(
            f"❌ SILENCE CORRECTNESS REGRESSION: "
            f"{baseline_sc:.2%} → {current_sc:.2%} "
            f"(-{delta_sc:.2%})"
        )

    # Print results
    print("\n" + "=" * 70)
    print("BACKTEST REGRESSION ANALYSIS")
    print("=" * 70 + "\n")

    if issues:
        print("REGRESSIONS DETECTED:\n")
        for issue in issues:
            print(f"  {issue}")
        print()
        return False
    else:
        print("✅ No regressions detected!\n")

        # Print improvements if any
        improvements = []

        if delta_hc < 0:
            improvements.append(
                f"✓ High-confidence failure rate improved: {delta_hc:.2%}"
            )
        if delta_fp < 0:
            improvements.append(f"✓ False pressure rate improved: {delta_fp:.2%}")
        if delta_lag < 0:
            improvements.append(f"✓ Regime lag improved: {delta_lag:.2f} days")
        if delta_ks > 0:
            improvements.append(f"✓ Kill-switch anticipation improved: {delta_ks:.2%}")
        if delta_sc > 0:
            improvements.append(f"✓ Silence correctness improved: {delta_sc:.2%}")

        if improvements:
            print("IMPROVEMENTS:\n")
            for improvement in improvements:
                print(f"  {improvement}")
            print()

        return True


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Check backtest metrics for regressions"
    )
    parser.add_argument(
        "--baseline", type=str, help="Path to baseline metrics JSON file"
    )
    parser.add_argument(
        "--current", type=str, required=True, help="Path to current metrics JSON file"
    )
    parser.add_argument(
        "--fail-on-regression",
        action="store_true",
        help="Exit with code 1 if regressions detected",
    )

    args = parser.parse_args()

    # Load metrics
    baseline = load_metrics(args.baseline)
    current = load_metrics(args.current)

    if not current:
        print("ERROR: Current metrics file not found")
        sys.exit(1)

    if not baseline:
        print("WARNING: No baseline metrics found. Skipping regression check.")
        print("Create baseline with: cp current.json baseline.json")
        sys.exit(0)

    # Check regressions
    passed = check_regressions(baseline, current)

    if not passed and args.fail_on_regression:
        sys.exit(1)

    sys.exit(0)


if __name__ == "__main__":
    main()
