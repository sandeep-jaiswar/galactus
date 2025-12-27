# Rust vs Python Boundary Enforcement

**This document defines concrete enforcement mechanisms for maintaining the strict separation between Rust (production) and Python (research).**

---

## Purpose

This document operationalizes the [Rust vs Python Contract](rust-vs-python-contract.md) with specific, actionable enforcement mechanisms.

It exists to:
- Prevent accidental boundary violations
- Catch research leakage into production
- Automate boundary validation where possible
- Provide clear remediation paths

---

## Enforcement Philosophy

**Prevention is better than detection.**

We enforce boundaries through:
1. **Physical separation** — Directory structure
2. **Process gates** — Promotion checklist
3. **Automated checks** — CI/CD validation
4. **Code review** — Human verification
5. **Cultural norms** — Shared understanding

No single mechanism is sufficient. Layered enforcement is required.

---

## Layer 1: Physical Separation

### Directory Structure

```
galactus/
├── core/rust/              # Production code ONLY
│   ├── src/                # Rust source (deterministic)
│   ├── tests/              # Rust tests (comprehensive)
│   └── Cargo.toml          # Rust dependencies
│
└── research/python/        # Research code ONLY
    ├── notebooks/          # Jupyter notebooks
    ├── src/                # Python prototypes
    ├── tests/              # Python sanity checks
    └── pyproject.toml      # Python dependencies
```

### Rules

1. **No Python in `core/rust/`** (except FFI bindings if needed)
2. **No Rust in `research/python/`** (except calling compiled binaries)
3. **No shared code directories**
4. **No symbolic links between layers**

### Rationale

Physical separation makes violations obvious and hard to do accidentally.

---

## Layer 2: Code Organization Standards

### Rust Code Standards (Production)

#### File Naming
- Use descriptive module names: `capital_pressure.rs`, `regime_classifier.rs`
- No generic names: `utils.rs`, `helpers.rs`, `common.rs`
- Group by domain, not by technical layer

#### Code Characteristics
- ✅ Pure functions (no side effects)
- ✅ Explicit error handling
- ✅ Comprehensive documentation
- ✅ Full test coverage
- ❌ Global mutable state
- ❌ Hidden randomness
- ❌ Undocumented assumptions

#### Dependency Rules
```toml
# In Cargo.toml, every dependency must be justified

[dependencies]
# Data processing
serde = { version = "1.0", features = ["derive"] }  # Serialization (required for events)
polars = "0.35"  # Data frames (approved for feature computation)

# Forbidden patterns:
# - Machine learning training frameworks (inference only)
# - Python interpreters (except optional bindings)
# - Non-deterministic libraries
```

### Python Code Standards (Research)

#### File Organization
```
research/python/
├── notebooks/
│   ├── exploratory/YYYY-MM-DD-topic.ipynb
│   ├── validation/validate-signal-name.ipynb
│   └── backtesting/backtest-signal-name.ipynb
├── src/
│   └── features/signal_name_prototype.py
```

#### Code Characteristics
- Exploratory and iterative
- Visualization-heavy
- Rapid prototyping
- Allowed to be messy
- No production inference logic

#### Dependency Rules
```toml
# In pyproject.toml
[tool.poetry.dependencies]
python = "^3.10"
pandas = "^2.0"
matplotlib = "^3.7"
jupyter = "^1.0"

# Forbidden:
# - Production inference libraries (stay in Rust)
# - Broker APIs (execution is out of scope)
# - Proprietary data libraries
```

---

## Layer 3: Promotion Process Gate

### Promotion Checklist Validation

Before any logic moves from Python to Rust:

#### Mandatory Artifacts
1. ✅ **Completed promotion checklist** ([link](../../06-research-framework/promotion-checklist.md))
2. ✅ **Economic rationale document** (Why this signal?)
3. ✅ **Empirical validation report** (Backtests across regimes)
4. ✅ **Failure mode documentation** (Known ways it breaks)
5. ✅ **Rust implementation specification** (Interface and tests)
6. ✅ **Decision log entry** (Documented approval)

#### Review Process
1. Research lead validates methodology
2. Architecture reviewer checks contract compliance
3. Security reviewer checks for vulnerabilities
4. Final approval from maintainer

If any artifact is missing → **Promotion blocked**

### Promotion Workflow

```
[Python Research]
      ↓
[Hypothesis Validated?] → No → Back to Research
      ↓ Yes
[Promotion Checklist Complete?] → No → Block
      ↓ Yes
[Documentation Updated?] → No → Block
      ↓ Yes
[Rust Spec Defined?] → No → Block
      ↓ Yes
[Implementation Review] → Fail → Block
      ↓ Pass
[Rust Production]
```

