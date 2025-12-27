# Galactus — Signal Lifecycle

## Purpose of This Document

This document defines the **lifecycle of a signal** in Project Galactus.

It exists to:
- Prevent premature productionization
- Enforce research discipline
- Ensure signals remain relevant over time
- Enable graceful deprecation

Signals are not permanent assets.  
They are hypotheses with expiration dates.

---

## Core Principle

A signal is considered **guilty until proven robust**.

Promotion is earned slowly.  
Deprecation is expected.

---

## Signal Lifecycle Stages

Every signal must move through the following stages, in order:

1. Hypothesis  
2. Research Validation  
3. Promotion Candidate  
4. Core Signal  
5. Monitoring  
6. Deprecation  
7. Retirement  

Skipping stages is forbidden.

---

## 1. Hypothesis Stage

### Description

An idea proposed to measure a specific capital behavior.

### Characteristics

- Exists only in research code
- May be informal or exploratory
- May fail quickly

### Requirements

- Clear statement of capital behavior
- Hypothesized constraint or pressure
- Explicit assumptions

No performance claims are required at this stage.

### Entry Criteria

- Alignment with signal philosophy (see [`signal-philosophy.md`](signal-philosophy.md))
- Capital behavior clearly identified
- Structural mechanism articulated

### Exit Criteria

To transition to Research Validation, the hypothesis must:
- Be documented in a research notebook
- Have clear testable predictions
- Define success/failure conditions

### Example Artifacts

- Research notebook with hypothesis statement
- Preliminary code exploration
- Sketch of expected behavior

### Anti-Patterns

- ❌ Vague or narrative-only hypotheses
- ❌ Starting with parameter optimization
- ❌ Hypothesis based on price patterns alone

---

## 2. Research Validation Stage

### Description

The signal is tested empirically under controlled conditions.

### Requirements

- Historical validation across:
  - Multiple expiries
  - Different liquidity regimes
  - At least one stress period
- Failure analysis
- Stability assessment

### Prohibited Practices

- Parameter tuning for performance
- Selective reporting
- Ignoring counterexamples

Success here does **not** guarantee promotion.

### Entry Criteria

- Completed hypothesis stage with documented predictions
- Access to required historical data
- Research methodology documented (see [`research-methodology.md`](../06-research-framework/research-methodology.md))

### Exit Criteria

To transition to Promotion Candidate, validation must show:
- Structural consistency across at least 3 different regimes
- Documented failure modes with explanations
- Deterministic, replayable results
- Stability evidence from walk-forward validation (see [`walk-forward-validation.md`](../07-backtesting-and-validation/walk-forward-validation.md))

### Validation Activities

1. **Historical Backtesting**
   - Event-time aligned testing
   - Multiple regime coverage
   - Stress scenario inclusion (see [`stress-scenarios.md`](../07-backtesting-and-validation/stress-scenarios.md))

2. **Failure Analysis**
   - Document at least 3 failure modes
   - Explain structural reasons for failure
   - Identify regime boundaries (see [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md))

3. **Stability Assessment**
   - Parameter sensitivity analysis
   - Regime transition behavior
   - Degradation patterns

### Example Artifacts

- Research validation notebook
- Failure mode documentation
- Backtest results across regimes
- Statistical stability analysis

### Anti-Patterns

- ❌ Optimization over single time period
- ❌ "Cherry-picking" successful periods
- ❌ Ignoring unexplained anomalies
- ❌ Claiming universality without stress testing

---

## 3. Promotion Candidate Stage

### Description

The signal is proposed for inclusion in the core engine.

### Requirements

- Formal documentation
- Mathematical definition
- Deterministic formulation
- Alignment with signal philosophy
- Explicit failure modes

A signal may remain a candidate indefinitely.

### Entry Criteria

- Successfully completed research validation
- All requirements from validation stage met
- Signal owner identified and committed

### Exit Criteria

To transition to Core Signal, must pass **all items** in the promotion checklist:
- Complete [`promotion-checklist.md`](../06-research-framework/promotion-checklist.md)
- Architecture review approval
- Rust implementation feasibility confirmed
- Documentation complete

### Promotion Process

1. **Formal Proposal**
   - Complete promotion checklist
   - Document signal specification
   - Identify failure modes
   - Specify monitoring requirements

2. **Review Phase**
   - Structural validity assessment
   - Determinism verification
   - Data discipline compliance
   - Regime awareness evaluation

3. **Implementation Planning**
   - Rust implementation design
   - Test case definition
   - Schema integration plan
   - Monitoring setup

4. **Decision**
   - Approved → Proceed to Core Signal
   - Rejected → Return to Research with feedback
   - Deferred → Remain as candidate with conditions

### Example Artifacts

- Signal specification document
- Completed promotion checklist
- Mathematical formulation
- Implementation design document
- Test case definitions

### Anti-Patterns

- ❌ Promotion without complete checklist
- ❌ "We'll document it later"
- ❌ Informal or verbal-only approval
- ❌ Promotion based on performance metrics alone

---

## 4. Core Signal Stage

### Description

The signal is implemented in the Rust core and becomes part of production inference.

### Requirements

- Deterministic Rust implementation
- Unit and golden tests
- Schema alignment
- Versioned configuration

Once promoted, the signal is considered **stable but not permanent**.

### Entry Criteria

- Promotion approved
- Implementation design complete
- Test cases defined

### Implementation Requirements

1. **Rust Core Integration**
   - Pure Rust implementation (no Python dependencies)
   - Deterministic computation
   - Event-driven architecture compliance (see [`event-driven-architecture.md`](../02-system-architecture/event-driven-architecture.md))
   - Boundary enforcement (see [`rust-python-boundary-enforcement.md`](../02-system-architecture/rust-python-boundary-enforcement.md))

2. **Testing**
   - Unit tests for all signal logic
   - Golden test cases from research validation
   - Edge case coverage
   - Regression test suite

3. **Documentation**
   - API documentation
   - Signal behavior specification
   - Known limitations and failure modes
   - Usage examples

4. **Monitoring Setup**
   - Metrics defined (see Monitoring Stage)
   - Alerting thresholds set
   - Dashboard configuration

### Exit Criteria

To transition to Monitoring (operational state):
- All tests passing
- Documentation reviewed and approved
- Monitoring infrastructure active
- Production deployment successful

### Example Artifacts

- Rust source code with tests
- Signal configuration files
- API documentation
- Deployment verification report

### Anti-Patterns

- ❌ Unversioned signal configuration
- ❌ Missing or incomplete tests
- ❌ Undocumented behavior changes
- ❌ No rollback plan

---

## 5. Monitoring Stage

### Description

The signal is continuously evaluated for relevance and health.

### Monitoring Dimensions

- Frequency of activation
- Regime sensitivity
- Degradation under stress
- False confidence incidents

Monitoring does not imply optimization.

### Continuous Evaluation

This is an operational state, not a transition stage. All Core Signals remain in Monitoring until flagged for Deprecation.

### Monitoring Activities

1. **Health Metrics**
   - Signal activation frequency
   - Confidence distribution over time
   - Correlation with regime changes
   - Computation performance

2. **Quality Metrics**
   - Consistency with documented behavior
   - Failure mode occurrence rates
   - Unexpected behavior incidents
   - Silent degradation indicators

3. **Structural Metrics**
   - Market relevance assessment
   - Capital behavior alignment
   - Regulatory environment changes
   - Market microstructure shifts

### Monitoring Tools

- Automated dashboards
- Anomaly detection
- Periodic review reports
- Incident logging (see [`incident-response.md`](../10-operational-playbooks/incident-response.md))

### Review Cadence

- **Daily**: Operational health checks (see [`daily-operations.md`](../10-operational-playbooks/daily-operations.md))
- **Weekly**: Behavior consistency review
- **Monthly**: Regime alignment assessment
- **Quarterly**: Structural relevance evaluation

### Trigger Conditions for Deprecation

Monitor for:
- Persistent activation anomalies
- Regime breakdown (see [`regime-breakdown.md`](../08-risk-and-failure-modes/regime-breakdown.md))
- Structural market changes
- Redundancy with superior signals
- Violation of design principles

### Decision Log

All monitoring observations that influence decisions must be recorded in the decision log (see [`decision-log.md`](../11-decision-log/decision-log.md)).

### Example Artifacts

- Monitoring dashboards
- Weekly review reports
- Incident logs
- Quarterly assessment summaries

### Anti-Patterns

- ❌ Monitoring without action triggers
- ❌ Ignoring gradual degradation
- ❌ Optimization without re-validation
- ❌ Silent parameter changes

---

## 6. Deprecation Stage

### Description

The signal is flagged for eventual removal.

### Triggers for Deprecation

- Structural market changes
- Persistent instability
- Redundancy with superior signals
- Violation of design principles

### Behavior

- Signal remains available
- Confidence is reduced
- Consumers are warned

Deprecation is a **success state**, not a failure.

### Entry Criteria

One or more deprecation triggers observed and verified:
- Market structure has fundamentally changed
- Signal persistently fails in documented scenarios
- Better alternative exists and is validated
- Underlying assumptions no longer hold

### Deprecation Process

1. **Evaluation Phase**
   - Verify deprecation trigger
   - Assess impact on dependent systems
   - Identify replacement (if applicable)
   - Plan transition timeline

2. **Announcement**
   - Document deprecation decision (see [`deprecation-policy.md`](../12-roadmap-and-deprecation/deprecation-policy.md))
   - Update signal documentation
   - Notify all consumers
   - Set retirement date

3. **Transition Period**
   - Reduce signal confidence progressively
   - Monitor consumer migration
   - Provide deprecation warnings
   - Support parallel operation with replacement

4. **Documentation**
   - Record in decision log
   - Update architecture documentation
   - Preserve historical behavior documentation

### Deprecation Timeline

- Minimum deprecation period: 90 days
- Extension possible if consumers require more time
- Hard retirement date set at deprecation announcement

### Confidence Reduction Strategy

- Month 1: Confidence multiplier 0.8
- Month 2: Confidence multiplier 0.5
- Month 3: Confidence multiplier 0.2
- Retirement: Signal removed

### Example Artifacts

- Deprecation announcement document
- Consumer impact assessment
- Migration guide (if replacement exists)
- Timeline and milestone plan
- Decision log entry

### Anti-Patterns

- ❌ Silent deprecation without announcement
- ❌ Immediate removal without transition period
- ❌ Deprecation without documented reason
- ❌ Ignoring consumer dependencies

---

## 7. Retirement Stage

### Description

The signal is removed from active inference.

### Requirements

- Documentation update
- Decision log entry
- Archival of historical behavior

Retired signals are not deleted; they are remembered.

### Entry Criteria

- Deprecation period completed
- All consumers migrated or notified
- Retirement date reached

### Retirement Process

1. **Pre-Retirement Verification**
   - Confirm all consumers are aware
   - Verify replacement signal operational (if applicable)
   - Final backup of signal behavior data

2. **Removal**
   - Remove from production inference
   - Archive source code
   - Preserve test cases
   - Update configuration

3. **Documentation**
   - Update signal catalog
   - Record retirement in decision log
   - Document retirement rationale
   - Preserve historical documentation

4. **Knowledge Preservation**
   - Archive research notebooks
   - Document lessons learned
   - Preserve failure mode analysis
   - Record structural insights

### Post-Retirement

- Signal code moved to archive
- Historical behavior documentation remains accessible
- Test cases preserved for reference
- Resurrection requires full re-validation through lifecycle

### Archival Requirements

Preserve:
- Signal specification and formulation
- Implementation code (archived)
- Test cases and golden data
- Monitoring data summary
- Deprecation rationale
- Lessons learned

### Example Artifacts

- Retirement announcement
- Archived codebase with tags
- Historical documentation package
- Decision log entry
- Lessons learned document

### Anti-Patterns

- ❌ Deletion without archival
- ❌ Undocumented retirement rationale
- ❌ Loss of historical behavior knowledge
- ❌ Resurrection without re-validation

### Resurrection Policy

Retired signals can only return to production by:
- Starting from Hypothesis stage
- Complete re-validation with current market structure
- New promotion approval
- Updated implementation

No shortcuts allowed.

---

## Governance and Accountability

- Every signal has an owner
- Lifecycle state must be explicit
- Changes require documentation updates

Signals without ownership are not allowed.

---

## Lifecycle State Tracking

### Signal Registry

All signals must be tracked in a signal registry that includes:
- Signal name and version
- Current lifecycle stage
- Signal owner
- Entry date to current stage
- Last review date
- Next review due date

### State Transitions

All transitions between lifecycle stages must be:
- Documented with rationale
- Recorded in decision log
- Approved by designated reviewers
- Versioned in source control

### Stage Duration Guidelines

While signals progress at their own pace, these guidelines help identify stalled signals:

- **Hypothesis → Research Validation**: 2-8 weeks
- **Research Validation → Promotion Candidate**: 4-12 weeks
- **Promotion Candidate → Core Signal**: 4-8 weeks
- **Core Signal → Monitoring**: Immediate upon deployment
- **Monitoring → Deprecation**: Variable (months to years)
- **Deprecation → Retirement**: 90+ days minimum

Signals stuck beyond these guidelines warrant review.

### Ownership Responsibilities

Signal owners are responsible for:
- Maintaining signal documentation
- Conducting periodic reviews
- Monitoring signal health
- Initiating deprecation when appropriate
- Preserving knowledge during retirement

Ownership transfer requires:
- Documented handoff
- Knowledge transfer session
- Updated registry
- Acceptance by new owner

---

## What Is Explicitly Forbidden

- “Temporary” signals
- Undocumented promotion
- Silent behavior changes
- Resurrection without re-validation

---

## Lifecycle Stage Summary

| Stage | Duration | Key Activities | Exit Criteria | Artifacts |
|-------|----------|----------------|---------------|-----------|
| **Hypothesis** | 2-8 weeks | Idea formulation, preliminary exploration | Clear testable predictions | Research notebook |
| **Research Validation** | 4-12 weeks | Empirical testing, failure analysis | Structural consistency, documented failures | Validation report, backtests |
| **Promotion Candidate** | 4-8 weeks | Formal proposal, review process | Complete promotion checklist | Specification, checklist |
| **Core Signal** | Implementation sprint | Rust implementation, testing | All tests pass, docs complete | Source code, tests |
| **Monitoring** | Continuous | Health tracking, periodic review | Deprecation trigger observed | Dashboards, reports |
| **Deprecation** | 90+ days | Consumer migration, confidence reduction | Retirement date reached | Deprecation notice |
| **Retirement** | Final | Removal, archival, knowledge preservation | Documentation complete | Archive package |

---

## Related Documents

### Foundation
- [`signal-philosophy.md`](signal-philosophy.md) — Core principles for valid signals
- [`metric-taxonomy.md`](metric-taxonomy.md) — Classification of metrics and signals
- [`normalization-standards.md`](normalization-standards.md) — Signal normalization requirements

### Research and Validation
- [`research-methodology.md`](../06-research-framework/research-methodology.md) — Research discipline and hypothesis testing
- [`experiment-design.md`](../06-research-framework/experiment-design.md) — Experiment framing and constraints
- [`promotion-checklist.md`](../06-research-framework/promotion-checklist.md) — Mandatory promotion requirements
- [`backtesting-guidelines.md`](../06-research-framework/backtesting-guidelines.md) — Backtest standards and integrity

### Validation Framework
- [`backtest-design.md`](../07-backtesting-and-validation/backtest-design.md) — Backtest structure and methodology
- [`walk-forward-validation.md`](../07-backtesting-and-validation/walk-forward-validation.md) — Stability testing approach
- [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md) — Failure mode documentation
- [`stress-scenarios.md`](../07-backtesting-and-validation/stress-scenarios.md) — Stress testing requirements

### Architecture and Implementation
- [`event-driven-architecture.md`](../02-system-architecture/event-driven-architecture.md) — System design principles
- [`rust-python-boundary-enforcement.md`](../02-system-architecture/rust-python-boundary-enforcement.md) — Implementation boundaries
- [`rust-vs-python-contract.md`](../02-system-architecture/rust-vs-python-contract.md) — Language separation contract
- [`component-boundaries.md`](../02-system-architecture/component-boundaries.md) — Component separation rules

### Operations
- [`daily-operations.md`](../10-operational-playbooks/daily-operations.md) — Daily monitoring procedures
- [`incident-response.md`](../10-operational-playbooks/incident-response.md) — Incident handling playbook

### Governance
- [`decision-log.md`](../11-decision-log/decision-log.md) — Architectural decision tracking
- [`deprecation-policy.md`](../12-roadmap-and-deprecation/deprecation-policy.md) — Deprecation process and policy
- [`sunset-criteria.md`](../12-roadmap-and-deprecation/sunset-criteria.md) — Component retirement criteria

---

## Why This Lifecycle Exists

Markets evolve.  
Signals decay.

Without a lifecycle:
- Systems accumulate ghosts
- Confidence inflates
- Understanding erodes

Galactus avoids this by treating signals as **living hypotheses**.

---

## Final Statement

**A signal that cannot die is a signal that should never be born.**
