"""
Galactus Feature Discovery Layer

This package provides isolated feature exploration utilities.

All features here are EXPERIMENTAL and must NOT be used in production.
Features must pass the promotion checklist before moving to core/rust.

See: docs/06-research-framework/feature-discovery-rules.md
"""

from .isolation import (FeatureIsolationError, mark_experimental,
                        prevent_production_use)

__all__ = [
    "mark_experimental",
    "prevent_production_use",
    "FeatureIsolationError",
]

__version__ = "0.1.0"

# Production guard - prevent accidental imports
import os

if os.getenv("GALACTUS_ENV") == "production":
    raise FeatureIsolationError(
        "Feature discovery code cannot be imported in production environment. "
        "Only promoted features in core/rust may be used in production."
    )
