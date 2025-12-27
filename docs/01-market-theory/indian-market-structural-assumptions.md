# Galactus — Indian Market Structural Assumptions

## Purpose of This Document

This document explicitly states the **structural assumptions about Indian financial markets** that underpin Galactus' capital behavior inference model.

These assumptions are:
- Foundational to all signal design and inference logic
- Derived from observable market structure and regulation
- Subject to validation and periodic review
- Critical for understanding system limitations and failure modes

If these assumptions are invalidated, Galactus' inferences may degrade.

---

## Core Meta-Assumption

**Galactus assumes that market structure constrains capital behavior in predictable and observable ways.**

If market structure becomes random, transient, or fundamentally unobservable, the system cannot function as designed.

---

## Assumption 1: Derivatives Dominance

### Statement

**Index and stock derivatives are the primary mechanism through which capital exposure and price constraints are expressed in Indian markets.**

### Specific Sub-Assumptions

1. **Volume Dominance**: Derivatives volume regularly exceeds cash market volume for liquid instruments.
2. **Price Discovery**: Price discovery frequently occurs in derivatives markets first, with cash markets following.
3. **Hedging Flows**: Dealer hedging activity constitutes a meaningful portion of intraday capital flows.
4. **Gamma Effects**: Gamma exposure significantly influences price behavior, especially near expiry.
5. **Open Interest Concentration**: Clustered open interest at specific strikes creates observable price constraints.

### Implications for Galactus

- Options data is treated as first-class input, not secondary
- Hedging-related flows are modeled explicitly as forced capital
- Strike concentration is used to infer price attraction/resistance zones
- Time-to-expiry is a core normalization factor for all inferences

### Validation Requirements

- Monitor ratio of derivatives-to-cash volume
- Track correlation between option positioning and price behavior
- Validate gamma-related flow predictions

### Failure Modes

- Sudden shift to cash-driven price discovery
- Collapse of dealer market-making capacity
- Regulatory changes limiting derivatives participation

**Related Documentation**: [`derivatives-dominance.md`](derivatives-dominance.md)

---

## Assumption 2: Retail Participation Asymmetry

### Statement

**Retail participants dominate trade count and options buying activity, but institutional participants dominate capital impact and price-setting ability.**

### Specific Sub-Assumptions

1. **Count vs Capital**: High retail transaction count masks low per-transaction capital size.
2. **Leverage Concentration**: Retail disproportionately uses options for leverage, not hedging.
3. **Reactive Behavior**: Retail capital typically reacts to institutional-driven moves rather than initiating them.
4. **Attention-Driven**: Retail participation concentrates around recent price action and media narratives.
5. **Limited Absorption**: Retail capital has limited capacity to absorb adverse price movement.
6. **Crowding Fragility**: Concentrated retail positioning creates non-linear unwind risk.

### Implications for Galactus

- Trade count alone is not used as a signal
- Retail metrics are used primarily to measure crowding intensity
- Institutional constraint detection is prioritized over retail sentiment
- Distinctions are made between capital initiation and amplification

### Validation Requirements

- Monitor relative share of retail vs institutional capital in key moves
- Track retail crowding metrics against realized volatility
- Validate that retail flows follow rather than lead institutional action

### Failure Modes

- Retail gaining primary price-setting capability
- Institutional capital withdrawing from markets
- Coordination mechanisms enabling retail to act institutionally

**Related Documentation**: [`retail-vs-institutional-dynamics.md`](retail-vs-institutional-dynamics.md)

---

## Assumption 3: Liquidity Fragmentation

### Statement

**Indian equity liquidity is highly concentrated in index constituents, with sharp degradation in non-index names, creating predictable liquidity cliffs and amplified price impact.**

### Specific Sub-Assumptions

1. **Index Concentration**: Top index constituents capture disproportionate liquidity.
2. **Free Float Constraints**: Effective free float is often smaller than reported float.
3. **Depth Illusion**: Displayed depth does not reflect true absorption capacity.
4. **Impact Amplification**: Small forced flows create disproportionate price impact in illiquid names.
5. **Liquidity Clustering**: Liquidity concentrates around specific price levels and times.
6. **Withdrawal Risk**: Liquidity can evaporate rapidly during stress.

### Implications for Galactus

- All signals are normalized by liquidity and free float metrics
- Price impact expectations vary significantly across instruments
- Inference confidence degrades for low-liquidity instruments
- Liquidity itself is treated as a dynamic constraint

### Validation Requirements

- Monitor bid-ask spreads and depth across liquidity tiers
- Track price impact per unit capital across instrument types
- Validate liquidity withdrawal during stress periods

### Failure Modes

