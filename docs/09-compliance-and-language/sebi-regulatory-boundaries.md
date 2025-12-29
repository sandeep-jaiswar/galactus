# Galactus — SEBI Regulatory Boundaries

## Purpose of This Document

This document explicitly defines **how Galactus remains outside the regulatory scope of SEBI-regulated investment advisory services**.

It exists to:
- Clarify what constitutes SEBI-regulated investment advisory
- Document explicit operational boundaries that keep Galactus non-advisory
- Provide clear guidelines for maintaining regulatory safe harbor
- Establish enforceable boundaries for all stakeholders
- Protect the system from regulatory classification as an investment advisor

**These boundaries are permanent and non-negotiable.**

This document is part of the locked vision framework. See [`VISION_LOCK.md`](../../VISION_LOCK.md) for enforcement policy.

---

## SEBI Investment Advisors Regulations 2013 — Key Definitions

### What Constitutes "Investment Advice" Under SEBI

Under the **SEBI (Investment Advisers) Regulations, 2013**, investment advice means:

> "Any advice relating to investing in, purchasing, selling or otherwise dealing in securities or investment products, and advice on investment portfolio containing securities or investment products."

Investment advice includes:
- **Personalized recommendations** on buying, selling, or holding securities
- **Portfolio management advice** specific to an individual's circumstances
- **Asset allocation recommendations** tailored to individual risk profiles
- **Timing recommendations** on when to enter or exit investments
- **Security selection advice** for individual investors
- **Investment strategy recommendations** customized to personal goals

### Who Must Register as an Investment Advisor

Any person or entity that:
- Provides investment advice for consideration (direct or indirect)
- Holds themselves out as an investment advisor
- Provides advice in a fiduciary capacity
- Customizes recommendations based on individual circumstances
- Provides advice that influences investment decisions

### Exemptions from SEBI Registration

The following activities are **not** considered investment advice requiring registration:
- **General market commentary** without personalized recommendations
- **Educational content** about markets and investment principles
- **Factual information** about securities without recommendations
- **Analytical tools** that provide objective data without advice
- **Academic research** published for educational purposes
- **News and market analysis** presented neutrally without actionable recommendations

---

## Galactus Operational Boundaries (SEBI-Safe)

Galactus is designed to operate **exclusively within exempted activities** that do not trigger SEBI investment advisor registration requirements.

### Boundary 1 — No Personalized Recommendations

**SEBI Requirement:** Investment advice must be tailored to individual circumstances.

**Galactus Boundary:**
- All outputs are **market-level structural analysis**, never personalized
- No consideration of individual portfolios, risk profiles, or financial situations
- No user-specific customization of outputs or recommendations
- All analysis is identical regardless of who requests it
- System architecture prevents user-specific output generation

**Technical Enforcement:**
- API does not accept user portfolio data
- No user-specific state or preferences stored
- Output generation logic is identical for all users
- Validation layer rejects any personalized language

**Why This Keeps Us Safe:**  
Without personalization, outputs cannot be construed as advice tailored to individual circumstances—a core requirement for SEBI-regulated advisory.

---

### Boundary 2 — Structural Inference, Not Investment Recommendations

**SEBI Requirement:** Investment advice involves recommendations on buying, selling, or holding securities.

**Galactus Boundary:**
- Outputs describe **capital pressure and market structure**, not investment actions
- No buy/sell/hold recommendations ever provided
- No entry/exit timing suggestions
- No position sizing or allocation advice
- No recommendations on which securities to trade

**Technical Enforcement:**
- Forbidden output categories documented in [`output-restrictions.md`](output-restrictions.md)
- API response validation blocks trading instructions
- Static analysis prevents advisory language in code
- Kill-switch triggers on detected violations

**Why This Keeps Us Safe:**  
Structural analysis that describes market conditions without recommending actions is educational/informational, not advisory.

---

### Boundary 3 — No Price Predictions or Return Expectations

**SEBI Requirement:** Investment advice often includes expected returns or price targets.

**Galactus Boundary:**
- No price targets, price forecasts, or price predictions
- No expected return calculations
- No upside/downside potential estimates
- No profit/loss projections
- Focus exclusively on capital constraints and pressure measurements

