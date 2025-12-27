"""
Data Integration Example for Galactus Signals

This example demonstrates how to use the GalactusDataProvider
to fetch real market data and feed it into signal computation modules.
"""

import sys
import os
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any

# Add the research path to import modules
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from src.data import GalactusDataProvider, get_futures_for_signal
from src.features.oi_decay_research import compute_oi_decay_pressure, OIDecayConfig
from src.features.hedge_pressure_research import (
    compute_hedge_pressure,
    HedgePressureConfig,
)
from src.features.basis_pressure_research import (
    compute_basis_pressure,
    BasisPressureConfig,
)


def demonstrate_data_integration():
    """
    Demonstrate how to integrate real market data with signal computations.
    """
    print("Galactus Data Integration Demo")
    print("=" * 50)

    # Initialize data provider
    provider = GalactusDataProvider()

    # Check market status
    print("Market Status:")
    is_open = provider.is_market_open()
    print(f"  Market Open: {is_open}")

    if not is_open:
        print("  Note: Market is closed - using cached/latest available data")

    # Get NIFTY futures data
    print("\nFetching NIFTY Futures Data...")
    futures_data = provider.get_futures_data("NIFTY")

    if futures_data:
        print(f"  Symbol: {futures_data.symbol}")
        print(f"  Spot Price: ₹{futures_data.spot_price:,.2f}")
        print(f"  Futures Price: ₹{futures_data.futures_price:,.2f}")
        print(f"  Basis: ₹{futures_data.futures_price - futures_data.spot_price:.2f}")
        print(f"  Time to Expiry: {futures_data.time_to_expiry_days} days")
    else:
        print("  Failed to fetch futures data")
        return

    # Get spot prices for key stocks
    print("\nFetching Spot Prices...")
    symbols = ["RELIANCE", "TCS", "INFY", "HDFCBANK"]
    spot_prices = {}

    for symbol in symbols:
        price = provider.get_spot_price(symbol)
        if price:
            spot_prices[symbol] = price
            print(f"  {symbol}: ₹{price:,.2f}")
        else:
            print(f"  {symbol}: Data unavailable")

    # Simulate OI decay signal computation
    print("\nSimulating OI Decay Signal...")
    print("  (Using synthetic OI data since live derivatives data limited)")

    # Create synthetic OI data for demonstration
    # In production, this would come from provider.get_option_chain()
    current_oi = {1000: 1000, 1100: 800, 1200: 600, 1300: 400}
    previous_oi = {1000: 1200, 1100: 900, 1200: 700, 1300: 500}

    try:
        oi_result = compute_oi_decay_pressure(
            current_oi=current_oi,
            previous_oi=previous_oi,
            time_delta_hours=1.0,
            config=OIDecayConfig(),
        )

        print(f"  OI Decay Pressure: {oi_result.pressure:.3f}")
        print(f"  Total Decay: {oi_result.total_decay}")
        print(f"  Total Build: {oi_result.total_build}")
        print(f"  Confidence: {oi_result.confidence:.3f}")

    except Exception as e:
        print(f"  Error computing OI decay: {e}")

    # Simulate hedge pressure signal
    print("\nSimulating Hedge Pressure Signal...")

    try:
        # For hedge pressure, we need option chain data
        # Using synthetic data since live options not available
        spot_price = futures_data.spot_price
        calls_oi = {
            spot_price * 0.95: 1000,
            spot_price * 1.05: 800,
        }  # Synthetic calls OI
        puts_oi = {spot_price * 0.95: 1200, spot_price * 1.05: 600}  # Synthetic puts OI

        hedge_result = compute_hedge_pressure(
            calls_oi=calls_oi,
            puts_oi=puts_oi,
            spot_price=spot_price,
        )

        print(f"  Hedge Pressure: {hedge_result.pressure:.3f}")
        print(f"  OI Imbalance Ratio: {hedge_result.imbalance_ratio:.3f}")
        print(f"  Confidence: {hedge_result.confidence:.3f}")

    except Exception as e:
        print(f"  Error computing hedge pressure: {e}")

    # Simulate basis pressure signal
    print("\nSimulating Basis Pressure Signal...")

    try:
        # Basis pressure uses futures vs spot data
        basis_result = compute_basis_pressure(
            futures_price=futures_data.futures_price,
            spot_price=futures_data.spot_price,
            time_to_expiry_days=futures_data.time_to_expiry_days,
            risk_free_rate=provider.get_risk_free_rate(),
        )

        print(f"  Basis Pressure: {basis_result.pressure:.3f}")
        print(f"  Fair Basis: {basis_result.fair_basis:.2f}")
        print(f"  Actual Basis: {basis_result.actual_basis:.2f}")
        print(f"  Divergence: {basis_result.divergence_pct:.3f}%")

    except Exception as e:
        print(f"  Error computing basis pressure: {e}")
        print(f"  Velocity: {basis_result.velocity:.3f}")

    except Exception as e:
        print(f"  Error computing basis pressure: {e}")

    # Show market turnover data
    print("\nMarket Turnover Data:")
    try:
        turnover = provider.get_market_turnover()
        if turnover:
            print(f"  Turnover data available: {len(turnover)} fields")
        else:
            print("  Turnover data unavailable")
    except Exception as e:
        print(f"  Error fetching turnover: {e}")

    print("\nData Integration Demo Complete!")
    print("\nNext Steps:")
    print("1. Integrate real derivatives data when available")
    print("2. Add historical data for backtesting")
    print("3. Implement proper error handling and fallbacks")
    print("4. Add data quality validation")


def get_signal_data_snapshot() -> Dict[str, Any]:
    """
    Get a complete snapshot of data needed for all signals.

    Returns:
        Dict containing all market data for signal computation
    """
    provider = GalactusDataProvider()

    snapshot = {
        "timestamp": datetime.now(),
        "market_open": provider.is_market_open(),
        "nifty_futures": provider.get_futures_data("NIFTY"),
        "risk_free_rate": provider.get_risk_free_rate(),
        "spot_prices": {},
        "market_status": provider.get_market_status(),
    }

    # Get key stock prices
    key_stocks = ["RELIANCE", "TCS", "INFY", "HDFCBANK", "ICICIBANK"]
    for stock in key_stocks:
        price = provider.get_spot_price(stock)
        if price:
            snapshot["spot_prices"][stock] = price

    return snapshot


if __name__ == "__main__":
    demonstrate_data_integration()
