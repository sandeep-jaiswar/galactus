#!/usr/bin/env python3
"""
Run Galactus backtesting from Jan 2020 to today using available historical data.

- Tries to fetch historical OHLC via GalactusDataProvider (jugaad-data).
- If real historical is unavailable, can use synthetic data when enabled.
- Builds one snapshot per trading day with regime heuristics.
- Runs BacktestHarness and exports results.

Usage:
  python scripts/run_backtest_range.py --symbol NIFTY --start 2020-01-01 --end today --allow-synthetic true --out backtest_results_2020_to_today
"""

import argparse
import logging
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

import pandas as pd

# Ensure src is on the path
ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "research" / "python" / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from data.provider import GalactusDataProvider
from backtesting.harness import (
    BacktestHarness,
    InferenceSnapshot,
    RegimeSnapshot,
    PressureSnapshot,
    ForcedFlowSnapshot,
    LiquiditySnapshot,
    DataQualitySnapshot,
)

logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


def parse_args():
    p = argparse.ArgumentParser(description="Run backtest from Jan 2020 to today")
    p.add_argument("--symbol", default="NIFTY", help="Underlying symbol")
    p.add_argument("--start", default="2020-01-01", help="Start date YYYY-MM-DD")
    p.add_argument("--end", default="today", help="End date YYYY-MM-DD or 'today'")
    p.add_argument(
        "--allow-synthetic",
        default="false",
        help="Allow synthetic data if real history unavailable (true/false)",
    )
    p.add_argument(
        "--out", default="backtest_results_2020_to_today", help="Output directory"
    )
    return p.parse_args()


def get_date_range(args):
    start = datetime.strptime(args.start, "%Y-%m-%d")
    end = (
        datetime.now()
        if args.end.lower() == "today"
        else datetime.strptime(args.end, "%Y-%m-%d")
    )
    # Truncate time component
    start = start.replace(hour=0, minute=0, second=0, microsecond=0)
    end = end.replace(hour=0, minute=0, second=0, microsecond=0)
    return start, end


def fetch_history(
    provider: GalactusDataProvider,
    symbol: str,
    start: datetime,
    end: datetime,
    allow_synth: bool,
) -> pd.DataFrame:
    days = (end - start).days
    # Hint to provider via env var
    os.environ["GALACTUS_ALLOW_SYNTHETIC"] = "true" if allow_synth else "false"
    df = provider.get_historical_data(symbol, days=days, allow_synthetic=allow_synth)

    if df is None or df.empty:
        raise RuntimeError(
            "Historical data unavailable. Provide a real data source or enable --allow-synthetic true."
        )

    # Filter date range if provider returned extended data
    if isinstance(df.index, pd.DatetimeIndex):
        df = df[(df.index >= start) & (df.index <= end)]

    if df.empty:
        raise RuntimeError("Historical data returned no rows in the requested range.")

    return df


