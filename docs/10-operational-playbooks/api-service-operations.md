# Galactus — API Service Operations

## Purpose
Define operational procedures for Galactus HTTP and gRPC API services.

---

## Service Overview

### Components
- **HTTP API**: REST interface on port 8080
- **gRPC API**: RPC interface on port 50051
- **Health Endpoint**: Service monitoring
- **Metrics Endpoint**: Performance tracking

### Service URLs
- HTTP API: `http://localhost:8080/api/v1`
- gRPC API: `localhost:50051`
- Grafana: `http://localhost:3000`
- Prometheus: `http://localhost:9090`

---

## Daily Operations

### Morning Startup Checks (09:00 IST)

```bash
# 1. Verify services are running
docker ps | grep galactus

# 2. Check HTTP API health
curl -s http://localhost:8080/api/v1/health | jq '.'
# Expected: status "healthy"

# 3. Check gRPC API health
grpcurl -plaintext localhost:50051 galactus.IntentService/HealthCheck
# Expected: status "SERVING"

# 4. Verify API metrics
curl -s http://localhost:8080/api/v1/metrics | jq '.api_stats'
# Review: error_rate, latency, active_connections

# 5. Check authentication
curl -s -X POST http://localhost:8080/api/v1/intent \
  -H "X-API-Key: invalid-key" \
  -d '{"signals":[],"client_id":"test"}'
# Expected: 401 Unauthorized
```

### Midday Health Check (12:00 IST, 15:00 IST)

```bash
# Quick health verification
galactus-cli api health-check --all-services

# Check for elevated error rates
galactus-cli api error-summary --last-hour

# Review rate limiting status
galactus-cli api rate-limit-status
```

### End of Day Review (16:00 IST)

```bash
# Generate daily API report
galactus-cli api daily-report --date today

# Review logs for errors
galactus-cli logs search --component api --level ERROR --last-day

# Check API usage statistics
galactus-cli api usage-stats --date today
```

---

## Service Monitoring

### Key Metrics

| Metric | Threshold | Check Interval | Alert Level |
|--------|-----------|----------------|-------------|
| API availability | > 99.9% | 1 minute | Critical if < 99% |
| Error rate | < 1% | 1 minute | Warning if > 1%, Critical if > 5% |
| Average latency | < 50ms | 5 minutes | Warning if > 100ms |
| P95 latency | < 100ms | 5 minutes | Warning if > 200ms |
| P99 latency | < 200ms | 5 minutes | Warning if > 500ms |
| Active connections | < 1000 | 1 minute | Warning if > 1000 |
| Request rate | Expected range | 5 minutes | Investigate outliers |

### Monitoring Commands

```bash
# Real-time API metrics
watch -n 5 'curl -s http://localhost:8080/api/v1/metrics | jq ".api_stats"'

# Error rate trend
galactus-cli api metrics error-rate --trend --last-hour

# Latency distribution
galactus-cli api metrics latency --percentiles 50,95,99 --last-hour

# Active connections
galactus-cli api metrics connections --current
```

### Grafana Dashboards

Access: `http://localhost:3000` (admin/admin)

**Primary Dashboards**:
1. **API Overview**: High-level health and metrics
2. **API Performance**: Latency and throughput details
3. **API Errors**: Error analysis and trends
4. **API Usage**: Client activity and patterns

---

## Common Issues and Resolution

### Issue 1: High Error Rate

**Symptoms**:
- Error rate > 5%
- 5xx status codes in responses

**Investigation**:
```bash
# Check recent errors
galactus-cli logs search --component api --level ERROR --last 15m

# Identify error types
curl -s http://localhost:8080/api/v1/metrics | jq '.error_distribution'

# Check system resources
docker stats galactus-core
```

**Common Causes**:
- Database connection issues
- Data pipeline failures
- Resource exhaustion
- Configuration errors

