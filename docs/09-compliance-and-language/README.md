# Compliance and Language Documentation

This directory contains all documentation related to regulatory compliance, output restrictions, and language guidelines for Galactus.

---

## Purpose

These documents exist to:
- Maintain SEBI regulatory safe harbor status
- Prevent misinterpretation of outputs as investment advice
- Enforce clear boundaries between structural analysis and advisory services
- Provide enforceable guidelines for all system outputs
- Protect users and the system from regulatory liability

**All documents in this directory are part of the locked vision framework.**

See [`VISION_LOCK.md`](../../VISION_LOCK.md) for enforcement policy.

---

## Documents in This Directory

### 📋 [`sebi-regulatory-boundaries.md`](sebi-regulatory-boundaries.md) — **START HERE**
**Defines how Galactus remains outside SEBI-regulated investment advisory scope.**

This is the foundational compliance document that:
- Explains SEBI Investment Advisors Regulations 2013
- Documents explicit operational boundaries
- Establishes what Galactus IS and IS NOT
- Provides regulatory safe harbor maintenance guidelines
- Defines violation response protocols

**Key takeaway:** Galactus operates as an educational and analytical platform, not an investment advisor.

---

### 🚫 [`output-restrictions.md`](output-restrictions.md)
**Defines forbidden output categories that Galactus must never emit.**

This document specifies hard restrictions on:
- Trading instructions (buy/sell/hold)
- Price targets and predictions
- Position sizing and allocation advice
- Timing and urgency language
- Performance promises
- Personalized recommendations

Includes enforcement mechanisms and validation rules.

**Key takeaway:** These restrictions are permanent and non-negotiable.

---

### 💬 [`language-guidelines.md`](language-guidelines.md)
**Defines allowed and forbidden language for all Galactus outputs.**

This document provides:
- Forbidden language categories (trading actions, directional bias, timing, etc.)
- Allowed language categories (structural inference, regime classification, etc.)
- Tone requirements for neutral, analytical communication
- Edge case clarifications
- Output examples (forbidden vs. allowed)

**Key takeaway:** Language shapes liability—every word choice matters.

---

### ⚠️ [`disclaimer-standards.md`](disclaimer-standards.md)
**Defines mandatory disclaimer language to prevent misinterpretation.**

This document establishes:
- Core disclaimer principles
- Mandatory disclaimer elements
- Placement rules for disclaimers
- Integration with regulatory boundaries

**Key takeaway:** Clear disclaimers prevent regulatory ambiguity.

---

## How These Documents Work Together

```
┌─────────────────────────────────────────┐
│  SEBI Regulatory Boundaries             │  ← Foundation: Legal framework
│  (What we can/cannot be)                │
└──────────────┬──────────────────────────┘
               │
               ├──────────────────────────────┐
               ↓                              ↓
┌──────────────────────────┐  ┌──────────────────────────┐
│  Output Restrictions     │  │  Language Guidelines     │  ← Implementation
│  (What we cannot say)    │  │  (How we must say it)    │
└──────────┬───────────────┘  └──────────┬───────────────┘
           │                              │
           └──────────────┬───────────────┘
                          ↓
           ┌──────────────────────────┐
           │  Disclaimer Standards    │  ← Presentation
           │  (How we frame it)       │
           └──────────────────────────┘
```

1. **SEBI Regulatory Boundaries** establishes the legal framework and operational boundaries
2. **Output Restrictions** translates those boundaries into forbidden output categories
3. **Language Guidelines** provides specific allowed and forbidden terminology
4. **Disclaimer Standards** ensures proper framing of all outputs

---

## Quick Reference: What You Need to Know

### For Developers
- Review **output-restrictions.md** before writing any output generation code
- Check **language-guidelines.md** for specific terminology rules
- Ensure **sebi-regulatory-boundaries.md** boundaries are maintained in system design
- Include disclaimers per **disclaimer-standards.md** in all user-facing outputs

### For Compliance/Legal
- **sebi-regulatory-boundaries.md** is the primary compliance document
- Review this document quarterly for regulatory changes
- Assess all documents when SEBI regulations change
- Consult legal counsel for material changes

### For Product/Business
- **sebi-regulatory-boundaries.md** defines what features are possible
- Any feature conflicting with these documents must be rejected
- These boundaries are non-negotiable and permanent
- Commercial pressure cannot override compliance requirements

### For Operations
- Monitor outputs for compliance with these guidelines
- Implement kill-switch protocols for critical violations
- Conduct periodic audits per compliance requirements
- Report violations immediately

---

## Enforcement

All documents in this directory are enforced through:

### Technical Controls
- API response validation
- Forbidden term detection
- Static analysis of code
- CI/CD pipeline checks
- Kill-switch for critical violations

### Procedural Controls
- Code review requirements
- Quarterly compliance audits
- Legal review process
- Violation response protocols
- Documentation maintenance schedule

See individual documents for specific enforcement mechanisms.

---

## Relationship to Other Documentation

### Vision and Non-Goals
- [`vision.md`](../00-vision-and-non-goals/vision.md) — Core system vision
- [`explicit-non-goals.md`](../00-vision-and-non-goals/explicit-non-goals.md) — What Galactus will never do
- [`design-principles.md`](../00-vision-and-non-goals/design-principles.md) — How decisions are made

### System Architecture
- [`system-boundaries.md`](../02-system-architecture/system-boundaries.md) — Technical boundaries
- [`high-level-design.md`](../02-system-architecture/high-level-design.md) — System architecture

### Risk Management
- [`known-risks.md`](../08-risk-and-failure-modes/known-risks.md) — Identified risks
- [`kill-switch.md`](../08-risk-and-failure-modes/kill-switch.md) — Emergency shutdown

---

## Revision Policy

These documents may be updated to:
- Reflect changes in SEBI regulations
- Strengthen enforcement mechanisms
- Clarify edge cases
- Add newly identified forbidden patterns
- Incorporate legal counsel recommendations

**Relaxation of restrictions requires:**
- Legal review and explicit approval
- Risk assessment of regulatory implications
- Leadership decision and rationale
- Decision log entry
- Stakeholder notification

**Temporary exceptions are not permitted.**

---

## Critical Principles

1. **Galactus analyzes market structure** — It does not provide investment advice
2. **Galactus describes capital pressure** — It does not recommend trades
3. **Galactus infers constraints** — It does not predict prices
4. **Galactus educates users** — It does not personalize advice
5. **Galactus maintains boundaries** — It does not cross into SEBI-regulated advisory

**These principles are permanent and enforced through the documents in this directory.**

---

## Contact and Escalation

For questions or concerns about compliance:

- **Regulatory interpretation:** Legal counsel
- **Technical enforcement:** Development team lead
- **Violation response:** Operations team
- **Policy changes:** Compliance committee

**When in doubt, err on the side of caution and escalate.**

---

## Document Information

**Last Updated:** 2025-12-29  
**Review Frequency:** Quarterly  
**Next Review:** Q2 2025  
**Owner:** Legal & Compliance
