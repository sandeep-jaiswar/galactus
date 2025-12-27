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
