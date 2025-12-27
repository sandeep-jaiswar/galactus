# Galactus — Forced Flow Detection

## Purpose of This Document

This document defines **forced flow** within Project Galactus and describes how the Intent Engine detects it.

It exists to:
- Distinguish mandatory capital action from discretionary behavior
- Prevent misclassification of reactive moves as intent
- Enable reliable identification of structural pressure

Forced flow is the most reliable source of inference in Galactus.

---

## Definition: Forced Flow

In Galactus, **forced flow** is defined as:

> Capital movement that must occur due to binding constraints, regardless of price, opinion, or preference.

Forced flow is non-negotiable capital behavior.

---

## Forced vs Discretionary Flow

### Forced Flow

**Definition**: Capital movement that must occur due to binding constraints, regardless of price, opinion, or preference.

**Characteristics**:
- Triggered by structural constraints
- Time-bound and deadline-driven
- Often price-insensitive
- Mechanically executed
- Predictable in timing and direction

**Examples**:
- Dealer hedging near option expiry
- Index rebalancing trades
- Margin call liquidations
- Settlement-driven adjustments
- ETF creation/redemption arbitrage

---

### Discretionary Flow

**Definition**: Capital movement that is optional and driven by strategy, opinion, or opportunity assessment.

**Characteristics**:
- Optional and opinion-driven
- Can be delayed or avoided
- Sensitive to price and liquidity
- Strategy-dependent
- Unpredictable in timing and magnitude

**Examples**:
- Tactical trades based on views
- Portfolio rebalancing without deadlines
- Retail speculation
- Opportunistic position-taking

---

## Forced vs Discretionary Flow Detection

Distinguishing between forced and discretionary flow is critical for inference quality.

### Detection Criteria for Forced Flow

Forced flow is identified when **all** of the following conditions are present:

1. **Binding Constraint Exists**
   - Regulatory deadline (expiry, settlement)
   - Risk limit breach (margin call, VaR limit)
   - Mandate requirement (index tracking, portfolio rebalancing)
   - Contractual obligation (hedging requirement, collateral posting)

2. **Time Urgency**
   - Constraint has a known deadline
   - Window for action is closing
   - Cannot be indefinitely deferred
   - Delay has increasing cost or penalty

3. **Mechanistic Response**
   - Action follows predictable rules
   - Not contingent on price targets
   - Magnitude is determinable from positioning
   - Direction is predetermined by constraint type

4. **Limited Discretion**
   - Capital manager cannot opt out
   - Alternative actions are unavailable or inferior
   - Risk of non-action exceeds cost of action

**Key Test**: *Would this capital act even if market conditions were unfavorable?*

- If yes → Likely forced flow  
- If no → Likely discretionary flow

---

### Detection Criteria for Discretionary Flow

Discretionary flow is identified when:

1. **No Binding Constraint**
   - Action is optional
   - No regulatory or structural deadline
   - Can be delayed without penalty

2. **Price/Opportunity Sensitivity**
   - Activity increases when prices are favorable
   - Volumes decline when liquidity is poor
   - Entry/exit is selective, not mechanical

3. **Strategy-Dependent Behavior**
   - Timing varies by participant
   - No predictable pattern across similar positions
   - Magnitude reflects conviction, not constraint

4. **High Discretion**
   - Manager can choose to wait
   - Multiple alternatives exist
   - Risk/reward tradeoff drives decision

**Key Test**: *Would this capital abstain if market conditions deteriorated?*

- If yes → Likely discretionary flow  
- If no → Re-evaluate for hidden constraints

---

### Detection Ambiguity and Edge Cases

Some flows exhibit characteristics of both types:

**Example 1: Reactive/Systematic Strategies**
- CTA trend-following, risk-parity rebalancing
- Discretionary in design (rules chosen by manager)
- Forced in execution (rules trigger mechanically)
- **Classification**: Treat as forced when rules are triggered, discretionary otherwise

**Example 2: "Soft" Constraints**
- Portfolio manager facing quarter-end benchmarking pressure
- Not legally required to act, but career consequences exist
- **Classification**: Generally discretionary, but may behave like forced flow near quarter-end

**Example 3: Large Discretionary Positions Near Expiry**
- A discretionary position holder who chose to hold into expiry
- Initial entry was discretionary, but now faces forced settlement
- **Classification**: Becomes forced flow as expiry approaches

**Resolution Strategy**:
When classification is ambiguous, Galactus:
- Flags the ambiguity explicitly
- Reduces confidence in flow classification
- Models both scenarios if material
- Prioritizes observable constraint evidence over inferred intent

Galactus prioritizes forced flow inference because it is more reliable and structurally grounded.

---

## Characteristics of Forced Flow

Forced flows exhibit several identifiable characteristics:

1. **Time Compression**
   - Action accelerates as deadlines approach

2. **Price Insensitivity**
   - Execution occurs despite unfavorable prices

3. **Predictable Windows**
   - Often tied to known calendar events

4. **Mechanical Execution**
   - Rule-based rather than discretionary

---

## Primary Sources of Forced Flow

Galactus explicitly models the following forced flow sources:

### 1. Derivatives Hedging

- Gamma-driven dealer hedging
- Vega and delta adjustments
- Expiry-driven unwinds

### 2. Index and Passive Flows

- Index inclusions/exclusions
- Weight rebalances
- ETF creation/redemption

### 3. Margin and Risk Constraints

- Margin calls
- Risk limit breaches
- Volatility-triggered de-risking

### 4. Settlement and Regulatory Events

- Settlement deadlines
- Regulatory compliance actions

---

## Detection Methodology (High-Level)

Forced flow detection relies on **confluence**, not single indicators.

Key detection elements include:

- Presence of binding constraints
- Size of capital subject to constraint
- Proximity to deadline
- Lack of discretionary delay behavior
- Alignment across multiple signals

No single metric is sufficient.

Each detected flow must be accompanied by structural justification (see below).

---

## Structural Justification: Definition and Requirements

**Structural justification** is the explicit, mechanistic explanation of why a detected flow is forced rather than discretionary.

It is the foundational requirement for all forced flow inferences in Galactus.

---

### What Is Structural Justification?

Structural justification is:

> A documented, falsifiable explanation that links observed capital behavior to a specific structural constraint or mechanical requirement, independent of opinion, sentiment, or price pattern.

It must identify:
1. **The constraint**: What structural mechanism is forcing action?
2. **The mechanism**: How does this constraint translate into observable flow?
3. **The capital**: Which type of capital is affected and why?
4. **The timeline**: When must action occur and why?
5. **The observables**: What evidence supports this interpretation?

---

### Why Structural Justification Is Mandatory

Without structural justification, inference degenerates into:
- Pattern recognition (mistaking correlation for causation)
- Price-following (reactive rather than predictive)
- Narrative-fitting (post-hoc rationalization)

Structural justification ensures:
- **Falsifiability**: Claims can be tested against observable constraints
- **Reproducibility**: Same constraints → same inference
- **Explainability**: Output can be traced to structural mechanics
- **Degradation**: When structure breaks, confidence degrades explicitly

**Core Principle**: *If you cannot explain the structural mechanism forcing the flow, you have not detected forced flow.*

---

### Components of Valid Structural Justification

A complete structural justification must include:

#### 1. Identification of Constraint Type

**Question**: What structural mechanism is binding?

**Valid constraint types**:
- **Time-bound obligations**: Expiry, settlement deadlines, rebalancing schedules
- **Risk limits**: Margin requirements, VaR limits, portfolio constraints
- **Regulatory/mandate requirements**: Index tracking, compliance rules
- **Hedging mechanics**: Delta/gamma hedging requirements, risk neutralization
- **Liquidity constraints**: Forced selling to meet redemptions, margin calls

**Invalid justifications**:
- "High volume suggests forced flow" (volume alone is not structural)
- "Price momentum indicates pressure" (price is effect, not cause)
- "Market sentiment appears bearish" (sentiment is not a constraint)

#### 2. Mechanistic Explanation

**Question**: How does this constraint translate into observable capital action?

**Requirements**:
- Describe the causal chain from constraint to flow
- Identify intermediate steps (e.g., dealer receives option order → must hedge delta → buys/sells underlying)
- Explain why the action is unavoidable given the constraint
- Quantify or bound the expected magnitude of flow

**Example**:
> "High gamma exposure near strike X creates forced hedging flow because:
> 1. Dealers are short options concentrated at strike X (observable via OI)
> 2. As price approaches X, dealer delta changes rapidly (gamma effect)
> 3. Dealers must continuously buy/sell underlying to maintain delta neutrality (hedging requirement)
> 4. Expected magnitude: proportional to gamma * spot move * open interest"

#### 3. Capital Identification

**Question**: Which capital is subject to this constraint and why?

**Requirements**:
- Identify the type of capital (dealers, index funds, leveraged retail, etc.)
- Explain why this capital specifically is constrained
- Estimate the size of capital subject to the constraint
- Describe alternative actions available (if any) and why they are inferior

**Example**:
> "Options dealers holding short straddles near expiry are constrained because:
> - They must maintain delta-neutral books (risk management requirement)
> - They cannot simply exit positions (liquidity costs exceed hedging costs)
> - Estimated exposure: ₹500 crore notional based on strike OI distribution"

#### 4. Timeline and Urgency

**Question**: When must this action occur and why can't it be delayed?

**Requirements**:
- Identify the deadline or forcing window
- Explain why delay is costly or impossible
- Describe how urgency increases as deadline approaches
- Account for roll vs settlement decisions

**Example**:
> "Forced flow must occur within 48 hours of expiry because:
> - Weekly options expire Thursday at 3:30 PM
> - Gamma exposure peaks in final 2 days
> - Cannot roll positions indefinitely (time decay dominates)
> - Settlement is cash-settled, requires final hedging adjustment"

#### 5. Observable Evidence

**Question**: What data confirms this structural interpretation?

**Requirements**:
- List specific observables supporting the inference
- Distinguish between direct evidence (e.g., OI distribution) and indirect evidence (e.g., price behavior)
- Identify data that would falsify the interpretation
- Flag gaps in observability

**Example**:
> "Evidence supporting forced dealer hedging flow:
> - Direct: 40% of weekly OI concentrated at 44000 strike (NSE data)
> - Direct: Spot price 43950, within high-gamma zone (calculated)
> - Indirect: Increased trading volume near strike as expiry approaches (consistent with hedging)
> - Falsification: If OI decays without corresponding volume, hedging interpretation is wrong"

---

### Structural Justification Across Flow Types

Different forced flow types require different structural justifications:

#### Example 1: Options Dealer Hedging

**Constraint**: Delta neutrality requirement  
**Mechanism**: Gamma-driven delta changes → continuous rehedging in underlying  
**Capital**: Options market makers and dealers  
**Timeline**: Continuous, intensifying near expiry  
**Observables**: Strike OI concentration, gamma exposure, correlated volume near strikes

#### Example 2: Index Rebalancing

**Constraint**: Index tracking mandate  
**Mechanism**: Index methodology change → forced buy/sell to match weights  
**Capital**: Passive index funds, ETFs  
**Timeline**: Known rebalancing dates (e.g., quarterly review)  
**Observables**: Announced index changes, AUM of tracking funds, pre-rebalance positioning

#### Example 3: Margin Call Liquidation

**Constraint**: Broker margin requirement breach  
**Mechanism**: Mark-to-market loss → insufficient margin → forced position closure  
**Capital**: Leveraged retail, proprietary traders  
**Timeline**: Intraday (margin calls must be met same day)  
**Observables**: Volatility spike, sudden volume surge, one-directional pressure

#### Example 4: Expiry Settlement

**Constraint**: Contractual settlement obligation  
**Mechanism**: In-the-money options → cash/physical settlement → underlying transaction required  
**Capital**: Option holders unable/unwilling to roll  
**Timeline**: Expiry day (typically morning)  
**Observables**: ITM OI approaching expiry, volume concentration at expiry

---

### When Structural Justification Is Weak

Not all detected flows have strong structural justification. Weak justification occurs when:

1. **Constraint is inferred, not observed**
   - Example: "Appears to be forced selling" without identifying the constraint
   - Resolution: Reduce confidence, flag as uncertain, seek more data

2. **Multiple competing explanations exist**
   - Example: Volume surge could be forced or discretionary
   - Resolution: Model both scenarios, assess relative likelihoods, flag ambiguity

3. **Observables are indirect or noisy**
   - Example: Inferring positioning from price action alone
   - Resolution: Downweight inference, require corroborating signals

4. **Mechanism is probabilistic, not deterministic**
   - Example: "Capital might face pressure if volatility increases"
   - Resolution: Model as contingent inference, not active forced flow

**Galactus Response**: When structural justification is weak, confidence degrades. Silence is preferable to low-confidence forced flow claims.

---

### Structural Justification vs Pattern Recognition

| **Aspect**                  | **Structural Justification**                          | **Pattern Recognition**                     |
|-----------------------------|------------------------------------------------------|---------------------------------------------|
| Foundation                  | Market structure and mechanics                       | Historical price/volume patterns            |
| Causality                   | Identifies forcing mechanism                         | Observes correlation                        |
| Predictive Basis            | Constraint-driven (causal)                           | Similarity-driven (correlational)           |
| Falsifiability              | Testable against constraint observables              | Difficult to falsify (patterns shift)       |
| Regime Robustness           | Stable across regimes (structure persists)           | Fragile to regime shifts (patterns break)   |
| Explainability              | Mechanistic explanation required                     | "Worked in the past" justification          |
| Galactus Usage              | Required for all forced flow inference               | Explicitly rejected                         |

**Key Distinction**: Patterns describe *what happened*. Structural justification explains *why it had to happen*.

Galactus prioritizes structural justification because it is the only approach compatible with:
- Deterministic inference
- Graceful degradation
- Explainable outputs
- Falsifiable claims

---

### Structural Justification in Practice

When Galactus detects potential forced flow, the inference pipeline must produce:

1. **Primary Output**: Magnitude and direction of forced flow
2. **Structural Justification**: Complete explanation per framework above
3. **Confidence Assessment**: Based on strength of structural justification
4. **Alternative Explanations**: Documented competing interpretations (if any)
5. **Falsification Criteria**: What observables would invalidate this inference

**Example Output**:
```
Forced Flow Detected: Dealer Hedging Pressure (Buying)

Magnitude: ₹300-500 crore estimated buying pressure
Confidence: High (0.85)

Structural Justification:
- Constraint: Delta neutrality requirement for options dealers
- Mechanism: 45% of weekly call OI concentrated at 44000 strike; spot at 43900
  creates negative dealer delta which requires buying underlying to hedge
- Capital: Options market makers (estimated ₹2000 crore gross exposure)
- Timeline: Next 48 hours (expiry Thursday 3:30 PM)
- Observables: Strike OI 44000: 15 lakh contracts, Gamma exposure: 0.65, 
  Volume increasing near strike consistent with hedging

Alternative Explanations: None identified (OI concentration and timing strongly suggest hedging)

Falsification: If spot moves >1% away from 44000 without volume increase, hedging thesis is invalidated
```

---

### Final Statement on Structural Justification

**Without structural justification, there is no forced flow detection—only speculation.**

Every forced flow inference in Galactus must be:
- Grounded in observable constraints
- Explained through mechanical transmission
- Testable against structural evidence
- Falsifiable by contradictory data

This requirement is non-negotiable. It is what separates Galactus from pattern-matching systems.

---

## Differentiating Forced Flow from Noise

Galactus avoids misclassification by:

- **Requiring structural justification** (see above) for all forced flow claims
- Normalizing flows by liquidity
- Conditioning inference on time urgency
- Rejecting short-lived, unstructured bursts
- Distinguishing between constraint-driven and sentiment-driven moves

**Key Distinction**:
- **Forced Flow**: Has structural justification, mechanistic explanation, observable constraint
- **Noise**: Lacks structural grounding, appears random, no identifiable forcing mechanism
- **Discretionary Flow**: May have rationale, but lacks binding constraint

Sudden volume without constraint context is not forced flow—it is noise or discretionary activity.

**Default Assumption**: Without structural justification, observed activity is assumed to be noise or discretionary until proven otherwise.

---

## Directionality of Forced Flow

Forced flow may be:
- Directional (net buying or selling)
- Neutral (pinning, stabilizing)
- Volatility-enhancing

Direction is inferred from **mechanism**, not price movement.

---

## Confidence Assessment

Forced flow detection outputs include:
- Confidence levels
- Assumption lists
- Known ambiguity flags

Ambiguous cases degrade confidence rather than forcing classification.

---

## Failure Modes

Forced flow detection may fail when:
- Constraints are misidentified
- Capital size is underestimated
- Liquidity collapses unexpectedly
- Exogenous shocks override structure

Failures must be explicit.

---

## Interaction with Other Engine Components

Forced flow detection informs:
- Capital pressure computation
- Regime classification
- Confidence evaluation

It does not operate in isolation.

---

## What Galactus Explicitly Avoids

Galactus does not:
- Infer forced flow from price momentum alone
- Label all large moves as forced
- Assume derivatives always dominate

Forced flow is situational.

---

## Final Statement

**When capital is forced, markets reveal structure.**

Galactus exists to detect those moments clearly.
