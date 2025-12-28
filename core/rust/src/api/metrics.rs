//! Metrics Collection Module
//!
//! Provides Prometheus-compatible metrics for monitoring.
//! Tracks system performance, data quality, and inference operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Metric types supported by the system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum MetricType {
    /// Counter that only increases
    Counter { value: u64 },
    /// Gauge that can increase or decrease
    Gauge { value: f64 },
    /// Histogram with buckets
    Histogram {
        count: u64,
        sum: f64,
        buckets: Vec<(f64, u64)>,
    },
}

/// A single metric with name and labels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric name
    pub name: String,
    /// Metric labels
    pub labels: HashMap<String, String>,
    /// Metric type and value
    pub metric_type: MetricType,
    /// Help text
    pub help: String,
}

/// Metrics collector for the system
pub struct MetricsCollector {
    metrics: Arc<Mutex<HashMap<String, Metric>>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        let collector = Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
        };

        // Initialize standard metrics
        collector.init_standard_metrics();
        collector
    }

    /// Initialize standard metrics
    fn init_standard_metrics(&self) {
        // Inference operation counter
        self.register_counter(
            "galactus_inference_operations_total",
            "Total number of inference operations",
        );

        // Data quality gauge
        self.register_gauge(
            "galactus_data_quality_score",
            "Current data quality score (0-1)",
        );

        // Confidence score gauge
        self.register_gauge(
            "galactus_confidence_score",
            "Current confidence score (0-1)",
        );

        // Stability score gauge
        self.register_gauge("galactus_stability_score", "Current stability score (0-1)");

        // Inference latency histogram
        self.register_histogram(
            "galactus_inference_latency_seconds",
            "Inference operation latency",
            vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0],
        );

        // Silence suppression counter
        self.register_counter(
            "galactus_silence_suppressions_total",
            "Total number of inferences suppressed",
        );

        // Active signals gauge
        self.register_gauge("galactus_active_signals", "Number of active signals");
    }

    /// Register a counter metric
    fn register_counter(&self, name: &str, help: &str) {
        let metric = Metric {
            name: name.to_string(),
            labels: HashMap::new(),
            metric_type: MetricType::Counter { value: 0 },
            help: help.to_string(),
        };

        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(name.to_string(), metric);
    }

    /// Register a gauge metric
    fn register_gauge(&self, name: &str, help: &str) {
        let metric = Metric {
            name: name.to_string(),
            labels: HashMap::new(),
            metric_type: MetricType::Gauge { value: 0.0 },
            help: help.to_string(),
        };

        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(name.to_string(), metric);
    }

    /// Register a histogram metric
    fn register_histogram(&self, name: &str, help: &str, buckets: Vec<f64>) {
        let bucket_counters = buckets.iter().map(|&b| (b, 0u64)).collect();

        let metric = Metric {
            name: name.to_string(),
            labels: HashMap::new(),
            metric_type: MetricType::Histogram {
                count: 0,
                sum: 0.0,
                buckets: bucket_counters,
            },
            help: help.to_string(),
        };

        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(name.to_string(), metric);
    }

    /// Increment a counter
    pub fn increment_counter(&self, name: &str) {
        self.increment_counter_by(name, 1);
    }

    /// Increment a counter by a specific value
    pub fn increment_counter_by(&self, name: &str, value: u64) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(metric) = metrics.get_mut(name) {
            if let MetricType::Counter { value: ref mut v } = &mut metric.metric_type {
                *v += value;
            }
        }
    }

    /// Set a gauge value
    pub fn set_gauge(&self, name: &str, value: f64) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(metric) = metrics.get_mut(name) {
            if let MetricType::Gauge { value: ref mut v } = &mut metric.metric_type {
                *v = value;
            }
        }
    }

    /// Observe a histogram value
    pub fn observe_histogram(&self, name: &str, value: f64) {
        let mut metrics = self.metrics.lock().unwrap();
        if let Some(metric) = metrics.get_mut(name) {
            if let MetricType::Histogram {
                count,
                sum,
                buckets,
            } = &mut metric.metric_type
            {
                *count += 1;
                *sum += value;

                // Increment bucket counters
                for (bucket_limit, bucket_count) in buckets.iter_mut() {
                    if value <= *bucket_limit {
                        *bucket_count += 1;
                    }
                }
            }
        }
    }

    /// Get all metrics
    pub fn get_metrics(&self) -> Vec<Metric> {
        let metrics = self.metrics.lock().unwrap();
        metrics.values().cloned().collect()
    }

    /// Export metrics in Prometheus text format
    pub fn export_prometheus(&self) -> String {
        let metrics = self.metrics.lock().unwrap();
        let mut output = String::new();

        for metric in metrics.values() {
            // Add help text
            output.push_str(&format!("# HELP {} {}\n", metric.name, metric.help));

            // Add type
            let type_str = match metric.metric_type {
                MetricType::Counter { .. } => "counter",
                MetricType::Gauge { .. } => "gauge",
                MetricType::Histogram { .. } => "histogram",
            };
            output.push_str(&format!("# TYPE {} {}\n", metric.name, type_str));

            // Add value(s)
            match &metric.metric_type {
                MetricType::Counter { value } => {
                    output.push_str(&format!("{} {}\n", metric.name, value));
                }
                MetricType::Gauge { value } => {
                    output.push_str(&format!("{} {}\n", metric.name, value));
                }
                MetricType::Histogram {
                    count,
                    sum,
                    buckets,
                } => {
                    for (bucket_limit, bucket_count) in buckets {
                        output.push_str(&format!(
                            "{}_bucket{{le=\"{}\"}} {}\n",
                            metric.name, bucket_limit, bucket_count
                        ));
                    }
                    output.push_str(&format!(
                        "{}_bucket{{le=\"+Inf\"}} {}\n",
                        metric.name, count
                    ));
                    output.push_str(&format!("{}_sum {}\n", metric.name, sum));
                    output.push_str(&format!("{}_count {}\n", metric.name, count));
                }
            }
            output.push('\n');
        }

        output
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let collector = MetricsCollector::new();

        collector.increment_counter("galactus_inference_operations_total");
        collector.increment_counter("galactus_inference_operations_total");

        let metrics = collector.get_metrics();
        let counter = metrics
            .iter()
            .find(|m| m.name == "galactus_inference_operations_total")
            .unwrap();

        if let MetricType::Counter { value } = counter.metric_type {
            assert_eq!(value, 2);
        } else {
            panic!("Expected counter metric");
        }
    }

    #[test]
    fn test_gauge_set() {
        let collector = MetricsCollector::new();

        collector.set_gauge("galactus_confidence_score", 0.85);

        let metrics = collector.get_metrics();
        let gauge = metrics
            .iter()
            .find(|m| m.name == "galactus_confidence_score")
            .unwrap();

        if let MetricType::Gauge { value } = gauge.metric_type {
            assert_eq!(value, 0.85);
        } else {
            panic!("Expected gauge metric");
        }
    }

    #[test]
    fn test_histogram_observe() {
        let collector = MetricsCollector::new();

        collector.observe_histogram("galactus_inference_latency_seconds", 0.015);
        collector.observe_histogram("galactus_inference_latency_seconds", 0.025);

        let metrics = collector.get_metrics();
        let histogram = metrics
            .iter()
            .find(|m| m.name == "galactus_inference_latency_seconds")
            .unwrap();

        if let MetricType::Histogram { count, sum, .. } = histogram.metric_type {
            assert_eq!(count, 2);
            assert!((sum - 0.04).abs() < 0.001);
        } else {
            panic!("Expected histogram metric");
        }
    }

    #[test]
    fn test_prometheus_export_format() {
        let collector = MetricsCollector::new();
        collector.increment_counter("galactus_inference_operations_total");
        collector.set_gauge("galactus_confidence_score", 0.9);

        let output = collector.export_prometheus();

        assert!(output.contains("# HELP galactus_inference_operations_total"));
        assert!(output.contains("# TYPE galactus_inference_operations_total counter"));
        assert!(output.contains("galactus_inference_operations_total 1"));
        assert!(output.contains("# TYPE galactus_confidence_score gauge"));
        assert!(output.contains("galactus_confidence_score 0.9"));
    }

    #[test]
    fn test_standard_metrics_initialized() {
        let collector = MetricsCollector::new();
        let metrics = collector.get_metrics();

        // Check that standard metrics are present
        let metric_names: Vec<&str> = metrics.iter().map(|m| m.name.as_str()).collect();

        assert!(metric_names.contains(&"galactus_inference_operations_total"));
        assert!(metric_names.contains(&"galactus_data_quality_score"));
        assert!(metric_names.contains(&"galactus_confidence_score"));
        assert!(metric_names.contains(&"galactus_stability_score"));
    }
}
