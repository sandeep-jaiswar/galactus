# Galactus — Capital Behavior Theory

## Purpose of This Document

This document establishes the **theoretical foundation** for why capital behavior under constraints is the primary inference primitive in Galactus.

It explains:
- Why capital behavior is more fundamental than price, sentiment, or opinion
- How constraints transform capital into a reliable inference primitive
- The epistemological superiority of modeling behavior over predicting outcomes
- Why this choice drives all other architectural decisions

This is the **why** behind Galactus. The **what** is in [`capital-behavior-model.md`](capital-behavior-model.md).

---

## The Central Thesis

**Capital behavior under constraints is the only observable market primitive that is both measurable and causal.**

Everything else—price, sentiment, opinion, volume—is either:
- A downstream effect (price)
- An unobservable internal state (opinion, sentiment)
- An ambiguous mixture (volume, volatility)

Capital behavior is unique because it:
1. **Exists prior to price** — Capital must move before price can change
2. **Is constrained by structure** — Rules, mandates, and mechanics limit optionality
3. **Leaves observable traces** — Positioning, flows, and hedging needs are detectable
4. **Is causally prior** — Behavior drives outcomes, not the reverse

Galactus is built on this primitive because it is the most defensible foundation for inference.

---

## Why Not Price?

Price is the most visible market artifact, but it is the **worst** primitive for inference.

### Price Is an Outcome, Not a Cause

Price is what happens *after* capital acts. It is:
- A lagging indicator of executed transactions
- A composite signal of multiple simultaneous forces
- Contaminated by liquidity mechanics, spread dynamics, and execution timing

Modeling price is like modeling the reading on a thermometer instead of the heat sources in a room.

### Price Conflates Causes

A price increase can result from:
- Forced institutional buying (index rebalancing)
- Discretionary accumulation
- Short covering (forced buying)
- Retail momentum chasing
- Low liquidity amplification
- Market-making dynamics

**Identical price action can arise from entirely different capital dynamics.**

Without decomposing price into its capital-behavior components, inference is impossible.

### Price Invites Circular Reasoning

Most price-based models suffer from tautology:
- "Price went up because buying exceeded selling"
- "Momentum exists because price continued its trend"
- "Support held because buyers appeared"

These statements are true but **explain nothing**. They describe outcomes without identifying causes.

Galactus rejects this approach. Price is evidence, not explanation.

---

## Why Not Sentiment or Opinion?

Sentiment and opinion are frequently cited as market drivers, but they are **epistemologically useless** as inference primitives.

### Sentiment Is Unobservable

True sentiment exists inside market participants' minds. It is:
- Not directly measurable
- Constantly shifting
- Subject to strategic misrepresentation
- Context-dependent

What is typically called "sentiment" is actually:
- Survey responses (stated preference, not revealed behavior)
- Social media activity (noisy, unrepresentative, gameable)
- Derivatives positioning (which reflects constraints, not opinion)

These are proxies for sentiment, not sentiment itself.

### Opinion Does Not Imply Action

A trader may:
- Be bullish but unable to deploy capital (constraint)
- Be bearish but forced to buy (hedging obligation)
- Hold strong opinions but choose inaction (risk management)

**Opinion without capital, or capital without freedom to act, does not move markets.**

Galactus models what capital *must* do, not what participants *think* will happen.

### Sentiment Is Reversible Without Cost

Opinions change instantly. Positions do not.

A participant can:
- Reverse their view without penalty
- Hold contradictory opinions simultaneously
- Express sentiment without financial consequence

Capital deployment, by contrast, is costly:
- Entry and exit costs
- Opportunity cost
- Exposure to risk during holding period

**Behavior reveals true belief better than stated opinion.**

### Sentiment Cannot Be Deterministic

Opinion is inherently subjective and contextual. Two participants seeing the same data may form opposite views.

This makes sentiment-based inference:
- Non-deterministic (same inputs, different outputs)
- Non-replayable (interpretation shifts over time)
- Non-testable (no ground truth for "correct" sentiment)

Galactus requires determinism. Sentiment cannot provide it.

---

## Why Not Volume or Volatility?

Volume and volatility are observable, but they are **ambiguous** as inference primitives.

### Volume Lacks Directionality

