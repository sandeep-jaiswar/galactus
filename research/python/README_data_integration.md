# Galactus Data Integration

This module provides unified data access for the Galactus trading system using the `jugaad-data` library.

## Overview

The `GalactusDataProvider` class serves as the single source of truth for all market data required by Galactus signals:

- **OI Decay Signals**: Futures data with open interest information
- **Hedge Pressure Signals**: Futures vs spot price relationships
- **Basis Pressure Signals**: Spot price movements and basis calculations
- **Intent Engine**: Market regime classification and risk metrics

## Key Features

- **Unified Interface**: Single class for all data requirements
- **Caching**: 5-minute cache for frequently accessed data
- **Error Handling**: Robust error handling with fallbacks
- **Approximations**: Smart approximations when live derivatives data unavailable
- **Risk-Free Rates**: Configurable risk-free rate for pricing models

## Usage

### Basic Usage

```python
from src.data import GalactusDataProvider

# Initialize provider
provider = GalactusDataProvider()

# Get market status
market_open = provider.is_market_open()

# Get NIFTY index
nifty_value = provider.get_nifty_index()

# Get futures data
futures = provider.get_futures_data("NIFTY")
if futures:
    spot = futures.spot_price
    futures_price = futures.futures_price
    time_to_expiry = futures.time_to_expiry_days

# Get spot prices
reliance_price = provider.get_spot_price("RELIANCE")
```

### Signal Integration

```python
from data_provider import get_futures_for_signal

# Get data for OI decay signal
futures_data = get_futures_for_signal("NIFTY")
if futures_data:
    # Use in signal computation
    basis = futures_data.futures_price - futures_data.spot_price
    # ... signal logic
```

### Bulk Data Retrieval

```python
# Get data for multiple symbols
symbols = ["RELIANCE", "TCS", "INFY"]
market_data = provider.get_bulk_market_data(symbols)
```

## Data Sources

- **NSE Live Data**: Real-time market data via jugaad-data
- **Index Data**: NIFTY 50 and other indices
- **Equity Data**: Spot prices for individual stocks
- **F&O Data**: Derivatives data (approximated when live data unavailable)
- **Market Status**: Trading hours and market state

## Configuration

- **Risk-Free Rate**: Default 6.5% (Indian G-Sec approximation)
- **Cache Duration**: 5 minutes for live data
- **Error Handling**: Graceful degradation when data unavailable

## Limitations

- **Derivatives Data**: Live F&O data may not be available outside market hours
- **Historical Data**: Not yet implemented (requires NSEArchive)
- **Real-time OI**: Open interest data approximated when not available
- **Pricing Models**: Futures prices approximated (production would need proper models)

## Future Enhancements

1. **Historical Data Integration**: Add NSEArchive for backtesting data
2. **Real-time Derivatives**: Enhanced F&O data access during market hours
3. **Multiple Exchanges**: BSE integration for broader coverage
4. **Advanced Pricing**: Proper futures pricing models with volatility
5. **Data Validation**: Quality checks and anomaly detection

## Dependencies

- `jugaad-data`: Primary NSE data library
- `pandas`: Data manipulation
- `numpy`: Numerical computations
- `dataclasses`: Data structures

## Testing

Run the built-in test:

```bash
python data_provider.py
```

Expected output shows successful data retrieval for:
- Market status
- NIFTY index value
- Futures data (approximated)
- Individual stock prices
- Risk-free rate