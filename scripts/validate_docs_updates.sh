#!/bin/bash
# Galactus Documentation Update Validator
# Enforces documentation updates alongside code changes
# Ensures that meaningful code changes are accompanied by appropriate documentation

set -e

echo "📚 Galactus Documentation Update Validator"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

EXIT_CODE=0
WARNINGS=0

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get the base branch for comparison
BASE_BRANCH="${GITHUB_BASE_REF:-production}"
CURRENT_BRANCH="${GITHUB_HEAD_REF:-$(git rev-parse --abbrev-ref HEAD)}"

echo "Comparing: $CURRENT_BRANCH against $BASE_BRANCH"
echo ""

# Determine the base reference for diff operations
if ! git rev-parse "$BASE_BRANCH" >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  Base branch '$BASE_BRANCH' not found, using HEAD~1 for comparison${NC}"
    BASE_REF="HEAD~1"
    # Verify HEAD~1 is available
    if ! git rev-parse HEAD~1 >/dev/null 2>&1; then
        echo -e "${RED}❌ Cannot determine base reference for comparison${NC}"
        echo "This might be a single-commit repository or detached HEAD state."
        exit 1
    fi
else
    BASE_REF="$BASE_BRANCH"
fi

# Get list of changed files (excluding certain patterns)
CHANGED_FILES=$(git diff --name-only "$BASE_REF" 2>/dev/null || echo "")

if [ -z "$CHANGED_FILES" ]; then
    echo -e "${GREEN}✅ No changes detected${NC}"
    echo ""
    exit 0
fi

echo "Changed files:"
echo "$CHANGED_FILES" | sed 's/^/  /'
echo ""

# Categorize changes
# CODE_CHANGES: Production code files (Rust, Python, config) but excluding:
#   - Documentation files (docs/, README, .md)
#   - CI validation scripts (scripts/validate_*)
#   - Workflow files (handled separately as configuration below)
CODE_CHANGES=$(echo "$CHANGED_FILES" | grep -E '\.(rs|py|toml|yaml|yml|json)$' | grep -v -E '^(docs/|README|\.md$|scripts/validate_|\.github/workflows/)' || true)
DOC_CHANGES=$(echo "$CHANGED_FILES" | grep -E '\.(md|rst|txt)$|^docs/' || true)
RUST_CHANGES=$(echo "$CODE_CHANGES" | grep -E '\.rs$|core/rust/' || true)
PYTHON_CHANGES=$(echo "$CODE_CHANGES" | grep -E '\.py$|research/python/' || true)
# CONFIG_CHANGES: Configuration files including workflow files
# Workflow files are treated as significant configuration and handled separately
# to ensure they're always included in configuration checks
CONFIG_CHANGES=$(echo "$CODE_CHANGES" | grep -E '\.(toml|yaml|yml|json)$' || true)
WORKFLOW_CHANGES=$(echo "$CHANGED_FILES" | grep -E '\.github/workflows/.*\.(yml|yaml)$' || true)
if [ -n "$WORKFLOW_CHANGES" ]; then
    CONFIG_CHANGES="${CONFIG_CHANGES:+$CONFIG_CHANGES$'\n'}$WORKFLOW_CHANGES"
fi

# Check 1: Core Rust changes should have documentation updates
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Check 1: Rust core changes have documentation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ -n "$RUST_CHANGES" ]; then
    echo "Rust code changes detected:"
    echo "$RUST_CHANGES" | sed 's/^/  /'
    echo ""
    
    # Check if docs or README were updated
    if [ -z "$DOC_CHANGES" ]; then
        echo -e "${RED}❌ FAIL: Rust code changes without documentation updates${NC}"
        echo ""
        echo "Rust production code changes must be documented."
        echo "Expected updates in:"
        echo "  - docs/02-system-architecture/ (for architectural changes)"
        echo "  - docs/05-intent-engine/ (for core logic changes)"
        echo "  - core/rust/README.md (for module-level changes)"
        echo "  - Inline code documentation"
        echo ""
        EXIT_CODE=1
    else
        # Check if documentation is relevant to the changes
        ARCH_DOCS=$(echo "$DOC_CHANGES" | grep -E 'docs/02-system-architecture/|docs/05-intent-engine/|core/rust/README.md' || true)
        
        if [ -n "$ARCH_DOCS" ]; then
            echo -e "${GREEN}✅ Documentation updated for Rust changes${NC}"
            echo "Updated documentation:"
            echo "$ARCH_DOCS" | sed 's/^/  /'
        else
            echo -e "${YELLOW}⚠️  WARNING: Rust changes detected but no architecture/core docs updated${NC}"
            echo ""
            echo "Consider updating:"
            echo "  - docs/02-system-architecture/"
            echo "  - docs/05-intent-engine/"
            echo "  - core/rust/README.md"
            WARNINGS=$((WARNINGS + 1))
        fi
    fi
else
    echo -e "${BLUE}ℹ️  No Rust core changes${NC}"
fi
echo ""

# Check 2: Research Python changes should have documentation
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Check 2: Python research changes have documentation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ -n "$PYTHON_CHANGES" ]; then
    echo "Python code changes detected:"
    echo "$PYTHON_CHANGES" | sed 's/^/  /'
    echo ""
    
    # Check for experiment files or notebooks
    EXPERIMENT_CHANGES=$(echo "$PYTHON_CHANGES" | grep -E 'experiments/|notebooks/' || true)
    
    if [ -n "$EXPERIMENT_CHANGES" ]; then
        # Experiments should follow the research framework
        RESEARCH_DOCS=$(echo "$DOC_CHANGES" | grep -E 'docs/06-research-framework/|research/python/README|issues/' || true)
        
        if [ -z "$RESEARCH_DOCS" ]; then
            echo -e "${YELLOW}⚠️  WARNING: Research/experiment changes without documentation${NC}"
            echo ""
            echo "Consider documenting:"
            echo "  - Experiment goals in docs/06-research-framework/"
            echo "  - Methodology in research/python/README.md"
            echo "  - Issue tracking in issues/"
            WARNINGS=$((WARNINGS + 1))
        else
            echo -e "${GREEN}✅ Research documentation updated${NC}"
            echo "Updated documentation:"
            echo "$RESEARCH_DOCS" | sed 's/^/  /'
        fi
    else
        # Regular Python code changes
        if [ -z "$DOC_CHANGES" ]; then
            echo -e "${YELLOW}⚠️  WARNING: Python changes without documentation updates${NC}"
            echo ""
            echo "Consider updating research/python/README.md or relevant docs"
            WARNINGS=$((WARNINGS + 1))
        else
            echo -e "${GREEN}✅ Documentation files updated${NC}"
        fi
    fi
else
    echo -e "${BLUE}ℹ️  No Python research changes${NC}"
fi
echo ""

# Check 3: Significant configuration changes should be documented
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Check 3: Configuration changes have documentation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ -n "$CONFIG_CHANGES" ]; then
    # Filter out minor config files - only look for significant configuration
    SIGNIFICANT_CONFIG=$(echo "$CONFIG_CHANGES" | grep -E 'Cargo\.toml|pyproject\.toml|\.github/workflows/.*\.(yml|yaml)$' || true)
    
    if [ -n "$SIGNIFICANT_CONFIG" ]; then
        echo "Significant configuration changes detected:"
        echo "$SIGNIFICANT_CONFIG" | sed 's/^/  /'
        echo ""
        
        # Check for decision log or architecture docs
        DECISION_DOCS=$(echo "$DOC_CHANGES" | grep -E 'docs/11-decision-log/|docs/02-system-architecture/|README.md' || true)
        
        if [ -z "$DECISION_DOCS" ]; then
            echo -e "${YELLOW}⚠️  WARNING: Configuration changes without decision log${NC}"
            echo ""
            echo "Consider documenting in:"
            echo "  - docs/11-decision-log/ (for architectural decisions)"
            echo "  - README.md (for setup changes)"
            WARNINGS=$((WARNINGS + 1))
        else
            echo -e "${GREEN}✅ Configuration documented${NC}"
            echo "Updated documentation:"
            echo "$DECISION_DOCS" | sed 's/^/  /'
        fi
    else
        echo -e "${BLUE}ℹ️  Minor configuration changes (no documentation required)${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  No configuration changes${NC}"