### Enforcement

- No direct commits from research to production
- All promotions tracked in decision log
- Checklist must be in pull request description
- Missing artifacts automatically fail CI

---

## Layer 4: Automated CI/CD Checks

### Pre-Commit Checks

```bash
# Check 1: No Python in core/rust/ (except bindings)
find core/rust/src -name "*.py" | grep -v "bindings" && echo "FAIL: Python in Rust" && exit 1

# Check 2: No Rust in research/python/ (except via FFI)
find research/python -name "*.rs" && echo "FAIL: Rust in Python" && exit 1

# Check 3: Rust code has tests
./scripts/check_rust_coverage.sh || exit 1

# Check 4: Promotion checklist reference in PR
if [[ -n "$PROMOTION_PR" ]]; then
    grep -q "promotion-checklist.md" PR_DESCRIPTION || echo "FAIL: Missing checklist" && exit 1
fi
```

### CI Pipeline Stages

#### Stage 1: Boundary Validation
- Verify directory structure integrity
- Check for cross-contamination
- Validate file naming conventions

#### Stage 2: Rust Production Checks
- Determinism validation (no rand without seed)
- Test coverage check (must be >90%)
- Documentation check (all pub items documented)
- Dependency audit (no forbidden crates)

#### Stage 3: Python Research Checks
- Sanity tests pass
- No production inference logic
- No broker/execution imports

#### Stage 4: Integration Validation
- Rust binaries can be called from Python
- Outputs match expected schemas
- No shared mutable state

### Automated Rejection Criteria

Automatic CI failure if:
- Python code in `core/rust/src/`
- Rust production logic in `research/python/`
- Promotion PR without checklist reference
- Test coverage below threshold
- Undocumented public APIs in Rust
- Forbidden dependencies detected

---

## Layer 5: Code Review Checklist

### For Rust (Production) Changes

Reviewers must verify:

- [ ] Logic has been promoted via formal process (or is infrastructure)
- [ ] All code is deterministic (no hidden randomness)
- [ ] Full test coverage exists (unit + integration)
- [ ] Documentation is complete and accurate
- [ ] No shortcuts for "performance" that break determinism
- [ ] Error handling is explicit
- [ ] Configuration is versioned and explicit
- [ ] Aligns with design principles

### For Python (Research) Changes

Reviewers should check:

- [ ] Hypothesis is clearly stated
- [ ] Methodology is sound
- [ ] Failure modes are explored
- [ ] No production inference logic
- [ ] No attempts to bypass promotion process
- [ ] Results are presented honestly (not cherry-picked)

### Red Flags in Code Review

🚨 **Immediate rejection:**
- "We'll formalize this later"
- "Just a quick hack for now"
- "Copying this from research temporarily"
- "Too complex to test"
- "Works but I don't know why"

---

## Layer 6: Violation Detection and Response

### Detecting Violations

#### Symptoms of Boundary Violations

1. **Logic duplication** — Same algorithm in both Python and Rust
2. **Divergent outputs** — Python and Rust produce different results
3. **Production drift** — Rust outputs no longer match research prototype
4. **Hidden dependencies** — Rust depends on Python at runtime
5. **Research in prod** — Python notebooks calling live systems

### Response Protocol

#### Minor Violation (Accidental)
1. Identify the violation
2. Document in issue tracker
3. Create remediation plan
4. Update enforcement to prevent recurrence
5. Document as anti-pattern

#### Major Violation (Systemic)
1. Immediate halt of affected system
2. Root cause analysis
3. Full audit of boundary integrity
4. Mandatory training for team
5. Process improvement
6. Decision log documentation

### Remediation Paths

#### For Research in Production
1. **Immediate:** Remove from production
2. **Short-term:** Revert to last known good state
3. **Long-term:** Promote properly via checklist or deprecate

#### For Production Re-implemented in Research
1. **Immediate:** Delete duplicate code
2. **Short-term:** Call Rust binary instead
3. **Long-term:** Document proper integration pattern

---

## Layer 7: Cultural Enforcement

### Team Norms

#### Good Practices
- ✅ "Let's prototype this in Python first"
- ✅ "Before we promote, let's validate thoroughly"
- ✅ "The promotion checklist will help us catch issues"
- ✅ "Let's keep production boring and research experimental"

#### Bad Practices
- ❌ "We don't have time for the checklist"
- ❌ "Let's just copy this to Rust quickly"
- ❌ "We'll add tests later"
- ❌ "This works in Python, it'll work in Rust"

