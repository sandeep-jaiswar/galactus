#!/bin/bash
# Galactus Boundary Integrity Checker
# Validates the separation between Rust (production) and Python (research)

set -e

echo "🔍 Checking Rust vs Python boundary integrity..."
echo ""

EXIT_CODE=0

# Check 1: No Python files in Rust core (except bindings)
echo "✓ Check 1: Python files in Rust core..."
if [ -d "core/rust" ]; then
    PY_IN_RUST=$(find core/rust/src -name "*.py" -not -path "*/bindings/*" 2>/dev/null || true)
    if [ -n "$PY_IN_RUST" ]; then
        echo "❌ FAIL: Python files found in Rust core:"
        echo "$PY_IN_RUST"
        echo ""
        echo "Python code is not allowed in the Rust production core."
        echo "See: docs/02-system-architecture/rust-python-boundary-enforcement.md"
        EXIT_CODE=1
    else
        echo "   ✅ No Python files in Rust core"
    fi
else
    echo "   ⚠️  Rust core directory not found (core/rust/)"
fi
echo ""

# Check 2: No Rust files in Python research (except when calling binaries)
echo "✓ Check 2: Rust files in Python research..."
if [ -d "research/python" ]; then
    RS_IN_PYTHON=$(find research/python -name "*.rs" 2>/dev/null || true)
    if [ -n "$RS_IN_PYTHON" ]; then
        echo "❌ FAIL: Rust files found in Python research:"
        echo "$RS_IN_PYTHON"
        echo ""
        echo "Rust production code should not be in the Python research layer."
        echo "See: docs/02-system-architecture/rust-python-boundary-enforcement.md"
        EXIT_CODE=1
    else
        echo "   ✅ No Rust files in Python research"
    fi
else
    echo "   ⚠️  Python research directory not found (research/python/)"
fi
echo ""

# Check 3: No shared code directories
echo "✓ Check 3: No shared code directories..."
if [ -d "shared" ] || [ -d "common" ] || [ -d "lib" ]; then
    echo "❌ FAIL: Shared code directory detected"
    echo ""
    echo "Shared code directories violate the separation principle."
    echo "Code should be either in core/rust (production) or research/python (research)."
    EXIT_CODE=1
else
    echo "   ✅ No shared code directories"
fi
echo ""

# Check 4: Symbolic links between layers
echo "✓ Check 4: Symbolic links between layers..."
SYMLINKS=0
if [ -d "core/rust" ]; then
    RUST_SYMLINKS=$(find core/rust -type l 2>/dev/null | wc -l)
    SYMLINKS=$((SYMLINKS + RUST_SYMLINKS))
fi
if [ -d "research/python" ]; then
    PYTHON_SYMLINKS=$(find research/python -type l 2>/dev/null | wc -l)
    SYMLINKS=$((SYMLINKS + PYTHON_SYMLINKS))
fi

if [ "$SYMLINKS" -gt 0 ]; then
    echo "⚠️  WARNING: $SYMLINKS symbolic link(s) found"
    echo "   Ensure they don't violate boundary separation"
else
    echo "   ✅ No symbolic links found"
fi
echo ""

# Check 5: README files exist
echo "✓ Check 5: Documentation present..."
MISSING_DOCS=0
if [ ! -f "core/rust/README.md" ]; then
    echo "⚠️  WARNING: Missing core/rust/README.md"
    MISSING_DOCS=$((MISSING_DOCS + 1))
fi
if [ ! -f "research/python/README.md" ]; then
    echo "⚠️  WARNING: Missing research/python/README.md"
    MISSING_DOCS=$((MISSING_DOCS + 1))
fi
if [ $MISSING_DOCS -eq 0 ]; then
    echo "   ✅ Documentation present"
fi
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Boundary integrity check PASSED"
    echo ""
    echo "The separation between Rust (production) and Python (research)"
    echo "is maintained correctly."
else
    echo "❌ Boundary integrity check FAILED"
    echo ""
    echo "Violations detected. Please review and fix the issues above."
    echo ""
    echo "For guidance, see:"
    echo "  - docs/02-system-architecture/rust-vs-python-contract.md"
    echo "  - docs/02-system-architecture/rust-python-boundary-enforcement.md"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE
