"""
Core Walk-Forward Validation Implementation

Implements the main validation orchestration and window management.
"""

from dataclasses import dataclass, field
from datetime import datetime, timedelta
from enum import Enum
from typing import List, Optional, Dict, Any, Callable
import hashlib
import json


class WindowType(Enum):
    """Type of validation window"""
    TRAINING = "training"  # Discovery window - hypothesis formulation only
    VALIDATION = "validation"  # Walk-forward evaluation window


@dataclass
class ValidationWindow:
    """
    Represents a time window for validation.
    
    Windows must not overlap (unless explicitly justified).
    Each window is immutable once created.
    """
    window_type: WindowType
    start_time: datetime
    end_time: datetime
    regime_labels: List[str] = field(default_factory=list)
    justification: Optional[str] = None
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        """Validate window integrity"""
        if self.end_time <= self.start_time:
            raise ValueError("end_time must be after start_time")
        
        if self.window_type == WindowType.TRAINING and not self.justification:
            raise ValueError("Training windows require justification")
    
    @property
    def duration_days(self) -> int:
        """Duration of window in days"""
        return (self.end_time - self.start_time).days
    
    def overlaps_with(self, other: 'ValidationWindow') -> bool:
        """Check if this window overlaps with another"""
        return not (self.end_time <= other.start_time or self.start_time >= other.end_time)


@dataclass
class FrozenLogic:
    """
    Represents frozen logic configuration for validation.
    
    All parameters, thresholds, and logic must be frozen before validation.
    Any changes invalidate the validation.
    """
    logic_hash: str  # Hash of the logic/code
    parameters: Dict[str, Any]
    thresholds: Dict[str, float]
    regime_definitions: Dict[str, Any]
    frozen_at: datetime
    
    @classmethod
    def from_config(cls, 
                   logic_code: str,
                   parameters: Dict[str, Any],
                   thresholds: Dict[str, float],
                   regime_definitions: Dict[str, Any]) -> 'FrozenLogic':
        """
        Create frozen logic from configuration.
        
        The logic hash ensures that any code changes invalidate the validation.
        """
        logic_hash = hashlib.sha256(logic_code.encode()).hexdigest()
        return cls(
            logic_hash=logic_hash,
            parameters=parameters,
            thresholds=thresholds,
            regime_definitions=regime_definitions,
            frozen_at=datetime.now()
        )
    
    def verify_unchanged(self, logic_code: str) -> bool:
        """Verify that logic has not changed since freezing"""
        current_hash = hashlib.sha256(logic_code.encode()).hexdigest()
        return current_hash == self.logic_hash


@dataclass
class WindowResult:
    """Results from evaluating a single validation window"""
    window: ValidationWindow
    observations: Dict[str, Any]
    failures: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    metrics: Dict[str, float] = field(default_factory=dict)
    regime_context: Dict[str, Any] = field(default_factory=dict)


class ValidationError(Exception):
    """Raised when validation rules are violated"""
    pass


@dataclass
class ValidationResult:
    """
    Complete validation result across all windows.
    
    This represents the outcome of a full walk-forward validation run.
    """
    config_hash: str
    frozen_logic: FrozenLogic
    training_window: ValidationWindow
    validation_windows: List[ValidationWindow]
    window_results: List[WindowResult]
    started_at: datetime
    
    # Aggregate assessment
    passed: bool
    failures: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    
    # Metadata
    completed_at: Optional[datetime] = None
    documentation: Optional[str] = None
    
    @property
    def total_validation_days(self) -> int:
        """Total days covered by validation windows"""
        if not self.validation_windows:
            return 0
        return sum(w.duration_days for w in self.validation_windows)
    
    @property
    def regime_coverage(self) -> Dict[str, int]:
        """Count of windows per regime type"""
        coverage = {}
        for window in self.validation_windows:
            for regime in window.regime_labels:
                coverage[regime] = coverage.get(regime, 0) + 1
        return coverage


class WalkForwardValidator:
    """
    Main walk-forward validation orchestrator.
    
    This class enforces:
    1. Event-time fidelity
    2. Logic freezing
    3. Sequential exposure
    4. Regime coverage
    5. Documentation requirements
    """
    
    def __init__(self, config: 'ValidationConfig'):
        """
        Initialize validator with frozen configuration.
        
        Args:
            config: ValidationConfig with all parameters frozen
        """
        from .config import ValidationConfig
        if not isinstance(config, ValidationConfig):
            raise TypeError("config must be ValidationConfig")
        
        self._config = config
        self._config_hash = self._compute_config_hash()
        self._frozen_logic: Optional[FrozenLogic] = None
        self._training_window: Optional[ValidationWindow] = None
        self._validation_windows: List[ValidationWindow] = []
        self._window_results: List[WindowResult] = []
        self._started_at: Optional[datetime] = None
    
    def _compute_config_hash(self) -> str:
        """Compute hash of configuration for immutability check"""
        config_str = json.dumps(vars(self._config), sort_keys=True, default=str)
        return hashlib.sha256(config_str.encode()).hexdigest()
    
    def freeze_logic(self,
                     logic_code: str,
                     parameters: Dict[str, Any],
                     thresholds: Dict[str, float],
                     regime_definitions: Dict[str, Any]) -> None:
        """
        Freeze logic before validation begins.
        
        This must be called before any validation windows are evaluated.
        Once frozen, any changes to logic invalidate the validation.
        
        Args:
            logic_code: The actual inference logic as string
            parameters: All parameters used by the logic
            thresholds: All thresholds used for decisions
            regime_definitions: Definitions of market regimes
        
        Raises:
            ValidationError: If logic is already frozen or validation has started
        """
        if self._frozen_logic is not None:
            raise ValidationError("Logic is already frozen - cannot refreeze")
        
        if self._validation_windows:
            raise ValidationError("Cannot freeze logic after validation has started")
        
        self._frozen_logic = FrozenLogic.from_config(
            logic_code=logic_code,
            parameters=parameters,
            thresholds=thresholds,
            regime_definitions=regime_definitions
        )
    
    def set_training_window(self, start_time: datetime, end_time: datetime,
                           justification: str) -> None:
        """
        Set the training/discovery window.
        
        This window is used only for hypothesis formulation and must not
        overlap with validation windows.
        
        Args:
            start_time: Start of training window
            end_time: End of training window (must be before first validation window)
            justification: Explicit justification for window choice
        
        Raises:
            ValidationError: If training window overlaps with validation windows
        """
        if self._training_window is not None:
            raise ValidationError("Training window already set")
        
        if not justification:
            raise ValidationError("Training window requires justification")
        
        window = ValidationWindow(
            window_type=WindowType.TRAINING,
            start_time=start_time,
            end_time=end_time,
            justification=justification
        )
        
        # Check for overlaps with existing validation windows
        for val_window in self._validation_windows:
            if window.overlaps_with(val_window):
                raise ValidationError(
                    "Training window overlaps with validation window - this violates temporal honesty"
                )
        
        self._training_window = window
    
    def add_validation_window(self,
                            start_time: datetime,
                            end_time: datetime,
                            regime_labels: List[str],
                            justification: Optional[str] = None) -> None:
        """
        Add a validation window for walk-forward evaluation.
        
        Windows must be added in chronological order.
        Windows should not overlap unless explicitly justified.
        
        Args:
            start_time: Start of validation window (must be after training window)
            end_time: End of validation window
            regime_labels: Market regime labels for this window
            justification: Optional justification for overlapping windows
        
        Raises:
            ValidationError: If window violates temporal ordering or overlap rules
        """
        if self._training_window and start_time < self._training_window.end_time:
            raise ValidationError(
                "Validation window starts before training window ends - this violates temporal honesty"
            )
        
        window = ValidationWindow(
            window_type=WindowType.VALIDATION,
            start_time=start_time,
            end_time=end_time,
            regime_labels=regime_labels,
            justification=justification
        )
        
        # Check chronological ordering
        if self._validation_windows:
            last_window = self._validation_windows[-1]
            if window.start_time < last_window.start_time:
                raise ValidationError("Validation windows must be added in chronological order")
            
            # Check for overlaps
            if window.overlaps_with(last_window):
                if not justification:
                    raise ValidationError(
                        "Overlapping validation windows require explicit justification"
                    )
        
        self._validation_windows.append(window)
    
    def evaluate_window(self,
                       window_index: int,
                       inference_func: Callable[[datetime, datetime], Dict[str, Any]]) -> WindowResult:
        """
        Evaluate a single validation window.
        
        The inference function is called with the window boundaries and must
        return observations using only information available up to the end time.
        
        Args:
            window_index: Index of window to evaluate
            inference_func: Function that performs inference for the window
                           Must respect event-time fidelity
        
        Returns:
            WindowResult with observations and failures
        
        Raises:
            ValidationError: If logic is not frozen or window is invalid
        """
        if self._frozen_logic is None:
            raise ValidationError("Logic must be frozen before evaluation")
        
        if window_index >= len(self._validation_windows):
            raise ValidationError(f"Invalid window index: {window_index}")
        
        if self._started_at is None:
            self._started_at = datetime.now()
        
        window = self._validation_windows[window_index]
        
        # Execute inference - this is where the actual signal logic runs
        # The caller is responsible for ensuring event-time honesty
        observations = inference_func(window.start_time, window.end_time)
        
        # Create result
        result = WindowResult(
            window=window,
            observations=observations,
            failures=[],
            warnings=[],
            metrics={},
            regime_context={"regimes": window.regime_labels}
        )
        
        self._window_results.append(result)
        return result
    
    def finalize(self, documentation: Optional[str] = None) -> ValidationResult:
        """
        Finalize validation and produce complete result.
        
        This checks all requirements and produces a pass/fail decision.
        
        Args:
            documentation: Required documentation of validation results
        
        Returns:
            ValidationResult with complete assessment
        
        Raises:
            ValidationError: If required components are missing
        """
        if self._frozen_logic is None:
            raise ValidationError("Cannot finalize without frozen logic")
        
        if not self._validation_windows:
            raise ValidationError("Cannot finalize without validation windows")
        
        if self._config.require_window_justification and not documentation:
            raise ValidationError("Documentation is required for this validation")
        
        failures = []
        warnings = []
        
        # Check regime coverage
        regime_coverage = {}
        for window in self._validation_windows:
            for regime in window.regime_labels:
                regime_coverage[regime] = regime_coverage.get(regime, 0) + 1
        
        # Validate regime requirements
        if self._config.require_multiple_liquidity_regimes:
            liquidity_regimes = [r for r in regime_coverage.keys() if 'liquidity' in r.lower()]
            if len(liquidity_regimes) < 2:
                failures.append("Validation must include multiple liquidity regimes")
        
        if self._config.require_multiple_volatility_regimes:
            volatility_regimes = [r for r in regime_coverage.keys() if 'volatility' in r.lower()]
            if len(volatility_regimes) < 2:
                failures.append("Validation must include multiple volatility regimes")
        
        if self._config.require_expiry_period:
            if not any('expiry' in r.lower() for r in regime_coverage.keys()):
                failures.append("Validation must include at least one expiry-heavy period")
        
        if self._config.require_regime_transition:
            if not any('transition' in r.lower() for r in regime_coverage.keys()):
                failures.append("Validation must include at least one regime transition window")
        
        # Check for documented failures in results
        has_failures = any(result.failures for result in self._window_results)
        if not has_failures:
            warnings.append(
                "No failures documented - signals that never fail are suspect. "
                "Ensure counterfactual scenarios were tested."
            )
        
        passed = len(failures) == 0
        
        result = ValidationResult(
            config_hash=self._config_hash,
            frozen_logic=self._frozen_logic,
            training_window=self._training_window,
            validation_windows=self._validation_windows,
            window_results=self._window_results,
            passed=passed,
            failures=failures,
            warnings=warnings,
            started_at=self._started_at,
            completed_at=datetime.now(),
            documentation=documentation
        )
        
        return result
