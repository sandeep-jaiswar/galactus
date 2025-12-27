"""
Walk-Forward Validation Framework for Galactus

This module implements the walk-forward validation framework as defined in:
docs/07-backtesting-and-validation/walk-forward-validation.md

Core Principles:
1. Event-time fidelity - No future information leakage
2. Logic freezing - Parameters and logic frozen during validation
3. Sequential exposure - Evaluation in chronological order

This is a research-layer tool. Production validation logic should be
promoted to Rust core after passing promotion checklist.
"""

from .walk_forward import (
    WalkForwardValidator,
    ValidationWindow,
    WindowType,
    ValidationResult,
    ValidationError,
)
from .rules import (
    EventTimeValidator,
    LogicFreezeValidator,
    RegimeCoverageValidator,
    ForbiddenPracticesDetector,
)
from .config import ValidationConfig

__all__ = [
    "WalkForwardValidator",
    "ValidationWindow",
    "WindowType",
    "ValidationResult",
    "ValidationError",
    "EventTimeValidator",
    "LogicFreezeValidator",
    "RegimeCoverageValidator",
    "ForbiddenPracticesDetector",
    "ValidationConfig",
]
