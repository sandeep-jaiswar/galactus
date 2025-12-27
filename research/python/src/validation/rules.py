"""
Validation Rules Implementation

Implements specific validation rules for:
- Event-time fidelity
- Logic freezing
- Regime coverage
- Forbidden practices detection
"""

from dataclasses import dataclass
from datetime import datetime, timedelta
from typing import List, Dict, Any, Optional, Set
from enum import Enum


class ViolationType(Enum):
    """Types of validation violations"""
    LOOK_AHEAD_BIAS = "look_ahead_bias"
    PARAMETER_CHANGE = "parameter_change"
    THRESHOLD_CHANGE = "threshold_change"
    RETROACTIVE_FIX = "retroactive_fix"
    REGIME_RECLASSIFICATION = "regime_reclassification"
    WINDOW_EXCLUSION = "window_exclusion"
    RESULT_SMOOTHING = "result_smoothing"


@dataclass
class Violation:
    """Represents a detected validation violation"""
    violation_type: ViolationType
    description: str
    timestamp: datetime
    severity: str  # "error" or "warning"
    context: Dict[str, Any]


class EventTimeValidator:
    """
    Validates event-time fidelity.
    
    Ensures that inference uses only information available at the time
    of the event, respecting disclosure delays and reporting lags.
    """
    
    def __init__(self, respect_disclosure_delays: bool = True):
        """
        Initialize event-time validator.
        
        Args:
            respect_disclosure_delays: Whether to enforce disclosure delay rules
        """
        self.respect_disclosure_delays = respect_disclosure_delays
        self._event_timeline: List[tuple[datetime, str]] = []
    
    def register_event(self, event_time: datetime, event_type: str) -> None:
        """
        Register an event in the timeline.
        
        Args:
            event_time: When the event occurred
            event_type: Type of event (e.g., "data_available", "inference_made")
        """
        self._event_timeline.append((event_time, event_type))
        self._event_timeline.sort()  # Maintain chronological order
    
    def validate_data_access(self,
                            data_timestamp: datetime,
                            access_timestamp: datetime,
                            disclosure_delay_hours: int = 0) -> Optional[Violation]:
        """
        Validate that data access respects event-time ordering.
        
        Args:
            data_timestamp: When the data was generated
            access_timestamp: When the data is being accessed
            disclosure_delay_hours: Required delay for data availability
        
        Returns:
            Violation if look-ahead bias detected, None otherwise
        """
        required_availability = data_timestamp + timedelta(hours=disclosure_delay_hours)
        
        if access_timestamp < required_availability:
            return Violation(
                violation_type=ViolationType.LOOK_AHEAD_BIAS,
                description=(
                    f"Data accessed before availability. "
                    f"Data timestamp: {data_timestamp}, "
                    f"Access timestamp: {access_timestamp}, "
                    f"Required delay: {disclosure_delay_hours}h"
                ),
                timestamp=datetime.now(),
                severity="error",
                context={
                    "data_timestamp": data_timestamp,
                    "access_timestamp": access_timestamp,
                    "disclosure_delay_hours": disclosure_delay_hours,
                    "gap_hours": (access_timestamp - required_availability).total_seconds() / 3600
                }
            )
        
        return None
    
    def check_timeline_integrity(self) -> List[Violation]:
        """
        Check for timeline integrity violations.
        
        Returns:
            List of violations detected in the event timeline
        """
        violations = []
        
        # Check for out-of-order events
        for i in range(1, len(self._event_timeline)):
            prev_time, prev_type = self._event_timeline[i-1]
            curr_time, curr_type = self._event_timeline[i]
            
            # If inference happens before data, that's a problem
            if curr_type.startswith("inference_") and prev_type.startswith("data_"):
                if curr_time < prev_time:
                    violations.append(Violation(
                        violation_type=ViolationType.LOOK_AHEAD_BIAS,
                        description=f"Inference at {curr_time} uses data from {prev_time}",
                        timestamp=datetime.now(),
                        severity="error",
                        context={"prev_event": prev_type, "curr_event": curr_type}
                    ))
        
        return violations


class LogicFreezeValidator:
    """
    Validates that logic remains frozen during validation.
    
    Detects parameter changes, threshold adjustments, or code modifications
    that violate the frozen logic principle.
    """
    
    def __init__(self, frozen_logic: 'FrozenLogic'):
        """
        Initialize with frozen logic state.
        
        Args:
            frozen_logic: The frozen logic configuration to validate against
        """
        from .walk_forward import FrozenLogic
        if not isinstance(frozen_logic, FrozenLogic):
            raise TypeError("frozen_logic must be FrozenLogic instance")
        
        self._frozen_logic = frozen_logic
        self._change_attempts: List[Violation] = []
    
    def check_parameter_change(self,
                              parameter_name: str,
                              old_value: Any,
                              new_value: Any) -> Optional[Violation]:
        """
        Check if a parameter change violates frozen logic.
        
        Args:
            parameter_name: Name of the parameter
            old_value: Original frozen value
            new_value: New value being applied
        
        Returns:
            Violation if change is detected, None if values match
        """
        if old_value != new_value:
            violation = Violation(
                violation_type=ViolationType.PARAMETER_CHANGE,
                description=(
                    f"Parameter '{parameter_name}' changed during validation. "
                    f"Old: {old_value}, New: {new_value}"
                ),
                timestamp=datetime.now(),
                severity="error",
                context={
                    "parameter": parameter_name,
                    "old_value": old_value,
                    "new_value": new_value
                }
            )
            self._change_attempts.append(violation)
            return violation
        
        return None
    
    def check_threshold_change(self,
                              threshold_name: str,
                              old_threshold: float,
                              new_threshold: float,
                              tolerance: float = 1e-10) -> Optional[Violation]:
        """
        Check if a threshold change violates frozen logic.
        
        Args:
            threshold_name: Name of the threshold
            old_threshold: Original frozen threshold
            new_threshold: New threshold being applied
            tolerance: Numerical tolerance for comparison
        
        Returns:
            Violation if change exceeds tolerance, None otherwise
        """
        if abs(old_threshold - new_threshold) > tolerance:
            violation = Violation(
                violation_type=ViolationType.THRESHOLD_CHANGE,
                description=(
                    f"Threshold '{threshold_name}' changed during validation. "
                    f"Old: {old_threshold}, New: {new_threshold}"
                ),
                timestamp=datetime.now(),
                severity="error",
                context={
                    "threshold": threshold_name,
                    "old_value": old_threshold,
                    "new_value": new_threshold,
                    "delta": abs(old_threshold - new_threshold)
                }
            )
            self._change_attempts.append(violation)
            return violation
        
        return None
    
    def verify_logic_hash(self, current_logic_code: str) -> Optional[Violation]:
        """
        Verify that logic code has not changed.
        
        Args:
            current_logic_code: Current version of logic code
        
        Returns:
            Violation if code has changed, None otherwise
        """
        if not self._frozen_logic.verify_unchanged(current_logic_code):
            return Violation(
                violation_type=ViolationType.PARAMETER_CHANGE,  # Code is a special parameter
                description="Logic code has changed since freezing",
                timestamp=datetime.now(),
                severity="error",
                context={"frozen_hash": self._frozen_logic.logic_hash}
            )
        
        return None
    
    def get_all_violations(self) -> List[Violation]:
        """Get all detected violations"""
        return self._change_attempts.copy()