def build_snapshots(df: pd.DataFrame, symbol: str) -> list:
    snapshots = []
    event_seq = 0

    # Simulate OI base (starts at ~25M, decays toward expiry)
    base_oi = 25_000_000
    days_to_expiry = 30  # Reset every ~30 days

    for ts, row in df.iterrows():
        # Expect columns like Open/High/Low/Close/Volume; handle case-insensitively
        def get_col(*names, default=None):
            for n in names:
                if n in row:
                    return row[n]
            for n in names:
                if n in df.columns:
                    return row[df.columns[df.columns.str.lower() == n.lower()][0]]
            return default

        open_p = get_col("Open", "open")
        high_p = get_col("High", "high")
        low_p = get_col("Low", "low")
        close_p = get_col("Close", "close")
        vol = get_col("Volume", "volume", default=0)

        if not all([open_p, high_p, low_p, close_p]):
            continue

        daily_vol = (high_p - low_p) / max(close_p, 1e-9)

        # Simulate OI decay pattern (faster near expiry)
        days_to_expiry = max(1, (days_to_expiry - 1) % 30 or 30)
        oi_decay_factor = min(1.0, days_to_expiry / 30.0)
        current_oi = int(base_oi * oi_decay_factor)

        # Calculate OI decay rate (higher = more pressure)
        if event_seq > 0:
            prev_oi = (
                snapshots[-1].forced_flow.estimated_magnitude / 100
                if snapshots[-1].forced_flow.estimated_magnitude
                else current_oi
            )
            oi_decay = max(0, prev_oi - current_oi)
        else:
            oi_decay = 0

        # Regime classification with better thresholds
        is_circuit_breaker = daily_vol > 0.08
        is_high_vol = daily_vol > 0.04
        is_down_day = close_p < open_p
        price_drop = (open_p - close_p) / open_p if open_p > 0 else 0

        if is_circuit_breaker:
            regime = "Forced Selling / Circuit Breaker"
            regime_conf = 0.95
        elif is_high_vol and price_drop > 0.03:
            regime = "Panic Volatility"
            regime_conf = 0.88
        elif price_drop > 0.05 and oi_decay > 500000:
            regime = "Capitulation"
            regime_conf = 0.82
        elif daily_vol < 0.015:
            regime = "Normal Derivatives Dominance"
            regime_conf = 0.85  # Higher confidence in stable periods
        else:
            regime = "Normal Derivatives Dominance"
            regime_conf = 0.75

        snapshot = InferenceSnapshot(
            snapshot_id=f"range_{event_seq:08d}",
            event_timestamp=ts.to_pydatetime().replace(tzinfo=timezone.utc),
            event_sequence=event_seq,
            record_timestamp=ts.to_pydatetime().replace(tzinfo=timezone.utc),
            instrument=symbol,
            regime=RegimeSnapshot(
                classification=regime,
                confidence=regime_conf,
                supporting_signals=["daily_volatility"],
                conflicting_signals=[],
                time_in_regime=0,
            ),
            capital_pressure=PressureSnapshot(
                # Realistic pressure detection based on OI decay and volatility
                detected=(
                    (oi_decay > 300000 and days_to_expiry < 5)  # Expiry pressure
                    or (daily_vol > 0.03 and oi_decay > 200000)  # Volatility + decay
                    or (is_circuit_breaker)  # Crisis pressure
                ),
                intensity=min(1.0, (oi_decay / 500000) + (daily_vol * 10)),
                confidence=min(0.92, 0.75 + (0.15 if oi_decay > 400000 else 0.0)),
                sources=[
                    s
                    for s in [
                        "oi_decay" if oi_decay > 300000 else None,
                        "volatility" if daily_vol > 0.03 else None,
                        "expiry_proximity" if days_to_expiry < 5 else None,
                    ]
                    if s
                ],
                false_positive_rate=0.08 if is_high_vol else 0.02,
            ),
            forced_flow=ForcedFlowSnapshot(
                estimated_magnitude=oi_decay * 100,
                confidence=min(0.90, 0.65 + (oi_decay / 2000000)),
                primary_driver=(
                    "expiry"
                    if days_to_expiry < 5
                    else ("volatility" if is_high_vol else None)
                ),
                estimated_duration_minutes=(
                    120 if is_circuit_breaker else (60 if is_high_vol else None)
                ),
            ),
            liquidity=LiquiditySnapshot(
                bid_ask_spread=10.0 if daily_vol < 0.02 else 20.0,
                depth=500.0 if daily_vol < 0.02 else 200.0,
                quality=(
                    "normal"
                    if daily_vol < 0.02
                    else ("degraded" if daily_vol < 0.05 else "severely_degraded")
                ),
                market_impact_time=5.0 if daily_vol < 0.02 else 15.0,
            ),
            overall_confidence=min(
                0.95,
                regime_conf
                * (1.0 - daily_vol * 8),  # Confidence degrades with volatility
            ),
            stability_indicator=max(0.0, 1.0 - daily_vol * 5),
            silenced=(
                regime_conf < 0.35  # Low regime confidence
                or daily_vol > 0.10  # Extreme volatility
            ),
            kill_switch_status="triggered" if is_circuit_breaker else "active",
            data_quality=DataQualitySnapshot(
                missing_sources=[],
                staleness_warnings=[],
                validation_failures=[],
                quality_score=0.85,
            ),
        )

        snapshots.append(snapshot)
        event_seq += 1

    return snapshots


def main():
    args = parse_args()
    start, end = get_date_range(args)

    logger.info(
        f"Backtesting {args.symbol} from {start.date()} to {end.date()} (allow_synthetic={args.allow_synthetic})"
    )

    allow_synth = args.allow_synthetic.strip().lower() == "true"
    provider = GalactusDataProvider()

    # Fetch history
    df = fetch_history(provider, args.symbol, start, end, allow_synth)
    logger.info(
        f"Historical rows: {len(df)} | source: {df.attrs.get('data_source','unknown')} | symbol: {df.attrs.get('symbol', args.symbol)}"
    )

    # Build snapshots
    snapshots = build_snapshots(df, args.symbol)
    logger.info(f"Snapshots built: {len(snapshots)}")

    # Run backtest
    harness = BacktestHarness(
        galactus_version="0.2.0",
        run_name=f"range_{args.symbol}_{start.date()}_{end.date()}",
    )
    harness.run_backtest(snapshots)

    # Export
    out_dir = ROOT / args.out
    out_dir.mkdir(exist_ok=True)
    harness.export_results(str(out_dir))
    logger.info(f"Results exported to {out_dir}")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logger.exception(f"Range backtest failed: {e}")
        sys.exit(1)
