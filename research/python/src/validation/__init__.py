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

from .config import ValidationConfig
from .rules import (EventTimeValidator, ForbiddenPracticesDetector,
                    LogicFreezeValidator, RegimeCoverageValidator)
from .walk_forward import (ValidationError, ValidationResult, ValidationWindow,
                           WalkForwardValidator, WindowType)

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