class RegimeCoverageValidator:
    """
    Validates that validation windows cover required market regimes.
    
    Ensures validation is not limited to favorable or calm periods.
    """
    
    def __init__(self,
                 require_multiple_liquidity: bool = True,
                 require_multiple_volatility: bool = True,
                 require_expiry_period: bool = True,
                 require_regime_transition: bool = True):
        """
        Initialize regime coverage validator.
        
        Args:
            require_multiple_liquidity: Require multiple liquidity regimes
            require_multiple_volatility: Require multiple volatility regimes
            require_expiry_period: Require at least one expiry-heavy period
            require_regime_transition: Require at least one transition window
        """
        self.require_multiple_liquidity = require_multiple_liquidity
        self.require_multiple_volatility = require_multiple_volatility
        self.require_expiry_period = require_expiry_period
        self.require_regime_transition = require_regime_transition
    
    def validate_coverage(self, window_regimes: List[List[str]]) -> List[Violation]:
        """
        Validate regime coverage across all windows.
        
        Args:
            window_regimes: List of regime labels for each window
        
        Returns:
            List of violations if coverage is insufficient
        """
        violations = []
        
        # Flatten all regimes
        all_regimes = set()
        for regimes in window_regimes:
            all_regimes.update(regimes)
        
        # Check liquidity regime coverage
        if self.require_multiple_liquidity:
            liquidity_regimes = {r for r in all_regimes if 'liquidity' in r.lower()}
            if len(liquidity_regimes) < 2:
                violations.append(Violation(
                    violation_type=ViolationType.WINDOW_EXCLUSION,
                    description="Insufficient liquidity regime coverage - need multiple regimes",
                    timestamp=datetime.now(),
                    severity="error",
                    context={"found_regimes": list(liquidity_regimes), "required": 2}
                ))
        
        # Check volatility regime coverage
        if self.require_multiple_volatility:
            volatility_regimes = {r for r in all_regimes if 'volatility' in r.lower()}
            if len(volatility_regimes) < 2:
                violations.append(Violation(
                    violation_type=ViolationType.WINDOW_EXCLUSION,
                    description="Insufficient volatility regime coverage - need multiple regimes",
                    timestamp=datetime.now(),
                    severity="error",
                    context={"found_regimes": list(volatility_regimes), "required": 2}
                ))
        
        # Check for expiry period
        if self.require_expiry_period:
            has_expiry = any('expiry' in r.lower() for r in all_regimes)
            if not has_expiry:
                violations.append(Violation(
                    violation_type=ViolationType.WINDOW_EXCLUSION,
                    description="No expiry-heavy period included in validation",
                    timestamp=datetime.now(),
                    severity="error",
                    context={"all_regimes": list(all_regimes)}
                ))
        
        # Check for regime transition
        if self.require_regime_transition:
            has_transition = any('transition' in r.lower() for r in all_regimes)
            if not has_transition:
                violations.append(Violation(
                    violation_type=ViolationType.WINDOW_EXCLUSION,
                    description="No regime transition window included in validation",
                    timestamp=datetime.now(),
                    severity="error",
                    context={"all_regimes": list(all_regimes)}
                ))
        
        return violations


