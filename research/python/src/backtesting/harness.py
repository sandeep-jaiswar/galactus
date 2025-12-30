"""
Galactus Backtesting Harness - Python Orchestrator

This module coordinates the backtesting process:
1. Event replay (deterministic, event-time ordered)
2. Inference snapshots (frozen state recording)
3. Structural evaluation (no outcome awareness)
4. Failure ledger (categorized, immutable)
5. Metrics reporting (non-PnL)

See: docs/07-backtesting-and-validation/galactus-backtesting-harness.md
"""

import json
import logging
from dataclasses import asdict, dataclass, field
from datetime import datetime, timedelta, timezone
from enum import Enum
from typing import Dict, List, Optional, Tuple

import pandas as pd


logger = logging.getLogger(__name__)


class ReplaySpeed(Enum):
    """Replay clock speed options"""

    REALTIME = 1.0
    FAST_10X = 10.0
    FAST_100X = 100.0


class BacktestFailureCategory(Enum):
    """Categories of backtesting failures"""

    REGIME_LAG = "regime_lag"
    FALSE_PRESSURE = "false_pressure"
    OVERCONFIDENCE = "overconfidence"
    INCORRECT_SILENCE = "incorrect_silence"
    INCORRECT_ACTIVATION = "incorrect_activation"
    KILL_SWITCH_LATE = "kill_switch_late"
    KILL_SWITCH_EARLY = "kill_switch_early"
    OTHER = "other"


@dataclass
class RegimeSnapshot:
    """Regime state at a point in time"""

    classification: str
    confidence: float  # 0.0 to 1.0
    supporting_signals: List[str] = field(default_factory=list)
    conflicting_signals: List[str] = field(default_factory=list)
    time_in_regime: int = 0  # Seconds


@dataclass
class PressureSnapshot:
    """Capital pressure state at a point in time"""

    detected: bool
    intensity: float  # 0.0 to 1.0
    confidence: float  # 0.0 to 1.0
    sources: List[str] = field(default_factory=list)
    false_positive_rate: float = 0.0


@dataclass
class ForcedFlowSnapshot:
    """Forced flow estimation"""

    estimated_magnitude: float
    confidence: float  # 0.0 to 1.0
    primary_driver: Optional[str] = None
    estimated_duration_minutes: Optional[int] = None


@dataclass
class LiquiditySnapshot:
    """Liquidity assessment"""

    bid_ask_spread: float
    depth: float
    quality: str  # "excellent", "normal", "degraded", "critical"
    market_impact_time: float  # Minutes to execute 10% ADV


@dataclass
class DataQualitySnapshot:
    """Data quality metrics"""

    missing_sources: List[str] = field(default_factory=list)
    staleness_warnings: List[str] = field(default_factory=list)
    validation_failures: List[str] = field(default_factory=list)
    quality_score: float = 1.0  # 0.0 to 1.0


@dataclass
class InferenceSnapshot:
    """Frozen inference snapshot at a specific point in event-time"""

    snapshot_id: str
    event_timestamp: datetime
    event_sequence: int
    record_timestamp: datetime
    instrument: str
    regime: RegimeSnapshot
    capital_pressure: PressureSnapshot
    forced_flow: ForcedFlowSnapshot
    liquidity: LiquiditySnapshot
    overall_confidence: float  # 0.0 to 1.0
    stability_indicator: float  # 0.0 to 1.0
    silenced: bool
    kill_switch_status: str  # "active", "triggered_data", etc.
    data_quality: DataQualitySnapshot = field(default_factory=DataQualitySnapshot)

    def to_dict(self):
        """Convert to dictionary, handling datetime serialization"""
        d = asdict(self)
        d["event_timestamp"] = self.event_timestamp.isoformat()
        d["record_timestamp"] = self.record_timestamp.isoformat()
        return d


@dataclass
class BacktestFailure:
    """A failure identified during backtesting"""

    timestamp: datetime
    event_sequence: int
    instrument: str
    regime: str
    confidence_at_failure: float
    was_silenced: bool
    category: BacktestFailureCategory
    description: str
    root_cause: str
    corrective_action: Optional[str] = None

    def to_dict(self):
        """Convert to dictionary, handling datetime serialization"""
        d = asdict(self)
        d["timestamp"] = self.timestamp.isoformat()
        d["category"] = self.category.value
        return d


class BacktestMetrics:
    """Summary metrics from a backtest run"""

    def __init__(self):
        self.total_snapshots: int = 0
        self.total_failures: int = 0
        self.high_confidence_failure_rate: float = 0.0
        self.false_pressure_rate: float = 0.0
        self.average_regime_lag_days: float = 0.0
        self.silence_correctness_rate: float = 0.0
        self.kill_switch_anticipation: float = 0.0
        self.confidence_calibration_error: float = 0.0
        self.failures_by_category: Dict[str, int] = {}
        self.regimes_observed: List[str] = []
        self.stress_periods_detected: int = 0

    def to_dict(self) -> dict:
        return {
            "total_snapshots": self.total_snapshots,
            "total_failures": self.total_failures,
            "high_confidence_failure_rate": self.high_confidence_failure_rate,
            "false_pressure_rate": self.false_pressure_rate,
            "average_regime_lag_days": self.average_regime_lag_days,
            "silence_correctness_rate": self.silence_correctness_rate,
            "kill_switch_anticipation": self.kill_switch_anticipation,
            "confidence_calibration_error": self.confidence_calibration_error,
            "failures_by_category": self.failures_by_category,
            "regimes_observed": self.regimes_observed,
            "stress_periods_detected": self.stress_periods_detected,
        }


class SnapshotRecorder:
    """Records inference snapshots (append-only ledger)"""

    def __init__(self):
        self.snapshots: List[InferenceSnapshot] = []
        self.current_sequence: int = 0

    def record_snapshot(self, snapshot: InferenceSnapshot) -> None:
        """Record a new snapshot (append-only)"""
        if snapshot.snapshot_id in [s.snapshot_id for s in self.snapshots]:
            raise ValueError(f"Duplicate snapshot ID: {snapshot.snapshot_id}")

        snapshot.record_timestamp = datetime.now(timezone.utc)
        if snapshot.event_sequence == 0:
            snapshot.event_sequence = self.current_sequence
            self.current_sequence += 1

        self.snapshots.append(snapshot)

    def get_all_snapshots(self) -> List[InferenceSnapshot]:
        """Get all recorded snapshots"""
        return self.snapshots.copy()

    def get_instrument_snapshots(self, instrument: str) -> List[InferenceSnapshot]:
        """Get snapshots for a specific instrument"""
        return [s for s in self.snapshots if s.instrument == instrument]

    def get_silenced_snapshots(self) -> List[InferenceSnapshot]:
        """Get snapshots where Galactus was silenced"""
        return [s for s in self.snapshots if s.silenced]

    def get_high_confidence_snapshots(
        self, threshold: float = 0.8
    ) -> List[InferenceSnapshot]:
        """Get snapshots with confidence > threshold"""
        return [s for s in self.snapshots if s.overall_confidence > threshold]

    def get_regime_transitions(self) -> List[Tuple[datetime, str, str]]:
        """Get regime transitions (from_regime, to_regime) with timestamps"""
        transitions = []
        prev_regime: Optional[str] = None

        for snapshot in self.snapshots:
            current_regime = snapshot.regime.classification
            if prev_regime and prev_regime != current_regime:
                transitions.append(
                    (snapshot.event_timestamp, prev_regime, current_regime)
                )
            prev_regime = current_regime

        return transitions

    def get_statistics(self) -> dict:
        """Get statistics about recorded snapshots"""
        if not self.snapshots:
            return {
                "total_snapshots": 0,
                "silenced_count": 0,
                "kill_switch_triggered_count": 0,
                "average_confidence": 0.0,
                "high_confidence_count": 0,
            }

        silenced = sum(1 for s in self.snapshots if s.silenced)
        kill_switch = sum(1 for s in self.snapshots if s.kill_switch_status != "active")
        avg_conf = sum(s.overall_confidence for s in self.snapshots) / len(
            self.snapshots
        )
        high_conf = sum(1 for s in self.snapshots if s.overall_confidence > 0.8)

        return {
            "total_snapshots": len(self.snapshots),
            "silenced_count": silenced,
            "kill_switch_triggered_count": kill_switch,
            "average_confidence": avg_conf,
            "high_confidence_count": high_conf,
        }

    def export_as_jsonl(self) -> str:
        """Export as JSONL (one snapshot per line)"""
        lines = []

        def _default(o):
            try:
                return o.item()
            except Exception:
                return str(o)

        for snapshot in self.snapshots:
            lines.append(json.dumps(snapshot.to_dict(), default=_default))
        return "\n".join(lines)

    def export_as_csv(self) -> str:
        """Export as CSV for spreadsheet analysis"""
        df = pd.DataFrame(
            [
                {
                    "timestamp": s.event_timestamp.isoformat(),
                    "event_sequence": s.event_sequence,
                    "instrument": s.instrument,
                    "regime": s.regime.classification,
                    "regime_confidence": s.regime.confidence,
                    "pressure_detected": s.capital_pressure.detected,
                    "pressure_confidence": s.capital_pressure.confidence,
                    "overall_confidence": s.overall_confidence,
                    "stability": s.stability_indicator,
                    "silenced": s.silenced,
                    "kill_switch": s.kill_switch_status,
                }
                for s in self.snapshots
            ]
        )
        return df.to_csv(index=False)


class BacktestEvaluator:
    """Evaluates inference quality without outcome awareness"""

    @staticmethod
    def compute_metrics(snapshots: List[InferenceSnapshot]) -> BacktestMetrics:
        """Compute metrics from snapshots"""
        metrics = BacktestMetrics()
        metrics.total_snapshots = len(snapshots)

        if not snapshots:
            return metrics

        # Evaluate confidence calibration
        (
            metrics.high_confidence_failure_rate,
            _,
        ) = BacktestEvaluator.evaluate_confidence_calibration(snapshots)

        # Evaluate false pressure
        metrics.false_pressure_rate = BacktestEvaluator.evaluate_false_pressure_rate(
            snapshots
        )

        # Evaluate regime lag
        metrics.average_regime_lag_days = BacktestEvaluator.evaluate_regime_lag(
            snapshots
        )

        # Evaluate silence correctness
        metrics.silence_correctness_rate = (
            BacktestEvaluator.evaluate_silence_correctness(snapshots)
        )

        # Evaluate kill-switch anticipation
        metrics.kill_switch_anticipation = (
            BacktestEvaluator.evaluate_kill_switch_anticipation(snapshots)
        )

        # Evaluate confidence calibration error
        metrics.confidence_calibration_error = (
            BacktestEvaluator.evaluate_confidence_calibration_error(snapshots)
        )

        # Count regimes
        regimes = set()
        for snapshot in snapshots:
            regimes.add(snapshot.regime.classification)
        metrics.regimes_observed = list(regimes)

        # Count stress periods
        metrics.stress_periods_detected = sum(
            1 for s in snapshots if s.kill_switch_status != "active"
        )

        return metrics

    @staticmethod
    def evaluate_confidence_calibration(
        snapshots: List[InferenceSnapshot],
    ) -> Tuple[float, List]:
        """Evaluate confidence calibration. Returns (failure_rate, failures)"""
        failures = []
        high_conf_count = 0
        high_conf_failures = 0

        for idx, snapshot in enumerate(snapshots):
            if snapshot.overall_confidence > 0.8:
                high_conf_count += 1

                # Failure if kill-switch triggered or regime instability
                is_failure = False

                if snapshot.kill_switch_status != "active":
                    is_failure = True

                if idx > 0 and idx < len(snapshots) - 1:
                    prev_regime = snapshots[idx - 1].regime.classification
                    next_regime = snapshots[idx + 1].regime.classification
                    curr_regime = snapshot.regime.classification
                    if prev_regime != curr_regime or next_regime != curr_regime:
                        is_failure = True

                if is_failure:
                    high_conf_failures += 1
                    failures.append(
                        {
                            "timestamp": snapshot.event_timestamp,
                            "confidence": snapshot.overall_confidence,
                            "regime": snapshot.regime.classification,
                        }
                    )

        failure_rate = (
            high_conf_failures / high_conf_count if high_conf_count > 0 else 0.0
        )
        return failure_rate, failures

    @staticmethod
    def evaluate_false_pressure_rate(snapshots: List[InferenceSnapshot]) -> float:
        """Evaluate false positive rate on pressure detection"""
        false_positives = 0
        total_detections = 0

        for snapshot in snapshots:
            if snapshot.capital_pressure.detected:
                total_detections += 1

                if snapshot.regime.confidence < 0.7 or snapshot.stability_indicator > 0.85:
                    false_positives += 1

        return false_positives / total_detections if total_detections > 0 else 0.0

    @staticmethod
    def evaluate_regime_lag(snapshots: List[InferenceSnapshot]) -> float:
        """Evaluate regime lag (days between structural break and regime change)"""
        lags = []
        prev_regime: Optional[str] = None
        regime_start_idx = 0

        for idx, snapshot in enumerate(snapshots):
            regime = snapshot.regime.classification

            if prev_regime and prev_regime != regime:
                if idx > regime_start_idx:
                    time_diff = (
                        snapshot.event_timestamp
                        - snapshots[regime_start_idx].event_timestamp
                    )
                    days = time_diff.total_seconds() / (24 * 3600)
                    lags.append(days)
                regime_start_idx = idx

            prev_regime = regime

        return sum(lags) / len(lags) if lags else 0.0

    @staticmethod
    def evaluate_silence_correctness(snapshots: List[InferenceSnapshot]) -> float:
        """Evaluate silence correctness"""
        correct_silences = 0
        total_should_silence = 0

        for snapshot in snapshots:
            # Should silence if data quality poor, kill-switch triggered, etc.
            should_silence = (
                snapshot.data_quality.quality_score < 0.7
                or snapshot.kill_switch_status != "active"
                or len(snapshot.regime.conflicting_signals) > 0
                or snapshot.overall_confidence < 0.3
            )

            if should_silence:
                total_should_silence += 1
                if snapshot.silenced:
                    correct_silences += 1

        return (
            correct_silences / total_should_silence if total_should_silence > 0 else 1.0
        )

    @staticmethod
    def evaluate_kill_switch_anticipation(snapshots: List[InferenceSnapshot]) -> float:
        """Evaluate kill-switch anticipation (% triggers before instability)"""
        anticipatory = 0
        total_triggers = 0

        for idx, snapshot in enumerate(snapshots):
            if snapshot.kill_switch_status != "active":
                total_triggers += 1

                # Check if instability follows in next 30 snapshots
                instability_follows = any(
                    s.stability_indicator < 0.5
                    for s in snapshots[idx + 1 : min(idx + 31, len(snapshots))]
                )

                if instability_follows:
                    anticipatory += 1

        return anticipatory / total_triggers if total_triggers > 0 else 0.0

    @staticmethod
    def evaluate_confidence_calibration_error(
        snapshots: List[InferenceSnapshot],
    ) -> float:
        """Evaluate confidence vs realized stability mismatch"""
        errors = [abs(s.overall_confidence - s.stability_indicator) for s in snapshots]
        return sum(errors) / len(errors) if errors else 0.0

    @staticmethod
    def build_failure_ledger(
        snapshots: List[InferenceSnapshot],
    ) -> List[BacktestFailure]:
        """Build failure ledger from snapshots"""
        failures = []

        for idx, snapshot in enumerate(snapshots):
            failure_categories = []

            # Regime lag
            if idx > 0:
                prev_regime = snapshots[idx - 1].regime.classification
                if prev_regime != snapshot.regime.classification:
                    if snapshot.overall_confidence > 0.7:
                        failure_categories.append(BacktestFailureCategory.REGIME_LAG)

            # False pressure
            if snapshot.capital_pressure.detected:
                if snapshot.regime.confidence < 0.6:
                    failure_categories.append(BacktestFailureCategory.FALSE_PRESSURE)

            # Overconfidence
            if (
                snapshot.kill_switch_status != "active"
                and snapshot.overall_confidence > 0.7
            ):
                failure_categories.append(BacktestFailureCategory.OVERCONFIDENCE)

            # Incorrect activation
            if snapshot.data_quality.quality_score < 0.5 and not snapshot.silenced:
                failure_categories.append(BacktestFailureCategory.INCORRECT_ACTIVATION)

            # Create failure records
            for category in failure_categories:
                failures.append(
                    BacktestFailure(
                        timestamp=snapshot.event_timestamp,
                        event_sequence=snapshot.event_sequence,
                        instrument=snapshot.instrument,
                        regime=snapshot.regime.classification,
                        confidence_at_failure=snapshot.overall_confidence,
                        was_silenced=snapshot.silenced,
                        category=category,
                        description=f"{category.value} in {snapshot.regime.classification} regime",
                        root_cause="To be determined",
                    )
                )

        return failures