### Onboarding

All new team members must:
1. Read the [Rust vs Python Contract](rust-vs-python-contract.md)
2. Understand the promotion process
3. Complete a practice promotion (on existing signal)
4. Demonstrate boundary awareness

### Regular Audits

#### Quarterly Boundary Audit
- Review all recent promotions
- Check for unauthorized crossings
- Validate enforcement mechanisms
- Update documentation

---

## Enforcement Tooling

### Automated Scripts

#### `scripts/check_boundaries.sh`
```bash
#!/bin/bash
# Validate Rust/Python separation

echo "Checking boundary integrity..."

# Check for Python in Rust
py_in_rust=$(find core/rust/src -name "*.py" -not -path "*/bindings/*")
if [ -n "$py_in_rust" ]; then
    echo "ERROR: Python files in Rust core:"
    echo "$py_in_rust"
    exit 1
fi

# Check for Rust in Python
rs_in_python=$(find research/python -name "*.rs")
if [ -n "$rs_in_python" ]; then
    echo "ERROR: Rust files in Python research:"
    echo "$rs_in_python"
    exit 1
fi

echo "✓ Boundary integrity check passed"
```

#### `scripts/validate_promotion.sh`
```bash
#!/bin/bash
# Validate promotion checklist completeness

if [ -z "$PR_DESCRIPTION" ]; then
    echo "ERROR: No PR description found"
    exit 1
fi

# Check for promotion checklist reference
if ! echo "$PR_DESCRIPTION" | grep -q "promotion-checklist.md"; then
    echo "ERROR: Promotion must reference checklist"
    exit 1
fi

# Check for decision log entry
if ! git diff --name-only | grep -q "decision-log.md"; then
    echo "WARNING: Consider updating decision log"
fi

echo "✓ Promotion validation passed"
```

### Manual Audit Tools

#### Boundary Audit Report
```bash
# Generate boundary audit report
./scripts/audit_boundaries.sh > reports/boundary-audit-$(date +%Y-%m-%d).md
```

---

## Common Violations and Solutions

### Violation 1: "Quick Python Fix" in Production Path

**Symptom:** Python script called from Rust production code

**Problem:** Breaks determinism, introduces hidden dependencies

**Solution:**
1. Remove Python call immediately
2. Implement logic properly in Rust
3. Follow promotion process if it's new logic

### Violation 2: Rust Logic Duplicated in Python

**Symptom:** Same algorithm in both codebases

**Problem:** Creates divergence, maintenance burden

**Solution:**
1. Delete Python version
2. Call Rust binary from Python
3. Document proper integration pattern

### Violation 3: Research Notebook Using Production APIs

**Symptom:** Notebook calling live production services

**Problem:** Research may inadvertently affect production

**Solution:**
1. Use historical data instead
2. Call Rust in validation mode only
3. Never write to production state from research

### Violation 4: Promoted Without Checklist

**Symptom:** New Rust feature without promotion artifacts

**Problem:** Bypassed validation, unknown quality

**Solution:**
1. Stop deployment immediately
2. Complete checklist retroactively
3. If checklist fails, revert and re-research
4. Update CI to prevent future bypasses

---

## Metrics and Monitoring

### Boundary Health Metrics

Track monthly:
- Number of promotions attempted
- Number of promotions approved
- Number of boundary violations detected
- Time to detect violations
- Number of enforcement rules added

### Success Indicators

Healthy boundary enforcement shows:
- Promotions are rare and deliberate
- Violations are caught early
- Team actively references the contract
- No production incidents from research leakage

---

## Evolution of Enforcement

### When to Update Enforcement

Update enforcement mechanisms when:
- A new type of violation is discovered
- Current checks are insufficient
- Technology stack changes
- Team grows and needs stronger automation

### Continuous Improvement

After each violation:
1. Document the violation pattern
2. Add detection mechanism
3. Update onboarding materials
4. Share lessons learned

---

## References

- [Rust vs Python Contract](rust-vs-python-contract.md) — The foundational contract
- [Promotion Checklist](../../06-research-framework/promotion-checklist.md) — Required for all promotions
- [Design Principles](../../00-vision-and-non-goals/design-principles.md) — Why separation matters
- [Component Boundaries](component-boundaries.md) — Related boundary definitions

---

## Final Statement

**The boundary between research and production is sacred.**

These enforcement mechanisms exist to preserve that boundary even as the system grows and evolves.

Prevention through process. Detection through automation. Response through discipline.
