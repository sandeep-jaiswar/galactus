"""
Configuration for Walk-Forward Validation

Defines configuration structures for validation parameters and requirements.
"""

from dataclasses import dataclass, field
from typing import Optional, Dict, Any
from enum import Enum


class RegimeType(Enum):
    """Market regime types that must be covered in validation"""
    HIGH_LIQUIDITY = "high_liquidity"
    LOW_LIQUIDITY = "low_liquidity"
    HIGH_VOLATILITY = "high_volatility"
    LOW_VOLATILITY = "low_volatility"
    EXPIRY_HEAVY = "expiry_heavy"
    REGIME_TRANSITION = "regime_transition"


@dataclass(frozen=True)
class ValidationConfig:
    """
    Configuration for walk-forward validation.
    
    All parameters are frozen to prevent mid-validation changes.
    This enforces the "no silent adaptation" principle.
    """
    
    # Window configuration
    training_window_days: int
    validation_window_days: int
    step_size_days: int
    
    # Event-time enforcement
    enforce_event_time: bool = True
    respect_disclosure_delays: bool = True
    
    # Regime coverage requirements
    require_multiple_liquidity_regimes: bool = True
    require_multiple_volatility_regimes: bool = True
    require_expiry_period: bool = True
    require_regime_transition: bool = True
    
    # Logic freezing
    allow_parameter_changes: bool = False
    allow_threshold_changes: bool = False
    allow_retroactive_fixes: bool = False
    
    # Documentation requirements
    require_window_justification: bool = True
    require_failure_documentation: bool = True
    require_regime_documentation: bool = True
    
    # Validation metadata (optional)
    description: Optional[str] = None
    tags: Dict[str, Any] = field(default_factory=dict)
    
    def __post_init__(self):
        """Validate configuration parameters"""
        if self.training_window_days <= 0:
            raise ValueError("training_window_days must be positive")
        if self.validation_window_days <= 0:
            raise ValueError("validation_window_days must be positive")
        if self.step_size_days <= 0:
            raise ValueError("step_size_days must be positive")
        
        # Enforce conservative defaults
        if not self.enforce_event_time:
            raise ValueError("enforce_event_time cannot be disabled - temporal honesty is mandatory")
