# Galactus — Experiment Design

## Purpose

This document defines the **intent, constraints, and discipline** governing **experiment design** within Project Galactus.

It exists to ensure that all experiments:
- Advance structural understanding of markets
- Remain grounded in capital behavior
- Avoid outcome-driven or performance-led bias
- Produce results that are reproducible, interpretable, and promotable

Experiments are not vehicles for validation.  
They are tools for **learning and falsification**.

---

## Scope

### This Document Covers

This document governs:

- How experiments are framed and justified
- What constitutes a valid experiment in Galactus
- Constraints on data, methodology, and interpretation
- How experiments relate to features, signals, and promotion
- What experimental results are considered meaningful

### This Document Does NOT Cover

This document explicitly does **not** cover:

- Trading strategy design
- Execution or PnL optimization
- Hyperparameter tuning for performance
- UI, visualization, or presentation concerns
- Promotion criteria (covered separately in the Promotion Checklist)

Experiments are **pre-decision artifacts**, not production candidates.

---

## Definition: Experiment

In Galactus, an **experiment** is defined as:

> A controlled investigation designed to test whether a specific capital behavior or constraint produces observable, structurally consistent effects under defined conditions.

An experiment must aim to **invalidate or refine a hypothesis**, not to confirm it.

---

## Core Principles

### 1. Determinism Over Cleverness

- Experiments must be reproducible end-to-end
- Same inputs must produce the same results
- No hidden randomness, sampling tricks, or adaptive logic

Clever experiments that cannot be replayed are invalid.

---

### 2. Inference Over Prediction

- Experiments test *mechanisms*, not forecasts
- Outcomes are evaluated structurally, not financially
- “Did pressure exist?” matters more than “Did price move?”

Predictive success without explanation is insufficient.

---

### 3. Capital Behavior as First-Class Truth

Every experiment must explicitly define:

- What capital is involved
- What constraint is being tested
- Why this constraint should force or bias action
- Over what time window the effect should appear

Experiments without capital grounding are rejected.

---

### 4. Failure Is a Valid Outcome

Experiments are successful if they:
- Disprove a hypothesis
- Reveal instability
- Expose regime dependence
- Identify incorrect assumptions

Experiments that “work everywhere” are suspect.

---

## Experiment Framing Requirements

Every experiment must clearly document:

- **Hypothesis**  
  What specific capital behavior is being tested

- **Mechanism**  
  Why the behavior should exist structurally

- **Expected Observations**  
  What evidence would support or refute the hypothesis

- **Failure Conditions**  
  When and why the experiment should fail

Experiments without explicit failure conditions are incomplete.

---

### Detailed Framing Template

A properly framed experiment must answer the following:

#### 1. Capital Identification
- Which capital pool(s) are involved? (e.g., FII derivatives positions, domestic mutual funds, retail cash flow)
- What is the estimated size or scale of this capital?
- What is the liquidity constraint of this capital?

#### 2. Constraint Definition
- What specific constraint binds this capital? (e.g., regulatory requirement, margin call, contract expiry)
- Why is this constraint non-optional?
- What is the time horizon of the constraint?

#### 3. Observable Implications
- What specific market behavior should result from this constraint?
- Where should this behavior be visible? (specific instruments, time periods, market segments)
- What magnitude of effect is structurally plausible?

#### 4. Falsification Criteria
- Under what conditions should this hypothesis fail?
- What observations would contradict the hypothesis?
- What alternative explanations exist for the expected observations?

#### 5. Regime Dependency
- In which market regimes should this behavior be observable?
- In which regimes should this behavior be absent or weakened?
- How should regime transitions affect the behavior?

---

## Constraint Specifications

Experiments must operate within strict boundaries to maintain validity and alignment with Galactus principles.

### Data Constraints

All experiments must:

- **Use Approved Data Sources Only**
  - Public market data from documented sources
  - No proprietary, paid, or privileged data feeds
  - No personalized or user-level data
  - All data sources must be listed in [`data-sources.md`](../03-data-and-schemas/data-sources.md)

