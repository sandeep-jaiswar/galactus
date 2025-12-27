#!/bin/bash
# Galactus Promotion Checklist Validator
# Enforces hard gating for promotion from research to production
# This script validates promotion checklist files and blocks promotion if requirements are not met

set -e

echo "🔒 Galactus Promotion Checklist Validator"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

EXIT_CODE=0
WARNINGS=0
ERRORS=0

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if yq is available for YAML parsing
if ! command -v yq &> /dev/null; then
    echo -e "${YELLOW}⚠️  WARNING: yq not found. Installing basic YAML parsing...${NC}"
    echo ""
    # Fallback to basic grep-based validation if yq is not available
    USE_BASIC_PARSING=true
else
    USE_BASIC_PARSING=false
fi

# Find all promotion checklist files (exclude template)
CHECKLIST_DIR="research/python/promotions"
if [ ! -d "$CHECKLIST_DIR" ]; then
    echo -e "${YELLOW}ℹ️  No promotion checklist directory found${NC}"
    echo "   This is OK if no promotions are in progress"
    echo ""
    exit 0
fi

CHECKLISTS=$(find "$CHECKLIST_DIR" -name "*.yml" -not -name "TEMPLATE-*.yml" 2>/dev/null || true)

if [ -z "$CHECKLISTS" ]; then
    echo -e "${GREEN}✅ No active promotion checklists found${NC}"
    echo "   This is OK if no promotions are in progress"
    echo ""
    exit 0
fi

echo "📋 Found promotion checklist(s) to validate:"
echo "$CHECKLISTS" | sed 's/^/   - /'
echo ""

