# Galactus — Research Methodology

## Purpose of This Document

This document defines the **research methodology** used in Project Galactus.

It exists to:
- Prevent p-hacking and overfitting
- Enforce discipline in hypothesis testing
- Separate insight from coincidence
- Ensure research results are reproducible and honest

Research is how Galactus learns.  
Methodology is how it avoids self-deception.

---

## Core Research Philosophy

Galactus research is guided by three beliefs:

1. **Most hypotheses are wrong**
2. **Performance is not proof**
3. **Understanding matters more than optimization**

Research exists to invalidate ideas quickly, not to confirm them eagerly.

---

## Hypothesis-Driven Research

All research must begin with an explicit hypothesis.

A valid hypothesis must state:
- The capital behavior being tested
- The constraint or mechanism involved
- The expected observable effect
- The conditions under which it should fail

Exploration without hypothesis is permitted, but promotion without hypothesis is forbidden.

### Hypothesis Formulation Standards

Every research hypothesis must satisfy the following requirements:

#### 1. Falsifiability
- The hypothesis must be falsifiable with available data
- Success criteria must be defined **before** testing begins
- Failure criteria must be equally explicit
- Null hypothesis must be stated explicitly

#### 2. Structural Grounding
- Hypothesis must derive from market structure theory
- Capital behavior mechanism must be identified
- Timing windows must be justified by constraint mechanics
- Cross-instrument applicability must be declared

#### 3. Prior Declaration
- Hypothesis must be documented **before** data analysis
- Test design must be specified upfront
- Parameter choices must be justified ex-ante
- Any deviations from original hypothesis must be documented

#### 4. Testable Predictions
- Observable outcomes must be specific and measurable
- Predictions must include magnitude, direction, and timing
- Counterfactual scenarios must be identified
- Alternative explanations must be enumerated

---

## Research Stages

Research proceeds through the following stages:

1. Exploration  
2. Hypothesis Formulation  
3. Controlled Testing  
4. Failure Analysis  
5. Validation Across Regimes  

Skipping stages is not allowed.

---

## Data Discipline

Research must:
- Use the same canonical data sources as production
- Respect schema versions
- Avoid forward-looking bias
- Document any data limitations

Synthetic or external data must be clearly labeled.

---

## Bias Control

Research integrity depends on rigorous bias prevention across multiple dimensions.

### Look-Ahead Bias Prevention

All research must:
- Align strictly to event time
- Avoid using future information implicitly
- Respect disclosure and publication delays
- Model information arrival explicitly
- Use point-in-time data snapshots only

Any look-ahead contamination invalidates results.

**Enforcement mechanisms:**
- Event timestamp validation before computation
- Explicit disclosure delay modeling
- Automated checks for future data leakage
- Temporal ordering verification in all pipelines

### Selection Bias Controls

Research must avoid:
- Cherry-picking time periods that favor results
- Selective instrument inclusion based on outcomes
- Post-hoc sample filtering without justification
- Survivorship bias in instrument selection

**Required practices:**
- Pre-declare sample selection criteria
- Document all exclusions with structural rationale
- Test across complete instrument universe
- Include delisted and failed instruments where applicable

### Confirmation Bias Mitigation

Researchers must:
- Seek disconfirming evidence actively
- Test opposing hypotheses with equal rigor
- Document results that contradict expectations
- Avoid narrative retrofitting of results

**Required documentation:**
- Pre-registered hypothesis before testing
- Failed tests documented with equal detail
- Alternative explanations considered explicitly
- Revision history of hypothesis iterations

### Overfitting and P-Hacking Prevention

Research must avoid:
- Multiple testing without correction
- Parameter tuning based on test set results
- Selective reporting of favorable metrics
- Iterative refinement without train/test discipline

**Mandatory safeguards:**
- Hold-out validation sets never used for development
- Multiple comparison corrections applied
- All tested variations documented
- Parameter choices justified ex-ante

### Data Snooping Prevention

Research must:
- Separate exploration from validation
- Document all preliminary analyses
- Avoid reusing data across hypothesis tests
- Maintain audit trail of data access

**Implementation requirements:**
- Clear separation of exploratory vs. confirmatory phases
- Independent validation data sets
- Documented data usage for each hypothesis
- Prohibition on test set feedback into research

---

## Parameter Discipline

Parameters must:
- Be few and interpretable
- Have economic justification
- Remain stable across regimes

Hyperparameter tuning for performance is strongly discouraged.

---

## Evaluation Philosophy

Research evaluation focuses on:
- Stability across regimes
- Behavior under stress
- Failure characteristics

Single-number metrics (e.g., accuracy) are insufficient.

---

## Counterfactual Thinking

Every research result must consider:
- What would disprove this?
- Under what conditions does this fail?
- Is the result structurally plausible?

Results without counterfactual analysis are incomplete.

---

## Reproducibility Requirements

All research must be:
- Reproducible from raw data
- Version-controlled
- Documented sufficiently for re-execution

If a result cannot be reproduced, it does not exist.

### Technical Reproducibility Standards

#### 1. Computational Determinism
- Identical inputs must produce identical outputs
- Random seed management must be explicit
- Floating-point operations must be documented
- Hardware dependencies must be declared

**Implementation requirements:**
- Fixed seeds for any stochastic processes
- Documented numerical precision requirements
- Platform-independent computation where possible
- Explicit handling of non-deterministic operations

#### 2. Environment Reproducibility
- All dependencies must be version-pinned
- Execution environment must be documented
- System requirements must be explicit
- Runtime configuration must be captured

**Required artifacts:**
- Dependency lock files (requirements.txt, Cargo.lock)
- Environment specification (Python version, Rust version)
- System library dependencies documented
- Configuration files version-controlled

#### 3. Data Provenance
- Data sources must be explicitly documented
- Data versions must be tracked
- Data transformations must be auditable
- Raw data preservation required

**Mandatory documentation:**
- Source URLs and access timestamps
- Data schema versions used
- Preprocessing steps with parameters
- Data quality checks performed

#### 4. Code Reproducibility
- All code must be version-controlled
- Exact commit hashes must be recorded
- Build processes must be documented
- Execution order must be explicit

**Version control requirements:**
- Git commit hash recorded in outputs
- Branch and tag information captured
- Build configuration stored with results
- Execution scripts version-controlled

#### 5. Result Reproducibility
- All outputs must be bit-for-bit reproducible
- Intermediate results must be cacheable
- Reproduction tests must be automated
- Divergence must trigger investigation

**Verification standards:**
- Automated reproducibility tests in CI
- Hash verification of outputs
- Tolerance thresholds explicitly defined
- Reproduction failures treated as bugs

### Documentation Requirements for Reproducibility

Every research artifact must include:

1. **Execution Manifest**
   - Date and time of execution
   - Code version (commit hash)
   - Environment specifications
   - Runtime configuration

2. **Data Manifest**
   - Data sources and versions
   - Sample selection criteria
   - Data quality metrics
   - Known data issues

3. **Methodology Documentation**
   - Hypothesis statement
   - Test design
   - Parameter choices and justification
   - Success/failure criteria

4. **Results Package**
   - Raw outputs
   - Summary statistics
   - Diagnostic plots
   - Interpretation notes

### Reproducibility Validation Process

Before publication or promotion, research must pass:

1. **Fresh Environment Test**
   - Execute in clean environment
   - Verify identical outputs
   - Document any environment dependencies

2. **Independent Replication**
   - Different researcher re-executes
   - Results must match bit-for-bit
   - Discrepancies must be resolved

3. **Time-Shifted Validation**
   - Re-execute after time delay
   - Verify data access still works
   - Confirm environment still buildable

---

## Negative Results

Negative results are:
- Valuable
- Documented
- Preserved

Discarding failed hypotheses is a success, not a loss.

---

## Promotion Readiness

Research is considered promotion-ready only when:
- The hypothesis is structurally sound
- Behavior is stable across regimes
- Failure modes are understood
- Results are reproducible

Performance alone is insufficient.

---

## Ethical Guardrails

Research must:
- Avoid personalized inference
- Respect compliance boundaries
- Avoid outcome-driven framing

Insight must not cross into advice.

---

## Related Documentation

For detailed experiment design guidance:
- [`experiment-design.md`](experiment-design.md) — Complete experiment framework
- [`experiment-template.md`](experiment-template.md) — Documentation template
- [`experiment-checklist.md`](experiment-checklist.md) — Validation checklist
- [`experiment-examples.md`](experiment-examples.md) — Practical examples

For promotion process:
- [`promotion-checklist.md`](promotion-checklist.md) — Promotion requirements
- [`backtesting-guidelines.md`](backtesting-guidelines.md) — Validation standards

---

## Final Statement

**Research in Galactus exists to reduce ignorance, not to create confidence.**

Truth survives scrutiny. Hypotheses do not.
