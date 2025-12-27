# Promotion Checklists Directory

This directory contains **machine-readable promotion checklists** for signals and features being promoted from research (Python) to production (Rust core).

## Purpose

Every promotion from research to the Intent Engine must have a completed checklist file in this directory. This is a **hard requirement** enforced by CI/CD.

## Files in This Directory

- **`TEMPLATE-promotion-checklist.yml`** — Template for creating new promotion checklists
- **`PROMOTION-REGISTRY.md`** — Track all active, completed, and rejected promotions
- **`YYYY-MM-DD-signal-name.yml`** — Individual promotion checklists (one per promotion)

## How to Use

### 1. Starting a New Promotion

```bash
# Copy the template
cp research/python/promotions/TEMPLATE-promotion-checklist.yml \
   research/python/promotions/2024-12-27-my-signal.yml

# Edit the file and fill all sections
vim research/python/promotions/2024-12-27-my-signal.yml
```

### 2. Completing the Checklist

- Mark each item as `complete`, `incomplete`, or `not_applicable`
- Provide evidence paths for all `complete` items
- Provide justification in notes for all `not_applicable` items
- Create all required artifacts (notebooks, docs, tests, etc.)
- List all artifacts in the checklist file

### 3. Requesting Reviews

- Fill in reviewer names
- Request reviews from all four required reviewers:
  - Research lead
  - Architecture reviewer
  - Security reviewer
  - Final approval (maintainer)

### 4. Validation

Run local validation before pushing:

```bash
./scripts/validate_checklist.sh
```

If validation passes (exit code 0), you can push to your PR branch.

The CI/CD pipeline will run this validation automatically and **block merge** if it fails.

### 5. After Approval

Once all reviews are approved and validation passes:

1. Update the `PROMOTION-REGISTRY.md` file
2. Begin Rust implementation
3. Update checklist progress as you go

## Validation Rules

The validation script (`scripts/validate_checklist.sh`) enforces:

- ✅ All required fields present
- ✅ No `incomplete` checklist items
- ✅ Evidence provided for all `complete` items
- ✅ Justification provided for all `not_applicable` items
- ✅ All artifact files exist
- ✅ Decision log entry added
- ✅ All four reviews approved
- ✅ Status consistency

**If any rule fails, promotion is BLOCKED.**

## Common Errors

### Error: Incomplete checklist items

```
❌ FAIL: Found 20 incomplete checklist item(s)
```

**Fix:** Complete all items by marking them as `complete` with evidence or `not_applicable` with justification.

### Error: Missing artifacts

```
❌ FAIL: No artifacts listed
```

**Fix:** Add at least one artifact in each required category:
- research_notebooks
- documentation
- backtest_results
- implementation_design
- tests

### Error: Missing decision log entry

```
❌ FAIL: Decision log entry not added
```

**Fix:** 
1. Add an entry to `docs/11-decision-log/decision-log.md`
2. Set `entry_added: true` in the checklist file
3. Ensure the decision log file path is correct

### Error: Pending reviews

```
⚠️  WARNING: 4 review(s) still pending
```

**Fix:** Obtain approvals from all four reviewers. Update their status to `approved` in the checklist file.

## Documentation

See complete documentation:

- **Format Specification**: [`docs/06-research-framework/promotion-checklist-format.md`](../../docs/06-research-framework/promotion-checklist-format.md)
- **Requirements**: [`docs/06-research-framework/promotion-checklist.md`](../../docs/06-research-framework/promotion-checklist.md)
- **Boundary Enforcement**: [`docs/02-system-architecture/rust-python-boundary-enforcement.md`](../../docs/02-system-architecture/rust-python-boundary-enforcement.md)

## Why This Exists

This hard gating mechanism ensures:

- **Quality**: Only validated signals enter production
- **Accountability**: Clear ownership and review trail
- **Reproducibility**: All evidence and artifacts preserved
- **Discipline**: No shortcuts or "temporary" promotions
- **Trust**: Galactus maintains its promise of correctness

**The gate is absolute. There are no exceptions.**

Galactus promotes slowly so it can remain trusted forever.