High volume can indicate:
- Strong conviction (accumulation or distribution)
- Panic (forced liquidation)
- Rebalancing (index flows)
- Arbitrage (market-neutral activity)

Without decomposing volume by capital type and constraint, it has no predictive power.

### Volatility Is an Effect, Not a Cause

Volatility measures price dispersion. It is the result of:
- Capital reacting to uncertainty
- Liquidity withdrawal
- Forced covering or hedging
- Regime transitions

Volatility tells you *that something happened*, not *what or why*.

### Both Are Derivatives of Capital Behavior

Volume and volatility are outputs of capital action:
- Volume = aggregated transactions
- Volatility = rate of price change under pressure

They are useful as **context** for capital behavior inference, but they cannot serve as the foundation.

Galactus uses volume and volatility as supporting evidence, not as core primitives.

---

## Why Capital Behavior Under Constraints?

Capital behavior becomes a reliable primitive when it is **constrained**.

### Constraints Make Behavior Predictable

Unconstrained capital is optional. It may:
- Act or not act
- Enter now or later
- Scale up or down

But constrained capital has reduced degrees of freedom:
- Must act within a time window (expiry, rebalancing)
- Must maintain hedges (risk limits, regulatory rules)
- Must respond to margin calls (forced liquidation)

**Constraints convert optionality into inevitability.**

### Constraints Are Observable

Unlike sentiment, constraints are external and measurable:
- Option expiry dates are public
- Index rebalancing rules are documented
- Margin requirements are known
- Regulatory deadlines are scheduled

These are **deterministic inputs** that can be modeled without guesswork.

### Constraints Create Causal Structure

When capital is constrained:
- Timing is forced
- Direction is often predetermined
- Magnitude can be estimated

This gives Galactus a **causal model**:
- Constraint binds → Capital must act → Market impact probable

This is not prediction (outcomes are still uncertain), but it is **inference** (probabilities are non-random).

---

## The Epistemological Advantage

Galactus is built on capital behavior because it offers **epistemological superiority** over alternatives.

### 1. Observability

Capital behavior leaves traces:
- Derivatives positioning (open interest, implied volatility)
- Flow data (institutional holdings, ETF creations)
- Liquidity signals (bid-ask spreads, depth)

These are measurable without relying on subjective interpretation.

### 2. Causality

Capital behavior is **causally prior** to price:
- Capital moves → Orders execute → Price changes

Modeling behavior means modeling the cause, not the effect.

### 3. Determinism

Constraints are rule-based. Given the same constraints and capital positioning, inference is reproducible:
- Same inputs → Same pressure measurement → Same inference

This allows Galactus to be deterministic and testable.

### 4. Structural Grounding

Capital behavior is grounded in market structure:
- Clearing mechanics
- Margin systems
- Index methodologies
- Hedging requirements

These are durable features of the market, not ephemeral patterns.

### 5. Failure Modes Are Identifiable

When capital behavior models fail, the failure is attributable:
- Constraint was misidentified
- Capital size was misestimated
- Liquidity vanished unexpectedly

This enables **graceful degradation** instead of silent failure.

---

## Inference vs Prediction

Galactus is an inference engine, not a prediction engine.

### Prediction Claims Outcomes

Prediction says: "Price will reach X by time T."

This requires:
- Perfect information
- Perfect modeling of all forces
- No exogenous shocks

Prediction is brittle and overconfident.

### Inference Describes Pressure

Inference says: "Given current constraints, capital pressure is directional and increasing."

This acknowledges:
- Partial observability
- Competing forces
- Exogenous uncertainty

Inference is modest and robust.

### Why Inference Is Superior

Inference focuses on what can be known:
- Which capital is constrained
- What it must do
- When it must act

Outcomes remain uncertain, but **probabilities are structured**, not random.

This is the only honest approach in a complex, partially observable system.

---

## Capital Behavior as Primitive: Implications

Treating capital behavior as the primary primitive has cascading implications for Galactus.

### All Signals Must Map to Capital Behavior

Every signal in Galactus must answer:
- Which capital is affected?
- What constraint is binding?
- Why is action unavoidable?

Signals without capital interpretation are invalid.

### Price Is Evidence, Not Explanation

Price movements are used to validate or refine capital models, but they do not drive inference.

Galactus does not react to price. It models the forces that will shape price.