**Resolution**:
```bash
# If data-related: Check data pipeline
galactus-cli data pipeline-status

# If resource-related: Scale service
docker-compose up -d --scale galactus-core=2

# If config-related: Verify configuration
galactus-cli config validate --service api

# Restart service if needed
docker-compose restart galactus-core
```

### Issue 2: High Latency

**Symptoms**:
- P95 latency > 200ms
- Slow response times

**Investigation**:
```bash
# Check latency breakdown
galactus-cli api metrics latency-breakdown --last-hour

# Identify slow endpoints
galactus-cli api slow-requests --threshold 200ms --last-hour

# Check system resources
top -p $(pgrep galactus)
```

**Common Causes**:
- Expensive signal computations
- Database query performance
- Network latency
- Resource contention

**Resolution**:
```bash
# Enable query caching
galactus-cli config set cache_enabled true

# Optimize signal computation
galactus-cli signals optimize --profile

# Add read replicas if database-bound
# See: docs/02-system-architecture/scaling.md
```

### Issue 3: Authentication Failures

**Symptoms**:
- 401 errors increasing
- Valid API keys rejected

**Investigation**:
```bash
# Check authentication service
galactus-cli api auth-status

# Review authentication logs
galactus-cli logs search --pattern "authentication" --level ERROR --last-hour

# Test API key validation
galactus-cli api test-auth --api-key [test-key]
```

**Resolution**:
```bash
# Verify API key configuration
galactus-cli api-keys verify-config

# Reload API keys if needed
galactus-cli api-keys reload

# Check for expired keys
galactus-cli api-keys check-expiration
```

### Issue 4: Rate Limiting Issues

**Symptoms**:
- 429 errors from clients
- Legitimate requests blocked

**Investigation**:
```bash
# Check rate limit configuration
galactus-cli config show --section rate_limiting

# Review rate limit metrics
curl -s http://localhost:8080/api/v1/metrics | jq '.rate_limiting'

# Identify clients hitting limits
galactus-cli api rate-limit-violations --last-hour
```

**Resolution**:
```bash
# Adjust rate limits if appropriate
galactus-cli config set rate_limit_rps [new_value]

# Whitelist specific clients if needed
galactus-cli api-keys set-rate-limit --client-id [client] --limit [value]

# Review and block abusive clients
galactus-cli api block-client --client-id [client] --reason "[reason]"
```

---

## Service Deployment

### Rolling Restart

```bash
# Zero-downtime restart
docker-compose up -d --no-deps --build galactus-core

# Verify health after restart
sleep 10
curl -s http://localhost:8080/api/v1/health
```

### Configuration Changes

```bash
# Update configuration
vim config/api-config.yaml

# Validate configuration
galactus-cli config validate --file config/api-config.yaml

# Apply configuration (triggers reload)
galactus-cli config apply --file config/api-config.yaml

# Verify changes
galactus-cli config show --section [changed_section]
```

### Scaling

```bash
# Scale HTTP API horizontally
docker-compose up -d --scale galactus-core=3

# Verify load distribution
for i in {1..10}; do curl -s http://localhost:8080/api/v1/health | jq '.instance_id'; done

# Check load balancer status
galactus-cli loadbalancer status
```

---

## Security Operations

### API Key Management

```bash
# Generate new API key
galactus-cli api-keys generate \
  --client-id [client] \
  --roles compute,read \
  --rate-limit 100

# List active keys
galactus-cli api-keys list --status active

# Revoke compromised key
galactus-cli api-keys revoke --api-key [key] --reason "Compromised"

# Rotate expiring keys
galactus-cli api-keys rotate --expiring-in 7d
```

### Audit Logging

```bash
# Enable audit logging
galactus-cli config set audit_logging_enabled true

# Review audit logs
galactus-cli audit-logs search --last-day --client-id [client]

# Generate audit report
galactus-cli audit-logs report --period week
```

### Security Scanning

```bash
# Check for security issues
galactus-cli security scan --component api

# Review TLS configuration
galactus-cli security check-tls

# Verify authentication strength
galactus-cli security auth-audit
```

---

## Performance Optimization

### Connection Pooling

```yaml
# config/api-config.yaml
connection_pool:
  max_connections: 100
  min_connections: 10
  connection_timeout_ms: 5000
  idle_timeout_ms: 300000
```

### Caching Strategy

```bash
# Enable response caching
galactus-cli config set cache_enabled true
galactus-cli config set cache_ttl_seconds 300

# Monitor cache hit rate
curl -s http://localhost:8080/api/v1/metrics | jq '.cache_stats'

# Clear cache if needed
galactus-cli cache clear --confirm
```

### Request Compression

```bash
# Enable gzip compression
galactus-cli config set compression_enabled true
galactus-cli config set compression_level 6

# Verify compression working
curl -H "Accept-Encoding: gzip" http://localhost:8080/api/v1/intent -v
```

---

## Backup and Recovery

### Configuration Backup

```bash
# Backup current configuration
galactus-cli config backup \
  --output /backups/api-config-$(date +%Y%m%d).yaml

# Restore from backup
galactus-cli config restore \
  --file /backups/api-config-20240101.yaml
```

### Service State Backup

```bash
# Backup service state
galactus-cli state backup \
  --components api,auth \
  --output /backups/state-$(date +%Y%m%d).tar.gz
```

---

## Disaster Recovery

### Service Failure

```bash
# 1. Check service logs
docker logs galactus-core --tail 100

# 2. Attempt restart
docker-compose restart galactus-core

# 3. If restart fails, rebuild
docker-compose up -d --build --force-recreate galactus-core

# 4. Verify recovery
curl -s http://localhost:8080/api/v1/health
```

### Complete Outage

```bash
# 1. Stop all services
docker-compose down

# 2. Verify clean state
docker ps -a | grep galactus

# 3. Restore from backup if needed
galactus-cli state restore --file /backups/state-latest.tar.gz

# 4. Start services
docker-compose up -d

# 5. Verify all services
galactus-cli system health-check --all
```

---

## Monitoring Alerts

### Critical Alerts

```yaml
# Prometheus alert rules
groups:
  - name: api_critical
    rules:
      - alert: APIDown
        expr: up{job="galactus-api"} == 0
        for: 1m
        annotations:
          summary: "API service is down"
          
      - alert: HighErrorRate
        expr: rate(api_errors_total[5m]) > 0.05
        for: 5m
        annotations:
          summary: "API error rate > 5%"
          
      - alert: HighLatency
        expr: histogram_quantile(0.95, api_latency_seconds) > 0.2
        for: 10m
        annotations:
          summary: "P95 latency > 200ms"
```

### Warning Alerts

```yaml
  - name: api_warning
    rules:
      - alert: ModerateErrorRate
        expr: rate(api_errors_total[5m]) > 0.01
        for: 10m
        annotations:
          summary: "API error rate > 1%"
          
      - alert: HighConnectionCount
        expr: api_active_connections > 1000
        for: 5m
        annotations:
          summary: "High connection count"
```

---

## Testing and Validation

### Health Check Test

```bash
# Test all endpoints
galactus-cli api test health-checks

# Test with load
galactus-cli api load-test \
  --endpoint /api/v1/intent \
  --requests 1000 \
  --concurrent 10
```

### Integration Tests

```bash
# Run API integration tests
cd core/rust
cargo test --test api_integration_test

# Run end-to-end tests
galactus-cli test e2e --suite api
```

---

## Documentation

### API Documentation
- OpenAPI spec: `docs/api/openapi.yaml`
- Usage examples: `docs/api/examples/`
- Authentication guide: `docs/api/authentication.md`

### Internal Documentation
- Architecture: `docs/02-system-architecture/`
- Deployment: `docs/10-operational-playbooks/`

---

## Final Statement
**API reliability is inference availability.**  
**Monitor continuously, respond immediately.**  
**Document everything for future operations.**