# Validate each checklist
for CHECKLIST_FILE in $CHECKLISTS; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${BLUE}Validating: $(basename $CHECKLIST_FILE)${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    
    # Check 1: File is valid YAML
    echo "✓ Check 1: YAML syntax validation..."
    if [ "$USE_BASIC_PARSING" = false ]; then
        if yq eval '.' "$CHECKLIST_FILE" > /dev/null 2>&1; then
            echo -e "   ${GREEN}✅ Valid YAML syntax${NC}"
        else
            echo -e "   ${RED}❌ FAIL: Invalid YAML syntax${NC}"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
            continue
        fi
    else
        # Basic validation - check if file is readable
        if [ -r "$CHECKLIST_FILE" ]; then
            echo -e "   ${GREEN}✅ File is readable${NC}"
        else
            echo -e "   ${RED}❌ FAIL: Cannot read file${NC}"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
            continue
        fi
    fi
    echo ""
    
    # Check 2: Required top-level fields present
    echo "✓ Check 2: Required fields present..."
    REQUIRED_FIELDS=("version" "promotion" "checklist" "artifacts" "decision_log" "reviews" "promotion_status")
    MISSING_FIELDS=0
    
    for FIELD in "${REQUIRED_FIELDS[@]}"; do
        if grep -q "^${FIELD}:" "$CHECKLIST_FILE"; then
            echo -e "   ${GREEN}✅ Field '${FIELD}' present${NC}"
        else
            echo -e "   ${RED}❌ FAIL: Missing required field '${FIELD}'${NC}"
            MISSING_FIELDS=$((MISSING_FIELDS + 1))
            ERRORS=$((ERRORS + 1))
        fi
    done
    
    if [ $MISSING_FIELDS -gt 0 ]; then
        EXIT_CODE=1
        echo ""
        continue
    fi
    echo ""
    
    # Check 3: No incomplete checklist items
    echo "✓ Check 3: Checklist completeness..."
    INCOMPLETE_COUNT=$(grep -c 'status: "incomplete"' "$CHECKLIST_FILE" || true)
    
    if [ "$INCOMPLETE_COUNT" -gt 0 ]; then
        echo -e "   ${RED}❌ FAIL: Found $INCOMPLETE_COUNT incomplete checklist item(s)${NC}"
        echo "   All checklist items must be 'complete' or 'not_applicable'"
        echo ""
        echo "   Incomplete items:"
        grep -B 1 'status: "incomplete"' "$CHECKLIST_FILE" | grep -E '^\s+\w+:' | sed 's/^/     - /' || true
        ERRORS=$((ERRORS + 1))
        EXIT_CODE=1
    else
        echo -e "   ${GREEN}✅ No incomplete items${NC}"
    fi
    echo ""
    
    # Check 4: not_applicable items have justification
    echo "✓ Check 4: Not applicable items justified..."
    NOT_APPLICABLE_LINES=$(grep -n 'status: "not_applicable"' "$CHECKLIST_FILE" | cut -d: -f1 || true)
    UNJUSTIFIED=0
    
    for LINE_NUM in $NOT_APPLICABLE_LINES; do
        # Check if there's a non-empty notes field within next 3 lines
        NOTES_LINE=$((LINE_NUM + 2))
        NOTES_CONTENT=$(sed -n "${NOTES_LINE}p" "$CHECKLIST_FILE" | grep "notes:" || true)
        
        if echo "$NOTES_CONTENT" | grep -q 'notes: ""'; then
            UNJUSTIFIED=$((UNJUSTIFIED + 1))
        fi
    done
    
    if [ $UNJUSTIFIED -gt 0 ]; then
        echo -e "   ${RED}❌ FAIL: Found $UNJUSTIFIED not_applicable item(s) without justification${NC}"
        echo "   All 'not_applicable' items must include justification in 'notes'"
        ERRORS=$((ERRORS + 1))
        EXIT_CODE=1
    else
        echo -e "   ${GREEN}✅ All not_applicable items justified${NC}"
    fi
    echo ""
    
    # Check 5: Evidence provided for complete items
    echo "✓ Check 5: Evidence provided for complete items..."
    COMPLETE_WITHOUT_EVIDENCE=$(grep -A 1 'status: "complete"' "$CHECKLIST_FILE" | grep 'evidence: ""' | wc -l || true)
    
    if [ "$COMPLETE_WITHOUT_EVIDENCE" -gt 0 ]; then
        echo -e "   ${RED}❌ FAIL: Found $COMPLETE_WITHOUT_EVIDENCE complete item(s) without evidence${NC}"
        echo "   All 'complete' items must include evidence path"
        ERRORS=$((ERRORS + 1))
        EXIT_CODE=1
    else
        echo -e "   ${GREEN}✅ All complete items have evidence${NC}"
    fi
    echo ""
    
    # Check 6: Artifacts exist
    echo "✓ Check 6: Artifact files exist..."
    MISSING_ARTIFACTS=0
    
    # Extract artifact paths (basic grep approach)
    ARTIFACT_PATHS=$(grep -A 50 "^artifacts:" "$CHECKLIST_FILE" | grep '^\s\+- "' | sed 's/.*"\(.*\)".*/\1/' || true)
    
    if [ -z "$ARTIFACT_PATHS" ]; then
        echo -e "   ${RED}❌ FAIL: No artifacts listed${NC}"
        echo "   At least one artifact required in each category"
        ERRORS=$((ERRORS + 1))
        EXIT_CODE=1
    else
        for ARTIFACT in $ARTIFACT_PATHS; do
            if [ -f "$ARTIFACT" ]; then
                echo -e "   ${GREEN}✅${NC} $ARTIFACT"
            else
                echo -e "   ${RED}❌ MISSING:${NC} $ARTIFACT"
                MISSING_ARTIFACTS=$((MISSING_ARTIFACTS + 1))
            fi
        done
        
        if [ $MISSING_ARTIFACTS -gt 0 ]; then
            echo -e "   ${RED}❌ FAIL: $MISSING_ARTIFACTS artifact file(s) not found${NC}"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
        fi
    fi
    echo ""
    
    # Check 7: Decision log entry exists
    echo "✓ Check 7: Decision log entry..."
    ENTRY_ADDED=$(grep "entry_added:" "$CHECKLIST_FILE" | grep -o "true\|false" || echo "false")
    # Get overall status early for conditional validation
    OVERALL_STATUS_CHECK=$(grep "^  overall_status:" "$CHECKLIST_FILE" | sed 's/.*: "\(.*\)".*/\1/' || echo "unknown")
    
    if [ "$ENTRY_ADDED" != "true" ]; then
        # Only enforce for approved promotions
        if [ "$OVERALL_STATUS_CHECK" = "approved" ]; then
            echo -e "   ${RED}❌ FAIL: Decision log entry not added${NC}"
            echo "   Must update decision log before promotion approval"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
        else
            echo -e "   ${YELLOW}⚠️  Decision log entry not added (OK for pending/in_progress)${NC}"
        fi
    else
        # Verify decision log file exists
        DECISION_LOG=$(grep "entry_path:" "$CHECKLIST_FILE" | sed 's/.*: "\(.*\)".*/\1/' || true)
        if [ -f "$DECISION_LOG" ]; then
            echo -e "   ${GREEN}✅ Decision log entry recorded${NC}"
        else
            echo -e "   ${RED}❌ FAIL: Decision log file not found: $DECISION_LOG${NC}"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
        fi
    fi
    echo ""
    
    # Check 8: All required reviews approved
    echo "✓ Check 8: Review approvals..."
    REQUIRED_REVIEWERS=("research_lead" "architecture_reviewer" "security_reviewer" "final_approval")
    PENDING_REVIEWS=0
    REJECTED_REVIEWS=0
    
    for REVIEWER in "${REQUIRED_REVIEWERS[@]}"; do
        STATUS=$(grep -A 4 "^  ${REVIEWER}:" "$CHECKLIST_FILE" | grep "status:" | sed 's/.*status: "\(.*\)".*/\1/' || echo "unknown")
        
        if [ "$STATUS" = "approved" ]; then
            echo -e "   ${GREEN}✅${NC} $REVIEWER: approved"
        elif [ "$STATUS" = "pending" ]; then
            echo -e "   ${YELLOW}⏳${NC} $REVIEWER: pending"
            PENDING_REVIEWS=$((PENDING_REVIEWS + 1))
        elif [ "$STATUS" = "rejected" ]; then
            echo -e "   ${RED}❌${NC} $REVIEWER: REJECTED"
            REJECTED_REVIEWS=$((REJECTED_REVIEWS + 1))
        else
            echo -e "   ${RED}❌${NC} $REVIEWER: status unclear"
            PENDING_REVIEWS=$((PENDING_REVIEWS + 1))
        fi
    done
    
    if [ $REJECTED_REVIEWS -gt 0 ]; then
        echo -e "   ${RED}❌ FAIL: $REJECTED_REVIEWS review(s) rejected${NC}"
        echo "   Promotion cannot proceed with rejected reviews"
        ERRORS=$((ERRORS + 1))
        EXIT_CODE=1
    elif [ $PENDING_REVIEWS -gt 0 ]; then
        # Only enforce for approved promotions
        OVERALL_STATUS_CHECK=$(grep "^  overall_status:" "$CHECKLIST_FILE" | sed 's/.*: "\(.*\)".*/\1/' || echo "unknown")
        if [ "$OVERALL_STATUS_CHECK" = "approved" ]; then
            echo -e "   ${RED}❌ FAIL: $PENDING_REVIEWS review(s) still pending but status is approved${NC}"
            echo "   All reviews must be approved before promotion can be marked approved"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
        else
            echo -e "   ${YELLOW}⚠️  WARNING: $PENDING_REVIEWS review(s) still pending (OK for pending/in_progress)${NC}"
            WARNINGS=$((WARNINGS + 1))
        fi
    else
        echo -e "   ${GREEN}✅ All reviews approved${NC}"
    fi
    echo ""
    
    # Check 9: Promotion status consistency
    echo "✓ Check 9: Promotion status consistency..."
    OVERALL_STATUS=$(grep "^  overall_status:" "$CHECKLIST_FILE" | sed 's/.*: "\(.*\)".*/\1/' || echo "unknown")
    
    echo "   Current status: $OVERALL_STATUS"
    
    if [ "$OVERALL_STATUS" = "approved" ]; then
        if [ $PENDING_REVIEWS -gt 0 ] || [ $REJECTED_REVIEWS -gt 0 ]; then
            echo -e "   ${RED}❌ FAIL: Status is 'approved' but reviews incomplete/rejected${NC}"
            ERRORS=$((ERRORS + 1))
            EXIT_CODE=1
        else
            echo -e "   ${GREEN}✅ Status consistent with reviews${NC}"
        fi
    elif [ "$OVERALL_STATUS" = "rejected" ]; then
        echo -e "   ${RED}ℹ️  Promotion rejected - return to research${NC}"
    elif [ "$OVERALL_STATUS" = "pending" ] || [ "$OVERALL_STATUS" = "in_progress" ]; then
        echo -e "   ${YELLOW}ℹ️  Promotion in progress${NC}"
    else
        echo -e "   ${RED}❌ FAIL: Unknown promotion status${NC}"
        ERRORS=$((ERRORS + 1))
        EXIT_CODE=1
    fi
    echo ""
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
done

# Final summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 VALIDATION SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Errors:   $ERRORS"
echo "Warnings: $WARNINGS"
echo ""

if [ $EXIT_CODE -eq 0 ]; then
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠️  VALIDATION PASSED WITH WARNINGS${NC}"
        echo ""
        echo "All hard requirements met, but some items need attention."
    else
        echo -e "${GREEN}✅ VALIDATION PASSED${NC}"
        echo ""
        echo "All promotion requirements satisfied."
        echo "Promotion may proceed according to the checklist status."
    fi
else
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo ""
    echo "Promotion is BLOCKED. Fix all errors before proceeding."
    echo ""
    echo "Common issues:"
    echo "  - Incomplete checklist items"
    echo "  - Missing evidence or artifacts"
    echo "  - Missing decision log entry"
    echo "  - Pending or rejected reviews"
    echo ""
    echo "See docs/06-research-framework/promotion-checklist-format.md"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE
