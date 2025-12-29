# Galactus — Incident Response

## Purpose
Define response protocol for inference incidents and system failures.

An incident is any event that compromises the reliability, accuracy, or safety of Galactus inference.

---

## Incident Classification

### Severity Levels

| Level | Definition | Response Time | Examples |
|-------|------------|---------------|----------|
| **P0 - Critical** | Production inference compromised | Immediate (< 5 min) | False confidence, Silent failure, Data corruption |
| **P1 - High** | Service degraded but functional | < 30 min | API errors, Confidence degradation, Data delays |
| **P2 - Medium** | Minor impact, workaround exists | < 4 hours | Non-critical metrics, Logging issues |
| **P3 - Low** | No immediate impact | Next business day | Documentation gaps, Minor UX issues |

---

## Incident Types

### 1. False Confidence

**Definition**: System reports high confidence when inference is unreliable.

**Indicators**:
- High confidence during regime transition
- Confidence doesn't degrade with data quality issues
- Confidence mismatched with signal strength

**Example**:
```
Confidence: 0.85 during known regime shift
Expected: Confidence < 0.6 during transition
```

### 2. Silent Failure

**Definition**: System fails without alerting or degrading gracefully.

**Indicators**:
- No output when expected
- Stale timestamps in responses
- Kill switch not triggered when it should be
- Missing error logs for failures

**Example**:
```
Data ingestion failed 30 minutes ago
API still returning "healthy" status
No alerts triggered
```

### 3. Incorrect Inference

**Definition**: Intent vector computation produces wrong result.

**Indicators**:
- Pressure direction contradicts signal inputs
- Confidence calculation error
- Signal aggregation bug
- Regime misclassification

**Example**:
```
All signals show bearish pressure (-0.4 to -0.6)
Intent vector shows bullish pressure (+0.3)
```

### 4. API Failure

**Definition**: External API becomes unavailable or returns errors.

**Indicators**:
- HTTP 5xx errors
- Request timeouts
- Authentication failures
- Rate limit exceeded

### 5. Data Pipeline Failure

**Definition**: Data ingestion or processing fails.

See: [Data Failure Playbook](data-failure-playbook.md)

---

## Response Steps

### Step 1: Kill Switch (< 1 minute)

**Action**: Immediately halt inference if accuracy is in question.

```bash
# Activate kill switch
galactus-cli kill-switch activate \
  --reason "[Brief reason]" \
  --severity [P0|P1|P2|P3]

# Verify activation
galactus-cli kill-switch status
# Expected: ACTIVATED

# Confirm API reflects safe state
curl -s http://localhost:8080/api/v1/health
# Expected: status "degraded" or "unhealthy"
```

**When to activate**:
- ✅ Any P0 incident
- ✅ False confidence detected
- ✅ Silent failure discovered
- ✅ Incorrect inference confirmed
- ✅ Data quality compromised
- ❌ Minor API errors (P2/P3)
- ❌ Non-inference issues (logging, metrics)

### Step 2: Impact Assessment (< 5 minutes)

**Determine**:

1. **Scope**: What is affected?
```bash
# Check affected components
galactus-cli diagnostics scope-analysis

# Check affected time range
galactus-cli incidents timeline --current

# Check affected signals
galactus-cli signals status --show-health
```

2. **Duration**: How long has this been happening?
```bash
# Check logs for first occurrence
galactus-cli logs search \
  --pattern "[error pattern]" \
  --lookback 24h \
  --first-occurrence

# Review metrics for anomaly start
curl -s http://localhost:8080/api/v1/metrics | jq '.error_history'
```

3. **User Impact**: Are external clients affected?
```bash
# Check API request success rate
galactus-cli metrics api-success-rate --last 1h

# Check client impact
galactus-cli api clients-affected
```

4. **Data Integrity**: Is persisted data corrupt?
```bash
# Verify data integrity
galactus-cli data integrity-check --deep

# Check for corrupt outputs
galactus-cli output verify-integrity --last 24h
```

### Step 3: Documentation (< 10 minutes)

**Create incident record immediately**:

```bash
# Create incident
galactus-cli incidents create \
  --severity [P0|P1|P2|P3] \
  --type [false_confidence|silent_failure|incorrect_inference|api_failure|data_pipeline] \
  --title "[Short description]" \
  --impact "[Brief impact statement]"

# This creates: /logs/incidents/INC-YYYYMMDD-XXXX.md
```

**Initial incident report template**:

```markdown
# Incident INC-YYYYMMDD-XXXX

## Status
INVESTIGATING

## Severity
[P0|P1|P2|P3]

## Type
[Incident type]

## Start Time
YYYY-MM-DD HH:MM:SS IST

## Impact
[What is affected and how]

## Timeline
- HH:MM:SS - Incident detected
- HH:MM:SS - Kill switch activated (if applicable)
- HH:MM:SS - Impact assessed
- HH:MM:SS - Investigation started

## Detection Method
[How was this discovered - automated alert, manual check, user report]

## Current Actions
[What is being done right now]

## Next Steps
[What will be investigated next]
```

### Step 4: Fix or Deprecation

#### Option A: Quick Fix (< 1 hour)

If root cause is clear and fix is simple:

```bash
# Apply fix
[fix commands]

# Test fix
galactus-cli test verify-fix --incident INC-YYYYMMDD-XXXX

# Verify in staging
galactus-cli test integration --environment staging

# Deploy fix
galactus-cli deploy --with-verification

# Deactivate kill switch
galactus-cli kill-switch deactivate --verify-safety

# Monitor closely for 30 minutes
galactus-cli monitor --enhanced --duration 30m
```

#### Option B: Temporary Workaround (< 4 hours)

If fix requires more time:

```bash
# Implement workaround
[workaround commands]

# Document workaround limitations
galactus-cli incidents update INC-YYYYMMDD-XXXX \
  --workaround "[Workaround description]" \
  --limitations "[Known limitations]"

# Schedule proper fix
galactus-cli tasks create \
  --title "Proper fix for INC-YYYYMMDD-XXXX" \
  --priority high \
  --due-date [date]
```

#### Option C: Feature Deprecation

If feature is fundamentally flawed:

```bash
# Disable feature
galactus-cli features disable --feature [feature_name] \
  --reason "Incident INC-YYYYMMDD-XXXX"

# Update documentation
galactus-cli docs deprecate-feature --feature [feature_name]

# Notify stakeholders
galactus-cli alerts send \
  --type deprecation \
  --feature [feature_name] \
  --reason "Reliability concerns"

# Add to deprecation log
# See: docs/12-roadmap-and-deprecation/
```

---

## Communication

### Internal Communication

#### Incident Channel (Slack #galactus-incidents)

```
🚨 INCIDENT ALERT

Severity: [P0|P1|P2|P3]
Type: [Type]
Status: [INVESTIGATING|MITIGATING|RESOLVED]

Impact: [Brief impact]
Actions: [Current actions]
ETA: [Estimated resolution time]

Details: /logs/incidents/INC-YYYYMMDD-XXXX.md
```

#### Regular Updates

- **P0**: Every 15 minutes until resolved
- **P1**: Every 30 minutes until resolved  
- **P2**: Every 2 hours or at major milestones
- **P3**: Daily or when resolved

### External Communication

#### Status Page Updates

```bash
# Update status page
galactus-cli status-page update \
  --component [component] \
  --status [degraded|down] \
  --message "[User-facing message]"
```

**User-facing language**:
- ✅ "Intent computation temporarily unavailable"
- ✅ "Experiencing higher than normal latency"
- ✅ "Service operating in degraded mode"
- ❌ "Database crashed" (too technical)
- ❌ "Bug in signal aggregation" (too specific)

---

## Post-Incident Activities

### Post-Mortem (Within 48 hours)

```bash
# Generate post-mortem template
galactus-cli incidents postmortem INC-YYYYMMDD-XXXX
```

**Required sections**:

```markdown
# Post-Mortem: INC-YYYYMMDD-XXXX

## Summary
[2-3 sentences describing what happened]

## Impact
- Affected users: [Number or description]
- Duration: [Time]
- Data integrity: [Affected/Not affected]
- Financial impact: [If applicable]

## Root Cause
[Detailed technical explanation]

## Timeline
[Detailed timeline from detection to resolution]

## What Went Well
- [Things that worked in response]

## What Went Wrong
- [Things that didn't work or could be better]

## Action Items
- [ ] [Action 1] - Owner: [Name] - Due: [Date]
- [ ] [Action 2] - Owner: [Name] - Due: [Date]

## Lessons Learned
[Key takeaways for future incidents]
```

### Preventive Measures

1. **Add Monitoring**
```bash
# Add alert for this incident type
galactus-cli alerts create \
  --name "[Alert name]" \
  --condition "[Condition]" \
  --severity [level]
```

2. **Add Tests**
```bash
# Add regression test
# Location: core/rust/tests/incident_regression_tests.rs
```

3. **Update Documentation**
```bash
# Update relevant playbooks
# Update decision log
# Update system documentation
```

4. **Code Review**
```bash
# Review related code
# Add code comments
# Improve error handling
```

---

## Incident Review Meeting

### Weekly Incident Review (Every Monday 10:00 IST)

**Agenda**:
1. Review all incidents from past week
2. Discuss patterns or trends
3. Review action item progress
4. Update playbooks if needed

**Participants**:
- Engineering lead
- On-call engineer
- Operations team

---

## Testing and Drills

### Monthly Incident Drill

```bash
# Simulate incident
galactus-cli test simulate-incident \
  --type [type] \
  --severity [level] \
  --verify-response

# Expected response time:
# P0: < 5 minutes to kill switch
# P1: < 30 minutes to mitigation
```

### Chaos Engineering

```bash
# Inject failures to test resilience
galactus-cli chaos inject \
  --failure-type [type] \
  --duration [seconds] \
  --verify-detection
```

---

## Escalation Matrix

| Severity | Initial Response | Escalation (30min) | Escalation (2hr) |
|----------|-----------------|-------------------|------------------|
| P0 | On-call engineer | Engineering lead | CTO |
| P1 | On-call engineer | Engineering lead | - |
| P2 | Assigned engineer | Engineering lead (next day) | - |
| P3 | Assigned engineer | - | - |

### Contact Information

```bash
# Get on-call rotation
galactus-cli oncall who

# Page on-call engineer (P0 only)
galactus-cli oncall page --severity P0 --incident INC-YYYYMMDD-XXXX
```

---

## Metrics and Tracking

### Key Metrics

```bash
# Incident statistics
galactus-cli incidents stats --period month

# MTTR (Mean Time To Resolution)
galactus-cli metrics mttr --by-severity

# MTTD (Mean Time To Detection)
galactus-cli metrics mttd --by-type
```

### Continuous Improvement

**Monthly Review**:
- Incident frequency trends
- Response time improvements
- Action item completion rate
- Preventive measures effectiveness

---

## Decision Log Integration

All incidents with system impact must be documented in decision log:

```bash
# Create decision log entry
galactus-cli decision-log create \
  --title "System change from INC-YYYYMMDD-XXXX" \
  --context "Incident response" \
  --decision "[Decision made]" \
  --rationale "[Why this decision]" \
  --alternatives "[Alternatives considered]"
```

---

## Final Statement
**Incidents are learning opportunities, not PR events.**  
**Fast response prevents bad inference from reaching users.**  
**Every incident makes the system more resilient.**
