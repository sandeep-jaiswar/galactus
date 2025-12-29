"""
Galactus Backtesting Framework

Deterministic replay and structural evaluation of inference outputs.

See: docs/07-backtesting-and-validation/galactus-backtesting-harness.md
"""

from .harness import (
    BacktestEvaluator,
    BacktestFailure,
    BacktestFailureCategory,
    BacktestFailureLedger,
    BacktestHarness,
    BacktestMetrics,
    DataQualitySnapshot,
    ForcedFlowSnapshot,
    InferenceSnapshot,
    LiquiditySnapshot,
    PressureSnapshot,
    RegimeSnapshot,
    ReplaySpeed,
    SnapshotRecorder,
)

__all__ = [
    "BacktestHarness",
    "BacktestEvaluator",
    "SnapshotRecorder",
    "BacktestFailureLedger",
    "BacktestMetrics",
    "BacktestFailure",
    "BacktestFailureCategory",
    "InferenceSnapshot",
    "RegimeSnapshot",
    "PressureSnapshot",
    "ForcedFlowSnapshot",
    "LiquiditySnapshot",
    "DataQualitySnapshot",
    "ReplaySpeed",
]
