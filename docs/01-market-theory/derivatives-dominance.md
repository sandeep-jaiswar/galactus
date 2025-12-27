# Galactus — Derivatives Dominance

## Purpose of This Document

This document explains **why derivatives markets dominate price behavior** in Indian equities and indices, and how this dominance creates observable, repeatable capital pressure.

It defines:
- The role of derivatives in price formation
- The behavior of derivative-linked capital
- What Galactus models explicitly
- What Galactus deliberately ignores

All derivative-related signals must align with this document.

---

## Core Observation

In Indian markets, **derivatives are not a side market**.

They are the **primary mechanism through which price constraints are expressed**.

Cash markets frequently reflect the resolution of pressures created in derivatives, not the other way around.

---

## Why Derivatives Dominate in India

Several structural factors contribute:

- Extremely high options participation
- Weekly index expiries
- Low cost of leverage
- Retail preference for convex payoffs
- Institutional use of derivatives for hedging and exposure management

As a result:
- A large portion of capital exposure exists off the cash market
- Hedging activity often exceeds directional intent
- Price is constrained by positioning rather than conviction

---

## Structural Mechanics of Price Discovery

### The Price Discovery Hierarchy

In Indian markets, price discovery follows a structural hierarchy:

**Derivatives → Cash → Secondary Effects**

This is not an empirical observation—it is a mechanical consequence of market structure.

### Why Derivatives Lead Price Discovery

Three structural mechanisms explain derivatives' dominance in price discovery:

#### 1. Capital Efficiency and Velocity

Derivatives markets enable:
- **Leverage**: Control of notional exposure with fractional capital
- **Lower transaction costs**: Reduced STT, stamp duty, and execution costs
- **Faster position adjustment**: No delivery settlement lag

**Mechanical consequence**: 
- Capital moves faster in derivatives than cash
- Price-relevant information reaches derivatives first
- Information gets embedded in derivatives positioning before cash accumulation

This is not about speculation—it is about **structural efficiency**.

A participant needing to express a view or hedge exposure will:
1. Act in derivatives (fast, cheap, leveraged)
2. Only later (if at all) adjust cash positions

#### 2. Margin-Based Capital Amplification

Derivatives operate on margin systems that create structural amplification:

**Initial margin requirement**: ~10-20% of notional exposure  
**Result**: ₹1 of capital controls ₹5-10 of price-relevant exposure

This creates:
- **Velocity asymmetry**: Derivatives capital moves 5-10x faster per rupee deployed
- **Price impact amplification**: Small derivatives flows create large notional pressure
- **Forced unwind dynamics**: Margin calls create mechanically forced liquidations

**Mechanical consequence**:
The same capital has 5-10x more price impact when deployed in derivatives versus cash.

When this capital is forced to hedge or unwind, the pressure transmits to cash markets.

#### 3. Zero-Sum Structure and Liquidity Concentration

Derivatives are zero-sum contracts:
- Every long has a corresponding short
- No net capital is "invested" in the instrument
- Liquidity is concentrated, not fragmented across time

**Mechanical consequence**:
- Dealers and market makers provide continuous liquidity
- Hedging needs are centralized and observable
- Position concentration is visible via open interest
- Price discovery is not diluted by long-term holders

In cash markets:
- Capital is locked in delivery
- Long-term holders create dead liquidity
- True available liquidity is obscured

Derivatives strip away "passive" capital and expose only **active** price-setting flows.

---

## Hedging Mechanics: The Transmission Mechanism

The most important structural mechanism linking derivatives to cash market price discovery is **dealer hedging**.

### Delta Hedging: Continuous Price Transmission

Dealers (typically options sellers) maintain delta-neutral portfolios by hedging in the underlying:

**Mechanism**:
1. Retail/institutional buy call option → Dealer sells call
2. Dealer is now short delta exposure
3. Dealer buys futures/cash to neutralize delta
4. **This buying pressure moves price**

**Key properties**:
- Hedging is **continuous** as price moves and delta changes
- Hedging is **mechanical**, not discretionary
- Hedging magnitude scales with gamma exposure
- Hedging is **observable** via open interest and price action correlation

**Mechanical consequence**:
Derivatives positioning creates a **standing bid/offer** in the underlying that must be satisfied.

Price cannot drift far from strikes with concentrated open interest without triggering mechanical hedging flows.

### Gamma Exposure: Price Acceleration and Dampening

Gamma measures the rate of delta change. It determines whether dealer hedging **amplifies** or **dampens** price movement.

**Positive Gamma (dealers long options)**:
- Price rises → Dealer delta goes positive → Sell underlying to hedge → Dampens price rise
- Price falls → Dealer delta goes negative → Buy underlying to hedge → Dampens price fall
- **Result**: Mean reversion, price stability, range-bound behavior

**Negative Gamma (dealers short options)**:
- Price rises → Dealer delta goes more negative → Buy underlying to hedge → Amplifies price rise  
- Price falls → Dealer delta goes less negative → Sell underlying to hedge → Amplifies price fall
- **Result**: Trending behavior, breakouts, momentum

**Mechanical consequence**:
Gamma exposure determines the **mechanical response function** of the market to price shocks.

This is not psychology or sentiment—it is structural physics.

### Time Decay and Expiry Mechanics

As expiry approaches:
- Gamma increases exponentially near-the-money
- Time value evaporates
- Hedging adjustments become more frequent and violent
- Position unwinds become forced (cannot roll indefinitely)

**Mechanical consequence**:
Expiry creates a **structural forcing function** that removes optionality:
- Decisions that could be delayed must now be made
- Hedges that could be maintained must now be unwound
- Positions that were voluntary become forced settlements

Weekly expiries in India create **52 annual forcing events**, each compressing decision windows.

---

## Strike Concentration: The Gravity Wells of Price

Open interest concentration at specific strikes creates structural price constraints.

### The Pinning Mechanism

High open interest at a strike creates mechanical pinning pressure:

**At or near high OI strike**:
- Large gamma exposure exists
- Any price movement triggers offsetting hedging flows
- Net effect: Price is pulled back toward the strike

**Mechanical framework**:
1. Price approaches high-OI call strike from below
2. Calls move in-the-money → Dealer short deltas increase
3. Dealers buy underlying to hedge → Pushes price toward strike
4. Price approaches strike → Gamma peaks → Hedging intensifies
5. **Result**: Price "pins" to strike

This is not manipulation—it is the **mechanical equilibrium** of offsetting hedging flows.

### Max Pain as Structural Attractor

"Max pain" is the price at which total option value at expiry is minimized.

**Structural interpretation**:
- Not a conspiracy theory
- A natural attractor point where hedging flows balance
- Most option buyers lose (time decay)
- Dealers profit from theta
- Price gravitates toward minimal net hedging requirement

**Mechanical consequence**:
Price discovery is not purely information-driven. It is also **structure-driven**.

Derivatives positioning creates price attractors independent of fundamental value.

---

## Capital Efficiency Advantage

Derivatives have inherent structural advantages over cash markets for expressing views and managing risk.

### Transaction Cost Asymmetry

| **Cost Component**        | **Cash Market** | **Derivatives Market** |
|---------------------------|----------------|------------------------|
| STT (Securities Transaction Tax) | 0.1% (sell) | 0.05% (sell futures) / 0.125% (sell options) |
| Stamp Duty               | 0.015%         | 0.003% (futures) / 0.003% (options) |
| Margin Requirement       | 100% (delivery) | ~10-20% (margin)     |
| Settlement Time          | T+1 (delivery) | Same day (cash settlement) |
| Liquidity Concentration  | Fragmented     | Concentrated         |

**Mechanical consequence**:
For identical exposure, derivatives are:
- **Cheaper** to transact
- **Faster** to adjust
- **More capital efficient**

This is not a temporary advantage—it is **structurally embedded** in market design.

### Convexity Access

Options provide non-linear payoff structures unavailable in cash:
- Limited downside, unlimited upside (long calls)
- Defined risk, leveraged exposure (long puts)
- Time decay monetization (short options)

**Mechanical consequence**:
Sophisticated capital (hedgers, volatility traders, relative value players) **must** use derivatives.

This forces price-relevant information into derivatives markets first.

---

## Information Aggregation Mechanism

Derivatives aggregate information more efficiently than cash markets due to structural properties.

### Continuous Marking and Repricing

Derivatives contracts are continuously marked-to-market:
- Futures: Daily settlement based on closing price
- Options: Continuous IV recalibration based on trades

**Mechanical consequence**:
- New information is immediately reflected in derivatives pricing
- No lag from delivery or settlement mechanics
- Price discovery is **real-time**, not delayed

### Concentration of Informed Flow

Because derivatives are more efficient, informed participants preferentially trade derivatives:
- Hedgers with portfolio risk → Use index options/futures
- Relative value traders → Use multi-leg derivatives strategies
- Macro participants → Use derivatives for leverage and speed

**Mechanical consequence**:
The **information-to-noise ratio is higher** in derivatives than cash:
- Cash markets include long-term buy-and-hold flow (low information)
- Derivatives strip this out, exposing only active positioning
- Price discovery is faster and more accurate

This is not about derivatives being "smarter"—it is about structural filtering of passive capital.

---

## Leverage and Forced Unwind Dynamics

Leverage in derivatives creates forced unwind mechanics absent in cash markets.

