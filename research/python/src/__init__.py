"""
Galactus Research Package

This package contains research and experimental code for signal discovery.

DO NOT use this code in production.
Production logic belongs in core/rust after passing promotion checklist.
"""

__version__ = "0.1.0"
__author__ = "Galactus Research Team"

# Research modules
from . import data
from . import features
from . import validation

# Re-export commonly used data components for convenience
from .data import (
    get_data_provider,
    get_current_market_data,
    get_futures_for_signal,
    GalactusDataProvider,
)
