# Promotion Checklist Format Specification

## Purpose

This document defines the **machine-readable format** for promotion checklists in Project Galactus.

All signals, features, and logic promoted from research (Python) to production (Rust) **must** include a completed promotion checklist file that passes automated validation.

---

## File Location and Naming

### Location
Promotion checklist files must be placed in:
```
research/python/promotions/YYYY-MM-DD-signal-name.yml
```

### Naming Convention
- `YYYY-MM-DD`: Date when promotion process started
- `signal-name`: Descriptive name of the signal/feature (lowercase, hyphens)

**Examples:**
- `research/python/promotions/2024-03-15-forced-expiry-pressure.yml`
- `research/python/promotions/2024-04-22-regime-classifier-v2.yml`

---

## YAML Schema

### Required Structure

```yaml
version: "1.0"
promotion:
  signal_name: "Signal or Feature Name"
  description: "Brief description of what is being promoted"
  date_initiated: "YYYY-MM-DD"
  owner: "Owner Name or GitHub Handle"
  target_version: "Rust Core Version (e.g., v0.2.0)"
  
checklist:
  # Section 1: Structural Validity (Non-Negotiable)
  structural_validity:
    capital_behavior_mapping:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to documentation explaining capital behavior"
      notes: "Optional: Additional context"
    market_structure_alignment:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to market theory alignment documentation"
      notes: "Optional: Additional context"
  
  # Section 2: Determinism & Reproducibility
  determinism:
    deterministic_definition:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to mathematical/logical definition"
      notes: "Optional: Additional context"
    replayability:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to replay test results"
      notes: "Optional: Additional context"
  
  # Section 3: Data Discipline
  data_discipline:
    data_source_compliance:
      status: "complete|incomplete|not_applicable"
      evidence: "List of data sources used (all must be approved)"
      notes: "Optional: Additional context"
    schema_compliance:
      status: "complete|incomplete|not_applicable"
      evidence: "Schema version and compliance verification"
      notes: "Optional: Additional context"
  
  # Section 4: Normalization & Scaling
  normalization:
    normalization_justification:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to normalization documentation"
      notes: "Optional: Additional context"
    cross_instrument_validity:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to cross-instrument validation results"
      notes: "Optional: Additional context"
  
  # Section 5: Regime Awareness
  regime_awareness:
    regime_conditioning:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to regime documentation"
      notes: "Optional: Additional context"
    regime_failure_handling:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to failure handling logic"
      notes: "Optional: Additional context"
  
  # Section 6: Failure Analysis (Mandatory)
  failure_analysis:
    known_failure_modes:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to failure modes documentation (min 3 scenarios)"
      notes: "Optional: Additional context"
    historical_failure_evidence:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to historical failure analysis"
      notes: "Optional: Additional context"
  
  # Section 7: Backtesting Integrity
  backtesting:
    backtest_framing:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to backtest results (event-time aligned)"
      notes: "Optional: Additional context"
    stability_evidence:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to stability analysis across regimes"
      notes: "Optional: Additional context"
  
  # Section 8: Interpretability & Explainability
  interpretability:
    explanation_test:
      status: "complete|incomplete|not_applicable"
      evidence: "Example explanations for signal activations"
      notes: "Optional: Additional context"
    component_transparency:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to component documentation"
      notes: "Optional: Additional context"
  
  # Section 9: Confidence & Stability Integration
  confidence:
    confidence_logic:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to confidence calculation documentation"
      notes: "Optional: Additional context"
    stability_behavior:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to stability behavior documentation"
      notes: "Optional: Additional context"
  
  # Section 10: Rust Readiness (Final Gate)
  rust_readiness:
    rust_implementability:
      status: "complete|incomplete|not_applicable"
      evidence: "Rust implementation design document path"
      notes: "Optional: Additional context"
    testing_requirements:
      status: "complete|incomplete|not_applicable"
      evidence: "Path to test specifications and golden cases"
      notes: "Optional: Additional context"

artifacts:
  research_notebooks:
    - "Path to primary research notebook"
    - "Path to validation notebook"
  documentation:
    - "Path to signal specification document"
    - "Path to failure modes documentation"
  backtest_results:
    - "Path to backtest results"
  implementation_design:
    - "Path to Rust implementation design"
  tests:
    - "Path to test case definitions"

decision_log:
  entry_added: true|false
  entry_path: "docs/11-decision-log/decision-log.md"
  entry_date: "YYYY-MM-DD"

reviews:
  research_lead:
    reviewer: "Reviewer Name"
    status: "approved|pending|rejected"
    date: "YYYY-MM-DD"
    notes: "Optional review notes"
  architecture_reviewer:
    reviewer: "Reviewer Name"
    status: "approved|pending|rejected"
    date: "YYYY-MM-DD"
    notes: "Optional review notes"
  security_reviewer:
    reviewer: "Reviewer Name"
    status: "approved|pending|rejected"
    date: "YYYY-MM-DD"
    notes: "Optional review notes"
  final_approval:
    reviewer: "Maintainer Name"
    status: "approved|pending|rejected"
    date: "YYYY-MM-DD"
    notes: "Optional approval notes"

promotion_status:
  overall_status: "pending|approved|rejected|in_progress"
  rust_implementation_started: true|false
  rust_implementation_completed: true|false
  tests_passing: true|false
  documentation_complete: true|false
  deployment_ready: true|false
```