### Margin Call Cascades

When price moves against leveraged derivatives positions:
1. Mark-to-market losses accrue
2. Margin requirements increase (or initial margin is breached)
3. Participants must either:
   - Add capital (often impossible at speed)
   - Close positions (forced liquidation)

**Mechanical consequence**:
Leveraged positions create **automatic liquidation pressure** when price moves.

This is a **one-way forcing function**:
- Losses trigger forced selling
- Gains do not trigger forced buying (asymmetric)

### Amplification Through Deleveraging

When forced liquidations occur:
1. Participant closes derivatives position
2. Dealer unwinds corresponding hedge in underlying
3. **Underlying price moves**
4. Other leveraged participants hit margin thresholds
5. **Cascade effect**

**Mechanical consequence**:
Small initial moves in derivatives can trigger non-linear amplification in underlying cash markets.

This is not "irrational panic"—it is the **deterministic outcome** of margin mechanics.

---

## Weekly Expiries: Structural Time Compression

India's weekly index expiries create unique structural forcing mechanisms.

### Time Compression Effect

Weekly expiries mean:
- Option theta decay is compressed into 5-7 days
- Hedging adjustments are more frequent
- Roll decisions cannot be indefinitely delayed
- Position unwinds are forced on a weekly cycle

**Mechanical consequence**:
Decision-making timelines are **structurally shortened**.

Capital that in monthly expiry regimes could "wait and see" must now act within days.

### Predictable Forcing Windows

52 weekly expiries per year create:
- Predictable hedging windows (final 2-3 days before expiry)
- Predictable unwind pressure (morning of expiry)
- Predictable roll activity (day after expiry)

**Mechanical consequence**:
Forced capital actions are **time-stamped and observable**.

Galactus explicitly models these windows as structural constraints.

---

## Why Cash Markets Follow Derivatives

Cash markets are **structurally subordinate** to derivatives in Indian markets due to:

1. **Capital velocity**: Derivatives adjust faster
2. **Transaction costs**: Derivatives are cheaper
3. **Leverage**: Derivatives amplify exposure
4. **Hedging needs**: Derivatives positioning forces cash flows
5. **Expiry mechanics**: Derivatives create time-bound pressure
6. **Information efficiency**: Derivatives attract informed flow

**Mechanical consequence**:
Price discovery in cash markets increasingly reflects:
- Resolution of derivatives-driven pressure
- Hedging-induced flows
- Forced settlement mechanics

**Not**:
- Pure supply/demand for delivery
- Long-term fundamental revaluation
- Organic accumulation/distribution

This is not speculation—it is the **observable structural reality** of Indian markets.

---


## Key Actors in the Derivatives Ecosystem

### 1. Options Buyers (Primarily Retail)

Characteristics:
- Seek convex outcomes
- Accept frequent small losses
- Cluster around round strikes and narratives

Their behavior:
- Creates concentrated open interest
- Does not directly move price
- Forces other participants to hedge

### 2. Options Sellers / Dealers

Characteristics:
- Typically neutral on direction
- Sensitive to gamma and vega exposure
- Manage risk dynamically

Their behavior:
- Creates **forced hedging flows**
- Responds mechanically to price movement
- Can pin or accelerate price near key strikes

Dealers are a primary source of **forced capital** in Galactus' model.

### 3. Institutional Hedgers

Characteristics:
- Use derivatives to manage portfolio risk
- Often indifferent to short-term price
- Operate under mandate and risk constraints

Their activity:
- Introduces structural demand for options
- Amplifies hedging-related flows

---

## The Derivatives-to-Cash Transmission Framework

### Formal Transmission Channels

Derivatives positioning transmits to cash market price through five mechanical channels:

#### Channel 1: Delta Hedging Flow
- **Source**: Dealer neutralization of option exposure
- **Mechanism**: Continuous buying/selling of underlying to maintain delta neutrality
- **Magnitude**: Proportional to delta-weighted open interest
- **Timing**: Continuous, intensifies near expiry

#### Channel 2: Forced Settlement Flow
- **Source**: Expiry-driven contract settlement
- **Mechanism**: Physical/cash settlement requires underlying transaction
- **Magnitude**: Proportional to in-the-money open interest at expiry
- **Timing**: Concentrated on expiry day/morning

#### Channel 3: Roll Flow
- **Source**: Position holders rolling from expiring to next-dated contracts
- **Mechanism**: Simultaneous sell (expiring) + buy (next-dated) creates spread pressure
- **Magnitude**: Proportional to total open interest being rolled
- **Timing**: 1-3 days before and after expiry

#### Channel 4: Margin-Induced Flow
- **Source**: Adverse price movement triggering margin calls
- **Mechanism**: Forced liquidation of derivatives positions → dealer hedge unwind in underlying
- **Magnitude**: Non-linear, depends on leverage distribution
- **Timing**: Unpredictable triggers, cascade effects possible

#### Channel 5: Gamma-Driven Feedback Flow
- **Source**: Price movement changing dealer delta exposure
- **Mechanism**: Positive gamma dampens moves, negative gamma amplifies moves
- **Magnitude**: Proportional to gamma exposure at current price
- **Timing**: Instantaneous response to price changes

**Mechanical consequence**:
These channels are **simultaneously active**, creating multi-dimensional pressure on cash markets.

Galactus models each channel explicitly and aggregates their combined effect.

---

## What Galactus Models Explicitly

Galactus models:
- **Open interest concentration and change**: Identifies price attractors and gravity wells
- **Gamma exposure distribution**: Determines mechanical price response function (acceleration vs dampening)
- **Time decay pressure**: Quantifies forced action timelines
- **Delta-hedging-induced capital flows**: Estimates standing bid/offer from dealer hedging needs
- **Interaction between derivatives and cash liquidity**: Assesses transmission efficiency and impact amplification
- **Expiry-driven forced flow**: Predicts timing and magnitude of settlement pressure
- **Margin-based unwind risk**: Monitors leverage-driven cascade potential

These are modeled as **pressure gradients**, not directional bets.

Each model component maps to a specific structural transmission mechanism.

---

## What Galactus Ignores Deliberately

Galactus does **not** model:
- **Individual option trades**: Too granular, noisy, and non-systematic
- **Implied volatility as a standalone signal**: IV is derivative of positioning and risk appetite, not causal
- **Option "sentiment"**: Ambiguous and non-binding; positioning matters, not opinion
- **Retail payoff narratives**: Stories and dreams do not move markets; capital under constraints does

These are either noisy or derivative of deeper mechanics.

**Rationale**:
Galactus focuses on **structural transmission mechanisms**, not surface-level indicators.

IV, sentiment, and narratives may correlate with price, but they do not **mechanically force** price movement.

Only capital under constraints has that property.

---

## Structural vs Empirical Claims

### What This Document Claims (Structural)

✅ Derivatives markets have **inherent structural advantages**:
- Lower transaction costs
- Higher capital efficiency  
- Faster adjustment speed
- Embedded leverage
- Continuous repricing

✅ Dealer hedging creates **mechanical transmission** of derivatives positioning to cash markets

✅ Weekly expiries create **time-bound forcing functions** for capital action

✅ Margin systems create **amplification and cascade potential**

✅ These structural properties are **durable** because they are embedded in market design

### What This Document Does NOT Claim (Empirical)

❌ Derivatives positioning always predicts price direction correctly

❌ Hedging flows always dominate other capital forces

❌ Pinning always occurs at max pain

❌ Expiry effects are uniformly strong across all market conditions

❌ Structural advantages guarantee profitability

**Critical distinction**:
Structural mechanics explain **how** derivatives influence price, not **how much** or **how reliably**.

Galactus infers pressure from structure. Outcomes remain uncertain.

---

## Failure Modes in Derivatives Modeling

Galactus acknowledges that:
- Sudden news can overwhelm derivative constraints
- Liquidity can disappear unexpectedly
- Extreme positioning can unwind non-linearly

Derivative dominance is **situational**, not absolute.

The structural mechanics documented here describe **how** derivatives influence price when market structure is functioning normally. They do not claim derivatives constraints are unbreakable.

---

## Implications for System Design

Because derivatives dominate price discovery through structural mechanics:
- Options data is first-class input (not secondary or supplementary)
- Expiry calendars are structural constraints (not background context)
- Intraday behavior cannot be analyzed without derivative context (cash-only analysis is incomplete)
- Hedging flows must be modeled explicitly (they are causal, not correlational)

Any system ignoring derivatives is structurally blind in Indian markets.

---

## Final Statement

**In Indian markets, price often moves to satisfy derivatives constraints, not to express belief.**

Galactus exists to quantify those constraints.

This document formalizes the **structural mechanics** that make derivatives the primary price discovery mechanism:

1. **Capital efficiency**: Leverage, lower costs, faster adjustment
2. **Hedging transmission**: Dealer delta/gamma hedging creates mechanical flows
3. **Forced settlement**: Expiry and margin mechanics create time-bound pressure
4. **Information concentration**: Informed capital preferentially uses derivatives
5. **Amplification dynamics**: Leverage creates non-linear cascade potential

These are not empirical observations subject to regime change—they are **embedded in market structure**.

Understanding these mechanics is necessary for inference. Galactus does not guess at price—it models the structural forces that constrain price formation.

