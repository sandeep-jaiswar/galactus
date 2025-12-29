# Galactus — Daily Operations

## Purpose
Define routine operational checks for Galactus inference engine.

---

## Daily Checklist

### 1. Data Ingestion Health (09:00 IST)

Check data ingestion pipeline status:

```bash
# Check data ingestion metrics
curl -s http://localhost:8080/api/v1/metrics | jq '.data_ingestion'

# Verify last data timestamp
galactus-cli data status --check-freshness

# Expected: Data < 5 minutes old during market hours
```

**Action Required If:**
- Data is > 10 minutes stale during market hours
- Data ingestion errors > 5 in last hour
- Missing critical data sources (OI, price, volume)

### 2. Schema Validation (09:15 IST)

Verify data schema compliance:

```bash
# Run schema validation
galactus-cli data validate-schema --last-hour

# Check validation metrics
curl -s http://localhost:8080/api/v1/metrics | jq '.schema_validation'

# Expected: 0 schema validation failures
```

**Action Required If:**
- Schema validation failures > 0
- New unrecognized fields in data
- Type mismatches detected

### 3. Confidence Distribution Sanity (12:00 IST, 15:30 IST)

Review confidence levels across signals:

```bash
# Get confidence distribution
galactus-cli signals confidence-distribution --last-4h

# Check for confidence degradation
curl -s http://localhost:8080/api/v1/metrics | jq '.confidence_stats'

# Expected: Mean confidence > 0.6, no signals < 0.3
```

**Action Required If:**
- Mean confidence < 0.5 across all signals
- Any signal consistently < 0.3 for > 2 hours
- Sudden confidence drop > 20% without regime change

### 4. Kill Switch Status (Every Hour During Market)

Verify kill switch is operational:

```bash
# Check kill switch status
galactus-cli kill-switch status

# Test kill switch (dry-run)
galactus-cli kill-switch test --dry-run

# Expected: ACTIVE and responding
```

**Action Required If:**
- Kill switch status is UNKNOWN or ERROR
- Kill switch test fails
- Last kill switch check > 2 hours old

### 5. API Health Check (09:00 IST, 12:00 IST, 15:00 IST)

Monitor API service health:

```bash
# Check HTTP API
curl -s http://localhost:8080/api/v1/health | jq '.'

# Check gRPC API
grpcurl -plaintext localhost:50051 galactus.IntentService/HealthCheck

# Check API metrics
curl -s http://localhost:8080/api/v1/metrics | jq '.api_stats'

# Expected: Status "healthy", error_rate < 0.01
```

**Action Required If:**
- API status is "unhealthy" or "degraded"
- Error rate > 5%
- Average latency > 100ms
- Any endpoint returning 5xx errors

### 6. Monitoring Dashboard Review (10:00 IST, 14:00 IST)

Review Grafana dashboards:

```bash
# Open Grafana
# URL: http://localhost:3000
# Dashboard: "Galactus Operations"
```

**Check:**
- Signal quality metrics
- Intent computation latency
- Resource utilization (CPU, memory)
- Error rates and types

### 7. Log Review (End of Day)

Review system logs for anomalies:

```bash
# Check for errors
galactus-cli logs search --level ERROR --last-day

# Check for warnings
galactus-cli logs search --level WARN --last-day

# Review intent computation logs
galactus-cli logs intent-history --last-day
```

---

## Key Metrics Thresholds

| Metric | Normal Range | Warning | Critical |
|--------|--------------|---------|----------|
| Data Freshness | < 5 min | 5-10 min | > 10 min |
| Schema Failures | 0 | 1-5 | > 5 |
| Mean Confidence | > 0.7 | 0.5-0.7 | < 0.5 |
| API Error Rate | < 1% | 1-5% | > 5% |
| API Latency | < 50ms | 50-100ms | > 100ms |
| CPU Usage | < 60% | 60-80% | > 80% |
| Memory Usage | < 70% | 70-85% | > 85% |

---

## Monitoring Tools

- **Grafana**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9090
- **API Health**: http://localhost:8080/api/v1/health
- **API Metrics**: http://localhost:8080/api/v1/metrics

---

## Escalation

### Warning Level
- Log in operations journal
- Monitor for 30 minutes
- Document if persists

### Critical Level
- Trigger incident response playbook
- Activate kill switch if inference affected
- Notify on-call engineer
- Document in incident log

Anomalies must be logged same day in:
- Operations journal: `logs/operations/YYYY-MM-DD.md`
- Monitoring dashboard annotations
- Decision log if system changes required

---

## Operations Journal Template

```markdown
# Operations Log - YYYY-MM-DD

## Morning Checks (09:00-09:30)
- [ ] Data ingestion: HEALTHY / WARNING / CRITICAL
- [ ] Schema validation: PASS / FAIL
- [ ] Kill switch test: PASS / FAIL
- [ ] API health: HEALTHY / DEGRADED / DOWN

## Midday Checks (12:00-12:15)
- [ ] Confidence distribution: NORMAL / DEGRADED
- [ ] API health: HEALTHY / DEGRADED / DOWN

## Afternoon Checks (15:00-15:30)
- [ ] Confidence distribution: NORMAL / DEGRADED
- [ ] API health: HEALTHY / DEGRADED / DOWN

## End of Day (16:00)
- [ ] Log review: Clean / Issues Found
- [ ] Metrics review: Normal / Anomalies

## Anomalies
[List any anomalies detected and actions taken]

## Notes
[Any other operational notes]
```

---

## Final Statement
**Boring operations keep inference honest.**  
**Daily discipline prevents silent failures.**  
**Documented operations enable learning.**