---

## Field Definitions

### Status Values

Each checklist item must have one of these statuses:

- **`complete`**: Item fully satisfied with evidence provided
- **`incomplete`**: Item not yet satisfied (blocks promotion)
- **`not_applicable`**: Item doesn't apply to this specific promotion (must justify in notes)

### Evidence Requirements

Every `complete` checklist item must include:
- Path to documentation or artifact
- Must be verifiable (file exists and contains required information)
- Must be accessible in the repository

### Review Status Values

- **`pending`**: Awaiting review
- **`approved`**: Reviewer approved
- **`rejected`**: Reviewer rejected with notes

### Overall Promotion Status

- **`pending`**: Initial state, checklist being filled
- **`in_progress`**: Active promotion in progress, checklist complete
- **`approved`**: All reviewers approved, ready for implementation
- **`rejected`**: Promotion rejected, return to research

---

## Validation Rules

### Mandatory Requirements

1. **All Non-Applicable Items Must Be Justified**
   - If status is `not_applicable`, `notes` field must explain why

2. **All Complete Items Must Have Evidence**
   - Evidence paths must point to existing files
   - Evidence must be substantive (not placeholder)

3. **All Incomplete Items Block Promotion**
   - No incomplete items allowed in approved promotions
   - Must transition to `complete` or `not_applicable`

4. **Minimum Artifact Requirements**
   - At least 1 research notebook
   - At least 1 documentation file
   - At least 1 backtest result
   - Rust implementation design document
   - Test specifications

5. **Review Requirements**
   - All four reviewers must approve (research_lead, architecture_reviewer, security_reviewer, final_approval)
   - Review dates must be present for approved reviews
   - Rejected reviews must include notes explaining why

6. **Decision Log Requirement**
   - `entry_added` must be `true`
   - `entry_path` must point to decision log file
   - Entry must exist in the decision log

---

## Automated Validation

The `scripts/validate_checklist.sh` script performs the following checks:

1. **Schema Validation**
   - YAML is well-formed
   - All required fields present
   - Field values are valid

2. **Evidence Verification**
   - All evidence paths point to existing files
   - Evidence files are not empty

3. **Completeness Check**
   - No `incomplete` items remain
   - All `not_applicable` items have justification

4. **Review Status Check**
   - All required approvals obtained
   - Review dates are valid and sequential

5. **Artifact Verification**
   - All listed artifacts exist
   - Artifacts meet minimum requirements

6. **Decision Log Verification**
   - Entry exists in decision log
   - Entry date matches promotion date

---

## Example: Minimal Valid Checklist

