"""
Example: Running the Galactus Backtesting Harness

This example demonstrates how to:
1. Create synthetic snapshots
2. Run the backtesting harness
3. Analyze results
"""

import logging
from datetime import datetime, timedelta, timezone

from backtesting import (
    BacktestHarness,
    DataQualitySnapshot,
    ForcedFlowSnapshot,
    InferenceSnapshot,
    LiquiditySnapshot,
    PressureSnapshot,
    RegimeSnapshot,
)


logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def create_example_snapshots():
    """Create synthetic snapshots for demonstration"""
    snapshots = []
    now = datetime.now(timezone.utc)

    for i in range(100):
        ts = now - timedelta(minutes=100 - i)

        # Simulate regime transition at snapshot 50
        if i < 50:
            regime_class = "Normal Derivatives Dominance"
            regime_conf = 0.92
        else:
            regime_class = "Expiry Compression"
            regime_conf = 0.85

        # Simulate pressure detection
        pressure_detected = i > 40
        pressure_conf = 0.8 if pressure_detected else 0.5

        # Create snapshot
        snapshot = InferenceSnapshot(
            snapshot_id=f"snap_{i:04d}",
            event_timestamp=ts,
            event_sequence=i,
            record_timestamp=ts + timedelta(seconds=1),
            instrument="NIFTY",
            regime=RegimeSnapshot(
                classification=regime_class,
                confidence=regime_conf,
                supporting_signals=["oi_decay", "basis_pressure"],
                conflicting_signals=[],
                time_in_regime=i * 5,
            ),
            capital_pressure=PressureSnapshot(
                detected=pressure_detected,
                intensity=0.7 if pressure_detected else 0.2,
                confidence=pressure_conf,
                sources=["expiry_proximity"] if pressure_detected else [],
                false_positive_rate=0.05,
            ),
            forced_flow=ForcedFlowSnapshot(
                estimated_magnitude=1200.0 if pressure_detected else 0.0,
                confidence=0.8 if pressure_detected else 0.3,
                primary_driver="expiry" if pressure_detected else None,
                estimated_duration_minutes=30 if pressure_detected else None,
            ),
            liquidity=LiquiditySnapshot(
                bid_ask_spread=12.5 if pressure_detected else 10.0,
                depth=450 if not pressure_detected else 250,
                quality="normal" if not pressure_detected else "degraded",
                market_impact_time=5.0,
            ),
            overall_confidence=0.85 if pressure_detected else 0.9,
            stability_indicator=0.9 if not pressure_detected else 0.6,
            silenced=False,
            kill_switch_status="active",
            data_quality=DataQualitySnapshot(
                missing_sources=[],
                staleness_warnings=[],
                validation_failures=[],
                quality_score=1.0,
            ),
        )

        snapshots.append(snapshot)

    return snapshots


def main():
    """Run example backtest"""
    logger.info("Creating example snapshots...")
    snapshots = create_example_snapshots()

    logger.info(f"Created {len(snapshots)} snapshots")

    # Create and run harness
    harness = BacktestHarness(
        galactus_version="0.1.0", run_name="example_backtest_2025_12_29"
    )

    logger.info("Running backtest...")
    harness.run_backtest(snapshots)

    # Print summary
    logger.info("\n" + "=" * 60)
    logger.info("BACKTEST RESULTS")
    logger.info("=" * 60)

    summary = harness.get_summary_report()

    logger.info(f"Run: {summary['run_name']}")
    logger.info(f"Version: {summary['galactus_version']}")
    logger.info(f"Period: {summary['period_start']} to {summary['period_end']}")
    logger.info("")

    metrics_data = summary["metrics"]
    logger.info("KEY METRICS:")
    logger.info(f"  Total Snapshots: {metrics_data['total_snapshots']}")
    logger.info(f"  Total Failures: {metrics_data['total_failures']}")
    logger.info(
        f"  High-Confidence Failure Rate: {metrics_data['high_confidence_failure_rate']:.2%}"
    )
    logger.info(f"  False Pressure Rate: {metrics_data['false_pressure_rate']:.2%}")
    logger.info(
        f"  Regime Lag (avg): {metrics_data['average_regime_lag_days']:.2f} days"
    )
    logger.info(
        f"  Silence Correctness Rate: {metrics_data['silence_correctness_rate']:.2%}"
    )
    logger.info(
        f"  Kill-Switch Anticipation: {metrics_data['kill_switch_anticipation']:.2%}"
    )
    logger.info(
        f"  Confidence Calibration Error: {metrics_data['confidence_calibration_error']:.3f}"
    )
    logger.info("")

    failure_summary = summary["failure_summary"]
    logger.info("FAILURE SUMMARY:")
    logger.info(f"  Total: {failure_summary['total_failures']}")
    logger.info(
        f"  High-Confidence Failures: {failure_summary['high_confidence_failures']}"
    )
    logger.info(f"  By Category: {failure_summary['failures_by_category']}")
    logger.info("")

    snap_stats = summary["snapshot_statistics"]
    logger.info("SNAPSHOT STATISTICS:")
    logger.info(f"  Total Snapshots: {snap_stats['total_snapshots']}")
    logger.info(f"  Silenced: {snap_stats['silenced_count']}")
    logger.info(f"  Kill-Switch Triggered: {snap_stats['kill_switch_triggered_count']}")
    logger.info(f"  Average Confidence: {snap_stats['average_confidence']:.3f}")
    logger.info("")

    # Show regime transitions
    logger.info("REGIME TRANSITIONS:")
    transitions = harness.snapshot_recorder.get_regime_transitions()
    for ts, from_r, to_r in transitions:
        logger.info(f"  {ts}: {from_r} -> {to_r}")
    logger.info("")

    # Show failures
    logger.info("FAILURE LEDGER (first 10):")
    failures = harness.failure_ledger.get_all_failures()[:10]
    for failure in failures:
        logger.info(
            f"  [{failure.timestamp}] {failure.instrument} | "
            f"{failure.category.value} | conf={failure.confidence_at_failure:.2f}"
        )

    logger.info("")
    logger.info("=" * 60)

    # Export results for CI/CD
    logger.info("\nExporting results to backtest_results/...")
    harness.export_results("./backtest_results")
    logger.info("Export complete!")


if __name__ == "__main__":
    main()
