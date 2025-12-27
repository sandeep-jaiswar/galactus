"""
Galactus Data Module

Provides unified data access for all Galactus components.
"""

from .provider import (
    GalactusDataProvider,
    DataQualityValidator,
    DataQualityMetrics,
    MarketData,
    FuturesData,
    OptionData,
    IndexData,
    get_data_provider,
    get_current_market_data,
    get_futures_for_signal,
)

__all__ = [
    "GalactusDataProvider",
    "DataQualityValidator",
    "DataQualityMetrics",
    "MarketData",
    "FuturesData",
    "OptionData",
    "IndexData",
    "get_data_provider",
    "get_current_market_data",
    "get_futures_for_signal",
]
