# Galactus Production Monitoring

This directory contains monitoring and observability configurations for Galactus production deployment.

## Components

### 1. Health Checks (`core/rust/src/api/health.rs`)

Health check endpoints for monitoring system components:
- Core system status
- Data ingestion health
- Inference engine status

**Usage:**
```rust
use galactus_core::api::HealthChecker;

let checker = HealthChecker::new();
let health = checker.check();
println!("Status: {:?}", health.status);
```

### 2. Metrics Collection (`core/rust/src/api/metrics.rs`)

Prometheus-compatible metrics for system observability:
- `galactus_inference_operations_total` - Total inference operations (counter)
- `galactus_data_quality_score` - Current data quality (gauge, 0-1)
- `galactus_confidence_score` - Current confidence score (gauge, 0-1)
- `galactus_stability_score` - Current stability score (gauge, 0-1)
- `galactus_inference_latency_seconds` - Inference operation latency (histogram)
- `galactus_silence_suppressions_total` - Total silence suppressions (counter)
- `galactus_active_signals` - Number of active signals (gauge)

**Usage:**
```rust
use galactus_core::api::MetricsCollector;

let metrics = MetricsCollector::new();
metrics.increment_counter("galactus_inference_operations_total");
metrics.set_gauge("galactus_confidence_score", 0.85);
metrics.observe_histogram("galactus_inference_latency_seconds", 0.015);

// Export in Prometheus format
let prometheus_text = metrics.export_prometheus();
```

### 3. Alerting Rules (`alerting_rules.yml`)

Prometheus alerting rules for production monitoring:

**Alert Groups:**
- **galactus_health** - Service availability and error rates
- **galactus_data_quality** - Data quality and ingestion monitoring
- **galactus_inference** - Inference quality and performance
- **galactus_stability** - System stability metrics
- **galactus_resources** - Resource utilization

**Alert Severities:**
- `critical` - Requires immediate attention
- `warning` - Should be investigated
- `info` - Informational alerts

**Key Alerts:**
- `GalactusServiceDown` - Service is not responding
- `GalactusCriticalDataQuality` - Data quality below critical threshold
- `GalactusLowConfidence` - Sustained low confidence in inferences
- `GalactusInferenceLatency` - High inference latency detected

### 4. Grafana Dashboard (`grafana_dashboard.json`)

Production monitoring dashboard with:
- System health indicators
- Data quality, confidence, and stability gauges
- Inference operation rates and latency
- Resource utilization graphs
- Silence suppression tracking

## Deployment

### Prometheus Configuration

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'galactus'
    static_configs:
      - targets: ['localhost:8080']
    scrape_interval: 15s
    scrape_timeout: 10s
```

### Alerting Rules

Load the alerting rules:

```yaml
# prometheus.yml
rule_files:
  - "alerting_rules.yml"
```

### Grafana Dashboard

1. Import `grafana_dashboard.json` in Grafana UI
2. Configure Prometheus data source
3. Set appropriate refresh intervals

## Monitoring Best Practices

### 1. Deterministic Monitoring
- Metrics should be deterministic (same input = same metric value)
- Avoid non-deterministic timing measurements in core logic
- Use metrics for observability, not for inference decisions

### 2. Quality Over Quantity
- Focus on actionable metrics
- Avoid metric explosion
- Monitor what matters for capital pressure inference

### 3. Alert Fatigue Prevention
- Set appropriate alert thresholds
- Use proper severity levels
- Include context in alert descriptions
- Avoid duplicate alerts

### 4. Performance Impact
- Metrics collection should have minimal overhead
- Use sampling for high-frequency operations
- Aggregate metrics appropriately

## Development

### Adding New Metrics

1. Define metric in `MetricsCollector::init_standard_metrics()`
2. Update this documentation
3. Add corresponding alerts if needed
4. Update Grafana dashboard

### Testing Health Checks

```bash
# Run health check tests
cd core/rust
cargo test health
```

### Testing Metrics

```bash
# Run metrics tests
cd core/rust
cargo test metrics
```

## Production Readiness

### Prerequisites
- [ ] Prometheus server configured and running
- [ ] Alerting rules loaded in Prometheus
- [ ] Grafana dashboard imported
- [ ] Alert notification channels configured
- [ ] On-call rotation established

### Validation
- [ ] Health checks respond correctly
- [ ] Metrics are being scraped by Prometheus
- [ ] Alerts trigger appropriately
- [ ] Dashboard visualizations work correctly
- [ ] Alert notifications reach appropriate channels

## Troubleshooting

### No Metrics Appearing

1. Check Prometheus targets: `http://prometheus:9090/targets`
2. Verify metrics endpoint is accessible
3. Check firewall rules
4. Review Prometheus logs

### Alerts Not Firing

1. Verify alerting rules syntax
2. Check alert evaluation in Prometheus UI
3. Verify notification channels are configured
4. Review Alertmanager logs

### Dashboard Not Loading

1. Verify Prometheus data source is configured
2. Check time range selection
3. Verify metrics exist in Prometheus
4. Review browser console for errors

## References

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [Best Practices for Monitoring](https://prometheus.io/docs/practices/)
