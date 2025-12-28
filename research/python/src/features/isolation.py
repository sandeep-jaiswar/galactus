"""
Feature Isolation Mechanisms

Provides decorators and utilities to enforce feature isolation and prevent
contamination of production logic during feature discovery.

Core Principles:
1. Features are experimental until promoted
2. No feature code may call production systems
3. Features must be self-contained and documented
4. Features must track their hypothesis and assumptions
"""

import functools
import warnings
from datetime import datetime
from typing import Any, Callable, Dict, Optional


class FeatureIsolationError(Exception):
    """Raised when feature isolation boundaries are violated."""

    pass


class FeatureMetadata:
    """Metadata tracking for experimental features."""

    def __init__(
        self,
        name: str,
        hypothesis: str,
        created: datetime,
        assumptions: Optional[list] = None,
        failure_modes: Optional[list] = None,
    ):
        self.name = name
        self.hypothesis = hypothesis
        self.created = created
        self.assumptions = assumptions or []
        self.failure_modes = failure_modes or []
        self.promoted = False
        self.production_ready = False

    def to_dict(self) -> Dict[str, Any]:
        """Convert metadata to dictionary format."""
        return {
            "name": self.name,
            "hypothesis": self.hypothesis,
            "created": self.created.isoformat(),
            "assumptions": self.assumptions,
            "failure_modes": self.failure_modes,
            "promoted": self.promoted,
            "production_ready": self.production_ready,
        }


def mark_experimental(
    hypothesis: str,
    assumptions: Optional[list] = None,
    failure_modes: Optional[list] = None,
) -> Callable:
    """
    Decorator to mark a feature as experimental and enforce isolation.

    This decorator:
    - Documents the feature's hypothesis and assumptions
    - Prevents the feature from being used in production
    - Tracks when the feature was created
    - Emits warnings when the feature is used

    Args:
        hypothesis: The capital behavior hypothesis this feature tests
        assumptions: List of assumptions the feature relies on
        failure_modes: Known or expected failure modes

    Returns:
        Decorated function with isolation enforcement

    Example:
        >>> @mark_experimental(
        ...     hypothesis="Forced unwinding shows in option OI decay",
        ...     assumptions=["Clean derivatives data", "Sufficient liquidity"],
        ...     failure_modes=["Fails during low volume periods"]
        ... )
        ... def compute_oi_decay_pressure(data):
        ...     # Feature implementation
        ...     pass
    """

    def decorator(func: Callable) -> Callable:
        # Create metadata for this feature
        metadata = FeatureMetadata(
            name=func.__name__,
            hypothesis=hypothesis,
            created=datetime.now(),
            assumptions=assumptions,
            failure_modes=failure_modes,
        )

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            # Check for production environment
            prevent_production_use(func.__name__)

            # Emit warning about experimental status
            warnings.warn(
                f"Using experimental feature '{func.__name__}'. "
                f"Hypothesis: {hypothesis}. "
                "This feature has NOT been validated for production use.",
                category=UserWarning,
                stacklevel=2,
            )

            # Execute the function
            return func(*args, **kwargs)

        # Attach metadata to the function
        wrapper.__feature_metadata__ = metadata
        wrapper.__experimental__ = True

        return wrapper

    return decorator


def prevent_production_use(feature_name: str) -> None:
    """
    Raise an error if called in production environment.

    Args:
        feature_name: Name of the feature being protected

    Raises:
        FeatureIsolationError: If GALACTUS_ENV is set to 'production'
    """
    import os

    if os.getenv("GALACTUS_ENV") == "production":
        msg = (
            f"Feature '{feature_name}' is experimental and cannot be used "
            "in production. "
            "Features must pass the promotion checklist and be implemented "
            "in core/rust before production use. "
            "See: docs/06-research-framework/promotion-checklist.md"
        )
        raise FeatureIsolationError(msg)


def validate_feature_isolation(func: Callable) -> bool:
    """
    Check if a function is properly isolated as an experimental feature.

    Args:
        func: The function to validate

    Returns:
        True if the function is properly marked as experimental
    """
    return hasattr(func, "__experimental__") and func.__experimental__


def get_feature_metadata(func: Callable) -> Optional[FeatureMetadata]:
    """
    Retrieve metadata for an experimental feature.

    Args:
        func: The decorated feature function

    Returns:
        FeatureMetadata if available, None otherwise
    """
    return getattr(func, "__feature_metadata__", None)


def list_experimental_features(module) -> Dict[str, FeatureMetadata]:
    """
    List all experimental features in a module.

    Args:
        module: Python module to scan for experimental features

    Returns:
        Dictionary mapping feature names to their metadata
    """
    features = {}

    for name in dir(module):
        obj = getattr(module, name)
        if callable(obj) and validate_feature_isolation(obj):
            metadata = get_feature_metadata(obj)
            if metadata:
                features[name] = metadata

    return features


class FeatureIsolationContext:
    """
    Context manager to enforce feature isolation during exploration.

    Usage:
        >>> with FeatureIsolationContext() as ctx:
        ...     # Feature exploration code here
        ...     result = experimental_feature(data)
    """

    def __init__(self):
        self.features_used = []

    def __enter__(self):
        """Enter the isolation context."""
        import os

        # Ensure we're not in production
        if os.getenv("GALACTUS_ENV") == "production":
            raise FeatureIsolationError(
                "Cannot enter feature isolation context in production environment"
            )
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Exit the isolation context."""
        # Could log feature usage statistics here
        pass

    def track_feature_use(self, feature_name: str):
        """Track that a feature was used in this context."""
        self.features_used.append(feature_name)


def require_hypothesis(func: Callable) -> Callable:
    """
    Decorator that ensures a feature has an associated hypothesis.

    This enforces the requirement from feature-discovery-rules.md that
    every feature must have a hypothesis stating what capital behavior
    it represents.
    """

    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        metadata = get_feature_metadata(func)
        if not metadata or not metadata.hypothesis:
            raise FeatureIsolationError(
                f"Feature '{func.__name__}' lacks a hypothesis. "
                "All features must state what capital behavior they represent. "
                "See: docs/06-research-framework/feature-discovery-rules.md"
            )
        return func(*args, **kwargs)

    return wrapper