**Technical Enforcement:**
- Price prediction logic explicitly forbidden in codebase
- Validation rejects outputs resembling price targets
- Metrics limited to structural measurements (see [`signal-philosophy.md`](../04-signal-and-metrics/signal-philosophy.md))
- No optimization for prediction accuracy

**Why This Keeps Us Safe:**  
Without price predictions or return expectations, outputs cannot be construed as investment recommendations based on expected outcomes.

---

### Boundary 4 — No Fiduciary Relationship or Consideration

**SEBI Requirement:** Investment advisors operate in a fiduciary capacity and receive consideration (payment) for advice.

**Galactus Boundary:**
- No fiduciary relationship established with users
- Outputs provided as general information, not fiduciary advice
- No duty of care or suitability assessment
- Users explicitly informed that outputs are not personalized advice
- No advisory fees charged for personalized recommendations

**Technical Enforcement:**
- Terms of service explicitly disclaim fiduciary relationship
- Disclaimers on all outputs (see [`disclaimer-standards.md`](disclaimer-standards.md))
- No collection of data necessary for fiduciary duty
- No assessment of user suitability for investments

**Why This Keeps Us Safe:**  
Without fiduciary duty or personalized consideration, the relationship remains informational rather than advisory.

---

### Boundary 5 — Educational and Analytical Purpose Only

**SEBI Requirement:** Educational content and market analysis are exempt from regulation.

**Galactus Boundary:**
- System purpose is **education about market structure** and capital behavior
- Outputs explain market mechanics, not prescribe actions
- All content is analytical and educational in nature
- Focus on teaching users how capital constraints work
- Promotes informed decision-making rather than providing decisions

**Technical Enforcement:**
- Vision documents establish educational mission
- Language guidelines enforce educational tone (see [`language-guidelines.md`](language-guidelines.md))
- Outputs include mechanistic explanations
- Uncertainty explicitly acknowledged

**Why This Keeps Us Safe:**  
Educational content that helps users understand markets, without telling them what to do, falls outside advisory scope.

---

### Boundary 6 — No Portfolio Management or Execution

**SEBI Requirement:** Portfolio management and execution services are regulated.

**Galactus Boundary:**
- No portfolio management capabilities
- No order execution or broker integration
- No position tracking or portfolio monitoring
- No rebalancing recommendations
- No discretionary or non-discretionary portfolio management

**Technical Enforcement:**
- Architecture explicitly excludes execution logic (see [`explicit-non-goals.md`](../00-vision-and-non-goals/explicit-non-goals.md))
- No broker API integration
- No portfolio construction logic
- No position sizing algorithms

**Why This Keeps Us Safe:**  
Without portfolio management or execution capabilities, Galactus cannot perform regulated advisory functions.

---

### Boundary 7 — Objective Data and Analysis Only

**SEBI Requirement:** Pure data provision and objective analysis are generally exempt.

**Galactus Boundary:**
- Outputs based on **objective, observable market data**
- Measurements and classifications, not opinions or recommendations
- Quantified metrics with explicit confidence levels
- Replicable, deterministic analysis methodology
- No subjective judgments or proprietary opinions presented as advice

**Technical Enforcement:**
- All signals derived from public data (see [`data-sources.md`](../03-data-and-schemas/data-sources.md))
- Deterministic calculation methods documented
- Confidence scoring system (see [`intent-engine-overview.md`](../05-intent-engine/intent-engine-overview.md))
- No blackbox or subjective interpretations

**Why This Keeps Us Safe:**  
Objective data analysis without recommendations or opinions is informational, not advisory.

---

## What Galactus IS (SEBI-Compliant)

Galactus operates as:

✅ **Market Structure Analysis Tool**  
- Analyzes capital constraints and pressure
- Measures observable market structure
- Classifies regime states

✅ **Educational Platform**  
- Teaches users about capital behavior mechanics
- Explains market structure concepts
- Provides learning resources about forced flows

✅ **Analytical Information Service**  
- Delivers objective data analysis
- Provides statistical measurements
- Reports quantified market observations

✅ **Research Infrastructure**  
- Supports academic and systematic research
- Provides tools for market structure investigation
- Enables empirical hypothesis testing

---

## What Galactus IS NOT (Would Require SEBI Registration)

Galactus explicitly avoids:

