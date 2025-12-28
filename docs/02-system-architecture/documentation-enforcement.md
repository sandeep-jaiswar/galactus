# Documentation Enforcement Policy

## Overview

Galactus enforces documentation updates alongside code changes through automated CI checks. This ensures that the system remains explainable and maintainable over time.

## Philosophy

Galactus is built on the principle that **explainability is mandatory**. Every component, signal, and decision must be documentable and understandable. This philosophy extends to the development process itself:

- Code without documentation is incomplete
- Design decisions must be recorded
- Future maintainers need context
- Market structure reasoning must be preserved

## Enforcement Mechanism

### CI Validation

The `validate_docs_updates.sh` script runs automatically on every pull request to the production branch. It performs the following checks:

#### 1. Rust Core Changes

When Rust code in `core/rust/` is modified, documentation must be updated in one or more of:

- `docs/02-system-architecture/` — for architectural changes
- `docs/05-intent-engine/` — for core inference logic changes
- `core/rust/README.md` — for module-level changes
- Inline code documentation (doc comments)

**Enforcement Level**: HARD GATE (blocks merge)

#### 2. Python Research Changes

When Python code in `research/python/` is modified, documentation should be updated in:

- `docs/06-research-framework/` — for experimental methodologies
- `research/python/README.md` — for research module changes
- `issues/` — for tracking experiment progress

**Enforcement Level**: WARNING (does not block, but strongly recommended)

#### 3. Configuration Changes

When significant configuration files are modified (`Cargo.toml`, `pyproject.toml`, workflow files), documentation should be updated in:

- `docs/11-decision-log/` — for architectural decisions
- `README.md` — for setup or build process changes

**Enforcement Level**: WARNING (does not block, but strongly recommended)

#### 4. New Features

When new code files are added (not tests), documentation must be updated to explain:

- What the feature does
- How it fits into the system architecture
- Usage examples
- Design decisions and trade-offs

**Enforcement Level**: HARD GATE (blocks merge)

## What Counts as "Documentation"?

### Acceptable Documentation Updates

1. **Architecture Documents** — Explaining system design, component interactions, boundaries
2. **Module README Files** — Describing purpose, API, usage of specific modules
3. **Decision Logs** — Recording why certain choices were made
4. **Research Framework Docs** — Methodology, experiment design, validation criteria
5. **Inline Code Comments** — For complex logic or non-obvious implementations

### Not Sufficient

- Only updating changelog or version numbers
- Minor typo fixes unrelated to the code changes
- Auto-generated documentation without meaningful content

## Bypass Mechanisms

### Documentation-Only Changes

If a pull request only modifies documentation files with no code changes, the validation automatically passes with a success message.

### Exempted Changes

The following types of changes do not trigger documentation requirements:

- Test files (`tests/`, `test_*.py`, `*_test.rs`)
- CI/CD validation scripts (`scripts/validate_*`)
- GitHub workflow configuration (`.github/workflows/`)
- Build artifacts and dependencies

### Override (Emergency Only)

In rare cases where immediate merge is critical (e.g., security hotfix), the documentation requirement can be overridden by:

1. Documenting the reason in the PR description
2. Creating a follow-up issue to add documentation
3. Getting explicit approval from architecture reviewer

This should be extremely rare and requires post-merge documentation within 24 hours.

## Best Practices

### When to Document

Document as you code, not after. This ensures:

- Design decisions are captured while fresh
- Reasoning is accurate and complete
- Context is preserved for code reviews

### Where to Document

Choose the appropriate documentation location:

- **High-level architecture** → `docs/02-system-architecture/`
- **Core inference logic** → `docs/05-intent-engine/`
- **Research methodology** → `docs/06-research-framework/`
- **Design decisions** → `docs/11-decision-log/`
- **Module-specific** → Component README files
- **Implementation details** → Inline code comments

### What to Document

Focus on the "why" and "what", not just the "how":

- **Why** was this approach chosen?
- **What** problem does it solve?
- **What** trade-offs were considered?
- **What** are the limitations?
- **How** does it fit into the larger system?

### Documentation Smells

Watch out for these signs of inadequate documentation:

- ❌ "Updated code" without explanation
- ❌ Documentation that just repeats the code
- ❌ Vague statements without specifics
- ❌ Missing context or reasoning
- ❌ No explanation of design trade-offs

Good documentation:

- ✅ Explains the problem being solved
- ✅ Provides context for design decisions
- ✅ Describes trade-offs and alternatives considered
- ✅ Links to related components or documents
- ✅ Includes examples where appropriate

## Examples

### Example 1: Adding a New Signal

**Code Change**: New Rust module `core/rust/src/signals/basis_pressure.rs`

**Required Documentation**:
- `docs/04-signal-and-metrics/basis-pressure.md` — Signal definition and market theory
- `docs/05-intent-engine/signal-integration.md` — How the signal integrates with intent engine
- `core/rust/src/signals/README.md` — Update with new signal description
- Decision log entry explaining why this signal was promoted from research

### Example 2: Refactoring Python Research Code

**Code Change**: Restructure `research/python/src/experiments/`

**Required Documentation**:
- `research/python/README.md` — Update with new structure
- May not need formal docs if purely internal restructuring

**Result**: WARNING (not blocked, but recommended)

### Example 3: Updating Cargo.toml Dependencies

**Code Change**: Bump dependency version in `core/rust/Cargo.toml`

**Required Documentation**:
- `docs/11-decision-log/` — Entry explaining why dependency was updated
- Note any breaking changes or compatibility considerations

**Result**: WARNING (not blocked, but strongly recommended)

## Validation Script Details

### Location

`scripts/validate_docs_updates.sh`

### How It Works

1. Compares current branch against base branch (typically `production`)
2. Identifies changed files by type (Rust, Python, config, docs)
3. Applies enforcement rules based on change type
4. Reports errors (blocking) and warnings (non-blocking)
5. Provides guidance on where to add documentation

### Running Locally

```bash
# From repository root
./scripts/validate_docs_updates.sh
```

The script will analyze your current changes and report any documentation requirements.

### CI Integration

The validation runs automatically on pull requests via the `enforce-documentation` job in `.github/workflows/boundary-enforcement.yml`.

## Rationale

### Why Enforce Documentation?

1. **Explainability is Core** — Galactus exists to provide explainable capital behavior inferences
2. **System Complexity** — The inference engine is complex and requires thorough documentation
3. **Long-term Maintenance** — Future developers need to understand design decisions
4. **Knowledge Preservation** — Market structure reasoning must be captured
5. **Quality Control** — Documentation forces clarity of thought during development

### Why Automated Enforcement?

1. **Consistency** — Manual review can miss documentation gaps
2. **Early Feedback** — Catch documentation issues during development, not after merge
3. **Cultural Norm** — Makes documentation a first-class requirement
4. **Reduced Review Burden** — Automated check frees reviewers to focus on content quality

## Related Documents

- [System Architecture Overview](./README.md)
- [Design Principles](../00-vision-and-non-goals/design-principles.md)
- [Decision Log Template](../11-decision-log/README.md)
- [Research Framework](../06-research-framework/README.md)

## Revision History

- **2025-12-28**: Initial documentation enforcement policy established