fi
echo ""

# Check 4: New features or significant additions require comprehensive docs
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Check 4: New files/features have documentation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

NEW_FILES=$(git diff --name-status "$BASE_REF" 2>/dev/null | grep "^A" | awk '{print $2}' || echo "")
NEW_CODE_FILES=$(echo "$NEW_FILES" | grep -E '\.(rs|py)$' | grep -v -E '^(tests/|test_)' || true)

if [ -n "$NEW_CODE_FILES" ]; then
    NEW_FILE_COUNT=$(echo "$NEW_CODE_FILES" | wc -l)
    echo "New code files detected: $NEW_FILE_COUNT"
    echo "$NEW_CODE_FILES" | sed 's/^/  /'
    echo ""
    
    # For new files, documentation is strongly recommended
    if [ -z "$DOC_CHANGES" ]; then
        echo -e "${RED}❌ FAIL: New code files added without documentation${NC}"
        echo ""
        echo "New features must be documented."
        echo "Required documentation:"
        echo "  - What the feature does"
        echo "  - How it fits into the system"
        echo "  - Usage examples"
        echo "  - Design decisions"
        echo ""
        EXIT_CODE=1
    else
        echo -e "${GREEN}✅ New features documented${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  No new code files${NC}"
fi
echo ""

# Check 5: Only documentation changes (informational)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✓ Check 5: Change summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ -z "$CODE_CHANGES" ] && [ -n "$DOC_CHANGES" ]; then
    echo -e "${GREEN}✅ Documentation-only changes${NC}"
    echo ""
    echo "This PR only updates documentation."
    echo "No code validation required."
elif [ -n "$CODE_CHANGES" ] && [ -n "$DOC_CHANGES" ]; then
    echo -e "${GREEN}✅ Code and documentation both updated${NC}"
    echo ""
    echo "Code changes: $(echo "$CODE_CHANGES" | wc -l) files"
    echo "Documentation changes: $(echo "$DOC_CHANGES" | wc -l) files"
elif [ -n "$CODE_CHANGES" ] && [ -z "$DOC_CHANGES" ]; then
    echo -e "${YELLOW}⚠️  Code changes without documentation updates${NC}"
    echo ""
    echo "Code changes: $(echo "$CODE_CHANGES" | wc -l) files"
    echo "Documentation changes: 0 files"
else
    echo -e "${BLUE}ℹ️  No significant changes detected${NC}"
fi
echo ""

# Final summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 VALIDATION SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Errors:   $((EXIT_CODE))"
echo "Warnings: $WARNINGS"
echo ""

if [ $EXIT_CODE -eq 0 ]; then
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}⚠️  VALIDATION PASSED WITH WARNINGS${NC}"
        echo ""
        echo "Documentation requirements met, but consider addressing warnings."
        echo ""
        echo "Warnings indicate that while not strictly required, additional"
        echo "documentation would improve code maintainability and clarity."
    else
        echo -e "${GREEN}✅ VALIDATION PASSED${NC}"
        echo ""
        echo "All documentation requirements satisfied."
    fi
else
    echo -e "${RED}❌ VALIDATION FAILED${NC}"
    echo ""
    echo "Documentation updates are REQUIRED for these changes."
    echo ""
    echo "Why documentation matters:"
    echo "  - Galactus is a complex inference system"
    echo "  - Explainability is a core principle"
    echo "  - Future maintainers need context"
    echo "  - Design decisions must be recorded"
    echo ""
    echo "Where to document:"
    echo "  - System architecture: docs/02-system-architecture/"
    echo "  - Intent engine logic: docs/05-intent-engine/"
    echo "  - Research methodology: docs/06-research-framework/"
    echo "  - Design decisions: docs/11-decision-log/"
    echo "  - Component README files"
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE
