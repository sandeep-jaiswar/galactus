# Galactus — Output Restrictions

## Purpose of This Document

This document defines **forbidden output categories** that Galactus must never emit in any form.

It exists to:
- Prevent actionable trading recommendations that could be construed as investment advice
- Maintain regulatory safe harbor as an informational system
- Protect users from misinterpretation of outputs as trading instructions
- Preserve Galactus's identity as an inference engine, not an advisory service
- Enforce clear boundaries between structural analysis and actionable advice

**These restrictions are permanent and non-negotiable.**

Any violation of output restrictions is considered a **critical system failure** requiring immediate intervention.

This document is part of the locked vision framework. See [`VISION_LOCK.md`](../../VISION_LOCK.md) for enforcement policy.

---

## Core Principle

**Galactus outputs must describe market structure, not prescribe market actions.**

All outputs must:
- Describe observable capital constraints and pressures
- Avoid implying recommended actions or decisions
- Never suggest specific trading positions or timing
- Never promise or estimate financial outcomes
- Never personalize advice to individual users

---

## Category 1 — Trading Instructions (HARD FORBIDDEN)

### Forbidden Outputs

Galactus may **never** output:

- **Direct Trading Actions:**
  - Buy / Sell instructions
  - Long / Short recommendations
  - Enter / Exit signals
  - Add to / Reduce position
  - Open / Close position instructions
  - Accumulate / Distribute recommendations
  - Scale in / Scale out suggestions
  - Take profit instructions
  - Stop loss recommendations
  - Execute / Trade commands
  - Hold / Wait instructions (when implying portfolio action)

- **Derivatives Trading Instructions:**
  - Buy call / Buy put
  - Sell call / Sell put
  - Spread recommendations
  - Option strategy suggestions
  - Futures position recommendations
  - Roll instructions
  - Hedge position commands

- **Action Verbs:**
  - "You should [action]"
  - "We recommend [action]"
  - "Consider [action]"
  - "Take [action]"
  - "Execute [action]"

### Why Forbidden

Trading instructions cross the regulatory boundary from informational to advisory.

They imply that Galactus:
1. Knows the user's financial situation
2. Has considered their risk tolerance
3. Is providing personalized investment advice
4. Recommends specific portfolio actions

All of these are false and create regulatory liability.

### Enforcement

- API response validation before delivery
- Metadata scanning for action verbs
- Static analysis of code for trading terminology
- CI checks for forbidden terms in comments and documentation

---

## Category 2 — Price Targets & Predictions (HARD FORBIDDEN)

### Forbidden Outputs

Galactus may **never** output:

- **Explicit Price Targets:**
  - Target price: ₹X
  - Price objective: $Y
  - Expected price: Z
  - Fair value estimate
  - Intrinsic value calculation
  - Fundamental price target
  - Technical price target

- **Price Range Predictions:**
  - Price range: ₹X to ₹Y
  - Expected trading range
  - Support / Resistance levels (when presented as trading targets)
  - Breakout targets
  - Measured move projections

- **Directional Price Predictions:**
  - Price will rise to X
  - Expected to reach Y
  - Should fall to Z
  - Projected to move to W
  - Likely to hit target
  - Anticipated price movement

- **Return Predictions:**
  - Expected return: X%
  - Target return: Y%
  - Upside potential: Z%
  - Downside risk: W%
  - Risk/reward ratio (when used prescriptively)
  - Return forecast

### Why Forbidden

Price predictions:
1. Are inherently uncertain and cannot be guaranteed
2. Create false expectations of accuracy
3. Imply Galactus has predictive capability it does not possess
4. Shift focus from capital mechanics to price outcomes
5. Create liability when predictions fail

Galactus infers capital pressure, not price outcomes.

### Enforcement

- Numeric output validation (reject outputs resembling price levels)
- Response metadata scanning for target/prediction terminology
- CI checks preventing price-related numeric outputs without structural context
- Manual review of any numeric values representing price levels

---

## Category 3 — Position Sizing & Allocation (HARD FORBIDDEN)

### Forbidden Outputs

Galactus may **never** output:

- **Position Size Recommendations:**
  - Position size: X shares
  - Allocate Y% of capital
  - Use Z lots
  - Recommended quantity: W units
  - Optimal position size
  - Capital allocation: X%

- **Risk Sizing:**
  - Risk X% of portfolio
  - Use Y% risk per trade
  - Allocate Z% to position
  - Maximum position size: W
  - Suggested allocation
  - Portfolio weight: X%

- **Leverage Recommendations:**
  - Use X leverage
  - Margin: Y%
  - Leverage ratio: Z:1
  - Recommended exposure: W
  - Maximum leverage: X

- **Portfolio Construction:**
  - Asset allocation percentages
  - Diversification recommendations
  - Portfolio rebalancing instructions
  - Concentration limits
  - Exposure recommendations

### Why Forbidden

Position sizing requires knowledge of:
1. User's total capital
2. Risk tolerance and constraints
3. Existing portfolio positions
4. Personal financial situation
5. Investment objectives and time horizon

Galactus has none of this information and must never pretend it does.

Providing position sizing is **personalized investment advice** requiring fiduciary duty.

### Enforcement

- Rejection of any numeric outputs representing portfolio quantities or percentages
- Scanning for allocation/sizing terminology in responses
- CI validation preventing percentage-based recommendations
- Static analysis preventing position sizing logic in code

---

## Category 4 — Timing & Urgency (HARD FORBIDDEN)

### Forbidden Outputs

Galactus may **never** output:

- **Time-Sensitive Action Prompts:**
  - Act now
  - Immediate action required
  - Don't miss this opportunity
  - Last chance to [action]
  - Window closing
  - Time running out
  - Move quickly
  - Before it's too late
  - Get in now
  - Exit immediately

- **Event Timing Predictions:**
  - Price will move by [date]
  - Action required before [time]
  - Opportunity expires on [date]
  - Move will happen within [timeframe]
  - Deadline for action: [time]

- **Optimal Timing Claims:**
  - Now is the best time to [action]
  - Perfect entry point
  - Ideal exit opportunity
  - Optimal time to act
  - Right moment to [action]

### Why Forbidden

Timing and urgency:
1. Create psychological pressure to act
2. Imply Galactus knows optimal action timing
3. Manipulate decision-making through false scarcity
4. Cross into persuasive territory incompatible with neutral analysis

### Enforcement

- Scanning for urgency keywords in responses
- Temporal validation (reject outputs with action deadlines)
- CI checks for urgency language
- Manual review of time-sensitive outputs

---

## Category 5 — Performance Promises (HARD FORBIDDEN)

### Forbidden Outputs

Galactus may **never** output:

- **Return Guarantees:**
  - Guaranteed return: X%
  - Assured profit
  - Expected gain: Y%
  - Projected returns: Z%
  - Performance warranty
  - Promised outcome

- **Risk Elimination Claims:**
  - Risk-free opportunity
  - No downside
  - Protected investment
  - Safe bet
  - Guaranteed not to lose
  - Assured safety

- **Performance Comparisons:**
  - Will beat market by X%
  - Outperform benchmark
  - Alpha generation: Y%
  - Expected to exceed returns
  - Superior to alternatives

- **Success Probability:**
  - X% probability of profit
  - Y% chance of success
  - Z% win rate
  - Success guarantee
  - High probability trade

### Why Forbidden

Performance promises:
1. Are legally and ethically prohibited
2. Create false expectations
3. Cannot be delivered in uncertain markets
4. Constitute fraud when outcomes differ
5. Fundamentally misrepresent market uncertainty

No system can guarantee financial outcomes. Claims otherwise are fraudulent.

### Enforcement

- Scanning for guarantee/promise terminology
- Validation rejecting probability-of-success claims
- CI checks for performance promise language
- Legal review of outputs claiming performance

---

## Category 6 — Personalized Recommendations (HARD FORBIDDEN)

### Forbidden Outputs

Galactus may **never** output:

- **Individual Advice:**
  - Best for your portfolio
  - Suited to your risk profile
  - Ideal for your situation
  - Matches your goals
  - Right for you
  - You should [action]
  - We recommend for you

- **Portfolio-Specific Guidance:**
  - Based on your holdings
  - Complements your positions
  - Hedges your exposure
  - Rebalance your portfolio
  - Adjust your allocation

- **Risk Profile Assumptions:**
  - For aggressive investors
  - Conservative approach would be
  - If you're risk-averse, [action]
  - Suitable for high-risk tolerance
  - Appropriate for your risk level

### Why Forbidden

Personalization:
1. Requires fiduciary relationship
2. Implies understanding of individual circumstances
3. Crosses into regulated investment advice
4. Creates duty of care Galactus cannot fulfill
5. Fundamentally misrepresents system capabilities

Galactus operates at market-structure level, not individual level.

### Enforcement

- Scanning for personalization keywords (you, your, we recommend)
- Rejection of user-specific output customization
- CI validation preventing personalized response generation
- Architecture review preventing user-specific advice paths

---

## Allowed Outputs (For Contrast)

To clarify boundaries, these outputs **are permitted**:

### Structural Descriptions

- "Capital pressure measured at 73rd percentile"
- "Rollover obligation estimated at 2.1σ above normal"
- "Open interest concentration at 19,000 strike"
- "Basis convergence pressure detected"
- "Hedge rebalancing constraint active"

### Regime Classifications

- "High-pressure regime"
- "Low-confidence regime"
- "Structural stability regime"
- "Constraint-driven regime"

### Mechanistic Explanations

- "Settlement mechanics force basis convergence"
- "Delta hedging propagates pressure to spot"
- "Margin requirements create forced unwinding"
- "Arbitrage bounds enforce price relationships"

### Uncertainty Acknowledgment

- "Confidence degraded due to data quality"
- "Insufficient evidence for inference"
- "Regime classification ambiguous"
- "Cannot determine with available data"

### Historical Context

- "Similar pressure configurations preceded settlement dislocation"
- "Historical episodes showed 3-5 day persistence"
- "Empirically observed behavior in Q3 2023"

**Key Difference:** These describe **what is**, not **what to do**.

---

## Validation Rules

### Pre-Delivery Validation

All outputs must pass validation before delivery:

1. **Forbidden Term Scan**
   - Check for any forbidden terms from Categories 1-6
   - Reject output if forbidden term detected
   - Log violation for review

2. **Numeric Context Validation**
   - Any numeric value must have structural context
   - Reject standalone numbers resembling prices or returns
   - Require explicit units and structural explanation

3. **Action Verb Detection**
   - Scan for imperative action verbs
   - Validate absence of "should", "must", "recommend"
   - Check for disguised instructions

4. **Personalization Detection**
   - Scan for second-person pronouns (you, your)
   - Validate absence of user-specific customization
   - Ensure market-level (not individual-level) outputs

5. **Urgency Detection**
   - Check for time-pressure keywords
   - Validate absence of action deadlines
   - Ensure temporal context is descriptive, not prescriptive

### Validation Implementation

Validation must occur at multiple layers:

- **API Layer:** Before response serialization
- **Intent Engine:** Before output generation
- **Static Analysis:** During development
- **CI/CD:** Before deployment
- **Runtime Monitoring:** Continuous output sampling

---

## Enforcement Mechanisms

### Development Phase

1. **Code Review Checklist**
   - Verify no forbidden terms in code or comments
   - Check output generation logic for compliance
   - Validate test outputs follow restrictions

2. **Static Analysis**
   - Automated scanning of source code for forbidden terms
   - Detection of output generation patterns that could violate restrictions
   - Linting rules enforcing compliant output structure

3. **Unit Tests**
   - Test cases validating rejection of forbidden outputs
   - Negative tests ensuring validation catches violations
   - Compliance test suite run on every commit

### CI/CD Phase

1. **Pre-Merge Validation**
   - CI job scanning all code changes for forbidden terms
   - Automated review of documentation updates
   - Hard gate preventing merge of non-compliant code

2. **Build-Time Checks**
   - Validation of example outputs in tests
   - Scanning of log statements and error messages
   - Documentation compliance verification

### Runtime Phase

1. **API Response Validation**
   - Every API response validated before delivery
   - Forbidden term detection in response body
   - Metadata scanning for non-compliant fields

2. **Monitoring & Alerting**
   - Continuous sampling of outputs
   - Alert on any detected violations
   - Automatic kill-switch trigger on critical violations

3. **Audit Logging**
   - Log all outputs for periodic compliance review
   - Flag suspicious outputs for manual review
   - Track validation failures and near-misses

### Kill-Switch Criteria

Automatic system shutdown (kill-switch activation) occurs if:

- Forbidden output detected in production API response
- Validation bypass or tampering detected
- Multiple validation failures in short time window
- Manual override of output restrictions attempted

See [`docs/08-risk-and-failure-modes/kill-switch.md`](../08-risk-and-failure-modes/kill-switch.md) for full kill-switch specification.

---

## Violation Response

### Development Violation

1. Reject commit or pull request
2. Require code revision
3. Document violation in review
4. Update prevention mechanisms if needed

### Testing Violation

1. Fail test suite
2. Block deployment
3. Require output revision
4. Review intent engine logic

### Production Violation

1. **Critical:** Trigger kill-switch immediately
2. **High:** Disable affected API endpoint
3. **Medium:** Alert operations team, manual review required
4. **Low:** Log for audit review

All violations require root cause analysis and prevention update.

---

## Examples

### ❌ FORBIDDEN Output Example

```
STRONG BUY SIGNAL - NIFTY 19,000 CALL

Target Price: ₹250
Stop Loss: ₹180
Position Size: 10% of portfolio
Expected Return: 25% in 30 days

This is an excellent opportunity with limited downside.
Act now before the window closes!

Confidence: 95% - Don't miss this trade!
```

**Violations:**
- Trading instruction (BUY)
- Price target (₹250)
- Stop loss (₹180)
- Position sizing (10%)
- Return estimate (25%)
- Urgency (act now)
- Personalization (implied)
- Performance promise (95% confidence on outcome)

---

### ✅ ALLOWED Output Example

```
Structural Analysis: NIFTY DEC Futures

Capital Pressure: 73rd percentile (historical range)
Direction: Negative (selling pressure)
Confidence: 0.72 (moderate, degraded from data sparsity)

Regime Classification: High-pressure, stable liquidity

Observed Structural Constraints:
- Near-term OI concentration: 68% at 19,000 strike
- Rollover pressure: 2.1σ above historical normal
- Delta hedging obligations: Elevated
- Basis: -12 points (contango, within normal range)

Mechanistic Context:
Settlement convergence mechanics active. Given current OI structure,
forced position adjustment may propagate pressure through calendar
spreads. Historical episodes with similar configurations showed
pressure persistence for 3-5 days (median).

Uncertainty Factors:
- Foreign flow data delayed 24 hours
- Volatility regime classification: Ambiguous
- Far strike data: Insufficient for full curve analysis

Confidence Degradation Triggers:
- Regime shift detection
- Data quality deterioration below threshold
- Structural assumption violation

Inference Valid Until: Settlement date or regime shift (whichever first)

---
DISCLAIMER: This output provides structural inference about capital
pressure and market mechanics. It does not constitute investment advice,
trading recommendation, or personalized guidance. Capital allocation
decisions require consideration of individual circumstances, risk
tolerance, and factors outside this system's scope.
```

**Compliant because:**
- Describes structure, not actions
- Quantifies measurements objectively
- Acknowledges uncertainty explicitly
- Explains mechanics without prescribing decisions
- No price targets or return estimates
- No position sizing or timing advice
- Includes required disclaimer
- Maintains analytical distance

---

## Related Documents

- [`language-guidelines.md`](language-guidelines.md) — Comprehensive language rules
- [`disclaimer-standards.md`](disclaimer-standards.md) — Required disclaimer formats
- [`sebi-regulatory-boundaries.md`](sebi-regulatory-boundaries.md) — SEBI compliance and regulatory safe harbor
- [`explicit-non-goals.md`](../00-vision-and-non-goals/explicit-non-goals.md) — System-level non-goals
- [`forbidden-metrics.md`](../04-signal-and-metrics/forbidden-metrics.md) — Forbidden measurement types
- [`kill-switch.md`](../08-risk-and-failure-modes/kill-switch.md) — Automatic shutdown criteria
- [`VISION_LOCK.md`](../../VISION_LOCK.md) — Vision immutability enforcement

---

## Revision Policy

This document may be updated to:
- Add newly identified forbidden output patterns
- Strengthen enforcement mechanisms
- Clarify edge cases with additional examples
- Improve validation rules

Relaxation of restrictions requires:
- Legal review and explicit approval
- Documentation of regulatory implications
- Stakeholder acknowledgment of liability changes
- Decision log entry with full rationale
- Vision document amendment

**Temporary exceptions are not permitted.**

---

## Final Statement

**Output restrictions are not negotiable.**

They exist to:
- Protect users from misleading information
- Preserve regulatory safe harbor status
- Maintain system integrity and identity
- Prevent liability from advisory interpretation

Every output must pass validation.  
Every violation must trigger response.  
Every exception must be documented and justified.

**Galactus infers capital pressure.**  
**Galactus does not instruct trades.**  
**Galactus does not predict prices.**  
**Galactus does not size positions.**

**This boundary is permanent.**