class ForbiddenPracticesDetector:
    """
    Detects forbidden practices that invalidate walk-forward validation.
    
    Monitors for:
    - Mid-validation parameter tuning
    - Retroactive regime reclassification
    - Selective window exclusion
    - Result smoothing across windows
    """
    
    def __init__(self):
        """Initialize forbidden practices detector"""
        self._detected_violations: List[Violation] = []
        self._excluded_windows: Set[int] = set()
        self._regime_changes: Dict[int, List[tuple[str, str]]] = {}
    
    def detect_window_exclusion(self,
                                total_windows: int,
                                excluded_window_indices: List[int],
                                justifications: Dict[int, str]) -> List[Violation]:
        """
        Detect selective exclusion of windows.
        
        Args:
            total_windows: Total number of windows defined
            excluded_window_indices: Indices of excluded windows
            justifications: Justification for each exclusion
        
        Returns:
            List of violations for unjustified exclusions
        """
        violations = []
        
        for idx in excluded_window_indices:
            self._excluded_windows.add(idx)
            
            if idx not in justifications or not justifications[idx]:
                violations.append(Violation(
                    violation_type=ViolationType.WINDOW_EXCLUSION,
                    description=f"Window {idx} excluded without justification",
                    timestamp=datetime.now(),
                    severity="error",
                    context={"window_index": idx, "total_windows": total_windows}
                ))
        
        # Warn if too many windows excluded
        exclusion_rate = len(excluded_window_indices) / total_windows
        if exclusion_rate > 0.2:  # More than 20% excluded
            violations.append(Violation(
                violation_type=ViolationType.WINDOW_EXCLUSION,
                description=f"High exclusion rate: {exclusion_rate:.1%} of windows excluded",
                timestamp=datetime.now(),
                severity="warning",
                context={
                    "excluded_count": len(excluded_window_indices),
                    "total_windows": total_windows,
                    "exclusion_rate": exclusion_rate
                }
            ))
        
        self._detected_violations.extend(violations)
        return violations
    
    def detect_retroactive_reclassification(self,
                                           window_index: int,
                                           original_regimes: List[str],
                                           new_regimes: List[str]) -> Optional[Violation]:
        """
        Detect retroactive regime reclassification.
        
        Args:
            window_index: Index of the window
            original_regimes: Originally assigned regime labels
            new_regimes: New regime labels being applied
        
        Returns:
            Violation if reclassification detected
        """
        if set(original_regimes) != set(new_regimes):
            changes = []
            for orig in original_regimes:
                if orig not in new_regimes:
                    changes.append((orig, "removed"))
            for new in new_regimes:
                if new not in original_regimes:
                    changes.append(("added", new))
            
            self._regime_changes.setdefault(window_index, []).extend(changes)
            
            violation = Violation(
                violation_type=ViolationType.REGIME_RECLASSIFICATION,
                description=(
                    f"Window {window_index} regime reclassified retroactively. "
                    f"Original: {original_regimes}, New: {new_regimes}"
                ),
                timestamp=datetime.now(),
                severity="error",
                context={
                    "window_index": window_index,
                    "original": original_regimes,
                    "new": new_regimes,
                    "changes": changes
                }
            )
            
            self._detected_violations.append(violation)
            return violation
        
        return None
    
    def detect_result_smoothing(self,
                               window_results: List[float],
                               reported_results: List[float],
                               tolerance: float = 1e-6) -> List[Violation]:
        """
        Detect smoothing of results across windows.
        
        Args:
            window_results: Raw results from each window
            reported_results: Results being reported
            tolerance: Numerical tolerance for comparison
        
        Returns:
            Violations if smoothing detected
        """
        violations = []
        
        if len(window_results) != len(reported_results):
            violations.append(Violation(
                violation_type=ViolationType.RESULT_SMOOTHING,
                description="Result count mismatch - possible aggregation or omission",
                timestamp=datetime.now(),
                severity="error",
                context={
                    "window_count": len(window_results),
                    "reported_count": len(reported_results)
                }
            ))
        
        for i, (raw, reported) in enumerate(zip(window_results, reported_results)):
            if abs(raw - reported) > tolerance:
                violations.append(Violation(
                    violation_type=ViolationType.RESULT_SMOOTHING,
                    description=f"Window {i} result modified. Raw: {raw}, Reported: {reported}",
                    timestamp=datetime.now(),
                    severity="error",
                    context={"window_index": i, "raw": raw, "reported": reported}
                ))
        
        self._detected_violations.extend(violations)
        return violations
    
    def detect_retroactive_fix(self,
                              fix_description: str,
                              fix_timestamp: datetime,
                              validation_start: datetime) -> Optional[Violation]:
        """
        Detect retroactive fixes during validation.
        
        Args:
            fix_description: Description of the fix
            fix_timestamp: When the fix was applied
            validation_start: When validation started
        
        Returns:
            Violation if fix is retroactive
        """
        if fix_timestamp > validation_start:
            violation = Violation(
                violation_type=ViolationType.RETROACTIVE_FIX,
                description=f"Retroactive fix applied during validation: {fix_description}",
                timestamp=datetime.now(),
                severity="error",
                context={
                    "fix": fix_description,
                    "fix_time": fix_timestamp,
                    "validation_start": validation_start
                }
            )
            
            self._detected_violations.append(violation)
            return violation
        
        return None
    
    def get_all_violations(self) -> List[Violation]:
        """Get all detected violations"""
        return self._detected_violations.copy()
    
    def has_violations(self) -> bool:
        """Check if any violations were detected"""
        return len(self._detected_violations) > 0