- Dramatic improvement in market-wide liquidity provision
- Collapse of index constituent liquidity
- Market-wide liquidity freeze

**Related Documentation**: [`indian-market-inefficiencies.md`](indian-market-inefficiencies.md) (Inefficiency 6)

---

## Assumption 4: Expiry Effects

### Statement

**Weekly and monthly expiries create time-bound, forced capital actions that dominate price behavior in the final days and hours before expiry.**

### Specific Sub-Assumptions

1. **Time Compression**: Decision windows compress as expiry approaches.
2. **Forced Settlement**: Expiry creates non-discretionary capital actions (settlements, rolls, unwinds).
3. **Hedging Amplification**: Dealer hedging becomes increasingly mechanical as expiry approaches.
4. **Strike Pinning**: High open interest strikes create temporary price equilibria near expiry.
5. **Volatility Spikes**: Implied volatility often spikes pre-expiry and collapses post-expiry.
6. **Predictable Timing**: Expiry effects follow a consistent weekly/monthly calendar.

### Implications for Galactus

- Time-to-expiry is a first-order variable in all derivative-related inferences
- Pre-expiry and post-expiry regimes are treated as distinct states
- Forced flow detection is weighted heavily during expiry windows
- Post-expiry signal carryover is explicitly prohibited

### Validation Requirements

- Monitor magnitude of price movement and volatility changes around expiries
- Track correlation between open interest concentration and price pinning
- Validate forced flow detection accuracy during expiry windows

### Failure Modes

- Move to longer-dated or non-expiring instruments
- Elimination of weekly expiries
- Smooth rolling mechanisms that eliminate forced settlement

**Related Documentation**: 
- [`derivatives-dominance.md`](derivatives-dominance.md) (Weekly Expiries section)
- [`expiry-day-playbook.md`](../10-operational-playbooks/expiry-day-playbook.md)
- [`indian-market-inefficiencies.md`](indian-market-inefficiencies.md) (Inefficiency 3)

---

## Assumption 5: Information Latency and Reaction Asymmetry

### Statement

**Information is disclosed discretely rather than continuously, and market participants digest this information at different speeds, creating observable capital response patterns.**

### Specific Sub-Assumptions

1. **Discrete Disclosure**: Material information arrives in discrete batches (results, filings, disclosures).
2. **Uneven Digestion**: Different participant types interpret and act on information at different speeds.
3. **Mechanical Priority**: Capital often reacts to mechanics before meaning.
4. **Response Patterns**: Different capital types have characteristic response functions to new information.
5. **Attention Cascades**: Information propagates through participant layers with observable delays.

### Implications for Galactus

- Disclosure timing is tracked as a regime-shift signal
- Capital response patterns are modeled explicitly
- Early vs late reactions are distinguished
- Mechanical responses are separated from interpretive responses

### Validation Requirements

- Monitor time-to-react for different participant types after disclosures
- Track correlation between disclosure type and capital response patterns
- Validate distinction between mechanical and interpretive responses

### Failure Modes

- Real-time continuous disclosure becoming standard
- All participants gaining equal information access and interpretation speed
- Elimination of discrete disclosure windows

**Related Documentation**: [`indian-market-inefficiencies.md`](indian-market-inefficiencies.md) (Inefficiency 5)

---

## Assumption 6: Constraint Asymmetry

### Statement

**Different capital types operate under different constraints (regulatory, risk, mandate, timing), creating predictable pressure patterns when constraints bind.**

### Specific Sub-Assumptions

1. **Constraint Heterogeneity**: Not all capital faces the same rules, risk limits, or mandates.
2. **Binding Events**: Constraints shift from non-binding to binding at observable thresholds.
3. **Delayed Reactions**: Some constraints allow delayed action; others force immediate action.
4. **Coordination Barriers**: Capital facing similar constraints cannot easily coordinate responses.
5. **Constraint Visibility**: Many constraints are observable through public data (indices, expiries, margins).

### Implications for Galactus

- Constraint identification is central to forced flow detection
- Different capital types require different constraint models
- Threshold detection is critical for regime classification
- Public constraint calendars (rebalancing, expiry) are first-class inputs

### Validation Requirements

- Monitor accuracy of constraint-binding predictions
- Track correlation between identified constraints and capital actions
- Validate that constraint models generalize across instruments

### Failure Modes

- Homogenization of capital constraints
- Removal of observable constraint calendars
- Coordination mechanisms that overcome individual constraints

**Related Documentation**: [`capital-behavior-model.md`](capital-behavior-model.md)

---

## Assumption Interdependencies

These assumptions are not independent. They interact to create the overall market structure:

