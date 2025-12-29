# Galactus — Data Failure Playbook

## Trigger Events

Data failures that require immediate response:

### 1. Missing Data
- Critical data source unavailable for > 10 minutes
- OI data missing during market hours
- Price/volume data gaps > 5 minutes
- Complete data feed outage

### 2. Corrupted Feeds
- Schema validation failures
- Malformed data records
- Data type mismatches
- Negative values where impossible (e.g., volume)
- Timestamp inconsistencies

### 3. Delayed Critical Events
- Expiry data delayed > 30 minutes
- Corporate action data missing on event day
- Market hours data delayed > 15 minutes
- Exchange holiday schedule mismatch

---

## Detection Methods

### Automated Monitoring

```bash
# Data freshness check (runs every 5 minutes)
galactus-cli data check-freshness --alert-if-stale

# Schema validation (continuous)
galactus-cli data validate-schema --continuous --alert-on-failure

# Data quality checks (every 15 minutes)
galactus-cli data quality-check --comprehensive
```

### Alert Triggers

Prometheus alerts configured for:
```yaml
- alert: DataIngestionStale
  expr: time() - data_ingestion_last_timestamp > 600
  severity: critical

- alert: SchemaValidationFailure
  expr: schema_validation_failures_total > 0
  severity: critical

- alert: DataQualityDegraded
  expr: data_quality_score < 0.8
  severity: warning
```

### Manual Checks

```bash
# Check data pipeline status
curl -s http://localhost:8080/api/v1/metrics | jq '.data_pipeline'

# Verify data sources
galactus-cli data sources --check-all

# Review recent data quality
galactus-cli data quality-report --last-hour
```

---

## Immediate Actions

### Step 1: Halt Inference (< 1 minute)

```bash
# Activate kill switch immediately
galactus-cli kill-switch activate --reason "data_failure_detected"

# Verify inference is halted
galactus-cli kill-switch status
# Expected: ACTIVATED

# Verify API returns safe state
curl -s http://localhost:8080/api/v1/health
# Expected: status "degraded" with data failure message
```

**Rationale**: Never compute intent with unreliable data.

### Step 2: Degrade Confidence (< 2 minutes)

```bash
# Mark all signals as low confidence
galactus-cli signals degrade-confidence --reason "data_failure"

# Set data quality flag
galactus-cli data set-quality-flag --status degraded

# Verify confidence degradation
galactus-cli signals confidence-distribution
# Expected: All signals confidence < 0.3
```

**Rationale**: Existing cached results must reflect data uncertainty.

### Step 3: Notify Operators (< 3 minutes)

```bash
# Send alert notifications
galactus-cli alerts send \
  --severity critical \
  --title "Data Failure Detected" \
  --message "Data ingestion failure - inference halted"

# Log incident
galactus-cli incidents create \
  --type data_failure \
  --severity critical \
  --status investigating
```

**Notification Channels:**
- Slack #galactus-alerts
- Email to on-call engineer
- PagerDuty escalation (if critical)
- Update status page

### Step 4: Document Initial State (< 5 minutes)

Capture diagnostic information:

```bash
# Save current system state
galactus-cli diagnostics snapshot \
  --output /logs/incidents/data-failure-$(date +%Y%m%d-%H%M%S).json

# Capture data pipeline state
galactus-cli data pipeline-state --detailed > \
  /logs/incidents/pipeline-state-$(date +%Y%m%d-%H%M%S).txt

# Save recent logs
galactus-cli logs export \
  --last 1h \
  --output /logs/incidents/failure-logs-$(date +%Y%m%d-%H%M%S).log
```

---

## Investigation Phase

### Identify Root Cause

#### 1. Check Data Sources

```bash
# Test each data source
galactus-cli data test-source --source nse_oi
galactus-cli data test-source --source nse_price
galactus-cli data test-source --source nse_volume

# Check source availability
curl -s https://www.nseindia.com/api/option-chain-indices?symbol=NIFTY
```

#### 2. Check Network Connectivity

```bash
# Test network path to exchanges
ping -c 5 www.nseindia.com
traceroute www.nseindia.com

# Check DNS resolution
nslookup www.nseindia.com

# Test TLS/SSL certificates
openssl s_client -connect www.nseindia.com:443 -servername www.nseindia.com
```

#### 3. Review Pipeline Logs

```bash
# Check ingestion logs
galactus-cli logs search --component data_ingestion --level ERROR --last 2h

# Check transformation logs
galactus-cli logs search --component data_transform --level ERROR --last 2h

# Check validation logs
galactus-cli logs search --component schema_validation --level ERROR --last 2h
```

#### 4. Validate Data Format

```bash
# Check recent data samples
galactus-cli data samples --last 10 --validate

# Compare with historical format
galactus-cli data format-diff --baseline yesterday --current now
```

---

## Recovery

### Option A: Source Recovery (Preferred)

If data source is restored:

```bash
# Verify source is healthy
galactus-cli data test-source --source <source_name> --verify

# Resume data ingestion
galactus-cli data resume-ingestion --source <source_name>

# Wait for data backfill
galactus-cli data wait-for-backfill --timeout 300

# Verify data quality
galactus-cli data quality-check --comprehensive

# Deactivate kill switch
galactus-cli kill-switch deactivate --verify-data-quality

# Restore normal confidence
galactus-cli signals restore-confidence
```

### Option B: Fallback Data Source

If primary source fails:

```bash
# Switch to backup data source
galactus-cli data switch-source \
  --from nse_primary \
  --to nse_backup

# Verify backup data quality
galactus-cli data quality-check --source nse_backup

# Resume inference with backup
galactus-cli kill-switch deactivate --with-fallback
```

### Option C: Historical Data Mode

If current data unavailable:

```bash
# Switch to historical analysis mode
galactus-cli mode set historical

# Use last known good data
galactus-cli data use-snapshot --timestamp <last_good_timestamp>

# Mark all outputs as historical
galactus-cli output set-mode --mode historical --warning-banner
```

---

## Reprocessing

After recovery, backfill missing data:

```bash
# Identify missing time ranges
galactus-cli data gaps identify --from <failure_start> --to <recovery_time>

# Backfill missing data
galactus-cli data backfill \
  --from <failure_start> \
  --to <recovery_time> \
  --verify

# Recompute signals for backfilled period
galactus-cli signals recompute \
  --from <failure_start> \
  --to <recovery_time>

# Verify recomputed results
galactus-cli signals verify-recomputation --sample-size 100
```

---

## Documentation Update

### Incident Report

Create incident report in `/logs/incidents/`:

```markdown
# Data Failure Incident - YYYY-MM-DD

## Summary
- **Start Time**: YYYY-MM-DD HH:MM:SS IST
- **End Time**: YYYY-MM-DD HH:MM:SS IST
- **Duration**: XX minutes
- **Impact**: [Description]
- **Root Cause**: [Identified cause]

## Timeline
- HH:MM:SS - Data failure detected
- HH:MM:SS - Kill switch activated
- HH:MM:SS - Root cause identified
- HH:MM:SS - Recovery initiated
- HH:MM:SS - Normal operations resumed

## Root Cause
[Detailed root cause analysis]

## Impact Assessment
- Missing data ranges: [List]
- Affected signals: [List]
- Inference downtime: XX minutes

## Actions Taken
1. [Action 1]
2. [Action 2]
...

## Preventive Measures
1. [Prevention 1]
2. [Prevention 2]
...

## Lessons Learned
[Key takeaways]
```

### Update Decision Log

If system changes required:

```bash
# Create decision log entry
galactus-cli decision-log create \
  --title "Data Failure Prevention Measures" \
  --context "Incident YYYY-MM-DD" \
  --decision "[Decision]" \
  --rationale "[Rationale]"
```

### Update Runbook

Document any new patterns or solutions.

---

## Prevention Measures

### Enhanced Monitoring

```yaml
# Add monitoring checks
- Data source health probes every 2 minutes
- Schema validation on every record
- Data quality scoring with trend analysis
- Network connectivity monitoring
```

### Redundancy

```bash
# Configure backup data sources
galactus-cli data configure-backup \
  --primary nse_primary \
  --backup nse_backup \
  --auto-failover true
```

### Circuit Breaker

```bash
# Configure circuit breaker
galactus-cli circuit-breaker configure \
  --failure-threshold 5 \
  --timeout 60 \
  --half-open-after 300
```

---

## Testing

### Regular Drills

```bash
# Monthly data failure drill
galactus-cli test simulate-data-failure \
  --scenario missing_oi_data \
  --duration 300 \
  --verify-response
```

### Chaos Engineering

```bash
# Test resilience
galactus-cli chaos inject-failure \
  --type data_corruption \
  --probability 0.1 \
  --duration 60
```

---

## Final Statement
**Data failure must never look like inference.**  
**Bad data is worse than no data.**  
**Document everything for future prevention.**
