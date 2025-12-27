# Galactus — Confidence and Stability

## Purpose of This Document

This document defines how **confidence and stability** are measured, represented, and surfaced by the Galactus Intent Engine.

It exists to:
- Prevent false precision
- Make uncertainty explicit
- Differentiate strong inference from fragile inference
- Encourage appropriate downstream interpretation

Confidence is not optimism.  
Stability is not permanence.

---

## Core Principle

Galactus does not aim to be confident.  
It aims to be **correctly uncertain**.

---

## Definition: Confidence

In Galactus, **confidence** represents:

> The degree to which current inference is supported by data quality, structural consistency, and regime alignment.

Confidence does **not** represent:
- Probability of profit
- Likelihood of price movement
- Strength of conviction

It represents **trustworthiness of inference**.

---

## Definition: Stability

**Stability** represents:

> The persistence and robustness of inferred capital pressure over time and across small perturbations.

A stable inference:
- Persists across adjacent events
- Is not highly sensitive to minor data changes
- Degrades gradually rather than collapsing

---

## Dimensions of Confidence

Confidence is computed as a function of multiple dimensions:

---

## 1. Data Quality Confidence

Based on:
- Completeness of input data
- Timeliness of events
- Consistency across sources

Poor data quality directly degrades confidence.

### Computation Formula

```
data_quality_confidence = min(completeness_score, timeliness_score, consistency_score)

where:
  completeness_score = (fields_present / fields_required)
  timeliness_score = exp(-delay_seconds / acceptable_delay_threshold)
  consistency_score = 1.0 - (contradictions_detected / total_cross_checks)
```

**Thresholds:**
- `fields_required`: All mandatory schema fields must be present
- `acceptable_delay_threshold`: 60 seconds for real-time events, 300 seconds for delayed data
- Critical degradation: `data_quality_confidence < 0.5` triggers silence

---

## 2. Structural Alignment Confidence

Based on:
- Clear presence of binding constraints
- Alignment with known market mechanics
- Absence of contradictory signals

Weak structural grounding reduces confidence.

### Computation Formula

```
structural_confidence = constraint_clarity * mechanic_alignment * (1.0 - contradiction_ratio)

where:
  constraint_clarity = (identified_constraints / expected_constraints)
  mechanic_alignment = (aligned_signals / total_signals)
  contradiction_ratio = (contradictory_signals / total_signals)
```

**Thresholds:**
- `expected_constraints`: Minimum 1 binding constraint must be identified
- `mechanic_alignment`: Must be >= 0.6 for acceptable inference
- Critical degradation: `structural_confidence < 0.4` triggers silence

---

## 3. Regime Consistency Confidence

Based on:
- Clarity of regime classification
- Stability of regime over time
- Alignment between signal and regime

Ambiguous or transitional regimes degrade confidence.

### Computation Formula

```
regime_confidence = regime_clarity * regime_stability * signal_regime_alignment

where:
  regime_clarity = max(regime_probabilities) - second_max(regime_probabilities)
  regime_stability = 1.0 - (regime_transitions / lookback_window)
  signal_regime_alignment = (consistent_signals / total_signals_in_regime)
```

**Thresholds:**
- `regime_clarity`: Must be >= 0.3 (dominant regime must be clear)
- `regime_stability`: Transitions exceeding 3 in 10-event window degrade to 0
- Critical degradation: `regime_confidence < 0.3` triggers silence

---

## 4. Signal Agreement Confidence

Based on:
- Confluence of independent signals
- Absence of offsetting pressures
- Transparency of aggregation

Single-signal inference carries lower confidence.

### Computation Formula

```
signal_confidence = confluence_factor * (1.0 - offset_ratio) * transparency_score

where:
  confluence_factor = sqrt(agreeing_signals / total_signals)
  offset_ratio = (offsetting_pressure / total_pressure)
  transparency_score = 1.0 if aggregation_method_documented else 0.5
```

**Thresholds:**
- `agreeing_signals`: Minimum 2 independent signals required
- `offset_ratio`: Must be < 0.4 for confident inference
- Critical degradation: `signal_confidence < 0.4` triggers silence

---

## Dimensions of Stability

Stability evaluates how inference behaves over time.

---

## 1. Temporal Stability

- Persistence across consecutive events
- Resistance to short-lived noise
- Gradual decay when conditions change

### Computation Formula

```
temporal_stability = persistence_score * noise_resistance * decay_factor

where:
  persistence_score = (consistent_events / total_events_in_window)
  noise_resistance = 1.0 - (variance / mean) for inference values
  decay_factor = exp(-time_since_last_confirmation / half_life)
```

**Thresholds:**
- `consistent_events`: Minimum 3 consecutive agreeing events
- `half_life`: 300 seconds (5 minutes) for most market conditions
- Critical degradation: `temporal_stability < 0.3` indicates fragile inference

---

## 2. Sensitivity Stability

- Low sensitivity to small data perturbations
- Robustness to minor schema or timing variations

### Computation Formula

```
sensitivity_stability = 1.0 - max(perturbation_impact)

where:
  perturbation_impact = |inference_perturbed - inference_baseline| / inference_baseline
```

**Thresholds:**
- `perturbation_impact`: Changes > 0.2 (20%) from small perturbations indicate instability
- Test perturbations: ±5% data values, ±10s timing variations
- Critical degradation: `sensitivity_stability < 0.5` indicates brittle inference

---

## 3. Regime Stability

- Inference remains coherent within a regime
- Predictable behavior at regime boundaries

### Computation Formula

```
regime_stability = regime_coherence * boundary_predictability

where:
  regime_coherence = 1.0 - (incoherent_inferences / total_inferences_in_regime)
  boundary_predictability = 1.0 - (surprise_transitions / total_transitions)
```

**Thresholds:**
- `incoherent_inferences`: Inference contradicting regime mechanics
- `surprise_transitions`: Transitions without precursor signals
- Critical degradation: `regime_stability < 0.4` indicates regime misclassification

---

## Confidence and Stability Outputs

All intent outputs must include:

- Explicit confidence score
- Stability indicator
- Assumption list
- Known ambiguity flags

No output is complete without uncertainty metadata.

### Output Schema

```
IntentOutput {
  inference: InferenceResult,
  confidence: OverallConfidence,
  stability: StabilityIndicator,
  metadata: UncertaintyMetadata
}

OverallConfidence {
  score: f64,                    // Range: [0.0, 1.0]
  components: {
    data_quality: f64,
    structural_alignment: f64,
    regime_consistency: f64,
    signal_agreement: f64
  },
  level: ConfidenceLevel        // LOW, MEDIUM, HIGH, CRITICAL
}

StabilityIndicator {
  score: f64,                    // Range: [0.0, 1.0]
  components: {
    temporal: f64,
    sensitivity: f64,
    regime: f64
  },
  level: StabilityLevel         // FRAGILE, MODERATE, ROBUST
}

UncertaintyMetadata {
  assumptions: Vec<String>,
  ambiguity_flags: Vec<String>,
  known_limitations: Vec<String>,
  confidence_bounds: (f64, f64)  // Lower and upper bounds
}
```

### Confidence Level Mapping

```
HIGH:     confidence >= 0.80 && stability >= 0.70
MEDIUM:   confidence >= 0.60 && stability >= 0.50
LOW:      confidence >= 0.40 && stability >= 0.30
CRITICAL: confidence < 0.40 || stability < 0.30
```

---

## Degradation Rules

When uncertainty increases:

- Confidence must degrade
- Stability indicators must weaken
- Output language must soften

Silence is acceptable when confidence drops below minimum thresholds.

### Degradation Formula

```
overall_confidence = geometric_mean(
  data_quality_confidence,
  structural_confidence,
  regime_confidence,
  signal_confidence
)

overall_stability = geometric_mean(
  temporal_stability,
  sensitivity_stability,
  regime_stability
)

geometric_mean(values) = (product of values)^(1/n)
```

**Rationale:** Geometric mean ensures that a single critically low component significantly degrades overall scores, preventing false confidence from averaging.

### Degradation Triggers

1. **Single component drops below 0.3**: Overall confidence classified as CRITICAL
2. **Two components below 0.5**: Overall confidence drops by additional 20%
3. **Stability drops below 0.3**: Inference flagged as "fragile - use with extreme caution"
4. **Both confidence and stability CRITICAL**: Inference suppressed entirely

---

## Hard Confidence Floors

Inference must be suppressed when:

- Data quality is critically degraded
- Regime classification is indeterminate
- Core assumptions are violated

Producing confident output under these conditions is forbidden.

### Silence Thresholds (Non-Negotiable)

```
MANDATORY_SILENCE_CONDITIONS:

1. overall_confidence < 0.30
   → Suppress all inference output
   → Emit: "Insufficient confidence for inference"

2. data_quality_confidence < 0.50
   → Suppress all inference output
   → Emit: "Data quality below acceptable threshold"

3. regime_confidence < 0.30
   → Suppress all inference output
   → Emit: "Regime classification indeterminate"

4. structural_confidence < 0.40 && signal_confidence < 0.40
   → Suppress all inference output
   → Emit: "Structural foundation insufficient"

5. overall_stability < 0.20
   → Suppress all inference output
   → Emit: "Inference too unstable for reliable interpretation"
```

### Partial Suppression Conditions

```
PARTIAL_SILENCE_CONDITIONS:

1. 0.30 <= overall_confidence < 0.40
   → Emit inference with explicit "LOW CONFIDENCE" warning
   → Include detailed breakdown of limiting factors

2. 0.30 <= overall_stability < 0.40
   → Emit inference with "FRAGILE INFERENCE" warning
   → Include stability concerns in output

3. signal_confidence < 0.40 (but other dimensions acceptable)
   → Emit with "LIMITED SIGNAL AGREEMENT" warning
   → Reduce assertion strength in language
```

---

## Interaction with Consumers

Downstream systems must:
- Respect confidence indicators
- Avoid reinterpreting low-confidence inference as actionable
- Surface uncertainty transparently

Confidence is part of the signal, not decoration.

---

## Failure Modes

Confidence assessment may fail if:
- Data quality issues are not detected
- Regime transitions are missed
- Novel market behavior emerges

Failure must reduce confidence, not mask it.

---

## Evolution of Confidence Models

Confidence logic may evolve as:
- New failure modes are identified
- Market structure changes
- Better diagnostics are developed

All changes must be documented and versioned.

---

## Computation Examples

### Example 1: High Confidence Scenario

```
Input Conditions:
- All data fields present, no delays
- 3 independent signals agree on capital pressure direction
- Regime clearly identified as "expiry week"
- No contradictory signals
- Inference stable over last 10 events

Computation:
data_quality_confidence = min(1.0, 1.0, 1.0) = 1.0
structural_confidence = 1.0 * 0.9 * 1.0 = 0.9
regime_confidence = 0.8 * 0.9 * 0.95 = 0.684
signal_confidence = sqrt(3/3) * 1.0 * 1.0 = 1.0

overall_confidence = (1.0 * 0.9 * 0.684 * 1.0)^(1/4) = 0.888

temporal_stability = 0.9 * 0.95 * 1.0 = 0.855
sensitivity_stability = 1.0 - 0.05 = 0.95
regime_stability = 0.95 * 0.9 = 0.855

overall_stability = (0.855 * 0.95 * 0.855)^(1/3) = 0.884

Result: HIGH confidence (0.89), ROBUST stability (0.88)
→ Output inference with full assertion
```

### Example 2: Medium Confidence Scenario

```
Input Conditions:
- One field missing (95% complete)
- 2 signals agree, 1 contradicts
- Regime classification with moderate clarity
- Some noise in recent events

Computation:
data_quality_confidence = min(0.95, 0.98, 0.90) = 0.90
structural_confidence = 0.8 * 0.75 * 0.90 = 0.54
regime_confidence = 0.5 * 0.85 * 0.80 = 0.34
signal_confidence = sqrt(2/3) * 0.75 * 1.0 = 0.612

overall_confidence = (0.90 * 0.54 * 0.34 * 0.612)^(1/4) = 0.574

temporal_stability = 0.7 * 0.85 * 0.98 = 0.583
sensitivity_stability = 1.0 - 0.15 = 0.85
regime_stability = 0.85 * 0.90 = 0.765

overall_stability = (0.583 * 0.85 * 0.765)^(1/3) = 0.721

Result: MEDIUM confidence (0.57), ROBUST stability (0.72)
→ Output inference with caveats and uncertainty flags
```

### Example 3: Critical - Silence Required

```
Input Conditions:
- Data delayed by 5 minutes
- Only 1 signal present
- Regime unclear (transitioning)
- High variance in recent inferences

Computation:
data_quality_confidence = min(0.92, exp(-300/60), 0.95) = 0.0067
structural_confidence = 0.7 * 0.5 * 0.85 = 0.2975
regime_confidence = 0.2 * 0.6 * 0.7 = 0.084
signal_confidence = sqrt(1/3) * 0.80 * 0.5 = 0.231

overall_confidence = (0.0067 * 0.2975 * 0.084 * 0.231)^(1/4) = 0.089

Result: CRITICAL confidence (0.09) < 0.30
→ SUPPRESS all inference output
→ Emit: "Data quality below acceptable threshold"
```

---

## Final Statement

**Galactus does not hide uncertainty.**

When confidence is low, Galactus speaks softly—or not at all.