### Sentiment and Opinion Are Ignored

Unless sentiment can be translated into measurable capital constraints, it is out of scope.

Galactus does not model "market mood." It models capital pressure.

### Time Horizons Are Explicit

Capital constraints operate over specific time windows:
- Expiry-driven pressure: days to weeks
- Rebalancing pressure: weeks to months
- Structural flows: months to quarters

Inference is always time-bounded.

### Confidence Degrades Gracefully

When constraints weaken or become ambiguous, Galactus reduces confidence rather than fabricating certainty.

"No meaningful inference" is a valid output.

---

## Distinguishing Capital Behavior From Alternatives

### Capital Behavior vs Sentiment

| **Aspect**            | **Capital Behavior**                     | **Sentiment**                          |
|-----------------------|------------------------------------------|----------------------------------------|
| Observability         | Observable via positioning and flows     | Unobservable (internal state)          |
| Measurement           | Deterministic, based on constraints      | Subjective, survey-based               |
| Causality             | Causally prior to price                  | Correlational at best                  |
| Reversibility         | Costly to reverse (real capital at risk) | Costless to change (just an opinion)   |
| Reliability           | Binding constraints force action         | Opinion does not guarantee action      |

### Capital Behavior vs Opinion

| **Aspect**            | **Capital Behavior**                     | **Opinion**                            |
|-----------------------|------------------------------------------|----------------------------------------|
| Commitment            | Capital is deployed (skin in the game)   | No financial commitment required       |
| Verifiability         | Traceable through positioning            | Stated preference, not revealed        |
| Predictive Power      | Constrained behavior is predictable      | Opinion may not translate to action    |
| Consistency           | Forced by constraints and risk limits    | Can be internally contradictory        |

### Capital Behavior vs Price Action

| **Aspect**            | **Capital Behavior**                     | **Price Action**                       |
|-----------------------|------------------------------------------|----------------------------------------|
| Timing                | Behavior precedes price change           | Price is lagging indicator             |
| Causality             | Behavior is causal                       | Price is effect                        |
| Decomposition         | Can identify capital type and constraint | Price conflates all forces             |
| Interpretability      | Explainable via constraints              | Ambiguous without context              |

---

## Practical Implications for Galactus

### What Galactus Measures

- Magnitude and direction of forced flows
- Hedge pressure from derivatives positioning
- Rebalancing necessity from index mechanics
- Liquidity absorption capacity
- Stability and persistence of pressure

### What Galactus Does Not Measure

- Market sentiment or mood
- Price momentum or trend strength (except as evidence of behavior)
- Social media activity or news sentiment
- Crowd opinion or consensus forecasts

### Why This Boundary Exists

By excluding non-causal, non-observable, or non-deterministic signals, Galactus:
- Avoids overfitting to noise
- Maintains interpretability
- Enables deterministic replay
- Produces stable, testable inference

---

## Limits of Capital Behavior Theory

This theory does not claim:

### 1. Perfect Knowledge

Capital positioning is only partially observable. Inference is always incomplete.

### 2. Deterministic Outcomes

Constrained capital creates pressure, not certainty. Exogenous shocks can override structure.

### 3. Universal Applicability

Some market conditions (e.g., extreme illiquidity, exogenous news shocks) may render behavioral inference ineffective.

### 4. Completeness

Not all capital is constrained. Discretionary capital remains unpredictable, and its actions can dominate in certain regimes.

### 5. Immunity to Regime Shifts

Market structure can change. When it does, capital behavior models must adapt or degrade gracefully.

---

## Why This Theory Is Locked

Capital behavior theory is the foundation of Galactus. Changing it would require rebuilding the system from scratch.

This theory may only be revised if:
- Market structure changes fundamentally
- Core assumptions are empirically invalidated
- New causal primitives are discovered that are superior

See [`VISION_LOCK.md`](../../VISION_LOCK.md) for the formal revision process.

---

## Final Statement

**Capital behavior under constraints is the only market primitive that is simultaneously observable, causal, deterministic, and structurally grounded.**

Sentiment is unobservable.  
Opinion is non-binding.  
Price is a lagging effect.

Capital behavior is the foundation because it is the only honest choice.

Galactus infers what capital must do, not what price might do.  
This is the only defensible approach to market inference.
