"""
Galactus Research Package

This package contains research and experimental code for signal discovery.

DO NOT use this code in production.
Production logic belongs in core/rust after passing promotion checklist.
"""

__version__ = "0.1.0"
__author__ = "Galactus Research Team"

# Research modules
from . import data, features, validation  # noqa: F401
# Re-export commonly used data components for convenience
from .data import GalactusDataProvider  # noqa: F401
from .data import get_current_market_data  # noqa: F401
from .data import get_data_provider  # noqa: F401
from .data import get_futures_for_signal  # noqa: F401
