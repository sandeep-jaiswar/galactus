# Galactus — Expiry Day Playbook

## Purpose
Define special handling for options and futures expiry days in Indian markets.

Expiry days create forced capital flows and regime compression that require special operational awareness and monitoring.

---

## Expiry Day Schedule

### Monthly Expiries
- **Index Options & Futures**: Last Thursday of month
- **Stock Options & Futures**: Last Thursday of month

### Weekly Expiries (Nifty & Bank Nifty)
- **Every Thursday**: Weekly options expiry
- **Special Cases**: If Thursday is holiday, expiry on previous trading day

### Key Times (IST)
- **09:15**: Market open - monitor opening flows
- **14:30-15:30**: Peak forced flow period
- **15:30**: Market close - expiry settlement

---

## Pre-Expiry Preparation

### Day Before Expiry (15:30-17:00 IST)

```bash
# Verify expiry configuration
galactus-cli expiry verify-config --date tomorrow

# Check time-urgency weights
galactus-cli config show --section time_urgency
# Expected: time_urgency_weight increased for expiry day

# Prepare monitoring dashboards
# Open Grafana dashboard: "Expiry Day Monitoring"

# Review historical expiry patterns
galactus-cli analysis expiry-patterns --last-6-months
```

### Expiry Day Morning (08:30-09:15 IST)

```bash
# Confirm expiry day configuration active
galactus-cli expiry status
# Expected: ACTIVE, time_urgency_multiplier = 1.5

# Verify data sources
galactus-cli data test-all-sources

# Check monitoring alerts
galactus-cli alerts status

# Verify kill switch
galactus-cli kill-switch test --dry-run
```

---

## Rules

### 1. Increase Time-Urgency Weighting

**Rationale**: Time decay accelerates near expiry, forced flows increase.

```bash
# Automatic activation on expiry day
# Configuration in: config/expiry-day-rules.yaml

time_urgency:
  normal_weight: 0.3
  expiry_day_weight: 0.5
  last_hour_weight: 0.7  # 14:30-15:30
```

**Verification:**

```bash
# Check current time-urgency weight
galactus-cli signals get-weight --signal time_urgency

# Verify weight increased
# Expected: > normal weight
```

### 2. Expect Rapid Regime Shifts

**Rationale**: Capital pressure can change quickly as positions unwind.

```bash
# Enable high-frequency regime monitoring
galactus-cli regime monitor --interval 5m --expiry-mode

# Set regime shift alerts
galactus-cli alerts configure \
  --type regime_shift \
  --sensitivity high \
  --notify-on-change
```

**Monitoring:**

```bash
# Track regime stability
curl -s http://localhost:8080/api/v1/metrics | jq '.regime_stats'

# Expected during expiry:
# - regime_changes: Higher than normal
# - regime_confidence: May be lower
# - regime_duration: Shorter periods
```

### 3. Monitor Forced Flow Detection

**Rationale**: Expiry creates mandatory position closures - true forced flow.

```bash
# Enhanced forced flow monitoring
galactus-cli signals monitor \
  --signal forced_flow \
  --threshold-high \
  --expiry-mode

# Check forced flow metrics
curl -s http://localhost:8080/api/v1/metrics | jq '.forced_flow'
```

**Key Indicators:**

- **OI Decay Rate**: Accelerated near expiry
- **Rollover Activity**: High in final days
- **Strike Concentration**: Shifts as ITM positions adjust
- **Volume Spikes**: Position squaring activity

### 4. Enhanced Data Quality Checks

```bash
# More frequent data validation
galactus-cli data validate-schema --interval 2m --expiry-mode

# Monitor data freshness closely
galactus-cli data check-freshness --alert-threshold 3m

# Track OI consistency
galactus-cli data verify-oi-consistency --continuous
```

---

## Hourly Monitoring Schedule

### 09:15-10:00: Opening Hour

```bash
# Check opening flows
galactus-cli analysis opening-flows --expiry-day

# Monitor OI changes
galactus-cli oi-analysis summary --compare-with close

# Expected: May see significant OI reduction as positions close
```

### 10:00-14:00: Mid-Day Monitoring

```bash
# Standard monitoring every hour
galactus-cli daily-ops checklist --expiry-mode

# Track rollover activity
galactus-cli analysis rollover-tracking

# Monitor regime stability
galactus-cli regime current --with-confidence
```

### 14:30-15:30: Peak Period (Every 15 minutes)

This is the critical period for forced flows:

```bash
# Intensive monitoring
galactus-cli monitor expiry-peak-period \
  --interval 15m \
  --alerts-high-sensitivity

# Track specific metrics:
# - OI decay rate
# - Volume spikes
# - Regime shifts
# - Confidence levels
# - Forced flow signals
```

**Alert Thresholds (Peak Period):**

| Metric | Normal | Expiry Peak | Action |
|--------|--------|-------------|--------|
| OI Decay Rate | < 10%/hr | 20-50%/hr | Monitor closely |
| Regime Changes | < 2/day | 3-5/hr | Expected, document |
| Confidence Drop | > 20% | Investigate |
| Volume Spike | > 3x avg | Expected near close |

### 15:30-16:00: Post-Close Analysis

```bash
# Generate expiry day report
galactus-cli reports generate-expiry-report --date today

# Review key metrics
galactus-cli analysis expiry-summary

# Document unusual patterns
galactus-cli incidents create \
  --type expiry_observation \
  --severity info \
  --description "[Document any unusual patterns]"
```