- **Respect Canonical Schemas**
  - Use schema versions documented in [`canonical-schemas.md`](../03-data-and-schemas/canonical-schemas.md)
  - Handle missing or malformed data explicitly
  - Document any data transformations applied

- **Avoid Look-Ahead Bias**
  - Align strictly to event time
  - Respect disclosure and publication delays
  - No implicit use of future information
  - Document the information horizon clearly

- **Declare All Assumptions**
  - List data quality assumptions
  - Identify any data limitations or gaps
  - Document simplifications made
  - Explain impact of assumptions on conclusions

### Methodology Constraints

Experiments must adhere to:

- **Determinism Requirement**
  - Same inputs must always produce same outputs
  - No hidden randomness or sampling
  - No environment-dependent behavior
  - Fully reproducible from raw data

- **Parameter Discipline**
  - Minimize number of parameters (prefer 0-3)
  - Each parameter must have structural justification
  - No parameter tuning after observing results
  - Document parameter selection rationale

- **Time Window Specification**
  - Explicitly define analysis time period
  - Justify window selection structurally (not by performance)
  - Include multiple regime periods if possible
  - Avoid cherry-picking favorable periods

- **Control Standards**
  - Define baseline or null hypothesis
  - Include negative control tests
  - Test counterfactual scenarios
  - Document what was NOT tested and why

### Scope Constraints

Experiments must define boundaries:

- **Instrument Scope**
  - Which instruments or asset classes are included?
  - Why are others excluded?
  - How does scope affect generalizability?

- **Regime Scope**
  - Which market regimes are covered?
  - Which regimes are explicitly out of scope?
  - How is regime classification determined?

- **Temporal Scope**
  - What time horizons are analyzed?
  - Are intraday, daily, or weekly patterns relevant?
  - How does time scale affect conclusions?

### Computational Constraints

- **Resource Limits**
  - Experiments should complete in reasonable time (< 1 hour preferred)
  - Memory requirements should be documented
  - Computational complexity should be understood

- **Reproducibility Requirements**
  - All code must be version controlled
  - Dependencies must be documented
  - Random seeds must be fixed (if randomness required)
  - Environment specifications must be recorded

---

## Failure Expectations and Analysis

In Galactus, **failure is not only expected but required** for an experiment to be valid.

### Philosophy of Failure

- Experiments that "work everywhere" are structurally suspect
- Failure reveals boundaries of applicability
- Understanding failure modes is as valuable as understanding success
- Silence (no inference) is preferable to confident nonsense

### Required Failure Documentation

Every experiment must document at least **three categories of failure**:

#### 1. Structural Failures

Failures arising from invalid assumptions or incorrect model:

- **Data Quality Failures**
  - Missing or delayed data
  - Schema violations
  - Corrupted or anomalous inputs
  - *Response*: Degrade confidence, do not adjust logic to compensate

- **Model Assumption Failures**
  - Constraint is not actually binding
  - Market structure has changed
  - Participant behavior differs from model
  - *Response*: Re-examine hypothesis, consider model revision

- **Mechanism Failures**
  - Expected capital pressure does not materialize
  - Timing does not align with constraints
  - Magnitude is structurally implausible
  - *Response*: Invalidate hypothesis, document findings

#### 2. Regime-Dependent Failures

Failures that occur in specific market regimes:

- **Invalid Regime Application**
  - Signal applied outside its valid regime
  - Regime misclassification
  - Regime transition instability
  - *Expected Behavior*: Signal should be silent or low-confidence

- **Regime Shift Failures**
  - Sudden regime transitions
  - Mixed regime states
  - Unstable regime boundaries
  - *Expected Behavior*: Increased uncertainty, graceful degradation

#### 3. Operational Failures

Failures in experiment execution or interpretation:

- **Look-Ahead Contamination**
  - Implicit use of future information
  - Timing misalignment
  - Publication delay not respected
  - *Impact*: Invalidates all results

- **Overfitting Indicators**
  - Performance driven by single period
  - Fragile parameter dependence
  - Inconsistent behavior across regimes
  - *Impact*: Results not promotable

- **Interpretation Failures**
  - Results cannot be explained structurally
  - Narrative relies on hindsight
  - Conflicting with documented theory
  - *Impact*: Hypothesis requires revision

### Failure Detection Mechanisms

Experiments must include:

- **Pre-defined Failure Indicators**
  - Specific metrics or conditions that signal failure
  - Thresholds based on structural plausibility (not optimization)
  - Automated checks where possible

- **Post-Event Analysis**
  - Comparison with actual market outcomes
  - Identification of false positives/negatives
  - Root cause analysis for failures

- **Stress Testing**
  - Behavior during market stress events
  - Edge case scenarios
  - Known historical failures

### Documenting Failure Modes

For each identified failure mode, document:

1. **Description**: What failed and how?
2. **Root Cause**: Why did it fail structurally?
3. **Frequency**: How often does this failure occur?
4. **Detection**: How can this failure be identified?
5. **Impact**: What are the consequences of this failure?
6. **Mitigation**: How should the system respond? (if applicable)

### Acceptable vs. Unacceptable Failures

**Acceptable Failures** (to be documented and handled):
- Failure during regime transitions
- Failure when data is degraded or missing
- Failure when constraints are not binding
- Failure in explicitly out-of-scope scenarios

**Unacceptable Failures** (invalidate the experiment):
- Unexplained failures
- Failures ignored or hidden
- Failures attributed to "market irrationality"
- Failures requiring ad-hoc fixes

### Success Criteria for Failure Analysis

An experiment's failure analysis is complete when:

- ✅ At least 3 distinct failure modes are documented
- ✅ Each failure mode has a structural explanation
- ✅ Historical examples of failures are identified
- ✅ Detection mechanisms are defined
- ✅ Response strategies are specified
- ✅ Boundaries of applicability are clear

**Remember**: A well-understood failure is more valuable than an unexplained success.

---

## Evaluation Standards

Experiments are evaluated on:

- Structural consistency across regimes
- Stability under stress and edge conditions
- Interpretability of results
- Alignment with documented market theory

Performance metrics (PnL, accuracy, returns) are **secondary and optional**, never primary.

---

## Relationship to Features and Signals

- Experiments may produce **insights**
- Insights may lead to **feature hypotheses**
- Features may eventually become **signals** via the signal lifecycle

No experiment directly produces a production signal.

---

## Invalid Experiment Patterns (Explicit)

The following invalidate an experiment:

- Outcome-led framing (“This works because returns are high”)
- Selective time period analysis
- Parameter tuning after observing results
- Silent data exclusions
- Narrative explanations without structural grounding

These are considered research failures.

---

## Documentation Requirements

Every experiment must produce comprehensive documentation following the structure below.

### Mandatory Documentation Sections

#### 1. Experiment Metadata

**Required Information:**
- Experiment title and identifier
- Researcher(s) conducting the experiment
- Date initiated and date completed
- Related experiments or prior work
- Links to code repository/notebook

**Template:**
```
Experiment ID: EXP-YYYY-MM-NNN
Title: [Descriptive Title]
Researcher: [Name]
Date: YYYY-MM-DD to YYYY-MM-DD
Related: [Links to related experiments]
Location: [Path to notebook/code]
```

#### 2. Hypothesis Statement

**Required Content:**
- Clear statement of the hypothesis
- Capital behavior being tested
- Structural mechanism explaining the behavior
- Observable implications
- Falsification criteria

**Template:**
```
Hypothesis: [One-sentence statement]

Capital Involved: [Specific capital pool]
Constraint: [What binds this capital]
Expected Observation: [What we should see]
Falsification: [What would disprove this]
```

#### 3. Methodology

**Required Content:**
- Data sources and versions used
- Time period analyzed
- Instruments included/excluded
- Key parameters and their justification
- Analysis approach
- Control mechanisms

**Quality Standards:**
- Must be reproducible by another researcher
- All assumptions must be explicit
- Deviations from standard methodology must be justified

#### 4. Results

**Required Content:**
- Primary findings (structured around hypothesis)
- Statistical summaries (if applicable)
- Regime-specific behavior
- Unexpected observations
- Confidence assessment

**Presentation Standards:**
- Lead with structural insights, not performance metrics
- Show distributions, not just point estimates
- Highlight contradictory or ambiguous results
- Include visualizations with clear labels

#### 5. Failure Analysis

**Required Content:**
- Documented failure modes (minimum 3)
- Historical failure examples
- Root cause analysis for each failure
- Frequency and conditions of failure
- Detection mechanisms

**Format:**
```
Failure Mode 1: [Name]
- Description: [What fails]
- Cause: [Why it fails structurally]
- Frequency: [How often]
- Detection: [How to identify]
- Response: [How system should handle]
```

#### 6. Regime Analysis

**Required Content:**
- Behavior in different market regimes
- Regime classification methodology
- Transition behavior
- Boundaries of applicability
- Confidence degradation patterns

#### 7. Interpretation

**Required Content:**
- Structural explanation of results
- Alignment with market theory
- Consistency with documented capital behavior
- Alternative interpretations considered
- Limitations of conclusions

**Standards:**
- Must be explainable without charts
- Must reference documented theory
- Must acknowledge uncertainty
- Must avoid narrative fallacies

#### 8. Learning Outcomes

**Required Content:**
- What was learned from this experiment
- How hypothesis was validated/invalidated
- Implications for future research
- Contribution to market understanding

#### 9. Recommendation

**Required Content:**
- Clear recommendation: Promote, Refine, or Reject
- Rationale for recommendation
- If Promote: readiness checklist status
- If Refine: specific improvements needed
- If Reject: lessons learned

#### 10. Open Questions

**Required Content:**
- Unresolved questions arising from experiment
- Suggested follow-up experiments
- Known limitations requiring further investigation

### Documentation Quality Standards

All experiment documentation must:

- ✅ **Be Discoverable**: Stored in standard location with clear naming
- ✅ **Be Reproducible**: Contain sufficient detail to reproduce results
- ✅ **Be Honest**: Report failures and limitations transparently
- ✅ **Be Contextual**: Link to related work and theory
- ✅ **Be Conclusive**: Provide clear outcomes and recommendations

### Documentation Formats

**Primary Format**: Markdown document

**Supplementary Formats**:
- Jupyter notebooks with narrative
- Code with inline documentation
- Visualizations with captions

**Storage Location**:
```
research/python/experiments/
├── YYYY-MM-experiment-name/
│   ├── README.md              # Main documentation
│   ├── notebook.ipynb         # Analysis notebook
│   ├── results/               # Output artifacts
│   └── data/                  # Experiment-specific data
```

### Documentation Lifecycle

1. **Pre-Experiment**: Document hypothesis and plan
2. **During Experiment**: Maintain research log/notebook
3. **Post-Experiment**: Complete all required sections
4. **Review**: Peer review of documentation
5. **Archive**: Store in permanent location with version

### Review Criteria

Experiment documentation passes review when:

- ✅ All mandatory sections are complete
- ✅ Hypothesis is clearly stated and testable
- ✅ Failures are documented honestly
- ✅ Conclusions are supported by evidence
- ✅ Recommendation is justified
- ✅ Another researcher could reproduce the work

### Anti-Patterns in Documentation

**🚫 Forbidden Practices:**
- Documenting only successful experiments
- Hiding or minimizing failures
- Outcome-driven narrative construction
- Cherry-picking favorable results
- Vague or ambiguous language
- Missing reproducibility information
- Undocumented parameter choices
- Assertions without structural grounding

### Documentation as a Gatekeeper

Incomplete or inadequate documentation:
- Prevents promotion to production
- Blocks related follow-up work
- Indicates experiment may need to be repeated
- Suggests hypothesis was not well-formed

**Rule**: If you cannot document it clearly, you should not have run the experiment.

Undocumented experiments do not exist.

---

## Open Questions

Open questions are expected and encouraged.

They must be:
- Explicitly stated
- Tracked across iterations
- Revisited as new evidence emerges

Untracked ambiguity is technical debt.

---

## Revision History

- v1.1 — Enhanced with detailed framing requirements, comprehensive constraint specifications, extensive failure expectations framework, and complete documentation requirements with templates
- v1.0 — Formalized experiment design discipline aligned with Galactus research framework  
- v0.1 — Initial draft
