#!/bin/bash
# Output Restrictions Enforcement Script
#
# Validates that code, comments, and documentation do not contain forbidden terms
# that could lead to producing restricted outputs.
#
# This script enforces output restrictions defined in:
# docs/09-compliance-and-language/output-restrictions.md
#
# Exit codes:
# 0 - No violations found
# 1 - Violations detected

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Forbidden terms that should never appear in outputs
# These are high-risk terms that indicate trading instructions or predictions

TRADING_TERMS=(
    "buy signal"
    "sell signal"
    "strong buy"
    "strong sell"
    "buy now"
    "sell now"
    "enter long"
    "enter short"
    "exit position"
    "take profit"
    "stop loss"
)

PREDICTION_TERMS=(
    "target price"
    "price target"
    "expected price"
    "will rise to"
    "will fall to"
    "expected to reach"
    "projected price"
)

SIZING_TERMS=(
    "position size"
    "allocate.*% of portfolio"
    "allocate.*% of capital"
    "recommended size"
    "optimal position"
    "use.*leverage"
)

# Forbidden terms for timing/urgency
# Note: "immediate action" is correctly forbidden for outputs.
# Internal code comments should use "immediate system response" instead.
URGENCY_TERMS=(
    "act now"
    "urgent"
    "don't miss"
    "last chance"
    "window closing"
    "immediate action"
    "time running out"
)

PROMISE_TERMS=(
    "guaranteed return"
    "expected return.*%"
    "assured profit"
    "risk-free"
    "can't lose"
    "will beat market"
    "guaranteed profit"
)

ADVICE_TERMS=(
    "you should buy"
    "you should sell"
    "we recommend buying"
    "we recommend selling"
    "best for your portfolio"
    "right for you"
)

echo -e "${BLUE}🔍 Checking for output restriction violations...${NC}"
echo ""

VIOLATIONS=0
WARNINGS=0

# Allowed terms in specific contexts (exceptions to forbidden terms)
ALLOWED_EXCEPTIONS=(
    "buying pressure"
    "selling pressure"
    "risk-free rate"
)

# Function to check if line contains allowed exceptions
is_allowed_exception() {
    local line="$1"
    for exception in "${ALLOWED_EXCEPTIONS[@]}"; do
        if echo "$line" | grep -qi "$exception"; then
            return 0  # Found exception, line is allowed
        fi
    done
    return 1  # No exception found
}

# Function to check all relevant files
check_category() {
    local category=$1
    shift
    local terms=("$@")
    
    echo -e "${BLUE}Checking category: $category${NC}"
    
    # Check Rust source files (excluding test files and the output_validation module)
    while IFS= read -r -d '' file; do
        # Skip the output_validation module itself (contains terms for validation purposes)
        if [[ "$file" == *"output_validation.rs" ]]; then
            continue
        fi
        
        for term in "${terms[@]}"; do
            # Check each line for violations, excluding allowed exceptions
            while IFS= read -r line; do
                if ! is_allowed_exception "$line"; then
                    echo -e "${RED}❌ VIOLATION in $file: '$term'${NC}"
                    echo "$line"
                    echo ""
                    VIOLATIONS=$((VIOLATIONS + 1))
                    break  # Only report first occurrence per term
                fi
            done < <(grep -inH "$term" "$file" 2>/dev/null)
        done
    done < <(find core/rust/src -name "*.rs" -not -path "*/tests/*" -print0)
    
    # Check API response generation code specifically
    while IFS= read -r -d '' file; do
        # Skip the output_validation module itself
        if [[ "$file" == *"output_validation.rs" ]]; then
            continue
        fi
        
        for term in "${terms[@]}"; do
            # Check each line for violations, excluding allowed exceptions
            while IFS= read -r line; do
                if ! is_allowed_exception "$line"; then
                    echo -e "${RED}❌ CRITICAL VIOLATION in API code $file: '$term'${NC}"
                    echo "$line"
                    echo ""
                    VIOLATIONS=$((VIOLATIONS + 1))
                    break  # Only report first occurrence per term
                fi
            done < <(grep -inH "$term" "$file" 2>/dev/null)
        done
    done < <(find core/rust/src/api -name "*.rs" -print0 2>/dev/null || true)
}

# Check each category
echo -e "${YELLOW}=== Checking Trading Instructions ===${NC}"
check_category "Trading Instructions" "${TRADING_TERMS[@]}"
echo ""

echo -e "${YELLOW}=== Checking Price Predictions ===${NC}"
check_category "Price Predictions" "${PREDICTION_TERMS[@]}"
echo ""

echo -e "${YELLOW}=== Checking Position Sizing ===${NC}"
check_category "Position Sizing" "${SIZING_TERMS[@]}"
echo ""

echo -e "${YELLOW}=== Checking Urgency Language ===${NC}"
check_category "Urgency Language" "${URGENCY_TERMS[@]}"
echo ""

echo -e "${YELLOW}=== Checking Performance Promises ===${NC}"
check_category "Performance Promises" "${PROMISE_TERMS[@]}"
echo ""

echo -e "${YELLOW}=== Checking Personalized Advice ===${NC}"
check_category "Personalized Advice" "${ADVICE_TERMS[@]}"
echo ""

# Check documentation examples for compliance
echo -e "${YELLOW}=== Checking Documentation Examples ===${NC}"
if [ -f "docs/09-compliance-and-language/output-restrictions.md" ]; then
    # Verify the documentation has the forbidden example section marked
    if ! grep -q "❌ FORBIDDEN Output Example" "docs/09-compliance-and-language/output-restrictions.md"; then
        echo -e "${YELLOW}⚠️  Warning: output-restrictions.md should have clearly marked FORBIDDEN examples${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
    
    if ! grep -q "✅ ALLOWED Output Example" "docs/09-compliance-and-language/output-restrictions.md"; then
        echo -e "${YELLOW}⚠️  Warning: output-restrictions.md should have clearly marked ALLOWED examples${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
    
    echo -e "${GREEN}✅ Documentation structure validated${NC}"
else
    echo -e "${RED}❌ Missing output-restrictions.md documentation${NC}"
    VIOLATIONS=$((VIOLATIONS + 1))
fi
echo ""

# Check that output validation module exists
echo -e "${YELLOW}=== Checking Output Validation Module ===${NC}"
if [ -f "core/rust/src/output_validation.rs" ]; then
    echo -e "${GREEN}✅ Output validation module exists${NC}"
    
    # Verify it has the validate_output function
    if grep -q "pub fn validate_output" "core/rust/src/output_validation.rs"; then
        echo -e "${GREEN}✅ validate_output function exists${NC}"
    else
        echo -e "${RED}❌ validate_output function not found${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    fi
    
    # Verify it has tests
    if grep -q "#\[cfg(test)\]" "core/rust/src/output_validation.rs"; then
        echo -e "${GREEN}✅ Output validation tests exist${NC}"
    else
        echo -e "${YELLOW}⚠️  Warning: Output validation should have tests${NC}"
        WARNINGS=$((WARNINGS + 1))
    fi
else
    echo -e "${RED}❌ Output validation module missing: core/rust/src/output_validation.rs${NC}"
    VIOLATIONS=$((VIOLATIONS + 1))
fi
echo ""

# Summary
echo -e "${BLUE}=====================================${NC}"
echo -e "${BLUE}Output Restriction Enforcement Results${NC}"
echo -e "${BLUE}=====================================${NC}"
echo ""

if [ $VIOLATIONS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✅ SUCCESS: No output restriction violations detected${NC}"
    echo ""
    echo "All checks passed:"
    echo "  ✓ No forbidden trading instructions"
    echo "  ✓ No price predictions or targets"
    echo "  ✓ No position sizing recommendations"
    echo "  ✓ No urgency or timing pressure"
    echo "  ✓ No performance promises"
    echo "  ✓ No personalized advice"
    echo "  ✓ Output validation module present"
    echo "  ✓ Documentation complete"
    echo ""
    exit 0
elif [ $VIOLATIONS -eq 0 ]; then
    echo -e "${YELLOW}⚠️  WARNINGS: ${WARNINGS} warnings found${NC}"
    echo ""
    echo "Review warnings above to improve compliance"
    echo ""
    exit 0
else
    echo -e "${RED}❌ FAILURE: ${VIOLATIONS} violations detected${NC}"
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠️  Also found ${WARNINGS} warnings${NC}"
    fi
    echo ""
    echo "Output restriction violations must be fixed before merge."
    echo "See docs/09-compliance-and-language/output-restrictions.md for details."
    echo ""
    exit 1
fi