---

## Forbidden Behavior

### 1. Carryover of Signals Post-Expiry

**Rule**: All time-urgency and expiry-specific signals must reset after 15:30.

```bash
# Verify signal reset (after 15:30)
galactus-cli signals verify-reset --expiry-day

# Expected: All expiry-specific adjustments removed
# Time-urgency weight returned to normal
```

**Verification:**

```bash
# Check configuration
galactus-cli config show --section time_urgency
# Expected: time_urgency_weight = normal_weight (0.3)

# Verify no stale expiry flags
galactus-cli expiry status
# Expected: INACTIVE or next expiry date
```

### 2. Using Stale OI Data

**Rule**: Never use OI data from before expiry for post-expiry analysis.

```bash
# Verify data cutoff
galactus-cli data verify-expiry-cutoff

# Check data timestamps
galactus-cli data check-timestamps --ensure-post-expiry
```

### 3. Ignoring Confidence Degradation

**Rule**: If confidence drops significantly, reduce intent certainty, don't ignore.

```bash
# Monitor confidence distribution
galactus-cli signals confidence-distribution --expiry-day

# Set strict confidence thresholds
galactus-cli config set confidence_min_threshold 0.6 --expiry-mode
```

---

## Anomaly Detection

### Expected Patterns

- **High Volume**: 2-3x normal in last hour
- **OI Reduction**: 30-60% reduction in expiring series
- **Regime Shifts**: More frequent (2-4 per day vs 0-1 normally)
- **Reduced Confidence**: 10-15% lower than normal
- **Forced Flow Signals**: Stronger magnitude

### Unexpected Patterns (Investigate)

```bash
# No OI reduction
# May indicate data failure or unusual market condition
galactus-cli data verify-oi-reduction --expiry-day --alert-if-low

# Extreme confidence degradation (> 30%)
# May indicate regime not captured in rules
galactus-cli analysis confidence-degradation --investigate

# Zero regime changes
# May indicate overly stable rules or data issues
galactus-cli regime verify-detection --expiry-day

# Volume anomalies (< 0.5x or > 5x normal)
# May indicate market disruption
galactus-cli analysis volume-anomaly --investigate
```

---

## Post-Expiry Actions

### Immediate (15:30-16:00)

```bash
# Deactivate expiry mode
galactus-cli expiry deactivate

# Verify configuration reset
galactus-cli config verify-reset --from expiry-mode

# Generate expiry day report
galactus-cli reports expiry-day --save
```

### End of Day (16:00-17:00)

```bash
# Complete expiry day analysis
galactus-cli analysis expiry-complete --date today

# Update historical patterns
galactus-cli expiry update-patterns --date today

# Document lessons learned
galactus-cli decision-log create \
  --title "Expiry Day Observations - YYYY-MM-DD" \
  --observations "[Key observations]"
```

---

## Expiry Day Report Template

```markdown
# Expiry Day Report - YYYY-MM-DD

## Configuration
- Expiry Type: [Monthly/Weekly]
- Instruments: [Nifty/BankNifty/Stocks]
- Time-Urgency Weight: [Value]

## Key Metrics
- OI Reduction: [Percentage]
- Rollover Percentage: [Value]
- Peak Volume Time: [Time]
- Regime Changes: [Count]
- Mean Confidence: [Value]

## Forced Flow Analysis
- Strong forced flow periods: [Times]
- Key signals activated: [List]
- Pressure magnitude: [Range]

## Anomalies
- [List any unexpected patterns]
- [Actions taken]

## Lessons Learned
- [Key observations]
- [Suggested improvements]

## Next Expiry
- Date: [YYYY-MM-DD]
- Preparation needed: [List]
```

---

## Testing

### Monthly Drill

```bash
# Test expiry day configuration
galactus-cli test expiry-config --dry-run

# Simulate expiry conditions
galactus-cli test simulate-expiry \
  --scenario high_forced_flow \
  --duration 60

# Verify monitoring alerts
galactus-cli test expiry-alerts --verify-all
```

---

## Key Monitoring Dashboards

### Grafana Dashboards
- **Expiry Day Overview**: Real-time metrics
- **OI Decay Tracking**: Position unwinding
- **Forced Flow Detection**: Enhanced monitoring
- **Regime Stability**: Change frequency

### Prometheus Alerts
```yaml
- alert: ExpiryDayOIAnomalyLow
  expr: oi_decay_rate < 0.1 on expiry day
  severity: warning

- alert: ExpiryDayConfidenceCritical
  expr: mean_confidence < 0.5 on expiry day
  severity: critical

- alert: ExpiryDayRegimeInstability
  expr: regime_changes > 5 in last hour
  severity: warning
```

---

## Historical Reference

### Typical Expiry Day Patterns (Indian Markets)

**Morning (09:15-12:00)**
- Gradual OI reduction
- Some rollover activity
- Moderate regime stability

**Afternoon (12:00-14:30)**
- Accelerating OI decay
- Increased rollover
- Higher volatility

**Final Hour (14:30-15:30)**
- Rapid OI reduction (50%+ of remaining)
- Peak forced flows
- Potential regime shifts
- Volume spikes

**Post-Close (15:30+)**
- All expiry positions closed
- Settlement calculations
- Next series becomes front month

---

## Final Statement
**Expiry days compress truth.**  
**Time urgency becomes capital imperative.**  
**Document patterns for future learning.**
