#!/usr/bin/env python3
"""
Run a live canary using GalactusDataProvider and LeadingIndicatorsAnalyzer.

This script fetches real market data via jugaad-data, computes leading
indicators, constructs an `InferenceSnapshot`, and runs the backtesting
Harness on the single snapshot to validate the live pipeline.

Usage:
  python scripts/run_live_canary.py --symbol NIFTY
"""

import argparse
import logging
import sys
from datetime import datetime, timezone

from pathlib import Path

# Ensure src is on the path
ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "research" / "python" / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from data.provider import GalactusDataProvider
from features.leading_indicators import LeadingIndicatorsAnalyzer
from backtesting.harness import (
    BacktestHarness,
    InferenceSnapshot,
    RegimeSnapshot,
    PressureSnapshot,
    ForcedFlowSnapshot,
    LiquiditySnapshot,
    DataQualitySnapshot,
)


logger = logging.getLogger(__name__)
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)


def build_snapshot_from_live(symbol: str = "NIFTY") -> InferenceSnapshot:
    provider = GalactusDataProvider()

    # Fetch futures data (with fallbacks inside provider)
    fut = provider.get_futures_data(symbol)
    if not fut:
        raise RuntimeError(f"Futures data unavailable for {symbol}")

    # Fetch option chain to estimate calls/puts OI distributions (optional)
    options = provider.get_option_chain(symbol)
    calls_oi = {}
    puts_oi = {}
    if options:
        for opt in options[:50]:  # sample top 50 for speed
            if opt.call_oi:
                calls_oi[int(opt.strike_price)] = int(opt.call_oi)
            if opt.put_oi:
                puts_oi[int(opt.strike_price)] = int(opt.put_oi)

    # Analyze leading indicators
    analyzer = LeadingIndicatorsAnalyzer()
    analysis = analyzer.analyze_all_indicators(
        futures_price=fut.futures_price,
        spot_price=fut.spot_price,
        time_to_expiry_days=fut.time_to_expiry_days,
        calls_oi=calls_oi or {int(fut.spot_price): 1},
        puts_oi=puts_oi or {int(fut.spot_price): 1},
        risk_free_rate=provider.get_risk_free_rate(),
    )

    # Basic regime assignment from leading indicators recommendation
    recommendation = analysis.recommendation.lower()
    if "expiry compression" in recommendation or "compression" in recommendation:
        regime = "Normal Derivatives Dominance"
        regime_conf = 0.75
    elif (
        "elevated volatility" in recommendation or "reduce confidence" in recommendation
    ):
        regime = "Panic Volatility"
        regime_conf = 0.80
    else:
        regime = "Normal Derivatives Dominance"
        regime_conf = 0.70

    # Pressure detection heuristics from OI concentration and basis divergence
    # Treat signals with strength > 0.6 as meaningful; >0.8 as strong
    pressure_detected = any(s.strength > 0.6 for s in analysis.individual_signals)
    pressure_intensity = (
        0.6
        if any(s.strength > 0.8 for s in analysis.individual_signals)
        else (0.3 if pressure_detected else 0.0)
    )

    now = datetime.now(timezone.utc)

    snapshot = InferenceSnapshot(
        snapshot_id=f"live_{int(now.timestamp())}",
        event_timestamp=now,
        event_sequence=0,
        record_timestamp=now,
        instrument=symbol,
        regime=RegimeSnapshot(
            classification=regime,
            confidence=regime_conf,
            supporting_signals=[
                f"{s.signal_type}:{s.implied_regime}"
                for s in analysis.individual_signals
            ],
            conflicting_signals=[],
            time_in_regime=0,
        ),
        capital_pressure=PressureSnapshot(
            detected=pressure_detected,
            intensity=pressure_intensity,
            confidence=min(0.9, regime_conf + 0.1),
            sources=[f"{s.signal_type}" for s in analysis.individual_signals],
            false_positive_rate=0.05,
        ),
        forced_flow=ForcedFlowSnapshot(
            estimated_magnitude=0.0,
            confidence=0.5,
            primary_driver=None,
            estimated_duration_minutes=None,
        ),
        liquidity=LiquiditySnapshot(
            bid_ask_spread=10.0,
            depth=500.0,
            quality="normal",
            market_impact_time=5.0,
        ),
        overall_confidence=regime_conf,
        stability_indicator=0.8,
        silenced=False,
        kill_switch_status="active",
        data_quality=DataQualitySnapshot(
            missing_sources=[],
            staleness_warnings=[],
            validation_failures=[],
            quality_score=(
                0.9
                if fut.data_quality and fut.data_quality.overall_score >= 0.8
                else 0.7
            ),
        ),
    )

    return snapshot


def main():
    parser = argparse.ArgumentParser(description="Run a live canary test")
    parser.add_argument(
        "--symbol", default="NIFTY", help="Underlying symbol, e.g., NIFTY, RELIANCE"
    )
    parser.add_argument("--out", default="live_canary_results", help="Output directory")
    args = parser.parse_args()

    logger.info(f"Starting live canary for {args.symbol}...")
    snapshot = build_snapshot_from_live(args.symbol)

    harness = BacktestHarness(
        galactus_version="0.2.0", run_name=f"live_canary_{args.symbol}"
    )
    harness.run_backtest([snapshot])
    summary = harness.get_summary_report()

    out_dir = ROOT / args.out
    out_dir.mkdir(exist_ok=True)
    harness.export_results(str(out_dir))

    logger.info("Live canary complete.")
    logger.info(f"Snapshots: {summary.get('metrics', {}).get('total_snapshots', 0)}")
    logger.info(
        f"Regime: {snapshot.regime.classification} @ {snapshot.regime.confidence:.2f}"
    )
    logger.info(f"Recommendation: {args.symbol} | {summary.get('run_name')}")
    logger.info(f"Results exported to {out_dir}")


if __name__ == "__main__":
    main()
