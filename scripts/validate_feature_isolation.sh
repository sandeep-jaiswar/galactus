#!/bin/bash
# Galactus Feature Isolation Validator
# Validates that feature discovery code is properly isolated from production

set -e

echo "🔍 Validating feature isolation..."
echo ""

EXIT_CODE=0

# Check 1: No experimental features in Rust production code
echo "✓ Check 1: No experimental Python features in Rust core..."
if [ -d "core/rust" ]; then
    # Check for Python files or Python-related patterns in Rust code
    # Rust shouldn't have Python files or call Python features directly
    PY_FILES=$(find core/rust/src -name "*.py" -not -path "*/bindings/*" 2>/dev/null || true)
    PYTHON_CALLS=$(grep -r "PyO3\|pyo3\|python!" core/rust/src 2>/dev/null | grep -v "^#" | grep -v "//" || true)
    
    if [ -n "$PY_FILES" ]; then
        echo "❌ FAIL: Python files found in Rust core"
        echo "$PY_FILES"
        EXIT_CODE=1
    elif [ -n "$PYTHON_CALLS" ]; then
        echo "⚠️  WARNING: Python integration detected in Rust core:"
        echo "$PYTHON_CALLS"
        echo "   Ensure these are official bindings, not feature imports"
    else
        echo "   ✅ No experimental feature usage in Rust core"
    fi
else
    echo "   ⚠️  Rust core directory not found"
fi
echo ""

# Check 2: Feature package has isolation decorators
echo "✓ Check 2: Feature isolation mechanisms present..."
if [ -f "research/python/src/features/isolation.py" ]; then
    # Check for key isolation functions
    if grep -q "mark_experimental" research/python/src/features/isolation.py && \
       grep -q "prevent_production_use" research/python/src/features/isolation.py && \
       grep -q "FeatureIsolationError" research/python/src/features/isolation.py; then
        echo "   ✅ Feature isolation mechanisms present"
    else
        echo "❌ FAIL: Missing isolation mechanisms in features package"
        EXIT_CODE=1
    fi
else
    echo "❌ FAIL: features/isolation.py not found"
    EXIT_CODE=1
fi
echo ""

# Check 3: All features use @mark_experimental decorator
echo "✓ Check 3: Features properly decorated..."
if [ -f "research/python/src/features/examples.py" ]; then
    # Count function definitions
    FUNC_COUNT=$(grep -c "^def compute_" research/python/src/features/examples.py || true)
    # Count @mark_experimental decorators
    DECORATOR_COUNT=$(grep -c "@mark_experimental" research/python/src/features/examples.py || true)
    
    if [ "$FUNC_COUNT" -gt 0 ] && [ "$DECORATOR_COUNT" -ge "$FUNC_COUNT" ]; then
        echo "   ✅ Features are properly decorated ($DECORATOR_COUNT decorators for $FUNC_COUNT functions)"
    else
        echo "⚠️  WARNING: Found $FUNC_COUNT functions but only $DECORATOR_COUNT decorators"
        echo "   All features should use @mark_experimental"
    fi
else
    echo "   ⚠️  No example features found"
fi
echo ""

# Check 4: Production environment guard in __init__.py
echo "✓ Check 4: Production environment guard..."
if [ -f "research/python/src/features/__init__.py" ]; then
    if grep -q "GALACTUS_ENV.*production" research/python/src/features/__init__.py; then
        echo "   ✅ Production environment guard present"
    else
        echo "❌ FAIL: Missing production environment guard in features/__init__.py"
        EXIT_CODE=1
    fi
else
    echo "❌ FAIL: features/__init__.py not found"
    EXIT_CODE=1
fi
echo ""

# Check 5: Feature documentation exists
echo "✓ Check 5: Feature documentation..."
if [ -f "research/python/src/features/README.md" ]; then
    echo "   ✅ Feature package documentation present"
else
    echo "⚠️  WARNING: Missing features/README.md"
fi
echo ""

# Check 6: No production API calls in feature code
echo "✓ Check 6: No production API calls in features..."
if [ -d "research/python/src/features" ]; then
    # Check for common production patterns
    PROD_CALLS=$(grep -r "requests\." research/python/src/features/*.py 2>/dev/null | grep -v "^#" || true)
    if [ -n "$PROD_CALLS" ]; then
        echo "⚠️  WARNING: Found potential API calls in features:"
        echo "$PROD_CALLS"
        echo "   Features should use synthetic data, not live APIs"
    else
        echo "   ✅ No production API calls detected"
    fi
fi
echo ""

# Check 7: Tests for feature isolation
echo "✓ Check 7: Feature isolation tests..."
if [ -f "research/python/tests/test_feature_isolation.py" ]; then
    echo "   ✅ Feature isolation tests present"
else
    echo "⚠️  WARNING: No tests for feature isolation (test_feature_isolation.py)"
fi
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Feature isolation validation PASSED"
    echo ""
    echo "Features are properly isolated from production logic."
else
    echo "❌ Feature isolation validation FAILED"
    echo ""
    echo "Critical issues detected. Please fix the violations above."
    echo ""
    echo "For guidance, see:"
    echo "  - docs/06-research-framework/feature-discovery-rules.md"
    echo "  - research/python/src/features/README.md"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE
