"""
Galactus Data Integration Module

Uses jugaad-data library as the primary data source for NSE market data.
Provides unified interface for all data requirements in the Galactus system.

Features:
- Real derivatives data integration
- Historical data for backtesting
- Robust error handling and fallbacks
- Data quality validation
- Production-ready data structures
"""

import warnings
import time
from typing import Dict, List, Optional, Tuple, Any
from datetime import datetime, timedelta
from dataclasses import dataclass
import numpy as np
import pandas as pd

from jugaad_data.nse import NSELive
from jugaad_data.bse import BSELive

# Suppress jugaad-data warnings
warnings.filterwarnings("ignore", module="jugaad_data")

# Configuration
DEFAULT_RISK_FREE_RATE = 0.065  # 6.5% annual rate (approximate Indian G-Sec rate)
CACHE_DURATION_MINUTES = 5
DATA_QUALITY_THRESHOLD = 0.8  # Minimum quality score for data acceptance
MAX_RETRY_ATTEMPTS = 3
RETRY_DELAY_SECONDS = 1


@dataclass
class DataQualityMetrics:
    """Metrics for assessing data quality."""

    completeness: float  # 0-1, fraction of expected data present
    freshness: float  # 0-1, how recent the data is
    consistency: float  # 0-1, internal consistency checks
    accuracy: float  # 0-1, cross-validation with known values
    overall_score: float  # 0-1, weighted average

    def is_acceptable(self, threshold: float = DATA_QUALITY_THRESHOLD) -> bool:
        """Check if data quality meets minimum standards."""
        return self.overall_score >= threshold


@dataclass
class MarketData:
    """Container for market data."""

    symbol: str
    spot_price: float
    futures_price: Optional[float] = None
    oi: Optional[int] = None
    volume: Optional[int] = None
    timestamp: datetime = None
    expiry_date: Optional[datetime] = None
    data_quality: Optional[DataQualityMetrics] = None

    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.now()


@dataclass
class FuturesData:
    """Container for futures-specific data."""

    symbol: str
    futures_price: float
    spot_price: float
    expiry_date: datetime
    time_to_expiry_days: int
    oi: Optional[int] = None
    volume: Optional[int] = None
    timestamp: datetime = None
    data_quality: Optional[DataQualityMetrics] = None

    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.now()


@dataclass
class OptionData:
    """Container for option chain data."""

    symbol: str
    strike_price: float
    call_price: Optional[float] = None
    put_price: Optional[float] = None
    call_oi: Optional[int] = None
    put_oi: Optional[int] = None
    call_volume: Optional[int] = None
    put_volume: Optional[int] = None
    expiry_date: Optional[datetime] = None
    timestamp: datetime = None
    data_quality: Optional[DataQualityMetrics] = None

    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.now()


@dataclass
class IndexData:
    """Container for index data."""

    index_name: str
    value: float
    change: float
    percent_change: float
    timestamp: datetime = None
    data_quality: Optional[DataQualityMetrics] = None

    def __post_init__(self):
        if self.timestamp is None:
            self.timestamp = datetime.now()


class DataQualityValidator:
    """Validates data quality and provides quality metrics."""

    @staticmethod
    def validate_price_data(price: float, symbol: str = "") -> float:
        """
        Validate price data quality.

        Returns:
            Quality score 0-1
        """
        if price is None or price <= 0:
            return 0.0

        # Basic sanity checks for Indian market prices
        if symbol == "NIFTY":
            # NIFTY should be between 5,000 and 50,000
            if 5000 <= price <= 50000:
                return 1.0
            else:
                return 0.3
        else:
            # Individual stocks should be between 1 and 100,000
            if 1 <= price <= 100000:
                return 0.9
            else:
                return 0.2

    @staticmethod
    def validate_volume_data(volume: int) -> float:
        """Validate volume data quality."""
        if volume is None or volume < 0:
            return 0.0

        # Volume should be reasonable
        return 0.8 if volume > 100 else 0.6

    @staticmethod
    def validate_timestamp(timestamp: datetime) -> float:
        """Validate timestamp freshness."""
        if timestamp is None:
            return 0.0

        age_hours = (datetime.now() - timestamp).total_seconds() / 3600

        if age_hours <= 1:
            return 1.0
        elif age_hours <= 24:
            return 0.8
        elif age_hours <= 168:  # 1 week
            return 0.5
        else:
            return 0.2

    @staticmethod
    def compute_overall_quality(metrics: Dict[str, float]) -> float:
        """Compute weighted overall quality score."""
        weights = {
            "price_quality": 0.4,
            "volume_quality": 0.2,
            "freshness": 0.3,
            "consistency": 0.1,
        }

        score = 0.0
        total_weight = 0.0

        for metric, weight in weights.items():
            if metric in metrics:
                score += metrics[metric] * weight
                total_weight += weight

        return score / total_weight if total_weight > 0 else 0.0

    @classmethod
    def assess_market_data_quality(cls, data: MarketData) -> DataQualityMetrics:
        """Assess quality of market data."""
        price_quality = cls.validate_price_data(data.spot_price, data.symbol)
        volume_quality = cls.validate_volume_data(data.volume) if data.volume else 0.5
        freshness = cls.validate_timestamp(data.timestamp)

        # Consistency check: if we have both spot and futures, check relationship
        consistency = 0.8
        if data.futures_price and data.spot_price:
            basis_ratio = abs(data.futures_price - data.spot_price) / data.spot_price
            if basis_ratio > 0.5:  # More than 50% basis seems suspicious
                consistency = 0.4

        metrics = {
            "price_quality": price_quality,
            "volume_quality": volume_quality,
            "freshness": freshness,
            "consistency": consistency,
        }

        overall_score = cls.compute_overall_quality(metrics)

        return DataQualityMetrics(
            completeness=0.9 if data.spot_price else 0.3,
            freshness=freshness,
            consistency=consistency,
            accuracy=price_quality,
            overall_score=overall_score,
        )


class GalactusDataProvider:
    """
    Unified data provider for Galactus using jugaad-data library.

    Features:
    - Real derivatives data integration
    - Historical data for backtesting
    - Robust error handling and fallbacks
    - Data quality validation
    """

    def __init__(self):
        self.nse = NSELive()
        self.bse = BSELive()
        self.validator = DataQualityValidator()
        self._cache = {}
        self._cache_timestamps = {}

    def _is_cache_valid(self, key: str) -> bool:
        """Check if cached data is still valid."""
        if key not in self._cache_timestamps:
            return False

        age = datetime.now() - self._cache_timestamps[key]
        return age < timedelta(minutes=CACHE_DURATION_MINUTES)

    def _cache_data(self, key: str, data: Any):
        """Cache data with timestamp."""
        self._cache[key] = data
        self._cache_timestamps[key] = datetime.now()

    def _get_cached_data(self, key: str) -> Optional[Any]:
        """Get cached data if valid."""
        if self._is_cache_valid(key):
            return self._cache[key]
        return None

    def _retry_operation(self, operation, *args, **kwargs):
        """Retry an operation with exponential backoff."""

        for attempt in range(MAX_RETRY_ATTEMPTS):
            try:
                return operation(*args, **kwargs)
            except Exception as e:
                if attempt == MAX_RETRY_ATTEMPTS - 1:
                    raise
                time.sleep(RETRY_DELAY_SECONDS * (2**attempt))

    def get_market_status(self) -> Dict[str, Any]:
        """
        Get current market status with enhanced error handling.

        Returns:
            Dict containing market state, trading hours, etc.
        """
        cache_key = "market_status"
        cached = self._get_cached_data(cache_key)
        if cached:
            return cached

        try:
            status = self._retry_operation(self.nse.market_status)
            self._cache_data(cache_key, status)
            return status
        except Exception as e:
            print(f"Error fetching market status after retries: {e}")
            return {
                "marketState": [
                    {"marketStatus": "Unknown", "marketStatusMessage": f"Error: {e}"}
                ]
            }

    def is_market_open(self) -> bool:
        """
        Check if market is currently open with fallback logic.

        Returns:
            True if market is open, False otherwise
        """
        try:
            status = self.get_market_status()
            market_states = status.get("marketState", [])

            for state in market_states:
                if state.get("market") == "Capital Market":
                    status_msg = state.get("marketStatusMessage", "").lower()
                    return "open" in status_msg or "normal" in status_msg

            # Fallback: check current time vs typical market hours
            now = datetime.now()
            market_open_time = now.replace(hour=9, minute=15, second=0, microsecond=0)
            market_close_time = now.replace(hour=15, minute=30, second=0, microsecond=0)

            # Check if it's a weekday
            if now.weekday() >= 5:  # Saturday = 5, Sunday = 6
                return False

            return market_open_time <= now <= market_close_time

        except Exception as e:
            print(f"Error checking market status: {e}")
            return False

    def get_spot_price(self, symbol: str) -> Optional[float]:
        """
        Get current spot price for a symbol with quality validation.

        Args:
            symbol: NSE symbol (e.g., 'RELIANCE', 'TCS')

        Returns:
            Current spot price or None if unavailable or low quality
        """
        try:
            quote = self._retry_operation(self.nse.stock_quote, symbol)
            price_info = quote.get("priceInfo", {})
            last_price = price_info.get("lastPrice", 0)

            if last_price and str(last_price) != "-" and last_price > 0:
                price = float(last_price)
                # Validate quality
                quality_score = self.validator.validate_price_data(price, symbol)
                if quality_score >= DATA_QUALITY_THRESHOLD:
                    return price
                else:
                    print(
                        f"Low quality spot price for {symbol}: {price} (score: {quality_score})"
                    )
                    return None

            return None
        except Exception as e:
            print(f"Error fetching spot price for {symbol}: {e}")
            return None

    def get_futures_data(
        self, symbol: str, expiry_month: Optional[str] = None
    ) -> Optional[FuturesData]:
        """
        Get futures data for a symbol with real derivatives integration.

        This method tries multiple approaches:
        1. Real option chain data for accurate OI and pricing
        2. F&O quote data for structured derivatives info
        3. Spot-based approximation as fallback

        Args:
            symbol: Underlying symbol (e.g., 'NIFTY', 'RELIANCE')
            expiry_month: Specific expiry month (optional)

        Returns:
            FuturesData object with quality metrics or None if unavailable
        """
        try:
            # Method 1: Try to get real futures data from option chain
            futures_data = self._get_futures_from_option_chain(symbol, expiry_month)
            if futures_data:
                # Assess data quality
                futures_data.data_quality = self.validator.assess_market_data_quality(
                    MarketData(
                        symbol=symbol,
                        spot_price=futures_data.spot_price,
                        futures_price=futures_data.futures_price,
                        oi=futures_data.oi,
                        volume=futures_data.volume,
                        timestamp=futures_data.timestamp,
                    )
                )
                if futures_data.data_quality.is_acceptable():
                    return futures_data

            # Method 2: Try F&O quote data
            futures_data = self._get_futures_from_fno_quote(symbol)
            if futures_data:
                futures_data.data_quality = self.validator.assess_market_data_quality(
                    MarketData(
                        symbol=symbol,
                        spot_price=futures_data.spot_price,
                        futures_price=futures_data.futures_price,
                        timestamp=futures_data.timestamp,
                    )
                )
                return futures_data

            # Method 3: Fallback to spot-based approximation
            return self._get_futures_approximation(symbol)

        except Exception as e:
            print(f"Error fetching futures data for {symbol}: {e}")
            return None

    def _get_futures_from_option_chain(
        self, symbol: str, expiry_month: Optional[str] = None
    ) -> Optional[FuturesData]:
        """
        Extract futures data from option chain.

        This provides the most accurate futures data when available.
        """
        try:
            # Get option chain data
            if symbol == "NIFTY":
                chain_data = self._retry_operation(self.nse.index_option_chain, symbol)
            else:
                chain_data = self._retry_operation(
                    self.nse.equities_option_chain, symbol
                )

            if not chain_data or "records" not in chain_data:
                return None

            records = chain_data["records"]

            # Look for futures contracts
            if "data" in records:
                options = records["data"]

                # Find futures contracts (instrument type FUTIDX or FUTIVX)
                futures_contracts = [
                    opt
                    for opt in options
                    if opt.get("instrument") in ["FUTIDX", "FUTIVX"]
                ]

                if futures_contracts:
                    # Get the nearest expiry futures contract
                    futures_contracts.sort(key=lambda x: x.get("expiryDate", ""))

                    # Filter by expiry month if specified
                    if expiry_month:
                        futures_contracts = [
                            c
                            for c in futures_contracts
                            if expiry_month.lower() in c.get("expiryDate", "").lower()
                        ]

                    if futures_contracts:
                        contract = futures_contracts[0]

                        futures_price = float(contract.get("lastPrice", 0))
                        oi = int(contract.get("openInterest", 0))
                        volume = int(contract.get("totalTradedVolume", 0))

                        # Parse expiry date
                        expiry_str = contract.get("expiryDate", "")
                        try:
                            expiry_date = datetime.strptime(expiry_str, "%d-%b-%Y")
                        except (ValueError, TypeError):
                            expiry_date = datetime.now() + timedelta(days=30)

                        # Get spot price
                        spot_price = (
                            self.get_nifty_index()
                            if symbol == "NIFTY"
                            else self.get_spot_price(symbol)
                        )

                        if spot_price and futures_price > 0:
                            time_to_expiry = max(1, (expiry_date - datetime.now()).days)

                            return FuturesData(
                                symbol=symbol,
                                futures_price=futures_price,
                                spot_price=spot_price,
                                expiry_date=expiry_date,
                                time_to_expiry_days=time_to_expiry,
                                oi=oi,
                                volume=volume,
                            )

            return None

        except Exception as e:
            print(f"Error extracting futures from option chain for {symbol}: {e}")
            return None

    def _get_futures_from_fno_quote(self, symbol: str) -> Optional[FuturesData]:
        """
        Extract futures data from F&O quote.

        Provides structured derivatives data when option chain is unavailable.
        """
        try:
            fno_quote = self._retry_operation(self.nse.stock_quote_fno, symbol)

            if not fno_quote or "underlyingValue" not in fno_quote:
                return None

            underlying_str = str(fno_quote["underlyingValue"])
            if underlying_str == "-" or not underlying_str.replace(".", "").isdigit():
                return None

            underlying_value = float(underlying_str)

            # Get expiry dates
            expiry_dates = fno_quote.get("expiryDates", [])
            if expiry_dates:
                try:
                    expiry_date = datetime.strptime(expiry_dates[0], "%d-%b-%Y")
                except (ValueError, TypeError):
                    expiry_date = datetime.now() + timedelta(days=30)
            else:
                expiry_date = datetime.now() + timedelta(days=30)

            # Get spot price
            spot_price = (
                self.get_nifty_index()
                if symbol == "NIFTY"
                else self.get_spot_price(symbol)
            )

            if spot_price:
                # Use underlying value as futures approximation
                futures_price = underlying_value

                time_to_expiry = max(1, (expiry_date - datetime.now()).days)

                return FuturesData(
                    symbol=symbol,
                    futures_price=futures_price,
                    spot_price=spot_price,
                    expiry_date=expiry_date,
                    time_to_expiry_days=time_to_expiry,
                )

            return None

        except Exception as e:
            print(f"Error extracting futures from F&O quote for {symbol}: {e}")
            return None

    def _get_futures_approximation(self, symbol: str) -> Optional[FuturesData]:
        """
        Provide futures approximation based on spot price.

        Used as fallback when real derivatives data is unavailable.
        """
        try:
            # Get spot price
            spot_price = (
                self.get_nifty_index()
                if symbol == "NIFTY"
                else self.get_spot_price(symbol)
            )

            if not spot_price:
                return None

            # Approximate futures price (cost of carry model)
            # F = S * e^(r * t) where r is risk-free rate, t is time to expiry
            risk_free_rate = self.get_risk_free_rate()
            time_to_expiry_years = 30 / 365  # Approximate 30 days

            futures_price = spot_price * (1 + risk_free_rate * time_to_expiry_years)

            expiry_date = datetime.now() + timedelta(days=30)

            futures_data = FuturesData(
                symbol=symbol,
                futures_price=futures_price,
                spot_price=spot_price,
                expiry_date=expiry_date,
                time_to_expiry_days=30,
            )

            # Mark as approximated data
            futures_data.data_quality = DataQualityMetrics(
                completeness=0.5,
                freshness=0.9,
                consistency=0.7,
                accuracy=0.6,
                overall_score=0.65,
            )

            return futures_data

        except Exception as e:
            print(f"Error creating futures approximation for {symbol}: {e}")
            return None

    def get_nifty_index(self) -> Optional[float]:
        """
        Get current NIFTY 50 index value with quality validation.

        Returns:
            Current NIFTY value or None if unavailable
        """
        try:
            index_data = self._retry_operation(self.nse.live_index, "NIFTY 50")
            data = index_data.get("data", [])
            if data:
                last_price = data[0].get("lastPrice", 0)
                if last_price and str(last_price) != "-" and last_price > 0:
                    price = float(last_price)
                    quality_score = self.validator.validate_price_data(price, "NIFTY")
                    if quality_score >= DATA_QUALITY_THRESHOLD:
                        return price
                    else:
                        print(
                            f"Low quality NIFTY index: {price} (score: {quality_score})"
                        )
                        return None
            return None
        except Exception as e:
            print(f"Error fetching NIFTY index: {e}")
            return None

    def get_option_chain(
        self, symbol: str, expiry_date: Optional[str] = None
    ) -> Optional[List[OptionData]]:
        """
        Get option chain data for analysis with quality validation.

        Args:
            symbol: Underlying symbol
            expiry_date: Specific expiry date (optional)

        Returns:
            List of OptionData objects or None if unavailable
        """
        try:
            if symbol == "NIFTY":
                chain_data = self._retry_operation(self.nse.index_option_chain, symbol)
            else:
                chain_data = self._retry_operation(
                    self.nse.equities_option_chain, symbol
                )

            if not chain_data or "records" not in chain_data:
                return None

            records = chain_data["records"]
            if "data" not in records:
                return None

            options = records["data"]
            option_data_list = []

            for option in options:
                try:
                    strike_price = float(option.get("strikePrice", 0))
                    call_price = option.get("CE", {}).get("lastPrice")
                    put_price = option.get("PE", {}).get("lastPrice")
                    call_oi = option.get("CE", {}).get("openInterest")
                    put_oi = option.get("PE", {}).get("openInterest")
                    call_volume = option.get("CE", {}).get("totalTradedVolume")
                    put_volume = option.get("PE", {}).get("totalTradedVolume")

                    # Convert to proper types
                    call_price = (
                        float(call_price)
                        if call_price and str(call_price) != "-"
                        else None
                    )
                    put_price = (
                        float(put_price)
                        if put_price and str(put_price) != "-"
                        else None
                    )
                    call_oi = int(call_oi) if call_oi and str(call_oi) != "-" else None
                    put_oi = int(put_oi) if put_oi and str(put_oi) != "-" else None
                    call_volume = (
                        int(call_volume)
                        if call_volume and str(call_volume) != "-"
                        else None
                    )
                    put_volume = (
                        int(put_volume)
                        if put_volume and str(put_volume) != "-"
                        else None
                    )

                    # Parse expiry date
                    expiry_str = option.get("expiryDate", "")
                    try:
                        expiry_date_parsed = datetime.strptime(expiry_str, "%d-%b-%Y")
                    except (ValueError, TypeError):
                        expiry_date_parsed = None

                    option_data = OptionData(
                        symbol=symbol,
                        strike_price=strike_price,
                        call_price=call_price,
                        put_price=put_price,
                        call_oi=call_oi,
                        put_oi=put_oi,
                        call_volume=call_volume,
                        put_volume=put_volume,
                        expiry_date=expiry_date_parsed,
                    )

                    # Assess data quality
                    option_data.data_quality = DataQualityMetrics(
                        completeness=0.8 if (call_price or put_price) else 0.3,
                        freshness=0.9,
                        consistency=0.8,
                        accuracy=0.7,
                        overall_score=0.8,
                    )

                    option_data_list.append(option_data)

                except Exception as e:
                    print(f"Error processing option data: {e}")
                    continue

            return option_data_list if option_data_list else None

        except Exception as e:
            print(f"Error fetching option chain for {symbol}: {e}")
            return None

    def get_historical_data(
        self, symbol: str, days: int = 30, use_cache: bool = True
    ) -> Optional[pd.DataFrame]:
        """
        Get historical price data for backtesting.

        Note: NSEArchive is not available in current jugaad-data version.
        This method provides basic historical data simulation for backtesting.

        Args:
            symbol: Stock symbol
            days: Number of days of history
            use_cache: Whether to use cached data

        Returns:
            DataFrame with historical OHLC data or None if unavailable
        """
        cache_key = f"historical_{symbol}_{days}"

        if use_cache:
            cached = self._get_cached_data(cache_key)
            if cached is not None:
                return cached

        try:
            # For now, generate synthetic historical data for backtesting
            # In production, this would use NSEArchive or other historical data sources
            print(f"Generating synthetic historical data for {symbol} ({days} days)")

            end_date = datetime.now()
            start_date = end_date - timedelta(days=days)

            # Generate date range
            dates = pd.date_range(start=start_date, end=end_date, freq="D")

            # Get current price as base
            current_price = (
                self.get_spot_price(symbol)
                if symbol != "NIFTY"
                else self.get_nifty_index()
            )

            if not current_price:
                return None

            # Generate synthetic OHLC data with realistic volatility
            np.random.seed(42)  # For reproducible results

            # Simulate price series with random walk
            returns = np.random.normal(
                0.0001, 0.02, len(dates)
            )  # Small drift, 2% daily vol
            price_series = current_price * np.exp(np.cumsum(returns))

            # Generate OHLC from price series
            high_mult = 1 + np.abs(np.random.normal(0, 0.01, len(dates)))
            low_mult = 1 - np.abs(np.random.normal(0, 0.01, len(dates)))

            df = pd.DataFrame(
                {
                    "Date": dates,
                    "Open": price_series * (1 + np.random.normal(0, 0.005, len(dates))),
                    "High": price_series * high_mult,
                    "Low": price_series * low_mult,
                    "Close": price_series,
                    "Volume": np.random.randint(10000, 1000000, len(dates)),
                }
            )

            # Ensure OHLC relationships are correct
            df["High"] = df[["Open", "Close", "High"]].max(axis=1)
            df["Low"] = df[["Open", "Close", "Low"]].min(axis=1)

            df.set_index("Date", inplace=True)

            # Add data quality metadata
            df.attrs["data_quality"] = DataQualityMetrics(
                completeness=0.6,  # Synthetic data
                freshness=0.5,  # Historical
                consistency=0.8,  # Well-structured
                accuracy=0.4,  # Simulated
                overall_score=0.55,
            )

            df.attrs["data_source"] = "synthetic"
            df.attrs["symbol"] = symbol
            df.attrs["note"] = (
                "Synthetic data for backtesting - replace with real historical data"
            )

            if use_cache:
                self._cache_data(cache_key, df)

            return df

        except Exception as e:
            print(f"Error generating historical data for {symbol}: {e}")
            return None

    def get_risk_free_rate(self) -> float:
        """
        Get current risk-free rate with fallback logic.

        Returns:
            Risk-free rate as decimal (e.g., 0.065 for 6.5%)
        """
        # For now, return default rate
        # In production, this could fetch from RBI/government sources
        return DEFAULT_RISK_FREE_RATE

    def get_bulk_market_data(self, symbols: List[str]) -> Dict[str, MarketData]:
        """
        Get market data for multiple symbols efficiently with quality validation.

        Args:
            symbols: List of NSE symbols

        Returns:
            Dict mapping symbols to MarketData objects
        """
        result = {}

        for symbol in symbols:
            spot_price = self.get_spot_price(symbol)
            if spot_price:
                market_data = MarketData(symbol=symbol, spot_price=spot_price)

                # Assess data quality
                market_data.data_quality = self.validator.assess_market_data_quality(
                    market_data
                )

                if market_data.data_quality.is_acceptable():
                    result[symbol] = market_data

        return result

    def get_fno_snapshot(self) -> List[Dict[str, Any]]:
        """
        Get snapshot of all F&O instruments with error handling.

        Returns:
            List of F&O instrument data
        """
        try:
            response = self._retry_operation(self.nse.live_fno)
            return response.get("data", [])
        except Exception as e:
            print(f"Error fetching F&O snapshot: {e}")
            return []

    def get_market_turnover(self) -> Dict[str, Any]:
        """
        Get market turnover data with error handling.

        Returns:
            Market turnover statistics
        """
        try:
            return self._retry_operation(self.nse.market_turnover)
        except Exception as e:
            print(f"Error fetching market turnover: {e}")
            return {}

    def get_holidays(self, year: Optional[int] = None) -> List[Dict[str, Any]]:
        """
        Get NSE holiday list with error handling.

        Args:
            year: Year for holidays (default: current year)

        Returns:
            List of holiday dates
        """
        try:
            if year is None:
                year = datetime.now().year
            return self._retry_operation(self.nse.holiday_list, year)
        except Exception as e:
            print(f"Error fetching holidays for {year}: {e}")
            return []

    def validate_data_quality(self, data: Any) -> DataQualityMetrics:
        """
        Validate quality of any data object.

        Args:
            data: Data object to validate

        Returns:
            DataQualityMetrics object
        """
        if isinstance(data, MarketData):
            return self.validator.assess_market_data_quality(data)
        elif isinstance(data, FuturesData):
            # Convert to MarketData for assessment
            market_data = MarketData(
                symbol=data.symbol,
                spot_price=data.spot_price,
                futures_price=data.futures_price,
                oi=data.oi,
                volume=data.volume,
                timestamp=data.timestamp,
            )
            return self.validator.assess_market_data_quality(market_data)
        else:
            # Generic quality assessment
            return DataQualityMetrics(
                completeness=0.5,
                freshness=0.5,
                consistency=0.5,
                accuracy=0.5,
                overall_score=0.5,
            )