- **Derivatives Dominance** + **Expiry Effects** → Forced hedging flows dominate near expiry
- **Retail Asymmetry** + **Liquidity Fragmentation** → Crowding creates fragile, thin liquidity
- **Constraint Asymmetry** + **Information Latency** → Different capital reacts at different times
- **Liquidity Fragmentation** + **Expiry Effects** → Impact amplifies during forced settlement windows

Galactus models these interactions explicitly.

---

## Assumption Validation and Monitoring

### Validation Requirements

Each assumption must be validated:
- **Empirically**: Through backtesting and walk-forward analysis
- **Continuously**: Through ongoing monitoring of market structure metrics
- **Explicitly**: Through formal assumption review process

### Monitoring Metrics

Key metrics to monitor for assumption validity:
1. Derivatives-to-cash volume ratio
2. Retail vs institutional capital impact share
3. Liquidity concentration and depth metrics
4. Expiry-related volatility and flow patterns
5. Information response time distributions
6. Constraint-binding event accuracy

### Review Frequency

- **Quarterly**: High-level assumption validity check
- **Annually**: Comprehensive structural review
- **Event-Driven**: Review triggered by major regulatory or structural changes

**Related Documentation**: [`walk-forward-validation.md`](../07-backtesting-and-validation/walk-forward-validation.md)

---

## Assumption Invalidation Scenarios

### What Would Invalidate These Assumptions?

These assumptions would be invalidated by:

1. **Regulatory Changes**:
   - Ban on derivatives for retail
   - Elimination of weekly expiries
   - Forced continuous disclosure

2. **Structural Changes**:
   - Dramatic improvement in market-wide liquidity
   - Collapse of derivatives market making
   - Homogenization of capital constraints

3. **Technological Changes**:
   - Real-time information equality across all participants
   - Perfect execution without market impact
   - Automated constraint coordination

4. **Market Evolution**:
   - Shift to purely algorithmic price formation
   - Elimination of forced settlement mechanisms
   - Disappearance of retail participation

### Response Protocol

If assumptions are invalidated:
1. Document the invalidation in decision log
2. Assess impact on system components
3. Degrade affected signals gracefully
4. Implement emergency confidence reduction if necessary
5. Initiate formal assumption revision process

**Related Documentation**: 
- [`decision-log.md`](../11-decision-log/decision-log.md)
- [`known-risks.md`](../08-risk-and-failure-modes/known-risks.md)

---

## Relationship to Core Vision

These assumptions are **subordinate to the core vision**.

They represent the current state of Indian market structure that makes capital behavior inference feasible.

If these assumptions change, the vision remains intact, but:
- Signal designs may need revision
- Confidence metrics may degrade
- System scope may narrow

The vision of inferring capital behavior is permanent.  
The specific structural assumptions that enable it are empirical and revisable.

**Related Documentation**: 
- [`vision.md`](../00-vision-and-non-goals/vision.md)
- [`VISION_LOCK.md`](../../VISION_LOCK.md)

---

## Design Implications

These assumptions inform core design decisions:

1. **Data Priorities**:
   - Options data is mandatory, not optional
   - Liquidity metrics are first-class inputs
   - Expiry calendars drive regime classification

2. **Model Architecture**:
   - Separate models for forced vs discretionary capital
   - Time-varying constraints drive inference
   - Multi-participant framework is required

3. **Signal Philosophy**:
   - Constraint-based signals over pattern-based signals
   - Capital-weighted metrics over trade-count metrics
   - Mechanistic explanations over statistical correlations

**Related Documentation**: 
- [`design-principles.md`](../00-vision-and-non-goals/design-principles.md)
- [`signal-philosophy.md`](../04-signal-and-metrics/signal-philosophy.md)

---

## What These Assumptions Do NOT Claim

These assumptions **do not** claim:

- ❌ Indian markets are uniquely inefficient
- ❌ These patterns guarantee profit
- ❌ All capital behavior is predictable
- ❌ Market structure will never change
- ❌ These are the only relevant structural factors

They claim only that:
- ✅ These patterns are currently observable
- ✅ They create detectible capital pressure
- ✅ They are stable enough to model
- ✅ They are worth monitoring for change

---

## Final Statement

**Galactus does not assume market efficiency or inefficiency.**

**It assumes market structure.**

Structure creates constraints.  
Constraints create pressure.  
Pressure creates observable capital behavior.

These assumptions document that structure as it exists today in Indian markets.

When structure changes, these assumptions must change with it.

---

## Document Authority

This document has the same authority level as other market theory documents.

Changes to these assumptions require:
- Empirical evidence of structural change
- Impact analysis on system components
- Formal documentation in decision log
- Updated validation requirements

**This document does not supersede the core vision.**

It operationalizes the vision within the current structural context.
