# Galactus — Decision Log

## Purpose
This document records significant architectural and design decisions.

---

## Decision 001: Core Vision Lock (2024)

### Date
2024 Q4

### Decision
Establish the core vision documents as immutable and create enforcement mechanisms to prevent scope creep, feature drift, and architectural compromise.

### Context
- Project Galactus requires a clear, permanent identity as a capital-pressure inference engine
- Need to explicitly exclude: trading systems, advisory tools, execution logic, and prediction engines
- Historical pattern shows systems drift from original vision under commercial/feature pressure
- Regulatory clarity requires permanent non-advisory stance
- Technical coherence depends on stable architectural foundation

### Alternatives Considered

**Alternative 1: Flexible Vision**
- Allow vision to evolve based on user feedback and market needs
- Rejected because:
  - Leads to scope creep and feature accumulation
  - Compromises core technical advantages (determinism, explainability)
  - Creates regulatory ambiguity
  - Dilutes competitive differentiation

**Alternative 2: No Formal Vision**
- Keep vision informal, let code drive architecture
- Rejected because:
  - Results in inconsistent decisions
  - No mechanism to reject out-of-scope proposals
  - Technical debt from conflicting objectives
  - Cannot justify trade-offs consistently

### Rationale
- **Design Principle: Determinism over cleverness** — Stable vision enables deterministic architecture
- **Design Principle: Minimalism in core** — Vision lock prevents feature accumulation
- **Market Theory: Capital behavior is the primitive** — Clear scope focuses on core competency
- Permanent non-advisory stance provides regulatory clarity
- Immutable boundaries enable technical excellence in narrow domain

### Implementation
- Created `VISION_LOCK.md` defining immutability policy
- Created `README.md` anchoring vision at repository root
- Established formal revision process requiring:
  - Evidence of market structure changes
  - Decision log entry
  - Documentation cascade
- Defined automatic rejection criteria for vision violations

### Trade-offs and Consequences

**Accepted Costs:**
- Reduced flexibility to pivot into adjacent markets
- May miss opportunities in trading/advisory space
- Requires discipline to reject attractive features
- Ongoing enforcement overhead

**Benefits:**
- Architectural coherence across all components
- Clear regulatory positioning
- Protection against feature drift
- Simplified decision-making framework
- Long-term technical viability

### Success Metrics
- Zero vision-violating features merged
- Consistent rejection of out-of-scope proposals
- Architectural decisions reference vision documents
- Quarterly compliance audits pass
- System maintains non-advisory language

### Revisit Conditions
This decision should be revisited if:
- Indian market structure changes fundamentally (regulatory framework, microstructure)
- Core assumption (capital constraints drive markets) is empirically invalidated
- Deterministic inference becomes technically impossible
- Legal/regulatory landscape makes current stance untenable

**Note:** Poor performance, competitive pressure, or feature requests are NOT valid revisit conditions.

### References
- `docs/00-vision-and-non-goals/vision.md`
- `docs/00-vision-and-non-goals/explicit-non-goals.md`
- `docs/00-vision-and-non-goals/design-principles.md`
- `VISION_LOCK.md`

---

## Decision 002: Rust vs Python Boundary Enforcement (2024)

### Date
2024 Q4

### Decision
Implement strict enforcement mechanisms to maintain separation between Rust (deterministic production core) and Python (research and discovery), preventing research code leakage into production.

### Context
- The [Rust vs Python Contract](../02-system-architecture/rust-vs-python-contract.md) establishes philosophical separation but lacks concrete enforcement
- Without enforcement, boundary violations are inevitable:
  - Research code accidentally promoted without validation
  - Production logic duplicated in Python for "convenience"
  - Hidden dependencies between layers
  - Gradual erosion of determinism guarantees
- Need for automated detection and prevention mechanisms
- Team growth requires clearer structural boundaries
- Production reliability depends on determinism, which requires strict separation

### Alternatives Considered

**Alternative 1: Trust-Based Approach**
- Rely on code review and team discipline alone
- No automated enforcement or directory structure
- Rejected because:
  - Scales poorly as team grows
  - Violations are easy to miss in review
  - Creates ambiguity about what belongs where
  - No way to prevent accidental violations

**Alternative 2: Monorepo Without Physical Separation**
- Keep all code in single directory structure
- Use naming conventions to separate concerns
- Rejected because:
  - Too easy to create hidden dependencies
  - Naming conventions are not enforced mechanically
  - Makes violations invisible to tooling
  - Reduces clarity of architectural intent

**Alternative 3: Separate Repositories**
- Maintain Rust and Python in completely separate repos
- Rejected because:
  - Creates synchronization overhead
  - Makes validated promotion harder
  - Reduces visibility across layers
  - Over-isolates research from production validation

### Rationale
- **Design Principle: Separation of discovery and enforcement** — Physical and automated separation operationalizes the philosophical principle
- **Design Principle: Determinism is a feature** — Preventing research leakage protects determinism guarantees
- **Design Principle: Explicit failure over hidden failure** — Automated checks surface violations immediately
- Prevention through structure is more reliable than prevention through discipline
- Layered enforcement (physical + process + automation + culture) provides defense in depth
- Clear boundaries reduce cognitive load and decision fatigue

### Implementation

#### Physical Structure
- Created `core/rust/` — Production inference engine only
- Created `research/python/` — Research and experimentation only
- Created comprehensive README.md in each directory defining responsibilities
- Added `.gitignore` to prevent accidental commits of build artifacts

#### Documentation
- Created [`rust-python-boundary-enforcement.md`](../02-system-architecture/rust-python-boundary-enforcement.md) — Detailed enforcement mechanisms
- Updated existing contract documentation with enforcement references
- Defined clear promotion workflow and artifacts

#### Automation
- Created `scripts/check_boundaries.sh` — Validates directory separation
- Created `scripts/validate_promotion.sh` — Checks promotion process compliance
- Created `.github/workflows/boundary-enforcement.yml` — CI/CD integration
- Automated detection of:
  - Python in Rust directories
  - Rust in Python directories (except FFI)
  - Shared code directories
  - Missing tests for new production code
  - Promotions without checklists

#### Process Gates
- Promotion requires completed [promotion checklist](../06-research-framework/promotion-checklist.md)
- New production code requires decision log entry
- CI blocks merges with boundary violations
- Code review checklist includes boundary validation

### Trade-offs and Consequences

**Accepted Costs:**
- Additional directory structure overhead
- Promotion process adds friction
- Enforcement scripts require maintenance
- Cannot "quickly prototype" in production
- May feel bureaucratic for small changes

**Benefits:**
- Prevents determinism violations
- Makes architectural intent explicit
- Reduces hidden dependencies
- Enables confident parallel development
- Protects against accidental violations
- Scales with team growth
- Reduces technical debt accumulation

**Expected Challenges:**
- Team must learn and internalize boundaries
- Initial setup and enforcement overhead
- Resistance to "extra process"
- False positives in automated checks

**Mitigation Strategies:**
- Clear documentation and examples
- Onboarding materials for new team members
- Regular boundary audits to refine enforcement
- Balance automation with pragmatism

### Success Metrics

Track monthly:
- Number of boundary violations caught in CI (should trend toward zero)
- Number of promotions (should be low and deliberate)
- Time from research to promotion (acceptable if thorough)
- Production incidents from boundary violations (should be zero)
- Team satisfaction with process (qualitative feedback)

Success indicators:
- Violations caught before merge, not after
- Promotions are rare and well-documented
- No production logic found in research
- No research logic found in production
- Team actively references boundary documentation

### Revisit Conditions

This decision should be revisited if:
- Enforcement creates excessive friction without preventing violations
- Technology changes make current enforcement obsolete (e.g., new languages)
- Alternative enforcement mechanisms prove more effective
- Team size or structure changes dramatically
- Automated checks have high false positive rate

This decision should NOT be revisited due to:
- "Too much process" complaints without specific violations prevented
- Desire for faster prototyping at expense of determinism
- Convenience arguments without safety justification

### References
- [Rust vs Python Contract](../02-system-architecture/rust-vs-python-contract.md) — Foundational contract
- [Rust Python Boundary Enforcement](../02-system-architecture/rust-python-boundary-enforcement.md) — Enforcement mechanisms
- [Promotion Checklist](../06-research-framework/promotion-checklist.md) — Required for all promotions
- [Design Principles](../00-vision-and-non-goals/design-principles.md) — Principle 5: Separation of discovery and enforcement
- `core/rust/README.md` — Production layer documentation
- `research/python/README.md` — Research layer documentation

---

# Decision Log Template

The following sections define the template for future decision log entries.

---

## Context Requirement

Context must describe:
- The problem being solved
- Constraints at the time
- Relevant documents or principles

Decisions without context are not defensible.

---

## Alternatives Requirement

At least one alternative must be documented.

Rejected alternatives should include:
- Why they were attractive
- Why they were rejected

This prevents rediscovery of rejected ideas.

---

## Rationale Requirement

Rationale must:
- Reference design principles
- Reference market theory where applicable
- Avoid outcome-based justification

Performance alone is insufficient.

---

## Trade-offs and Consequences

Every decision has costs.

These must be:
- Explicit
- Accepted knowingly
- Linked to future risk

Undocumented trade-offs are future failures.

---

## Revisit Conditions

Every decision must define:
- Conditions under which it should be revisited
- Signals that assumptions may be invalid

Decisions are not permanent.

---

## Governance Rules

- Decision logs are immutable
- Amendments require new entries
- Silent reversals are forbidden

---

## Review Cadence

Decision logs should be reviewed:
- During major refactors
- After significant failures
- When market structure changes

Memory decay is a systemic risk.

---

## Cultural Enforcement

Galactus values:
- Written justification
- Explicit uncertainty
- Reversibility

Strong opinions without records are discouraged.

---

## Final Statement

**Galactus remembers its decisions  
so it does not have to relearn them painfully.**
