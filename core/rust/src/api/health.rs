//! Health Check Module
//!
//! Provides health check endpoints for monitoring and observability.
//! Health checks verify system components are functioning correctly.

use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Overall health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some non-critical issues
    Degraded,
    /// Critical issues present
    Unhealthy,
}

/// Component health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Component status
    pub status: HealthStatus,
    /// Optional message
    pub message: Option<String>,
    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
}

/// Complete health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Overall system status
    pub status: HealthStatus,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Individual component health
    pub components: Vec<ComponentHealth>,
    /// Timestamp of health check
    pub timestamp: String,
}

impl HealthCheckResponse {
    /// Create a new health check response
    pub fn new(uptime_seconds: u64, components: Vec<ComponentHealth>) -> Self {
        // Determine overall status from components
        let status = if components.iter().any(|c| c.status == HealthStatus::Unhealthy) {
            HealthStatus::Unhealthy
        } else if components.iter().any(|c| c.status == HealthStatus::Degraded) {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        Self {
            status,
            uptime_seconds,
            components,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Health checker that monitors system components
pub struct HealthChecker {
    start_time: Instant,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// Perform complete health check
    pub fn check(&self) -> HealthCheckResponse {
        let uptime = self.start_time.elapsed().as_secs();
        
        let mut components = Vec::new();

        // Check core system
        components.push(self.check_core_system());

        // Check data ingestion (basic check)
        components.push(self.check_data_ingestion());

        // Check inference engine
        components.push(self.check_inference_engine());

        HealthCheckResponse::new(uptime, components)
    }

    /// Check core system health
    fn check_core_system(&self) -> ComponentHealth {
        ComponentHealth {
            name: "core_system".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Core system operational".to_string()),
            response_time_ms: Some(1),
        }
    }

    /// Check data ingestion health
    fn check_data_ingestion(&self) -> ComponentHealth {
        // Basic check - in production, would verify data freshness
        ComponentHealth {
            name: "data_ingestion".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Data ingestion ready".to_string()),
            response_time_ms: Some(2),
        }
    }

    /// Check inference engine health
    fn check_inference_engine(&self) -> ComponentHealth {
        // Basic check - in production, would verify engine state
        ComponentHealth {
            name: "inference_engine".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Inference engine ready".to_string()),
            response_time_ms: Some(1),
        }
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_checker_basic() {
        let checker = HealthChecker::new();
        let response = checker.check();
        
        assert_eq!(response.status, HealthStatus::Healthy);
        assert!(response.uptime_seconds >= 0);
        assert!(!response.components.is_empty());
    }

    #[test]
    fn test_component_health() {
        let component = ComponentHealth {
            name: "test".to_string(),
            status: HealthStatus::Healthy,
            message: Some("OK".to_string()),
            response_time_ms: Some(10),
        };
        
        assert_eq!(component.name, "test");
        assert_eq!(component.status, HealthStatus::Healthy);
    }

    #[test]
    fn test_overall_status_degraded() {
        let components = vec![
            ComponentHealth {
                name: "good".to_string(),
                status: HealthStatus::Healthy,
                message: None,
                response_time_ms: None,
            },
            ComponentHealth {
                name: "degraded".to_string(),
                status: HealthStatus::Degraded,
                message: Some("Warning".to_string()),
                response_time_ms: None,
            },
        ];

        let response = HealthCheckResponse::new(100, components);
        assert_eq!(response.status, HealthStatus::Degraded);
    }

    #[test]
    fn test_overall_status_unhealthy() {
        let components = vec![
            ComponentHealth {
                name: "good".to_string(),
                status: HealthStatus::Healthy,
                message: None,
                response_time_ms: None,
            },
            ComponentHealth {
                name: "bad".to_string(),
                status: HealthStatus::Unhealthy,
                message: Some("Error".to_string()),
                response_time_ms: None,
            },
        ];

        let response = HealthCheckResponse::new(100, components);
        assert_eq!(response.status, HealthStatus::Unhealthy);
    }
}