```yaml
version: "1.0"
promotion:
  signal_name: "Forced Expiry Pressure Signal"
  description: "Measures capital pressure from derivative expiry constraints"
  date_initiated: "2024-03-15"
  owner: "research-team"
  target_version: "v0.2.0"
  
checklist:
  structural_validity:
    capital_behavior_mapping:
      status: "complete"
      evidence: "research/python/notebooks/forced-expiry-analysis.ipynb"
      notes: "Documented in Section 3 of notebook"
    market_structure_alignment:
      status: "complete"
      evidence: "docs/01-market-theory/derivatives-dominance.md"
      notes: "Aligns with derivatives dominance theory"
  
  # ... (other sections following same pattern)
  
artifacts:
  research_notebooks:
    - "research/python/notebooks/forced-expiry-analysis.ipynb"
  documentation:
    - "docs/05-intent-engine/forced-expiry-pressure-spec.md"
  backtest_results:
    - "research/python/results/forced-expiry-backtest-2024-03.csv"
  implementation_design:
    - "core/rust/docs/forced-expiry-implementation-design.md"
  tests:
    - "core/rust/tests/test_forced_expiry_pressure.rs"

decision_log:
  entry_added: true
  entry_path: "docs/11-decision-log/decision-log.md"
  entry_date: "2024-03-20"

reviews:
  research_lead:
    reviewer: "Alice"
    status: "approved"
    date: "2024-03-18"
  architecture_reviewer:
    reviewer: "Bob"
    status: "approved"
    date: "2024-03-19"
  security_reviewer:
    reviewer: "Charlie"
    status: "approved"
    date: "2024-03-19"
  final_approval:
    reviewer: "Maintainer"
    status: "approved"
    date: "2024-03-20"

promotion_status:
  overall_status: "approved"
  rust_implementation_started: false
  rust_implementation_completed: false
  tests_passing: false
  documentation_complete: true
  deployment_ready: false
```

---

## Usage in Promotion Process

1. **Initiate Promotion**
   - Create new promotion checklist file in `research/python/promotions/`
   - Copy template and fill initial information
   - Set all checklist items to `incomplete`

2. **Complete Checklist Items**
   - Work through each section systematically
   - Update status to `complete` as evidence is created
   - Provide clear evidence paths and notes

3. **Request Reviews**
   - Update promotion status to `pending`
   - Notify reviewers via PR
   - Reviewers update their sections

4. **Automated Validation**
   - CI/CD runs `validate_checklist.sh` on every push
   - Validation must pass before merge is allowed
   - Fix any validation errors

5. **Approval and Implementation**
   - After all approvals, status moves to `approved`
   - Begin Rust implementation
   - Update progress in checklist file

---

## Anti-Patterns (Forbidden)

❌ **Placeholder Evidence**
```yaml
evidence: "TODO: Add evidence later"
evidence: "See notebook (not specified)"
```

❌ **Missing Justification for Not Applicable**
```yaml
status: "not_applicable"
notes: ""  # INVALID - must justify
```

❌ **Incomplete Items in Approved Promotion**
```yaml
promotion_status:
  overall_status: "approved"
checklist:
  structural_validity:
    capital_behavior_mapping:
      status: "incomplete"  # INVALID - blocks approval
```

❌ **Missing Required Artifacts**
```yaml
artifacts:
  research_notebooks: []  # INVALID - at least 1 required
```

❌ **Unapproved Promotion**
```yaml
reviews:
  final_approval:
    status: "pending"  # INVALID - must be approved
promotion_status:
  overall_status: "approved"  # INVALID - inconsistent
```

---

## Related Documents

- [`promotion-checklist.md`](promotion-checklist.md) — Human-readable promotion requirements
- [`research-methodology.md`](research-methodology.md) — Research discipline and validation
- [`signal-lifecycle.md`](../04-signal-and-metrics/signal-lifecycle.md) — Signal lifecycle stages
- [`rust-python-boundary-enforcement.md`](../02-system-architecture/rust-python-boundary-enforcement.md) — Boundary enforcement mechanisms

---

## Final Statement

**The machine-readable checklist is not optional.**

Every promotion must have a valid, complete checklist file.

This is how Galactus maintains quality and prevents premature productionization.