❌ **Investment Advisory Service**  
- Does not provide personalized investment advice
- Does not recommend buying, selling, or holding securities
- Does not tailor outputs to individual circumstances

❌ **Portfolio Management Service**  
- Does not manage portfolios
- Does not provide asset allocation advice
- Does not execute trades or interface with brokers

❌ **Trading Signal Service**  
- Does not generate buy/sell signals
- Does not recommend entry/exit points
- Does not suggest position sizes

❌ **Financial Planning Service**  
- Does not assess individual financial situations
- Does not provide personalized financial recommendations
- Does not create investment plans for individuals

❌ **Predictive Advisory Service**  
- Does not predict prices or returns
- Does not forecast investment outcomes
- Does not promise or imply future performance

---

## Regulatory Safe Harbor Maintenance

To maintain exemption from SEBI registration, Galactus must:

### Operational Requirements

1. **Never Cross Personalization Line**
   - All outputs remain general market-level analysis
   - No user-specific customization permitted
   - No collection of personal financial information

2. **Maintain Educational Character**
   - Emphasize learning and understanding
   - Explain mechanics, don't prescribe actions
   - Focus on market structure education

3. **Avoid Advisory Language**
   - Enforce forbidden language restrictions
   - Use structural, not prescriptive terminology
   - Maintain analytical, neutral tone

4. **Include Required Disclaimers**
   - Every output includes disclaimer
   - Clarifies non-advisory nature
   - Directs users to seek professional advice

5. **Document and Enforce Boundaries**
   - Maintain this document and related policies
   - Train all stakeholders on boundaries
   - Enforce through technical and procedural controls

### Monitoring and Compliance

1. **Continuous Output Validation**
   - API response scanning for advisory language
   - Automated detection of boundary violations
   - Kill-switch for critical breaches

2. **Periodic Compliance Review**
   - Quarterly audit of system outputs
   - Review of user-facing materials
   - Assessment of regulatory landscape changes

3. **Legal Review Process**
   - Annual legal review of compliance posture
   - Review of SEBI regulation changes
   - Update boundaries as needed

4. **Documentation Maintenance**
   - Keep compliance documentation current
   - Update enforcement mechanisms
   - Log all boundary decisions

---

## Violation Response Protocol

### Internal Violation (Development/Testing)

1. Immediately halt deployment
2. Identify root cause of boundary breach
3. Implement technical fix and process improvement
4. Review all similar code paths
5. Update validation mechanisms
6. Document in decision log

### External Violation (Production)

1. **Critical:** Trigger kill-switch immediately (see [`kill-switch.md`](../08-risk-and-failure-modes/kill-switch.md))
2. **High:** Disable affected API endpoint, assess scope
3. **Medium:** Flag for manual review, implement hotfix
4. **Low:** Log for compliance review, schedule fix

All violations require:
- Root cause analysis
- Legal assessment of regulatory implications
- Prevention mechanism updates
- Decision log entry

### User Interpretation Risk

If users misinterpret outputs as investment advice despite boundaries:

1. Review output language for clarity
2. Strengthen disclaimers if needed
3. Improve educational content
4. Consider user communication
5. Document incident and response

**Key Principle:** We cannot control user interpretation, but we can control what we output and how we position it.

---

## SEBI Regulatory Change Monitoring

### Monitoring Protocol

1. **Quarterly Review** of SEBI regulations and circulars
2. **Immediate Assessment** of any regulatory announcements affecting:
   - Definition of investment advice
   - Exemptions from registration
   - Technology-enabled advisory services
   - Data analytics and fintech regulations

3. **Legal Consultation** when regulatory changes are identified
4. **Proactive Adjustment** of boundaries if necessary

### Escalation Path

1. **Regulatory change identified** → Legal review
2. **Boundary impact assessed** → Risk committee evaluation
3. **Changes required** → Update documentation and systems
4. **Material changes** → Stakeholder notification
5. **Critical changes** → System modification or sunset evaluation

---

## SEBI vs. Non-SEBI Matrix

