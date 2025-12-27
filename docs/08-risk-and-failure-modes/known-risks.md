# Galactus — Known Risks

## Purpose of This Document

This document catalogs **known structural, data, behavioral, and model risks** inherent to the Galactus capital-pressure inference system.

It exists to:
- Acknowledge limitations and failure modes
- Inform monitoring and detection priorities
- Guide graceful degradation strategies
- Set realistic expectations for downstream consumers
- Document risk mitigation approaches

These are **known** risks—they can be anticipated, monitored, and managed. See [`unknown-unknowns.md`](unknown-unknowns.md) for non-enumerable risks.

---

## Risk Classification Framework

Risks are classified across four dimensions:

1. **Structural Risks** — Market structure changes that invalidate core assumptions
2. **Data Risks** — Data availability, quality, or integrity failures
3. **Behavioral Risks** — Participant behavior deviating from modeled patterns
4. **Model Risks** — Inference logic limitations or failure modes

Each risk is assessed on:
- **Likelihood**: LOW, MEDIUM, HIGH
- **Impact**: LOW, MEDIUM, HIGH, CRITICAL
- **Detectability**: EASY, MODERATE, DIFFICULT
- **Mitigation**: Documented response strategy

---

## Structural Risks

Structural risks arise from changes to market mechanics, regulation, or infrastructure that invalidate core assumptions.

### SR-1: Derivatives Market Disruption

**Description:**  
Sudden collapse or restriction of derivatives trading, eliminating the primary source of forced capital inference.

**Underlying Assumption:**  
Derivatives dominance (see [`indian-market-structural-assumptions.md`](../01-market-theory/indian-market-structural-assumptions.md), Assumption 1)

**Potential Triggers:**
- Regulatory ban on retail derivatives participation
- Exchange-level trading halts or circuit breakers
- Market-maker withdrawal during extreme stress
- Systemic clearing or settlement failures

**Impact:** CRITICAL  
- Primary inference signals become unavailable
- Forced flow detection fails completely
- Hedge pressure models become irrelevant

**Likelihood:** LOW (but non-zero during extreme crisis)

**Detectability:** EASY  
- Immediate and obvious from derivatives volume collapse
- Open interest stagnation or dramatic decline
- Bid-ask spreads widening beyond normal bounds

**Mitigation Strategy:**
1. Monitor derivatives-to-cash volume ratio continuously
2. Detect market-maker withdrawal patterns early
3. Implement kill switch when derivatives liquidity falls below threshold
4. Degrade to cash-only inference (limited capability)
5. Explicit system-wide confidence suppression
6. Surface warning: "Derivatives market disruption detected"

**Related Documentation:**
- [`kill-switch-criteria.md`](kill-switch-criteria.md) (Data Integrity Failure)
- [`derivatives-dominance.md`](../01-market-theory/derivatives-dominance.md)

---

### SR-2: Expiry Mechanics Change

**Description:**  
Regulatory or exchange changes to expiry structure (e.g., elimination of weekly expiries, introduction of T+0 settlement, cash settlement changes).

**Underlying Assumption:**  
Expiry effects create time-bound forced capital actions (see [`indian-market-structural-assumptions.md`](../01-market-theory/indian-market-structural-assumptions.md), Assumption 4)

**Potential Triggers:**
- Regulatory shift to monthly-only expiries
- Introduction of evergreen or perpetual contracts
- Changes to settlement mechanics reducing forced actions
- Move to smooth daily settlement instead of discrete expiry

**Impact:** HIGH  
- Time-bound pressure models lose primary driver
- Forced flow timing predictions fail
- Regime classification loses key anchor point

**Likelihood:** MEDIUM (regulatory discussions active)

**Detectability:** MODERATE  
- Advance warning through regulatory announcements
- Gradual behavioral changes before implementation
- Requires monitoring of market structure news

**Mitigation Strategy:**
1. Monitor regulatory announcements and consultations
2. Prepare fallback models not dependent on discrete expiries
3. Implement versioned expiry mechanics models
4. Gradual confidence degradation as expiry importance reduces
5. Adapt time-window models to new settlement patterns

**Related Documentation:**
- [`expiry-day-playbook.md`](../10-operational-playbooks/expiry-day-playbook.md)
- [`regime-classification.md`](../05-intent-engine/regime-classification.md)

---

### SR-3: Liquidity Regime Shift

**Description:**  
Fundamental change in market liquidity structure—either dramatic improvement (reducing inefficiencies) or collapse (making all inference meaningless).

**Underlying Assumption:**  
Liquidity fragmentation creates predictable impact patterns (see [`indian-market-structural-assumptions.md`](../01-market-theory/indian-market-structural-assumptions.md), Assumption 3)

**Potential Triggers:**
- Entry of large international market makers
- Algorithmic trading proliferation
- Regulatory changes to market-making incentives
- Crisis-driven liquidity withdrawal

**Impact:** HIGH  
- Price impact models miscalibrated
- Liquidity constraint detection fails
- Forced flow magnitude estimates incorrect

**Likelihood:** MEDIUM (gradual shift likely over years)

**Detectability:** MODERATE  
- Gradual degradation in bid-ask spreads
- Changes to price impact per unit capital
- Shift in depth distribution across order book

**Mitigation Strategy:**
1. Continuous monitoring of liquidity metrics
2. Adaptive calibration of impact models
3. Regime-specific liquidity profiles
4. Walk-forward validation of liquidity assumptions
5. Explicit confidence bounds that widen with liquidity uncertainty

**Related Documentation:**
- [`indian-market-inefficiencies.md`](../01-market-theory/indian-market-inefficiencies.md)
- [`walk-forward-validation.md`](../07-backtesting-and-validation/walk-forward-validation.md)

---

### SR-4: Participant Composition Shift

**Description:**  
Significant change in retail vs institutional capital balance, invalidating participant behavior models.

**Underlying Assumption:**  
Retail participation asymmetry (see [`indian-market-structural-assumptions.md`](../01-market-theory/indian-market-structural-assumptions.md), Assumption 2)

**Potential Triggers:**
- Mass retail disengagement or exodus
- Institutional withdrawal during crisis
- Rise of coordinated retail trading (meme-stock dynamics)
- Algorithmic retail trading via platforms

**Impact:** HIGH  
- Capital type classification fails
- Crowding metrics become invalid
- Forced vs discretionary capital distinction breaks down

**Likelihood:** MEDIUM (gradual shifts observable)

**Detectability:** DIFFICULT  
- Participant-level data often delayed or incomplete
- Behavioral changes precede detectable data shifts
- Requires inference from secondary signals

**Mitigation Strategy:**
1. Monitor trade size distributions and patterns
2. Track unusual correlation between retail indicators and price
3. Detect crowding anomalies early
4. Prepare alternative models for institutional-dominated regimes
5. Implement participant-agnostic pressure metrics as fallback

**Related Documentation:**
- [`retail-vs-institutional-dynamics.md`](../01-market-theory/retail-vs-institutional-dynamics.md)
- [`capital-behavior-model.md`](../01-market-theory/capital-behavior-model.md)

---

### SR-5: Constraint Homogenization

**Description:**  
Market participants adopt similar constraints, risk limits, or automated responses, creating correlated forced actions and cascade risks.

**Underlying Assumption:**  
Constraint asymmetry creates predictable pressure patterns (see [`indian-market-structural-assumptions.md`](../01-market-theory/indian-market-structural-assumptions.md), Assumption 6)

**Potential Triggers:**
- Proliferation of similar algorithmic strategies
- Standardization of risk management practices
- Regulatory harmonization across participant types
- Common use of third-party risk systems

**Impact:** HIGH  
- Constraint diversity assumptions violated
- Cascade risk amplification not modeled
- Forced flow independence assumptions break

**Likelihood:** MEDIUM (increasing with algo adoption)

**Detectability:** DIFFICULT  
- Requires meta-analysis of participant behavior
- Only visible during stress when correlations spike
- No direct observability of internal risk limits

**Mitigation Strategy:**
1. Monitor behavioral correlation across instruments
2. Detect unusual synchronization in capital actions
3. Stress-test for correlated unwind scenarios
4. Implement cascade risk indicators
5. Widen confidence bounds when correlation increases

**Related Documentation:**
- [`stress-scenarios.md`](../07-backtesting-and-validation/stress-scenarios.md)
- [`regime-breakdown.md`](regime-breakdown.md)

---

## Data Risks

Data risks arise from availability, quality, timeliness, or integrity failures in input data.

### DR-1: Exchange Data Feed Failure

**Description:**  
Partial or complete loss of real-time or historical data from primary exchange sources.

**Potential Triggers:**
- Exchange technical outages
- Data vendor failures
- Network connectivity issues
- API rate limiting or access restrictions

**Impact:** CRITICAL  
- Real-time inference becomes impossible
- Historical replay compromised
- Confidence calculation fails

**Likelihood:** MEDIUM (periodic minor outages expected)

**Detectability:** EASY  
- Immediate detection via data pipeline monitoring
- Absence of expected events within time windows
- Schema validation failures

**Mitigation Strategy:**
1. Multiple redundant data sources where possible
2. Automatic failover to backup feeds
3. Explicit kill switch on data unavailability
4. Clear communication to consumers about data gaps
5. No inference interpolation—silence is preferred

**Related Documentation:**
- [`kill-switch-criteria.md`](kill-switch-criteria.md) (Data Integrity Failure)
- [`data-sources.md`](../03-data-and-schemas/data-sources.md)
- [`data-failure-playbook.md`](../10-operational-playbooks/data-failure-playbook.md)

---

### DR-2: Options Chain Data Gaps

**Description:**  
Missing or incomplete options data (strikes, open interest, implied volatility) leading to forced flow misestimation.

**Potential Triggers:**
- Selective exchange data publication
- Data vendor coverage gaps
- Instrument-specific reporting delays
- Low liquidity strikes with no quotes

**Impact:** HIGH  
- Gamma exposure calculations incomplete
- Hedge pressure inference degraded
- Pin risk detection fails

**Likelihood:** MEDIUM (especially for far OTM strikes)

**Detectability:** MODERATE  
- Detectable via schema completeness checks
- Missing strikes or zero open interest (ambiguous)
- Requires cross-validation with known expiry patterns

**Mitigation Strategy:**
1. Define minimum required strike coverage for inference
2. Degrade confidence proportionally to coverage gaps
3. Flag instruments with insufficient options data
4. Suppress inference when core strikes missing
5. Explicit assumption documentation in outputs

**Related Documentation:**
- [`data-quality-rules.md`](../03-data-and-schemas/data-quality-rules.md) (Coverage dimension)
- [`forced-flow-detection.md`](../05-intent-engine/forced-flow-detection.md)

---

### DR-3: Historical Data Revisions

**Description:**  
Retroactive corrections or revisions to historical data creating inconsistency with past inferences.

**Potential Triggers:**
- Exchange data corrections (splits, bonuses, errors)
- Vendor data restatements
- Corporate action adjustments
- Data schema migrations

**Impact:** MEDIUM  
- Historical backtests become invalid
- Walk-forward validation compromised
- Reproducibility violated if not handled

**Likelihood:** MEDIUM (occasional corrections expected)

**Detectability:** MODERATE  
- Requires versioning and checksumming of data
- Comparison of current vs previous data snapshots
- Monitoring for unexpected historical changes

**Mitigation Strategy:**
1. Treat corrections as new forward-looking events, not retroactive mutations
2. Version all historical data snapshots
3. Maintain immutable event logs for audit
4. Recompute affected inferences explicitly if needed
5. Document data revisions in decision log

**Related Documentation:**
- [`schema-versioning.md`](../03-data-and-schemas/schema-versioning.md)
- [`backtest-design.md`](../07-backtesting-and-validation/backtest-design.md)

---

### DR-4: Time Synchronization Failures

**Description:**  
Event timestamp inconsistencies across sources or incorrect event ordering due to time sync issues.

**Potential Triggers:**
- Exchange time drift or errors
- Network latency variations
- Data vendor timestamp misalignment
- System clock desynchronization

**Impact:** HIGH  
- Event-time semantics violated
- Determinism compromised
- Causal ordering broken

**Likelihood:** LOW (but catastrophic when occurs)

**Detectability:** MODERATE  
- Detectable via out-of-order event detection
- Cross-source timestamp consistency checks
- Anomaly detection in event arrival patterns

**Mitigation Strategy:**
1. Strict event-time validation at ingestion
2. Out-of-order event rejection or buffering
3. Kill switch on event ordering ambiguity
4. Multiple time source validation
5. Mandatory NTP synchronization for all systems

**Related Documentation:**
- [`kill-switch-criteria.md`](kill-switch-criteria.md) (Time Semantics Violation)
- [`canonical-schemas.md`](../03-data-and-schemas/canonical-schemas.md)
- [`event-driven-architecture.md`](../02-system-architecture/event-driven-architecture.md)

---

### DR-5: Data Quality Degradation (Silent)

**Description:**  
Gradual degradation of data quality (accuracy, consistency) without explicit errors or alerts.

**Potential Triggers:**
- Data vendor methodology changes
- Exchange calculation updates
- Subtle schema interpretation differences
- Accumulation of small errors

**Impact:** MEDIUM  
- Inference slowly becomes unreliable
- Confidence metrics may not detect gradual drift
- Validation tests may not catch subtle changes

**Likelihood:** MEDIUM (inevitable over long horizons)

**Detectability:** DIFFICULT  
- Requires continuous quality monitoring
- Statistical process control on data characteristics
- Comparison with alternative sources
- Anomaly detection on derived metrics

**Mitigation Strategy:**
1. Continuous data quality monitoring and alerting
2. Statistical tests on data distributions over time
3. Periodic validation against ground truth sources
4. Walk-forward validation to catch drift
5. Explicit data quality confidence in all outputs

**Related Documentation:**
- [`data-quality-rules.md`](../03-data-and-schemas/data-quality-rules.md)
- [`confidence-and-stability.md`](../05-intent-engine/confidence-and-stability.md)

---

## Behavioral Risks

Behavioral risks arise when market participant actions deviate from modeled patterns.

### BR-1: Novel Forced Flow Mechanisms

**Description:**  
Emergence of new types of forced capital actions not captured by existing constraint models.

**Potential Triggers:**
- New derivative products with unique settlement mechanics
- Regulatory changes creating new forced actions
- Novel hedging strategies becoming widespread
- Introduction of new index rebalancing methodologies

**Impact:** HIGH  
- Forced flow detection misses significant capital pressure
- Constraint models incomplete
- Unexpected market impact from unmodeled forces

**Likelihood:** MEDIUM (market evolution inevitable)

**Detectability:** DIFFICULT  
- Requires active monitoring of new products and regulations
- Anomaly detection in unexplained price movements
- Research into emerging participant strategies

**Mitigation Strategy:**
1. Continuous research into market structure evolution
2. Placeholder for "unidentified forced flow" in models
3. Anomaly detection for unexplained capital pressure
4. Rapid signal development pipeline for new mechanisms
5. Explicit "known unknowns" flagging in outputs

**Related Documentation:**
- [`forced-flow-detection.md`](../05-intent-engine/forced-flow-detection.md)
- [`research-methodology.md`](../06-research-framework/research-methodology.md)
- [`unknown-unknowns.md`](unknown-unknowns.md)

---

### BR-2: Strategic Mimicry of Forced Flows

**Description:**  
Sophisticated participants intentionally creating signals that appear as forced flows to mislead inference systems.

**Potential Triggers:**
- Adversarial awareness of inference methodologies
- Strategic positioning to trigger mechanical responses
- Manipulation of visible open interest patterns
- Fake liquidity or spoofing-like behavior

**Impact:** MEDIUM  
- Inference produces false confidence
- Pressure direction or magnitude misestimated
- Downstream consumers misled

**Likelihood:** LOW (requires sophistication and scale)

**Detectability:** DIFFICULT  
- Indistinguishable from genuine forced flows initially
- Requires pattern analysis over time
- Detection via failure of expected pressure resolution

**Mitigation Strategy:**
1. Model validation via realized outcomes
2. Confidence degradation when signals contradict expectations
3. Multi-signal confluence requirements to avoid single-source manipulation
4. Monitoring for unusual pattern repetition
5. Avoid publishing detailed inference methodologies publicly

**Related Documentation:**
- [`confidence-and-stability.md`](../05-intent-engine/confidence-and-stability.md) (Signal Agreement dimension)
- [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md)

---

### BR-3: Regime Transition Blind Spots

**Description:**  
Markets transitioning between regimes faster than detection systems can identify, leading to inappropriate inference.

**Potential Triggers:**
- Sudden exogenous shocks (geopolitical, macro)
- Flash crashes or extreme volatility
- Regulatory announcements during trading
- Rapid shift in participant behavior

**Impact:** HIGH  
- Inference applies wrong regime assumptions
- Confidence remains high while inappropriate
- Delayed detection of regime invalidation

**Likelihood:** MEDIUM (inevitable during crisis)

**Detectability:** MODERATE  
- Rapid signal divergence across instruments
- Confidence metrics begin to degrade
- Unusual volatility spikes
- Requires fast regime monitoring

**Mitigation Strategy:**
1. Real-time regime monitoring with fast detection
2. Explicit "regime transition" state with suppressed inference
3. Conservative confidence during ambiguous periods
4. Multiple regime indicators for cross-validation
5. Kill switch on regime indeterminacy

**Related Documentation:**
- [`regime-classification.md`](../05-intent-engine/regime-classification.md)
- [`regime-breakdown.md`](regime-breakdown.md)
- [`kill-switch-criteria.md`](kill-switch-criteria.md) (Regime Indeterminacy)

---

### BR-4: Crowding and Feedback Loops

**Description:**  
Multiple systems or participants using similar inference logic, creating self-reinforcing feedback loops and crowded positioning.

**Potential Triggers:**
- Widespread adoption of similar capital-pressure methodologies
- Algorithmic strategies converging on common signals
- Retail platforms surfacing similar metrics
- Proliferation of derivatives-based strategies

**Impact:** MEDIUM  
- Pressure dynamics change due to inference-driven actions
- Models become self-referential
- Cascade risk during unwinds
- Independence assumptions violated

**Likelihood:** MEDIUM (increases with system adoption)

**Detectability:** DIFFICULT  
- Requires meta-analysis of market impact
- Observable only when feedback becomes dominant
- Correlation spikes during synchronized actions

**Mitigation Strategy:**
1. Monitor for unusual correlation between inference and subsequent price action
2. Detect abnormal capital concentration in inferred pressure zones
3. Model crowd behavior as a distinct capital type
4. Confidence degradation when feedback suspected
5. Avoid publishing detailed signal methodologies

**Related Documentation:**
- [`capital-behavior-model.md`](../01-market-theory/capital-behavior-model.md)
- [`unknown-unknowns.md`](unknown-unknowns.md)

---

### BR-5: Institutional Coordination

**Description:**  
Large institutional participants coordinating actions, creating capital flows that appear forced but are actually discretionary and coordinated.

**Potential Triggers:**
- Crisis-driven coordination (central bank, regulators)
- Index provider coordination (simultaneous rebalancing)
- Informal coordination among large participants
- Algorithmic strategy synchronization

**Impact:** MEDIUM  
- Forced vs discretionary classification fails
- Pressure magnitude misestimated
- Timing predictions incorrect

**Likelihood:** LOW (requires unusual circumstances)

**Detectability:** DIFFICULT  
- Appears similar to genuine forced flows
- Only detectable post-facto through analysis
- Requires external information (news, regulatory)

**Mitigation Strategy:**
1. Monitor for unusual synchronization across participants
2. Incorporate regulatory/crisis signals into regime classification
3. Widen uncertainty bounds during crisis regimes
4. Distinguish "crisis coordination" as separate capital type
5. Explicit assumptions about coordination absence

**Related Documentation:**
- [`capital-pressure-model.md`](../05-intent-engine/capital-pressure-model.md)
- [`stress-scenarios.md`](../07-backtesting-and-validation/stress-scenarios.md)

---

## Model Risks

Model risks arise from limitations, errors, or failures in inference logic and computational methods.

### MR-1: Constraint Misidentification

**Description:**  
Incorrectly identifying which constraints are binding, leading to false forced flow inference.

**Potential Triggers:**
- Incomplete understanding of participant constraints
- Changes to constraint mechanics not reflected in models
- Edge cases in constraint logic
- Interaction effects between multiple constraints

**Impact:** HIGH  
- False forced flow detection
- Incorrect pressure direction
- Misestimated capital magnitude
- Confident nonsense output

**Likelihood:** MEDIUM (complex constraint landscape)

**Detectability:** MODERATE  
- Validation via pressure resolution (did expected action occur?)
- Cross-signal agreement (do other signals confirm?)
- Backtesting against known constraint events

**Mitigation Strategy:**
1. Conservative constraint binding thresholds
2. Multi-signal validation before high confidence
3. Continuous validation of constraint models
4. Explicit assumption documentation for each constraint
5. Confidence degradation for novel or ambiguous constraints

**Related Documentation:**
- [`forced-flow-detection.md`](../05-intent-engine/forced-flow-detection.md)
- [`kill-switch-criteria.md`](kill-switch-criteria.md) (Constraint Misidentification)

---

### MR-2: Aggregation and Interaction Errors

**Description:**  
Incorrect aggregation of multiple signals or failure to model interaction effects between pressure sources.

**Potential Triggers:**
- Linear aggregation of non-linear effects
- Double-counting of related pressure sources
- Offsetting pressures not properly netted
- Complex interaction effects ignored

**Impact:** MEDIUM  
- Pressure magnitude over or underestimated
- Confidence incorrectly high when signals conflict
- Non-linear effects missed

**Likelihood:** MEDIUM (aggregation is complex)

**Detectability:** MODERATE  
- Requires sophisticated validation
- Observable via systematic bias in pressure estimates
- Detectable through residual analysis

**Mitigation Strategy:**
1. Explicit, documented aggregation rules
2. Conflict detection and surfacing
3. Non-linear interaction modeling where justified
4. Conservative confidence when multiple signals interact
5. Continuous validation of aggregation accuracy

**Related Documentation:**
- [`capital-pressure-model.md`](../05-intent-engine/capital-pressure-model.md)
- [`confidence-and-stability.md`](../05-intent-engine/confidence-and-stability.md)

---

### MR-3: Calibration Drift

**Description:**  
Model parameters calibrated on historical data becoming invalid as market structure evolves.

**Potential Triggers:**
- Gradual market microstructure changes
- Participant behavior evolution
- Regulatory changes
- Liquidity regime shifts

**Impact:** MEDIUM  
- Systematic bias in pressure estimates
- Confidence metrics miscalibrated
- Thresholds no longer appropriate

**Likelihood:** HIGH (inevitable over long horizons)

**Detectability:** MODERATE  
- Walk-forward validation degradation
- Systematic over/under-prediction patterns
- Confidence miscalibration (actual outcomes vs predicted confidence)

**Mitigation Strategy:**
1. Regular walk-forward validation
2. Periodic recalibration of parameters
3. Adaptive thresholds based on recent performance
4. Version control of all calibration parameters
5. Confidence degradation when validation degrades

**Related Documentation:**
- [`walk-forward-validation.md`](../07-backtesting-and-validation/walk-forward-validation.md)
- [`backtest-design.md`](../07-backtesting-and-validation/backtest-design.md)

---

### MR-4: Floating-Point Precision Issues

**Description:**  
Numerical precision limitations causing non-deterministic behavior or incorrect computations.

**Potential Triggers:**
- Platform-specific floating-point behavior
- Accumulation of rounding errors
- Large magnitude differences in aggregation
- Order-dependent operations on floats

**Impact:** LOW to MEDIUM  
- Determinism violated
- Incorrect threshold crossings
- Confidence calculation errors
- Replay inconsistency

**Likelihood:** LOW (with proper design)

**Detectability:** EASY  
- Determinism tests catch immediately
- Replay tests show divergence
- Cross-platform tests reveal

**Mitigation Strategy:**
1. Use fixed-point arithmetic for critical computations
2. Explicit tolerance levels for float comparisons
3. Documented rounding modes
4. Platform-independent numerical libraries
5. Comprehensive determinism testing

**Related Documentation:**
- [`intent-engine-overview.md`](../05-intent-engine/intent-engine-overview.md) (Determinism Guarantee section)
- [`testing-framework.md`](../testing-framework.md)

---

### MR-5: Confidence Model Failure

**Description:**  
Confidence scoring system fails to accurately reflect inference quality, producing overconfident or underconfident outputs.

**Potential Triggers:**
- Novel failure modes not captured in confidence logic
- Interaction effects between confidence dimensions
- Threshold miscalibration
- Data quality issues not detected

**Impact:** HIGH  
- False confidence in poor inference
- Downstream consumers misled
- Risk management failures
- Trust erosion

**Likelihood:** MEDIUM (confidence is hard to model)

**Detectability:** MODERATE  
- Requires validation against realized outcomes
- Confidence calibration analysis (actual vs predicted)
- Monitoring for high-confidence failures

**Mitigation Strategy:**
1. Conservative confidence defaults
2. Regular confidence calibration validation
3. Multiple independent confidence dimensions
4. Explicit "unknown unknowns" penalty in confidence
5. Continuous improvement based on failure analysis

**Related Documentation:**
- [`confidence-and-stability.md`](../05-intent-engine/confidence-and-stability.md)
- [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md)

---

### MR-6: State Management Errors

**Description:**  
Incorrect handling of state transitions, hidden state dependencies, or state corruption.

**Potential Triggers:**
- Complex state machines with edge cases
- Race conditions in concurrent processing
- State not properly versioned or persisted
- Inconsistent state across components

**Impact:** HIGH  
- Non-deterministic behavior
- Inference inconsistency
- System instability
- Difficult to debug failures

**Likelihood:** LOW (with stateless design)

**Detectability:** MODERATE  
- Determinism tests detect
- State audit mechanisms reveal
- Replay tests show divergence

**Mitigation Strategy:**
1. Prefer stateless design by default
2. Explicit, minimal state when required
3. State passed as input, not hidden
4. State versioning and audit trails
5. Comprehensive state transition testing

**Related Documentation:**
- [`intent-engine-overview.md`](../05-intent-engine/intent-engine-overview.md) (State Handling section)
- [`event-driven-architecture.md`](../02-system-architecture/event-driven-architecture.md)

---

## Cross-Cutting Risks

Some risks span multiple categories.

### CR-1: Black Swan Events

**Description:**  
Extreme, unprecedented market events that violate all assumptions and structural models.

**Examples:**
- Market-wide trading halts for extended periods
- Systemic clearing failures
- Complete loss of market structure
- Regulatory suspension of markets

**Impact:** CRITICAL  
- All inference invalid
- No applicable regime
- Complete system shutdown required

**Likelihood:** LOW (but non-zero)

**Detectability:** EASY (when occurs)

**Mitigation Strategy:**
1. Explicit crisis regime with full suppression
2. Kill switch on extreme anomaly detection
3. No attempt to infer during black swan
4. Clear communication of complete inference failure
5. Manual review required before restart

**Related Documentation:**
- [`kill-switch-criteria.md`](kill-switch-criteria.md)
- [`unknown-unknowns.md`](unknown-unknowns.md)
- [`incident-response.md`](../10-operational-playbooks/incident-response.md)

---

### CR-2: Adversarial Exploitation

**Description:**  
Sophisticated actors intentionally exploiting known inference patterns for profit or manipulation.

**Potential Methods:**
- Creating false signals to mislead systems
- Front-running inference-driven capital flows
- Spoofing or manipulation targeting inference logic
- Reverse-engineering and gaming models

**Impact:** MEDIUM  
- Inference accuracy degrades
- Confidence becomes miscalibrated
- Arms race dynamics
- Reputation damage

**Likelihood:** LOW initially, MEDIUM over time

**Detectability:** DIFFICULT  
- Requires sophisticated anomaly detection
- Pattern analysis over time
- Comparison with non-public alternative models

**Mitigation Strategy:**
1. Avoid publishing detailed methodologies
2. Diversify signal sources
3. Regular model updates and evolution
4. Anomaly detection for adversarial patterns
5. Confidence degradation when manipulation suspected

**Related Documentation:**
- [`unknown-unknowns.md`](unknown-unknowns.md)
- [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md)

---

### CR-3: Systemic Correlation During Stress

**Description:**  
All assumed independence breaks down during stress, with correlations going to 1.0 and diversification benefits disappearing.

**Potential Triggers:**
- Market crashes or panics
- Liquidity crises
- Systemic failures
- Regulatory interventions

**Impact:** HIGH  
- Signal independence assumptions violated
- Confidence aggregation fails
- Diversification assumptions wrong
- Pressure estimates highly uncertain

**Likelihood:** MEDIUM (during stress regimes)

**Detectability:** MODERATE  
- Observable via correlation metrics
- Volatility regime shifts
- Liquidity withdrawal indicators

**Mitigation Strategy:**
1. Stress regime detection with different inference logic
2. Confidence suppression during extreme correlation
3. No assumption of signal independence during stress
4. Stress scenarios as part of validation
5. Explicit stress regime classification

**Related Documentation:**
- [`stress-scenarios.md`](../07-backtesting-and-validation/stress-scenarios.md)
- [`regime-classification.md`](../05-intent-engine/regime-classification.md)
- [`regime-breakdown.md`](regime-breakdown.md)

---

## Risk Monitoring and Detection

### Monitoring Requirements

Each risk category requires specific monitoring:

**Structural Risks:**
- Market structure metrics (derivatives volume, liquidity, participant mix)
- Regulatory announcements and changes
- Structural assumption validation metrics

**Data Risks:**
- Data pipeline health and latency
- Schema validation pass rates
- Data quality scores across dimensions
- Cross-source consistency metrics

**Behavioral Risks:**
- Anomaly detection on capital behavior patterns
- Regime transition frequency and clarity
- Signal confluence and contradiction rates
- Unexpected market impact observations

**Model Risks:**
- Walk-forward validation performance
- Confidence calibration analysis
- Determinism test results
- Systematic bias detection

### Early Warning Indicators

Key metrics for early risk detection:
1. Derivatives-to-cash volume ratio declining
2. Confidence degradation trends across instruments
3. Increased regime transition frequency
4. Walk-forward validation deterioration
5. Data quality scores declining
6. Signal agreement decreasing
7. Unusual correlation spikes
8. Structural assumption violation indicators

### Escalation and Response

When risks materialize:
1. **Detection**: Automated monitoring alerts
2. **Assessment**: Manual review of risk severity
3. **Response**: Execute documented mitigation strategy
4. **Communication**: Inform downstream consumers
5. **Learning**: Update risk documentation and detection

**Related Documentation:**
- [`daily-operations.md`](../10-operational-playbooks/daily-operations.md)
- [`incident-response.md`](../10-operational-playbooks/incident-response.md)

---

## Living Document Philosophy

This risk catalog is a **living document**:

- New risks will be discovered and added
- Risk likelihoods and impacts will change over time
- Mitigation strategies will evolve with system capabilities
- Monitoring approaches will improve with experience

### Update Triggers

This document must be updated when:
- New failure modes are discovered
- Market structure changes materially
- System capabilities evolve
- Validation reveals unmodeled risks
- Incidents expose gaps in risk coverage

### Update Process

1. Document new risk or update to existing risk
2. Assess likelihood, impact, detectability
3. Define or update mitigation strategy
4. Update monitoring requirements
5. Communicate changes to stakeholders
6. Record in decision log

**Related Documentation:**
- [`decision-log.md`](../11-decision-log/decision-log.md)
- [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md)

---

## Relationship to Other Risk Documents

This document catalogs **known risks**. It complements:

- [`regime-breakdown.md`](regime-breakdown.md) — Specific regime failure patterns
- [`kill-switch-criteria.md`](kill-switch-criteria.md) — Mandatory halt conditions
- [`unknown-unknowns.md`](unknown-unknowns.md) — Non-enumerable failure philosophy
- [`stress-scenarios.md`](../07-backtesting-and-validation/stress-scenarios.md) — Validation under extreme conditions
- [`failure-analysis.md`](../07-backtesting-and-validation/failure-analysis.md) — Post-incident analysis

---

## Design Implications

Risk awareness drives design decisions:

1. **Conservative Defaults**: When in doubt, degrade confidence or suppress
2. **Explicit Uncertainty**: Surface all assumptions and limitations
3. **Graceful Degradation**: Partial failure preferred to total failure
4. **Kill Switches**: Non-negotiable halt conditions enforced
5. **Monitoring First**: If it can fail, it must be monitored
6. **Validation Continuous**: Walk-forward validation is mandatory
7. **Determinism Enforced**: Reproducibility enables debugging
8. **Explainability Required**: Every output must be traceable

**Related Documentation:**
- [`design-principles.md`](../00-vision-and-non-goals/design-principles.md)
- [`intent-engine-overview.md`](../05-intent-engine/intent-engine-overview.md)

---

## Final Statement

**Galactus acknowledges its limitations.**

Known risks are documented, monitored, and managed.  
Unknown risks are accepted with humility.

When risks materialize, Galactus stays silent rather than producing confident nonsense.

**Honest uncertainty is a feature, not a bug.**