class BacktestFailureLedger:
    """Immutable, append-only ledger of failures"""

    def __init__(self):
        self.failures: List[BacktestFailure] = []

    def record_failure(self, failure: BacktestFailure) -> None:
        """Record a failure"""
        self.failures.append(failure)

    def record_failures(self, failures: List[BacktestFailure]) -> None:
        """Record multiple failures"""
        self.failures.extend(failures)

    def get_all_failures(self) -> List[BacktestFailure]:
        """Get all failures"""
        return self.failures.copy()

    def get_failures_by_category(
        self, category: BacktestFailureCategory
    ) -> List[BacktestFailure]:
        """Get failures by category"""
        return [f for f in self.failures if f.category == category]

    def get_high_confidence_failures(
        self, threshold: float = 0.8
    ) -> List[BacktestFailure]:
        """Get failures where confidence was high"""
        return [f for f in self.failures if f.confidence_at_failure > threshold]

    def get_summary(self) -> dict:
        """Get summary statistics"""
        by_category = {}
        for failure in self.failures:
            cat = failure.category.value
            by_category[cat] = by_category.get(cat, 0) + 1

        high_conf = sum(1 for f in self.failures if f.confidence_at_failure > 0.8)

        return {
            "total_failures": len(self.failures),
            "high_confidence_failures": high_conf,
            "failures_by_category": by_category,
        }

    def export_as_csv(self) -> str:
        """Export as CSV"""
        df = pd.DataFrame(
            [
                {
                    "timestamp": f.timestamp.isoformat(),
                    "event_sequence": f.event_sequence,
                    "instrument": f.instrument,
                    "regime": f.regime,
                    "confidence": f.confidence_at_failure,
                    "was_silenced": f.was_silenced,
                    "category": f.category.value,
                    "description": f.description,
                    "root_cause": f.root_cause,
                }
                for f in self.failures
            ]
        )
        return df.to_csv(index=False)

    def export_as_json(self) -> str:
        """Export as JSON"""

        # Ensure numpy types and other non-serializable objects are handled
        def _default(o):
            try:
                # numpy types expose .item()
                return o.item()
            except Exception:
                return str(o)

        return json.dumps(
            [f.to_dict() for f in self.failures], indent=2, default=_default
        )