| Activity | SEBI-Regulated | Galactus Position | Boundary Maintained By |
|----------|----------------|-------------------|------------------------|
| Personalized investment recommendations | ✅ Yes | ❌ Never does | API architecture, validation |
| General market analysis | ❌ No | ✅ Core function | Language guidelines |
| Portfolio management | ✅ Yes | ❌ Explicitly excluded | System design, non-goals |
| Buy/sell/hold recommendations | ✅ Yes | ❌ Forbidden outputs | Output restrictions, validation |
| Educational content | ❌ No | ✅ Primary purpose | Vision documents, tone |
| Price predictions for advice | ✅ Yes | ❌ Never provides | Signal philosophy, enforcement |
| Objective data analysis | ❌ No | ✅ Core capability | Technical architecture |
| Asset allocation advice | ✅ Yes | ❌ Forbidden | Output restrictions |
| Trade execution | ✅ Yes | ❌ Not in scope | Explicit non-goals |
| Market structure research | ❌ No | ✅ Core mission | Vision documents |

---

## Stakeholder Responsibilities

### Development Team

- Understand and enforce SEBI boundaries
- Implement technical controls preventing violations
- Review code changes for boundary compliance
- Escalate uncertain cases for legal review

### Operations Team

- Monitor outputs for boundary violations
- Execute violation response protocols
- Conduct periodic compliance audits
- Maintain documentation currency

### Legal Counsel

- Review regulatory changes quarterly
- Assess boundary adequacy annually
- Advise on uncertain cases
- Update documentation as needed

### Leadership

- Maintain commitment to boundary preservation
- Resist pressure to cross into advisory territory
- Support compliance investments
- Make final decisions on material changes

---

## Related Documents

### Core Compliance Framework
- [`output-restrictions.md`](output-restrictions.md) — Forbidden output categories
- [`language-guidelines.md`](language-guidelines.md) — Allowed and forbidden language
- [`disclaimer-standards.md`](disclaimer-standards.md) — Required disclaimers

### Vision and Boundaries
- [`vision.md`](../00-vision-and-non-goals/vision.md) — Core system vision
- [`explicit-non-goals.md`](../00-vision-and-non-goals/explicit-non-goals.md) — What Galactus will never do
- [`system-boundaries.md`](../02-system-architecture/system-boundaries.md) — Technical system boundaries

### Risk Management
- [`known-risks.md`](../08-risk-and-failure-modes/known-risks.md) — Identified system risks
- [`kill-switch.md`](../08-risk-and-failure-modes/kill-switch.md) — Emergency shutdown criteria

---

## Legal Disclaimer

**This document is not legal advice.**

It represents Galactus's understanding and interpretation of SEBI regulations as of the document date. 

Compliance with SEBI regulations requires:
- Ongoing monitoring of regulatory changes
- Consultation with qualified legal counsel
- Adaptation to regulatory guidance and precedents
- Assessment of actual system operations (not just documented intent)

Any questions about regulatory compliance should be directed to qualified legal professionals with expertise in Indian securities law and SEBI regulations.

---

## Revision Policy

This document may be updated to:
- Reflect changes in SEBI regulations or guidance
- Strengthen boundary enforcement mechanisms
- Clarify edge cases or uncertain scenarios
- Incorporate legal counsel recommendations
- Respond to regulatory inquiries or interpretations

Changes relaxing boundaries require:
- Legal review and explicit approval
- Risk assessment of regulatory implications
- Formal decision by leadership
- Documentation in decision log with full rationale
- Acknowledgment of increased compliance obligations

**Temporary exceptions to boundaries are not permitted.**

---

## Final Statement

**SEBI regulatory boundaries are non-negotiable.**

They exist to:
- Protect Galactus from regulatory classification as an investment advisor
- Maintain operational freedom as an analytical and educational platform
- Preserve the non-advisory character essential to the core mission
- Avoid compliance obligations incompatible with system design
- Protect users from misinterpretation of outputs as personalized advice

**Galactus analyzes market structure.**  
**Galactus does not provide investment advice.**  
**Galactus does not recommend securities.**  
**Galactus does not manage portfolios.**

**These boundaries preserve both regulatory safe harbor and system integrity.**

**This boundary is permanent.**

---

## Document Information

**Last Updated:** 2025-12-29  
**Version:** 1.0  
**Review Frequency:** Quarterly  
**Next Review:** Q2 2025  
**Owner:** Legal & Compliance  
**Approvers:** Legal Counsel, Leadership
