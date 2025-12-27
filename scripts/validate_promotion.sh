#!/bin/bash
# Galactus Promotion Validation Script
# Validates that signal promotion follows the required process

set -e

echo "🔍 Validating promotion process compliance..."
echo ""

EXIT_CODE=0

# Check if this is a promotion PR (by checking for new code in core/rust)
if [ -n "$(git diff --name-only origin/main...HEAD | grep '^core/rust/src')" ]; then
    echo "📦 Detected changes in core/rust - checking promotion compliance..."
    echo ""
    
    # Check 1: PR description references promotion checklist
    echo "✓ Check 1: Promotion checklist referenced in PR..."
    if [ -n "$GITHUB_PR_DESCRIPTION" ]; then
        if echo "$GITHUB_PR_DESCRIPTION" | grep -q "promotion-checklist" || \
           echo "$GITHUB_PR_DESCRIPTION" | grep -q "Promotion Checklist"; then
            echo "   ✅ Promotion checklist referenced"
        else
            echo "   ❌ FAIL: PR must reference the promotion checklist"
            echo ""
            echo "   All new production logic must follow the promotion process."
            echo "   See: docs/06-research-framework/promotion-checklist.md"
            EXIT_CODE=1
        fi
    else
        echo "   ⚠️  Cannot check PR description (not in CI environment)"
    fi
    echo ""
    
    # Check 2: Decision log updated
    echo "✓ Check 2: Decision log updated..."
    if git diff --name-only origin/main...HEAD | grep -q "docs/11-decision-log/decision-log.md"; then
        echo "   ✅ Decision log updated"
    else
        echo "   ⚠️  WARNING: Consider updating decision log"
        echo "   Significant promotions should be documented."
    fi
    echo ""
    
    # Check 3: Documentation updated
    echo "✓ Check 3: Documentation updated..."
    DOC_CHANGES=$(git diff --name-only origin/main...HEAD | grep -E '\.md$' | wc -l)
    if [ "$DOC_CHANGES" -gt 0 ]; then
        echo "   ✅ Documentation updated ($DOC_CHANGES files)"
    else
        echo "   ⚠️  WARNING: No documentation updates detected"
        echo "   New production logic should include documentation."
    fi
    echo ""
    
    # Check 4: Tests added
    echo "✓ Check 4: Tests added for new code..."
    TEST_CHANGES=$(git diff --name-only origin/main...HEAD | grep -E 'test.*\.rs$|.*_test\.rs$' | wc -l)
    if [ "$TEST_CHANGES" -gt 0 ]; then
        echo "   ✅ Test files modified ($TEST_CHANGES files)"
    else
        echo "   ❌ FAIL: No test files detected"
        echo ""
        echo "   All production code must have comprehensive tests."
        echo "   See: docs/02-system-architecture/rust-python-boundary-enforcement.md"
        EXIT_CODE=1
    fi
    echo ""
    
else
    echo "ℹ️  No changes detected in core/rust - skipping promotion validation"
    echo ""
fi

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Promotion validation PASSED"
else
    echo "❌ Promotion validation FAILED"
    echo ""
    echo "Please ensure the promotion process is followed:"
    echo "  1. Complete the promotion checklist"
    echo "  2. Add comprehensive tests"
    echo "  3. Update documentation"
    echo "  4. Document decision in decision log"
    echo ""
    echo "See: docs/06-research-framework/promotion-checklist.md"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE
