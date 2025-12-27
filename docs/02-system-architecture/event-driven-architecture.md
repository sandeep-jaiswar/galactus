# Galactus — Event-Driven Architecture

## Purpose of This Document

This document defines the **event-driven architectural model** used by Project Galactus.

It explains:
- What constitutes an event
- Why Galactus is event-driven instead of tick-driven
- How time, ordering, and causality are handled
- How replayability is guaranteed

This document governs all decisions related to data flow and processing semantics.

---

## Core Principle

Galactus models **structural change**, not continuous motion.

Structural change occurs at **events**, not at every price tick.

---

## Why Event-Driven (Not Tick-Driven)

Tick-driven systems assume:
- Continuous price discovery
- Meaningful information in every micro-movement
- Latency as a competitive advantage

These assumptions do not hold for Galactus.

Galactus prioritizes:
- Capital constraints
- Forced flows
- Discrete market mechanics

These manifest at **specific moments**, not continuously.

---

## Event-Driven Reasoning

### The Philosophy of Event-Driven Architecture

Event-driven architecture is not merely a technical choice—it reflects fundamental beliefs about how markets work.

#### Markets Are Discrete, Not Continuous

While price tickers create the illusion of continuous motion, markets actually move through **discrete state transitions**:

- Options expire at specific times, forcing hedging adjustments
- Index rebalancing happens at predetermined moments, creating forced flows
- Liquidity evaporates at specific price levels, changing absorption capacity
- Delivery obligations crystalize on settlement dates

Between these events, much of the noise is just that—noise. Event-driven architecture focuses inference on moments that matter.

#### Causality Requires Discrete Events

To reason about cause and effect, we need:
- **Identifiable causes**: Specific events that impose constraints
- **Temporal precedence**: Clear before/after relationships
- **Bounded effects**: Observable changes that can be attributed

Tick-by-tick processing obscures causality. Event-driven processing makes it explicit.

#### Capital Behavior Is Episodic

Capital flows are driven by:
- **Forced actions**: Hedging requirements, margin calls, expiry obligations
- **Strategic positioning**: Deliberate accumulation or distribution
- **Structural constraints**: Regulatory requirements, mandate changes

These behaviors manifest as discrete events, not continuous drifts. The inference system must align with this reality.

#### Explainability Demands Discrete Attribution

Galactus must explain its inferences. This requires:
- **Traceable reasoning**: "This inference occurred because event X happened"
- **Verifiable causality**: Events can be audited and verified
- **Reproducible logic**: Same events lead to same conclusions

Continuous streams make attribution nearly impossible. Discrete events make it straightforward.

### What Event-Driven Enables

By adopting event-driven architecture, Galactus achieves:

1. **Causal reasoning**: Events provide natural causal boundaries
2. **Temporal precision**: Inference is timestamped at event time, not processing time
3. **Auditability**: Every inference can be traced to specific events
4. **Replayability**: Events can be replayed to verify inference
5. **Graceful degradation**: Missing events create explicit gaps, not silent corruption
6. **Testability**: Events can be synthesized for testing
7. **Explainability**: Inferences reference specific events in explanations

### What Event-Driven Requires

Event-driven architecture imposes obligations:

1. **Event identification**: Must clearly define what constitutes an event
2. **Event taxonomy**: Must classify events by semantic meaning
3. **Time discipline**: Must strictly separate event time from processing time
4. **Order handling**: Must explicitly handle late and out-of-order events
5. **Completeness monitoring**: Must detect missing events
6. **Versioning**: Must version event schemas and processing logic

These obligations are features, not bugs—they force architectural discipline.

### Comparison: Event-Driven vs Tick-Driven

| Aspect | Event-Driven (Galactus) | Tick-Driven (Traditional) |
|--------|-------------------------|---------------------------|
| **Focus** | Structural state changes | Continuous price motion |
| **Time semantics** | Event time (when it happened) | Processing time (when observed) |
| **Causality** | Explicit via events | Implicit via correlation |
| **Replayability** | First-class requirement | Often impossible |
| **Latency priority** | Correctness over speed | Speed over correctness |
| **Signal-to-noise** | High (events filtered) | Low (all ticks) |
| **Explainability** | Event-attributed | Pattern-based |
| **Resource usage** | Efficient (sparse events) | High (dense ticks) |

### When Event-Driven Is Not Appropriate

Event-driven architecture is wrong for:
- **Ultra-low-latency trading**: Microsecond arbitrage needs tick streams
- **Market making**: Continuous bid-ask management requires tick data
- **Technical analysis**: Pattern recognition often uses continuous charts

Galactus deliberately excludes these use cases. Event-driven architecture is right for capital-pressure inference.

---

## Definition: Event

In Galactus, an **event** is:

> A discrete, timestamped occurrence that materially alters the state of market constraints or capital behavior.

Events are not limited to price changes.

---

## Types of Events

### 1. Market Structure Events

Examples:
- Options expiry
- Index rebalancing
- Settlement cycles
- Regulatory changes

These events impose **hard constraints** on capital.

---

### 2. Positioning Events

Examples:
- Significant open interest changes
- Strike concentration shifts
- Gamma exposure transitions

These events alter **hedging requirements**.

---

### 3. Liquidity Events

Examples:
- Sudden volume exhaustion
- Delivery spikes
- Liquidity cliffs

These events change **absorption capacity**.

---

### 4. Information Events

Examples:
- Corporate disclosures
- Scheduled announcements

Galactus observes **capital reaction**, not narrative content.

---

## Event Time vs Processing Time

Galactus distinguishes strictly between:

- **Event Time**: When the market event actually occurred
- **Processing Time**: When the system processed the event

All inference is aligned to **event time**.

Processing delays must not alter inference results.

---

## Event-Time Guarantees

### Time Semantics

Galactus provides the following event-time guarantees:

1. **Event-time immutability**: Once assigned, an event's timestamp cannot change
2. **Event-time primacy**: All temporal reasoning uses event time, never processing time
3. **Event-time alignment**: Inference outputs are timestamped at the event time that caused them
4. **Late-arrival handling**: Events arriving out of order are reprocessed against correct event-time context

### Timestamp Requirements

All events must include:
- **Event timestamp**: The wall-clock time when the market event occurred (nanosecond precision)
- **Processing timestamp**: When Galactus first observed the event
- **Sequence number**: For ordering disambiguation when timestamps are identical

### Clock Synchronization

- Market data timestamps are assumed synchronized to market infrastructure clocks
- System clocks are synchronized via NTP or equivalent
- Clock skew is monitored and logged
- Timestamp drift beyond threshold triggers confidence degradation

### Late and Out-of-Order Events

When events arrive after subsequent events have been processed:

1. **Detection**: Late arrivals are detected via sequence gaps or timestamp ordering
2. **Reprocessing**: Affected downstream inference is recomputed with correct event-time ordering
3. **Versioning**: New inference outputs are versioned, preserving original outputs
4. **Notification**: Late arrivals trigger alerts for operational monitoring

### Time Boundary Conditions

At temporal boundaries (e.g., day boundaries, expiry times):
- Events are assigned to correct time partitions
- Cross-boundary causality is preserved
- Inference windows are calculated relative to event time
- No assumptions about processing-time alignment

---

## Event Ordering Guarantees

### Total Ordering Within Streams

Galactus enforces:

1. **Per-instrument ordering**: Events for a single instrument are totally ordered by event time
2. **Per-event-type ordering**: Events of the same type maintain causal order
3. **Sequence preservation**: Event sequence numbers are monotonically increasing within streams

### Cross-Stream Ordering

For events across multiple instruments or event types:

1. **Partial ordering**: Only causal relationships are ordered
2. **No artificial synchronization**: Unrelated events have no ordering constraint
3. **Explicit coordination**: When cross-stream ordering matters, it is modeled explicitly

### Ordering Conflict Resolution

When event timestamps are identical:

1. **Sequence number tiebreak**: Use monotonic sequence numbers
2. **Source priority**: If from different sources, use documented priority rules
3. **Explicit modeling**: If order is semantically meaningful, encode it in the event

### Deterministic Reprocessing

Replay guarantees:

1. **Same order, same results**: Replaying events in the same order produces identical outputs
2. **Order-independent where possible**: Design inference to minimize order sensitivity
3. **Order-dependent explicitly marked**: When order matters, document and test it

---

## Replayability Requirements

### Core Replayability Principles

Replayability is a **first-class architectural requirement** in Galactus. Every inference must be reproducible from event history.

### What Must Be Replayable

1. **All inference outputs**: Any intent vector, regime classification, or confidence metric
2. **Feature computations**: All intermediate features used in inference
3. **Configuration state**: The exact configuration active at event time
4. **Backtests**: Historical validation must use same code paths as live inference

### Replayability Guarantees

Galactus guarantees:

1. **Bitwise reproducibility**: Given the same events and configuration, outputs are byte-for-byte identical
2. **Temporal consistency**: Replaying events from time T produces the state that existed at time T
3. **Version isolation**: Replaying with a specific version uses that version's logic exclusively
4. **No hidden state**: All state needed for inference is either in events or explicit configuration

### Implementation Requirements

To ensure replayability:

1. **Immutable events**: Events are never modified after emission
2. **Versioned configuration**: All configuration is versioned and timestamped
3. **Deterministic computation**: No random numbers, system time, or non-deterministic operations
4. **Explicit state**: All state is either derived from events or explicitly persisted and versioned
5. **Pure functions**: Inference functions are pure or have explicit state parameters
6. **Dependency pinning**: All dependencies have exact versions recorded

### What Is NOT Replayable

Certain aspects are explicitly excluded from replayability:

1. **Processing time**: How long computation took is not preserved
2. **Resource usage**: CPU, memory, I/O metrics are not part of replay
3. **Network effects**: External API calls are mocked or recorded separately
4. **Non-deterministic debugging**: Print statements, logs with timestamps

### Replay Modes

Galactus supports multiple replay modes:

1. **Full replay**: Reprocess all events from beginning
2. **Window replay**: Reprocess events in a time window
3. **Incremental replay**: Replay new events on top of checkpoint
4. **Validation replay**: Replay with different versions for comparison

### Replay Validation

To verify replayability:

1. **Smoke tests**: Replay sample periods and compare outputs
2. **Full backtests**: Replay entire history periodically
3. **Continuous validation**: Automated replay tests on every code change
4. **Divergence detection**: Alert on any replay differences

### Replay Performance

Replay should be:
- **Fast enough**: Complete within acceptable operational windows
- **Resource-bounded**: Not consume excessive resources
- **Interruptible**: Can be stopped and resumed
- **Parallelizable**: Where event ordering allows

---

## Idempotency and Reprocessing

All event handling must be:
- **Idempotent**: Processing the same event multiple times produces the same result
- **Replay-safe**: Can be reprocessed without side effects
- **Deterministic**: Same inputs always produce same outputs

Reprocessing the same event stream must:
- Produce identical inference outputs
- Not depend on system state outside the stream
- Not create duplicate downstream effects
- Preserve causality relationships

---

## Windowing and Time Horizons

Galactus uses **explicit windows**, not implicit rolling assumptions.

Examples:
- Time-to-expiry windows
- Liquidity lookback windows
- Regime evaluation windows

All windows are:
- Explicitly defined
- Versioned
- Justified in documentation

---

## Practical Examples

### Example 1: Options Expiry Event

**Event**: Monthly options expiry at 15:30 on expiry day

**Event-time reasoning**:
- Event timestamp: 2024-01-25T15:30:00.000Z (exact expiry time)
- Processing timestamp: 2024-01-25T15:30:02.145Z (when system observed)
- Inference aligned to: Event time (15:30:00)

**Why this matters**:
- Hedging pressure inference references the exact expiry moment
- Late processing (by 2.145 seconds) does not shift the inference timestamp
- Replay from historical data uses 15:30:00, not 15:30:02

**Consequences**:
- Backtests and live runs produce identical inferences
- Explanations reference "at expiry" not "when we processed it"
- Causality is clear: expiry forced hedging, not our observation of it

### Example 2: Late-Arriving Liquidity Event

**Scenario**: High-volume spike occurs at 10:15:00 but data arrives at 10:17:30

**Event-driven handling**:
1. Event arrives at 10:17:30 (processing time)
2. Event timestamp is 10:15:00 (event time)
3. System detects gap: events for 10:15:30, 10:16:00, etc. already processed
4. System triggers reprocessing window: 10:15:00 - 10:17:30
5. Inference for this period is recomputed with correct event order
6. New inference version is stored, original version preserved
7. Downstream consumers notified of version update

**Guarantees preserved**:
- Event time correctness: 10:15:00 event influences 10:15:00 inference
- Replayability: Replaying includes the event at correct position
- Causality: Subsequent inferences correctly account for earlier event

### Example 3: Cross-Instrument Causality

**Scenario**: Index rebalancing (event A) causes forced equity flow (event B)

**Event modeling**:
- Event A timestamp: 2024-01-26T15:00:00.000Z (rebalance effective time)
- Event B timestamp: 2024-01-26T15:00:15.250Z (first observable equity flow)
- Causal relationship: A → B (explicitly modeled)

**Inference reasoning**:
- Observed flow at 15:00:15 is attributed to rebalance at 15:00:00
- Temporal precedence is clear and verifiable
- Explanation: "Flow consistent with index rebalance at 15:00:00"

**Without event-driven architecture**:
- Both events might be in same tick batch
- Causality would be ambiguous
- Attribution would be correlational, not causal

### Example 4: Replay Validation

**Use case**: Validate that code change preserves inference

**Process**:
1. Capture baseline: Replay January 2024 with version 1.2.0
2. Store outputs: Intent vectors for all instruments
3. Apply code change: Update to version 1.3.0
4. Replay same period: January 2024 with version 1.3.0
5. Compare outputs: Diff intent vectors
6. Verify: No unexpected changes

**What this validates**:
- Determinism: Same events produce reproducible outputs
- Compatibility: Code changes don't break existing inferences
- Regression: No unintended behavior changes

**Requirements**:
- Immutable event history
- Versioned configuration
- Deterministic inference
- Bitwise reproducibility

---

## Implementation Patterns

### Pattern 1: Event Schema

Every event must have:

```json
{
  "event_id": "uuid",
  "event_type": "options_expiry",
  "event_time": "2024-01-25T15:30:00.000000000Z",
  "processing_time": "2024-01-25T15:30:02.145000000Z",
  "sequence_number": 123456,
  "schema_version": "1.0",
  "source": "nse_bhavcopy",
  "payload": { /* event-specific data */ }
}
```

### Pattern 2: Inference Output Schema

Every inference output must reference:

```json
{
  "inference_id": "uuid",
  "inference_type": "capital_pressure",
  "inference_time": "2024-01-25T15:30:00.000Z",  // aligned to event time
  "caused_by_events": ["event_id_1", "event_id_2"],  // causal attribution
  "model_version": "1.2.0",
  "config_version": "1.0.5",
  "payload": { /* inference results */ }
}
```

### Pattern 3: Replay Implementation

```rust
// Pseudocode for replay logic
fn replay_period(start: EventTime, end: EventTime, version: Version) -> InferenceResults {
    // Load events in event-time order
    let events = event_store.load_range(start, end);
    
    // Load configuration for version
    let config = config_store.load_version(version);
    
    // Load inference engine for version
    let engine = inference_engine::load_version(version);
    
    // Initialize state
    let mut state = InferenceState::new();
    
    // Process events in order
    for event in events {
        // Apply deterministic inference
        let inference = engine.process(event, state, config);
        
        // Update state
        state = state.apply(inference);
        
        // Store result
        results.push(inference);
    }
    
    results
}
```

### Pattern 4: Late Event Handling

```rust
// Pseudocode for late event handling
fn handle_late_event(event: Event) {
    // Detect if late
    if event.event_time < last_processed_event_time {
        // Find affected time range
        let reprocess_start = event.event_time;
        let reprocess_end = last_processed_event_time;
        
        // Trigger reprocessing
        let new_inferences = replay_period(
            reprocess_start,
            reprocess_end,
            current_version
        );
        
        // Version the update
        inference_store.store_version(
            new_inferences,
            version_increment(),
            reason="late_event_reprocess"
        );
        
        // Notify downstream
        notify_consumers(InferenceUpdate {
            time_range: (reprocess_start, reprocess_end),
            new_version: version_increment(),
            reason: "late_event_arrival"
        });
    } else {
        // Process normally
        process_event(event);
    }
}
```

### Pattern 5: Deterministic Feature Computation

```rust
// Pseudocode for deterministic features
fn compute_feature(events: &[Event], config: &Config) -> Feature {
    // Use only event data and explicit config
    // NO: system time, random numbers, external API calls
    // YES: event timestamps, configured parameters, pure math
    
    let lookback_window = config.lookback_days;
    let relevant_events = filter_by_window(events, lookback_window);
    
    // Deterministic aggregation
    let value = relevant_events
        .iter()
        .map(|e| e.payload.volume)
        .sum();
    
    Feature {
        name: "total_volume",
        value,
        computed_at: events.last().event_time,  // event time, not now()
        config_version: config.version
    }
}
```

---

## Why This Matters for Inference

Event-driven processing allows Galactus to:

- Attribute cause before effect
- Separate structural pressure from noise
- Replay and audit inference decisions
- Align backtests with live behavior

Tick-driven systems obscure causality.

---

## Failure Modes and Safeguards

Galactus explicitly handles:
- Missing events
- Delayed events
- Event bursts near expiries
- Data source outages

When event integrity is compromised:
- Inference confidence must degrade
- Silence is acceptable

---

## Operational Considerations

### Monitoring Event Streams

Operations must monitor:

1. **Event arrival rate**: Detect anomalous gaps or bursts
2. **Processing lag**: Alert when processing time falls behind event time
3. **Late event frequency**: Track out-of-order arrivals
4. **Sequence gaps**: Detect missing events in sequence
5. **Schema violations**: Catch malformed events
6. **Source health**: Monitor upstream data provider status

### Event Quality Metrics

Track and alert on:

- **Completeness**: Percentage of expected events received
- **Timeliness**: Percentile distribution of processing lag
- **Ordering**: Frequency of late/out-of-order events
- **Freshness**: Age of most recent event per stream
- **Consistency**: Cross-source validation where applicable

### Backpressure and Flow Control

When event rates exceed processing capacity:

1. **Bounded buffers**: Prevent unbounded memory growth
2. **Explicit backpressure**: Signal upstream to slow down
3. **Graceful degradation**: Prioritize critical event types
4. **Confidence signaling**: Mark inferences as delayed/degraded
5. **Never drop silently**: All dropped events are logged and alerted

### Disaster Recovery

Event-driven architecture enables:

1. **Point-in-time recovery**: Restore state by replaying to timestamp
2. **Warm standby**: Secondary systems replay with lag
3. **Active-active**: Multiple systems process same events, compare outputs
4. **Incident replay**: Reproduce production issues by replaying events

Recovery procedures:

- Identify failure time T
- Replay events from checkpoint before T
- Validate state matches expected
- Resume live processing

---

## Testing Requirements

### Unit Tests

Every inference function must have:

1. **Determinism tests**: Same inputs produce same outputs (run 1000x)
2. **Event-time tests**: Verify inference uses event time, not system time
3. **Order sensitivity tests**: Verify behavior with different event orders
4. **Edge case tests**: Empty events, missing fields, boundary values

### Integration Tests

Event processing pipelines must have:

1. **Replay tests**: Replay sample periods, verify bitwise identical outputs
2. **Late event tests**: Inject late events, verify reprocessing
3. **Missing event tests**: Skip events, verify degradation behavior
4. **Version compatibility tests**: Replay with different versions

### Replayability Tests

Continuous validation:

1. **Daily replay**: Replay yesterday, compare to production outputs
2. **Weekly full replay**: Replay last 30 days, validate end state
3. **Release validation**: Replay key periods before deploying new version
4. **Cross-version comparison**: Replay with old and new versions, diff outputs

### Performance Tests

Event processing must be:

1. **Bounded latency**: P99 processing time < threshold
2. **Bounded memory**: No memory leaks during replay
3. **Scalable throughput**: Handle peak event rates (expiry days)
4. **Efficient replay**: Complete replay within operational windows

### Chaos Engineering

Regularly test:

1. **Random event drops**: How does system degrade?
2. **Delayed events**: Inject processing lag
3. **Out-of-order bursts**: Send events in random order
4. **Source failures**: Simulate upstream outages
5. **Version rollback**: Deploy old version, verify compatibility

---

## Documentation Requirements

### Per-Event Type

For each event type, document:

1. **Semantic meaning**: What market change does this represent?
2. **Schema**: Exact fields and types
3. **Frequency**: Expected arrival rate
4. **Latency**: Expected processing lag
5. **Ordering**: Any special ordering requirements
6. **Downstream effects**: What inferences depend on this event?

### Per-Inference Type

For each inference type, document:

1. **Input events**: Which event types are consumed?
2. **Lookback windows**: What historical depth is needed?
3. **Order sensitivity**: Does event order affect output?
4. **Reprocessing behavior**: What happens on replay?
5. **Causality**: How are events attributed to inferences?

### Per-Release

For each version release, document:

1. **Event schema changes**: Any new or modified events?
2. **Inference changes**: What inferences are affected?
3. **Replay compatibility**: Can new version replay old events?
4. **Migration path**: How to upgrade from previous version?

---

## Implications for System Design

Because Galactus is event-driven:
- Storage must be append-only
- Replay must be first-class
- Stateful shortcuts are forbidden
- Time must be modeled, not assumed

---

## Design Invariants

### Architectural Invariants

The following are **permanent constraints** on event-driven architecture:

1. **Event immutability**: Events never change after emission
2. **Event-time primacy**: Event time is always the truth
3. **Causal attribution**: Inferences reference causing events
4. **Deterministic processing**: Same events, same outputs
5. **Replayability**: All inference can be reproduced
6. **Explicit state**: No hidden state outside events/config
7. **Version tracking**: All schemas and logic are versioned

Violating these invariants breaks core guarantees.

### Data Invariants

1. **Append-only storage**: Events are never deleted or modified
2. **Schema versioning**: Event schema changes are versioned
3. **Sequence monotonicity**: Sequence numbers always increase
4. **Timestamp validity**: Event time ≤ processing time (with tolerance)
5. **Completeness marking**: Missing events are explicitly marked

### Processing Invariants

1. **Idempotency**: Processing event N times = processing once
2. **Order preservation**: Event order within streams is preserved
3. **No side effects**: Processing does not modify external state
4. **Bounded resources**: Processing uses bounded memory and time
5. **Graceful degradation**: Partial data produces partial inference, not failure

### Testing Invariants

1. **Replay equivalence**: Replay produces identical outputs
2. **Version compatibility**: New versions can replay old events
3. **Determinism validation**: Repeated runs produce same results
4. **Performance bounds**: Processing latency has upper bounds

---

## Anti-Patterns (Forbidden)

### ❌ Anti-Pattern 1: Using System Time for Inference

**Wrong**:
```rust
fn compute_pressure(event: Event) -> Pressure {
    let now = SystemTime::now();  // ❌ Non-deterministic
    let age = now - event.timestamp;
    pressure_from_age(age)
}
```

**Right**:
```rust
fn compute_pressure(event: Event, as_of: EventTime) -> Pressure {
    let age = as_of - event.event_time;  // ✅ Deterministic
    pressure_from_age(age)
}
```

**Why**: Using system time makes replay produce different results.

### ❌ Anti-Pattern 2: Implicit State

**Wrong**:
```rust
static mut LAST_PRICE: f64 = 0.0;  // ❌ Hidden global state

fn process_event(event: Event) -> Inference {
    unsafe {
        let change = event.price - LAST_PRICE;
        LAST_PRICE = event.price;
        Inference { change }
    }
}
```

**Right**:
```rust
struct InferenceState {
    last_price: f64
}

fn process_event(event: Event, state: &InferenceState) -> (Inference, InferenceState) {
    let change = event.price - state.last_price;
    let new_state = InferenceState { last_price: event.price };
    (Inference { change }, new_state)
}
```

**Why**: Explicit state can be serialized, versioned, and replayed.

### ❌ Anti-Pattern 3: Silent Event Drops

**Wrong**:
```rust
fn process_events(events: &[Event]) {
    for event in events {
        if is_valid(event) {
            process(event);
        }
        // ❌ Invalid events silently dropped
    }
}
```

**Right**:
```rust
fn process_events(events: &[Event]) -> ProcessingReport {
    let mut report = ProcessingReport::new();
    
    for event in events {
        if is_valid(event) {
            process(event);
            report.processed += 1;
        } else {
            log_error!("Invalid event: {:?}", event);
            report.dropped.push(event.id);
        }
    }
    
    report  // ✅ Explicit accounting
}
```

**Why**: Silent drops create invisible data loss and non-replayable gaps.

### ❌ Anti-Pattern 4: Processing-Time Inference

**Wrong**:
```rust
fn classify_regime(events: &[Event]) -> Regime {
    let recent = events.iter()
        .filter(|e| e.processing_time > now() - Duration::hours(24))  // ❌ Processing time
        .collect();
    analyze(recent)
}
```

**Right**:
```rust
fn classify_regime(events: &[Event], as_of: EventTime) -> Regime {
    let recent = events.iter()
        .filter(|e| e.event_time > as_of - Duration::hours(24))  // ✅ Event time
        .collect();
    analyze(recent)
}
```

**Why**: Processing-time filtering breaks replay.

### ❌ Anti-Pattern 5: Mutable Event Store

**Wrong**:
```rust
fn correct_event(event_id: EventId, new_data: EventData) {
    event_store.update(event_id, new_data);  // ❌ Mutation
}
```

**Right**:
```rust
fn correct_event(event_id: EventId, correction: EventData) -> Event {
    let correction_event = Event {
        event_type: EventType::Correction,
        corrects: event_id,
        data: correction,
        ...
    };
    event_store.append(correction_event)  // ✅ New event
}
```

**Why**: Mutating events destroys audit trail and breaks replay.

### ❌ Anti-Pattern 6: Non-Deterministic Algorithms

**Wrong**:
```rust
fn select_regime(candidates: Vec<Regime>) -> Regime {
    candidates.into_iter()
        .choose(&mut rand::thread_rng())  // ❌ Random
        .unwrap()
}
```

**Right**:
```rust
fn select_regime(candidates: Vec<Regime>, tiebreak_key: u64) -> Regime {
    candidates.into_iter()
        .min_by_key(|r| (r.score, r.hash() ^ tiebreak_key))  // ✅ Deterministic tiebreak
        .unwrap()
}
```

**Why**: Random selection produces different outputs on replay.

### ❌ Anti-Pattern 7: Hidden Configuration

**Wrong**:
```rust
fn analyze(event: Event) -> Analysis {
    const THRESHOLD: f64 = 0.05;  // ❌ Hard-coded magic number
    if event.value > THRESHOLD {
        Analysis::High
    } else {
        Analysis::Low
    }
}
```

**Right**:
```rust
struct AnalysisConfig {
    threshold: f64,
    version: String,
}

fn analyze(event: Event, config: &AnalysisConfig) -> Analysis {
    if event.value > config.threshold {
        Analysis::High
    } else {
        Analysis::Low
    }
}
```

**Why**: Explicit configuration can be versioned and tied to inferences.

---

## Summary of Guarantees

Event-driven architecture provides:

### Time Guarantees
- ✅ Event time is immutable and primary
- ✅ Processing delays don't affect inference timestamps
- ✅ Late events trigger reprocessing with correct timestamps
- ✅ Replay uses event time, not system time

### Ordering Guarantees
- ✅ Events within a stream are totally ordered
- ✅ Late events are reprocessed in correct order
- ✅ Cross-stream ordering is explicit, not implicit
- ✅ Replay preserves original event order

### Replayability Guarantees
- ✅ Bitwise reproducibility: same events → same outputs
- ✅ Version isolation: replay with specific version
- ✅ No hidden state: all state from events or config
- ✅ Backtests use same code as live

### Quality Guarantees
- ✅ Causal attribution: inferences reference causing events
- ✅ Auditability: complete event history
- ✅ Explainability: event-based reasoning
- ✅ Testability: deterministic and replayable

---

## What Galactus Explicitly Avoids

Galactus does not:
- React to every tick
- Optimize for minimal latency
- Infer meaning from microstructure noise
- Process events in non-deterministic ways
- Use processing time for inference
- Maintain hidden state

These omissions are intentional and permanent.

---

## Final Statement

**Markets change state at events, not at every moment.**

Galactus is designed to observe those state changes clearly, reason about them causally, and reproduce that reasoning indefinitely.

Event-driven architecture is not just a technical choice—it is the foundation of:
- **Determinism**: Same events always produce same inferences
- **Causality**: Clear attribution of cause to effect
- **Auditability**: Complete trace from event to inference
- **Replayability**: Infinite reproducibility of reasoning
- **Explainability**: Event-grounded explanations

These properties make Galactus trustworthy, verifiable, and maintainable over time.

**Without event-driven architecture, Galactus cannot fulfill its core mission.**