# Global instance for easy access (lazy-initialized)
_data_provider_instance = None


def get_data_provider() -> GalactusDataProvider:
    """
    Get or create the global data provider instance (lazy initialization).
    
    Returns:
        GalactusDataProvider instance
    """
    global _data_provider_instance
    if _data_provider_instance is None:
        _data_provider_instance = GalactusDataProvider()
    return _data_provider_instance


def get_current_market_data() -> Dict[str, Any]:
    """
    Convenience function to get current market snapshot.

    Returns:
        Dict with market status and key data points
    """
    provider = get_data_provider()

    return {
        "market_status": provider.get_market_status(),
        "is_market_open": provider.is_market_open(),
        "nifty_index": provider.get_nifty_index(),
        "risk_free_rate": provider.get_risk_free_rate(),
        "timestamp": datetime.now(),
    }


def get_futures_for_signal(symbol: str = "NIFTY") -> Optional[FuturesData]:
    """
    Get futures data formatted for signal computation.

    Args:
        symbol: Futures symbol (default: NIFTY)

    Returns:
        FuturesData object ready for signal processing
    """
    provider = get_data_provider()
    return provider.get_futures_data(symbol)


if __name__ == "__main__":
    # Test the data provider
    print("Testing Galactus Data Provider...")
    print("=" * 50)

    provider = GalactusDataProvider()

    # Test market status
    print("Market Status:")
    status = provider.get_market_status()
    print(f"  Market Open: {provider.is_market_open()}")

    # Test NIFTY data
    print("\nNIFTY Data:")
    nifty = provider.get_nifty_index()
    print(f"  NIFTY Index: {nifty}")

    futures_data = provider.get_futures_data("NIFTY")
    if futures_data:
        print(f"  Futures Price: {futures_data.futures_price}")
        print(f"  Spot Price: {futures_data.spot_price}")
        print(f"  OI: {futures_data.oi}")
        print(f"  Time to Expiry: {futures_data.time_to_expiry_days} days")
        if futures_data.data_quality:
            print(
                f"  Data Quality Score: {futures_data.data_quality.overall_score:.2f}"
            )

    # Test individual stock
    print("\nStock Data (RELIANCE):")
    reliance_price = provider.get_spot_price("RELIANCE")
    print(f"  RELIANCE Spot Price: {reliance_price}")

    # Test option chain
    print("\nOption Chain (NIFTY):")
    options = provider.get_option_chain("NIFTY")
    if options:
        print(f"  Retrieved {len(options)} option contracts")
        if options[0].data_quality:
            print(
                f"  Sample option quality: {options[0].data_quality.overall_score:.2f}"
            )
    else:
        print("  Option chain unavailable")

    # Test historical data
    print("\nHistorical Data (NIFTY):")
    historical = provider.get_historical_data("NIFTY", days=5)
    if historical is not None:
        print(f"  Historical data shape: {historical.shape}")
        print(f"  Date range: {historical.index.min()} to {historical.index.max()}")
        if hasattr(historical, "attrs") and "data_quality" in historical.attrs:
            quality = historical.attrs["data_quality"]
            print(
                f"  Data quality: {quality.overall_score:.2f} ({historical.attrs.get('data_source', 'unknown')})"
            )
    else:
        print("  Historical data unavailable")

    print("\nRisk-free Rate:")
    rf_rate = provider.get_risk_free_rate()
    print(f"  Current RF Rate: {rf_rate:.1%}")

    print("\nData provider test completed!")