class BacktestHarness:
    """Main backtesting harness orchestrator"""

    def __init__(self, galactus_version: str, run_name: str):
        self.galactus_version = galactus_version
        self.run_name = run_name
        self.snapshot_recorder = SnapshotRecorder()
        self.failure_ledger = BacktestFailureLedger()
        self.metrics: Optional[BacktestMetrics] = None
        self.period_start: Optional[datetime] = None
        self.period_end: Optional[datetime] = None

    def run_backtest(self, snapshots: List[InferenceSnapshot]) -> BacktestMetrics:
        """Run the backtest evaluation"""
        logger.info(
            f"Starting backtest: {self.run_name} (version {self.galactus_version})"
        )

        if not snapshots:
            logger.warning("No snapshots provided")
            return BacktestMetrics()

        # Set period from snapshots
        self.period_start = min(s.event_timestamp for s in snapshots)
        self.period_end = max(s.event_timestamp for s in snapshots)

        # Record all snapshots
        for snapshot in snapshots:
            self.snapshot_recorder.record_snapshot(snapshot)

        # Evaluate
        self.metrics = BacktestEvaluator.compute_metrics(snapshots)

        # Build failure ledger
        failures = BacktestEvaluator.build_failure_ledger(snapshots)
        self.failure_ledger.record_failures(failures)

        logger.info(
            f"Backtest complete: {self.metrics.total_snapshots} snapshots, "
            f"{self.metrics.total_failures} failures"
        )

        return self.metrics

    def get_summary_report(self) -> dict:
        """Get a summary report"""
        if not self.metrics:
            return {}

        return {
            "run_name": self.run_name,
            "galactus_version": self.galactus_version,
            "period_start": (
                self.period_start.isoformat() if self.period_start else None
            ),
            "period_end": self.period_end.isoformat() if self.period_end else None,
            "metrics": self.metrics.to_dict(),
            "failure_summary": self.failure_ledger.get_summary(),
            "snapshot_statistics": self.snapshot_recorder.get_statistics(),
        }

    def export_results(self, base_path: str) -> None:
        """Export all results to files"""
        import os

        os.makedirs(base_path, exist_ok=True)

        # Export metrics
        with open(os.path.join(base_path, "metrics.json"), "w") as f:
            json.dump(self.get_summary_report(), f, indent=2)

        # Export failure ledger
        with open(os.path.join(base_path, "failures.csv"), "w") as f:
            f.write(self.failure_ledger.export_as_csv())

        with open(os.path.join(base_path, "failures.json"), "w") as f:
            f.write(self.failure_ledger.export_as_json())

        # Export snapshots
        with open(os.path.join(base_path, "snapshots.jsonl"), "w") as f:
            f.write(self.snapshot_recorder.export_as_jsonl())

        with open(os.path.join(base_path, "snapshots.csv"), "w") as f:
            f.write(self.snapshot_recorder.export_as_csv())

        logger.info(f"Results exported to {base_path}")
