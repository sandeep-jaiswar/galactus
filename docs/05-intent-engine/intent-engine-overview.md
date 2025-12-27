# Galactus — Intent Engine Overview

## Purpose of This Document

This document defines the **Intent Engine**, the core inference component of Project Galactus.

It exists to:
- Translate market events into capital-intent inference
- Enforce deterministic, explainable reasoning
- Separate observation from interpretation
- Provide stable, auditable outputs for downstream consumers

The Intent Engine is the *brain* of Galactus, not its hands.

---

## What the Intent Engine Is

The Intent Engine is a **deterministic inference system** that:

- Consumes canonical market events
- Computes capital pressure metrics
- Infers intent and regime states
- Emits structured, non-actionable outputs

It reasons about **constraints and pressure**, not prices or trades.

---

## Scope

The Intent Engine has explicit, well-defined boundaries for what it owns and what it delegates.

### In Scope — What the Intent Engine Owns

The Intent Engine is responsible for:

1. **Event Processing**
   - Consuming canonical market events in event-time order
   - Validating event completeness and consistency
   - Maintaining event replay capability

2. **Capital Pressure Computation**
   - Computing forced flow metrics from derivative positions
   - Calculating liquidity constraint indicators
   - Measuring structural capital pressure from multiple sources
   - Aggregating pressure signals with explicit rules

3. **Regime Classification**
   - Determining current market regime state
   - Detecting regime transitions
   - Maintaining regime stability indicators
   - Providing regime confidence scores

4. **Intent Inference**
   - Inferring capital behavior intent from pressure metrics
   - Classifying intent as forced vs strategic
   - Measuring intent strength and directionality
   - Detecting conflicting or offsetting intents

5. **Confidence Assessment**
   - Computing confidence scores for all outputs
   - Assessing data quality impact on confidence
   - Measuring structural alignment confidence
   - Providing explicit uncertainty bounds

6. **Output Generation**
   - Emitting structured intent vectors
   - Producing explanatory metadata
   - Generating confidence and stability indicators
   - Surfacing assumptions and limitations explicitly

### Out of Scope — What the Intent Engine Delegates

The Intent Engine explicitly **does not** handle:

1. **Data Ingestion**
   - Raw data fetching from exchanges
   - Data normalization and cleaning
   - Schema transformation
   - Historical data backfill

2. **Execution or Trading**
   - Buy/sell signal generation
   - Order placement logic
   - Position management
   - Broker integration

3. **Portfolio Management**
   - Position sizing
   - Risk allocation
   - Capital deployment strategies
   - PnL tracking

4. **Presentation and Visualization**
   - Chart generation
   - UI rendering
   - Report formatting
   - Alert delivery

5. **Strategy Backtesting**
   - Trading strategy simulation
   - Performance attribution
   - Risk-adjusted return calculations
   - Strategy optimization

### Component Boundaries

The Intent Engine sits between:
- **Upstream**: Data ingestion and canonicalization layer
- **Downstream**: Analysis, research, and decision support systems

It maintains strict isolation:
- Does not reach back into raw data sources
- Does not make trading or advisory decisions
- Does not optimize for downstream strategy performance

---

## What the Intent Engine Is Not

The Intent Engine is explicitly **not**:

- A trading engine
- A signal generator for execution
- A prediction model
- A real-time decision system
- A black-box optimizer
- A data ingestion system
- A backtesting framework
- A portfolio management tool
- An advisory platform

Any attempt to extend the Intent Engine beyond inference is a violation of design principles.

---

## Inputs to the Intent Engine

The Intent Engine has strict input requirements to ensure determinism and reproducibility.

### Input Categories

#### 1. Canonical Market Events

**Required Properties:**
- Must arrive via standardized event schemas
- Must include explicit event timestamps (exchange time)
- Must be ordered in event-time sequence
- Must be immutable once published

**Event Types:**
- Trade executions (price, volume, timestamp)
- Order book snapshots (bid/ask levels, depth)
- Derivative positions (open interest, net delta, gamma exposure)
- Corporate actions (dividends, splits, bonus issues)
- Index rebalancing events
- Expiry and settlement events

**Data Quality Requirements:**
- Completeness: No missing critical fields
- Consistency: Cross-source validation passing
- Timeliness: Arrival within expected latency bounds
- Validity: Schema validation passing

#### 2. Configuration Parameters

**Versioned Configuration:**
- Intent computation parameters (versioned)
- Regime classification thresholds (versioned)
- Confidence scoring weights (versioned)
- Time window definitions (explicit)

**Configuration Properties:**
- Must be immutable within a computation run
- Must be versioned explicitly
- Must be auditable and reproducible
- Must be validated against schema

#### 3. Reference Data

**Static Reference Data:**
- Trading calendars (exchange holidays, expiry dates)
- Instrument metadata (lot size, tick size, circuit limits)
- Index constituents and weights
- Market microstructure parameters

**Reference Data Properties:**
- Must be time-versioned (valid-from, valid-to)
- Must be replayable for historical analysis
- Must be schema-validated
- Must support point-in-time queries

### Input Validation

The Intent Engine must validate:
- Schema compliance for all events
- Timestamp consistency and ordering
- Completeness of required fields
- Range validity for numeric fields
- Reference data availability for event timestamps

### Input Time Semantics

**Event Time:**
- All inputs use event time (exchange timestamp)
- Late-arriving events trigger recomputation if within acceptable window
- Out-of-order events are rejected or buffered for reordering

**Processing Time:**
- Tracked separately from event time
- Used only for monitoring and diagnostics
- Never influences computation results

### Input Boundaries

The Intent Engine:
- **Does not** fetch data from external sources directly
- **Does not** perform data cleaning or normalization
- **Does not** make assumptions about missing data
- **Does not** interpolate or impute values

All data preparation is the responsibility of the upstream ingestion layer.

---

## Core Responsibilities

The Intent Engine is responsible for the following steps:

1. **Event Validation and Ordering**
   - Validate incoming canonical events
   - Ensure event-time ordering
   - Detect and handle late or out-of-order events

2. **Deterministic Feature Computation**
   - Compute pressure metrics from raw events
   - Aggregate multi-source signals
   - Apply time-windowed transformations

3. **Capital Pressure Inference**
   - Identify forced flow components
   - Measure liquidity constraints
   - Detect structural capital pressure

4. **Regime Classification**
   - Classify current market regime
   - Detect regime transitions
   - Assess regime stability

5. **Confidence and Stability Evaluation**
   - Compute confidence scores
   - Assess output stability
   - Surface uncertainty explicitly

Each step is isolated, testable, and deterministic.

---

## Explicit Non-Goals

The Intent Engine explicitly **will never**:

### Never Predict Prices
- No future price forecasting
- No price target estimation
- No upside/downside range prediction
- No return forecasting models
- No time-series price prediction

**Rationale:** Price is an outcome, not a primitive. The Intent Engine infers **why capital must act**, not **what price will do**.

### Never Generate Trading Signals
- No buy/sell recommendations
- No entry/exit points
- No position sizing advice
- No stop-loss or take-profit levels
- No trade execution logic

**Rationale:** Trading decisions depend on external constraints (risk appetite, capital, portfolio) beyond the Intent Engine's scope.

### Never Optimize for Trading Performance
- No PnL optimization
- No Sharpe ratio maximization
- No win-rate targeting
- No drawdown minimization
- No alpha generation claims

**Rationale:** Performance optimization encourages overfitting and obscures structural inference.

### Never Provide Advisory Outputs
- No personalized recommendations
- No portfolio-specific advice
- No user-tailored inferences
- No risk profiling for individuals
- No investment guidance

**Rationale:** Advisory crosses regulatory boundaries and compromises objectivity.

### Never Hide Uncertainty
- No false precision when confidence is low
- No overconfident outputs
- No masking of data quality issues
- No smoothing over structural ambiguity
- No silent failure modes

**Rationale:** Uncertainty is information. Hiding it is dishonest and dangerous.

### Never Use Black-Box Logic
- No unexplainable models in production
- No opaque deep learning without causal grounding
- No "it works but we don't know why" components
- No inference without market-structure justification

**Rationale:** Explainability is mandatory. If it can't be explained, it can't be trusted.

### Never Optimize for Speed Over Correctness
- No microsecond latency requirements
- No shortcuts that compromise determinism
- No "fast path" that skips validation
- No real-time execution pressure

**Rationale:** Correctness and reproducibility matter more than speed.

### Never Accumulate Silent Technical Debt
- No temporary hacks that become permanent
- No undocumented assumptions
- No magic numbers without justification
- No "we'll document it later" logic

**Rationale:** Technical debt in inference logic is a time bomb.

### Never Cross Architectural Boundaries
- No data fetching from external sources
- No direct database access
- No presentation logic
- No backtesting logic
- No portfolio management

**Rationale:** Clean boundaries enable independent evolution and testing.

### Never Compromise Determinism
- No random number generation without seeds
- No system-time dependencies
- No hidden state
- No non-reproducible operations
- No platform-specific behavior

**Rationale:** Determinism is the foundation of trust.

---

## Outputs of the Intent Engine

The Intent Engine produces structured, non-actionable inference outputs.

### Output Structure: Intent Vectors

Each output is an **Intent Vector** containing:

#### 1. Capital Pressure Metrics

**Pressure Magnitude:**
- Numeric measure of capital constraint intensity
- Normalized to comparable scale (0-100 or similar)
- Includes component breakdown (forced vs strategic)

**Pressure Directionality:**
- Directional: Net buying or selling pressure
- Neutral: Pinning or stabilizing pressure
- Volatility-expanding: Amplification risk
- Indeterminate: Unclear or conflicting signals

**Pressure Composition:**
- Source attribution (derivative expiry, index rebalancing, etc.)
- Component weights and relative contributions
- Interaction effects between pressure sources

#### 2. Regime Classification

**Current Regime State:**
- Regime label (e.g., "Forced Expiry", "Quiet Accumulation", "Regime Transition")
- Regime start timestamp
- Expected regime duration (if determinable)

**Regime Stability:**
- Stability score (how persistent is the regime)
- Transition probability (likelihood of regime change)
- Historical regime frequency and duration

#### 3. Intent Classification

**Intent Type:**
- Forced: Capital must act due to constraints
- Strategic: Capital may act based on positioning
- Mixed: Combination of forced and strategic

**Intent Strength:**
- Magnitude of inferred capital intent
- Time horizon of intent (immediate, near-term, medium-term)
- Expected resolution timeline

#### 4. Confidence and Uncertainty

**Confidence Score:**
- Overall confidence in inference (0-100)
- Component confidence scores:
  - Data quality confidence
  - Structural alignment confidence
  - Regime consistency confidence
  - Signal agreement confidence

**Uncertainty Indicators:**
- Known limitations and assumptions
- Data quality issues affecting inference
- Regime ambiguity flags
- Conflicting signal warnings

#### 5. Explanatory Metadata

**Inference Justification:**
- Key events driving inference
- Market structure mechanisms at play
- Specific metrics and thresholds triggered

**Assumptions and Limitations:**
- Explicit list of assumptions made
- Known failure modes and edge cases
- Data limitations and coverage gaps

**Traceability:**
- Input event IDs that contributed to output
- Configuration version used
- Computation timestamp
- Engine version identifier

### Output Formats

**Schema Requirements:**
- All outputs must conform to versioned schemas
- Schema changes must be backward compatible or explicitly versioned
- Outputs must be serializable (JSON, Protocol Buffers, etc.)

**Output Stability:**
- Outputs must be deterministic given same inputs
- Schema structure must be stable within major version
- Field deprecation must follow explicit policy

### Output Contracts and Guarantees

#### Determinism Guarantee
Given identical:
- Event stream
- Configuration version
- Engine version
- Reference data

The Intent Engine **must** produce functionally identical outputs:
- All structural decisions (classifications, regime states, intent types) must be identical
- All integer metrics and counts must be identical
- All floating-point values must match within documented precision tolerances
- All metadata, timestamps, and IDs must be byte-for-byte identical

For critical computations requiring exact reproducibility, fixed-point arithmetic or explicit rounding must be used.

#### Completeness Guarantee
Every output must include:
- All required fields (no nulls for mandatory fields)
- Confidence scores for all inferences
- Explanatory metadata
- Timestamp and versioning information

#### Non-Actionability Guarantee
Outputs must **never** include:
- Buy/sell recommendations
- Price targets or predictions
- Position sizing advice
- Entry/exit signals
- Trade execution instructions

### Output Language Constraints

**Allowed Language:**
- "Capital pressure indicates..."
- "Regime classified as..."
- "Confidence level is..."
- "Structural mechanism suggests..."
- "Intent inferred as..."

**Forbidden Language:**
- "Buy" / "Sell"
- "Bullish" / "Bearish"
- "Target price"
- "Expected return"
- "Recommended action"

See [`docs/09-compliance-and-language/language-guidelines.md`](../09-compliance-and-language/language-guidelines.md) for language guidelines and [`docs/09-compliance-and-language/output-restrictions.md`](../09-compliance-and-language/output-restrictions.md) for output restrictions.

### Output Consumers

Expected downstream consumers:
- Research and analysis systems
- Monitoring and alerting systems
- Decision support tools (non-advisory)
- Archival and audit systems

The Intent Engine does not control or restrict how outputs are consumed, but it must not produce advisory or actionable content.

---

## Determinism Guarantee

Determinism is a non-negotiable requirement of the Intent Engine.

### Determinism Definition

The Intent Engine is deterministic if and only if:

**Given identical:**
1. Event stream (same events, same order, same timestamps)
2. Configuration version (same parameter values)
3. Engine version (same code version)
4. Reference data (same point-in-time state)

**Then:**
- The engine produces **functionally identical** outputs
- Every structural decision (classification, regime, intent type) is identical
- Every metric, score, and timestamp is identical
- Floating-point values match within documented precision tolerances (or use fixed-point)
- All metadata, IDs, and versioning information are byte-for-byte identical

### Determinism Requirements

#### No Hidden State
- The engine must not maintain internal state between runs
- State, if required, must be passed explicitly as input
- No in-memory caches that affect computation
- No global variables that influence results

#### No Randomness
- No random number generation in production logic
- No random sampling or Monte Carlo methods
- If randomness is required (e.g., research), it must use explicit, reproducible seeding
- Random components must be clearly marked and justified

#### No Time-Dependent Behavior
- System clock must not influence computation
- All time semantics use event time, not processing time
- No "current time" dependencies in inference logic
- Timeout behavior must be explicit and reproducible

#### No External Dependencies
- No network calls during computation
- No database queries that could return different results
- No file system dependencies that could change
- All required data passed as explicit inputs

#### Reproducibility Across Platforms
- Results must be identical across different machines
- Results must be identical across operating systems
- Floating-point operations must use deterministic rounding modes
- Library versions must be pinned and versioned

### Version Handling

#### Engine Version
- Every computation must record engine version
- Version changes must be tracked explicitly
- Breaking changes require major version bump
- Version compatibility must be documented

#### Configuration Version
- Configuration changes must be versioned
- Version must be recorded with every output
- Configuration history must be auditable
- Point-in-time configuration must be retrievable

#### Schema Version
- Input and output schemas must be versioned
- Schema evolution must be managed explicitly
- Backward compatibility must be maintained or documented
- Schema version must be included in outputs

### Edge Cases and Boundary Conditions

#### Handling Late Events
- Late events within tolerance window trigger recomputation
- Recomputation must be deterministic
- Late event policy must be explicit and versioned

#### Handling Missing Data
- Missing data must be detected and flagged
- No imputation or interpolation without explicit policy
- Confidence must degrade when data is missing
- Missing data handling must be deterministic

#### Handling Conflicting Signals
- Conflict resolution rules must be explicit
- Aggregation logic must be deterministic
- Conflicts must be surfaced in output metadata
- No arbitrary tie-breaking without documented policy

#### Handling Numerical Precision
- Floating-point operations must be documented
- Precision requirements must be explicit (e.g., epsilon for comparisons)
- Critical computations requiring exact reproducibility must use fixed-point arithmetic
- Comparison operations must use documented tolerance levels (e.g., 1e-10)
- Determinism tests must verify functional equivalence, not byte-level equality for floats
- Platform-specific floating-point behavior must be documented and tested

### Verification and Testing

#### Replay Testing
- All historical event streams must be replayable
- Replayed results must match original results exactly
- Replay tests must be part of CI/CD pipeline

#### Determinism Tests
- Same input runs must produce identical outputs
- Cross-platform tests must verify consistency
- Version upgrade tests must verify compatibility

#### Regression Testing
- Output changes must be intentional and documented
- Unintentional output changes must fail tests
- Historical baseline outputs must be maintained

### Violations and Exceptions

Any violation of determinism is considered a **critical bug** and must be:
- Fixed immediately
- Documented in incident log
- Analyzed for root cause
- Prevented through additional testing

**No exceptions are allowed** for production code.

### Monitoring Determinism

#### Continuous Verification
- Regular replay tests against historical data
- Cross-validation between different engine instances
- Automated alerts for non-deterministic behavior

#### Audit Trail
- Every computation must be auditable
- Input hashes must be recorded
- Output checksums must be tracked
- Reproducibility must be demonstrable

---

## Explainability Requirement

Every output must be traceable to:

- Specific input events
- Explicit metrics
- Documented assumptions

If an output cannot be explained in market-structure terms, it is invalid.

---

## State Handling

The Intent Engine:
- Is stateless by default
- Accepts state explicitly when required
- Does not maintain hidden internal state

State transitions must be explicit and auditable.

---

## Time Semantics

The engine reasons strictly in **event time**.

- Late events trigger recomputation
- Time windows are explicit
- Decay is modeled, not assumed

---

## Failure and Uncertainty Handling

The Intent Engine:
- Surfaces uncertainty explicitly
- Degrades confidence under poor data
- Is allowed to produce no inference

Silence is preferable to false confidence.

---

## Performance Characteristics

The engine is designed for:
- Predictable latency
- Throughput aligned with event frequency
- Stability under stress conditions (expiry, volatility spikes)

It does not optimize for microsecond latency.

---

## Boundaries and Interfaces

The Intent Engine:
- Consumes events via defined interfaces
- Emits intent vectors via stable schemas
- Is isolated from presentation and research layers

Boundary violations are architectural failures.

---

## Evolution Strategy

The Intent Engine evolves through:
- Addition of new signals (via lifecycle)
- Schema versioning
- Controlled deprecation

Its core responsibility remains constant.

---

## Final Statement

**The Intent Engine does not decide what to do.**

It decides **what is happening**—clearly, deterministically, and honestly.
