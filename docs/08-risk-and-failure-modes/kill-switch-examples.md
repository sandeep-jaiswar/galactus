# Kill Switch System - Usage Examples

This document provides practical examples of using the kill switch system in Galactus.

## Basic Usage

### Creating a Safe Intent Engine

```rust
use galactus_core::intent::{SafeIntentEngine, IntentConfig};
use galactus_core::kill_switch::KillSwitchConfig;

// Create with default configurations
let engine = SafeIntentEngine::default();

// Or create with custom configurations
let intent_config = IntentConfig {
    min_confidence_threshold: 0.7,
    max_alternatives: 3,
    // ... other config
};

let kill_switch_config = KillSwitchConfig {
    min_regime_confidence: 0.30,
    min_data_quality: 0.50,
    min_overall_confidence: 0.30,
    max_time_ambiguity_seconds: 60,
    min_data_completeness: 0.80,
    enforce_confidence_degradation: true,
};

let engine = SafeIntentEngine::new(intent_config, kill_switch_config);
```

### Processing Signals with Kill Switch Protection

```rust
use galactus_core::intent::SignalInput;
use galactus_core::kill_switch::{DataQualityMetrics, StructuralValidation};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Create signals
let signals = vec![
    SignalInput {
        name: "oi_decay".to_string(),
        value: -0.3,
        confidence: 0.85,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        metadata: HashMap::new(),
    },
    SignalInput {
        name: "hedge_pressure".to_string(),
        value: 0.4,
        confidence: 0.80,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        metadata: HashMap::new(),
    },
];

// Provide data quality metrics
let data_quality = DataQualityMetrics {
    data_completeness: 0.95,
    schema_valid: true,
    integrity_valid: true,
    max_time_ambiguity: 10,
    ordering_consistent: true,
};

// Provide structural validation
let structural = StructuralValidation {
    constraints_valid: true,
    forced_flow_consistent: true,
    violations: vec![],
};

// Process with kill switch protection
match engine.process_safe(signals, data_quality, structural) {
    Ok(result) => {
        // Check kill switch status
        match result.metadata.get("kill_switch_status") {
            Some("passed") => {
                println!("Inference succeeded with no kill switch warnings");
                println!("Pressure: {:.2}", result.intent.pressure);
                println!("Confidence: {:.2}", result.intent.confidence);
            }
            Some("warning") => {
                println!("Inference succeeded but with warnings:");
                for (key, value) in &result.metadata {
                    if key.starts_with("kill_switch_warning") {
                        println!("  - {}", value);
                    }
                }
                println!("Pressure: {:.2}", result.intent.pressure);
                println!("Confidence: {:.2}", result.intent.confidence);
            }
            _ => {}
        }
    }
    Err(e) => {
        eprintln!("Inference failed: {}", e);
        // Log incident for investigation
        // Alert operations team
        // Enter safe state
    }
}
```

## Kill Switch Scenarios

### Scenario 1: Data Integrity Failure

```rust
// Simulate corrupted data
let mut data_quality = DataQualityMetrics {
    data_completeness: 0.95,
    schema_valid: true,
    integrity_valid: false,  // INTEGRITY FAILURE
    max_time_ambiguity: 10,
    ordering_consistent: true,
};

let result = engine.process_safe(signals, data_quality, structural);
assert!(result.is_err());

// Result: Kill switch triggers with message:
// "Kill switch triggered: Inference halted due to 1 kill condition(s): 
//  Data integrity failure: Data integrity validation failed - 
//  corrupted or inconsistent canonical events detected"
```

### Scenario 2: Missing Core Data

```rust
// Simulate insufficient data
let mut data_quality = DataQualityMetrics {
    data_completeness: 0.60,  // BELOW THRESHOLD (0.80)
    schema_valid: true,
    integrity_valid: true,
    max_time_ambiguity: 10,
    ordering_consistent: true,
};

let result = engine.process_safe(signals, data_quality, structural);
assert!(result.is_err());

// Result: Kill switch triggers with message:
// "Kill switch triggered: Inference halted due to 1 kill condition(s): 
//  Missing core data: Missing core data across required windows - 
//  completeness 60.00% below required 80.00%"
```

### Scenario 3: Time Semantics Violation

```rust
// Simulate excessive time ambiguity
let mut data_quality = DataQualityMetrics {
    data_completeness: 0.95,
    schema_valid: true,
    integrity_valid: true,
    max_time_ambiguity: 120,  // EXCEEDS THRESHOLD (60s)
    ordering_consistent: true,
};

let result = engine.process_safe(signals, data_quality, structural);
assert!(result.is_err());

// Result: Kill switch triggers with message:
// "Kill switch triggered: Inference halted due to 1 kill condition(s): 
//  Time semantic violation: Event time cannot be reliably determined - 
//  ambiguity 120s exceeds allowed 60s"
```

### Scenario 4: Structural Violations

```rust
// Simulate constraint violations
let mut structural = StructuralValidation {
    constraints_valid: false,  // CONSTRAINT FAILURE
    forced_flow_consistent: true,
    violations: vec![
        "Option chain discontinuity at strike 19500".to_string(),
        "Futures settlement mismatch detected".to_string(),
    ],
};

let result = engine.process_safe(signals, data_quality, structural);
assert!(result.is_err());

// Result: Kill switch triggers with message:
// "Kill switch triggered: Inference halted due to 1 kill condition(s): 
//  Constraint misidentification: Core constraint assumptions invalidated - 
//  structural model inconsistent"
```

## Custom Threshold Configuration

### Stricter Thresholds for Production

```rust
let strict_config = KillSwitchConfig {
    min_regime_confidence: 0.40,      // Higher than default 0.30
    min_data_quality: 0.60,            // Higher than default 0.50
    min_overall_confidence: 0.40,      // Higher than default 0.30
    max_time_ambiguity_seconds: 30,    // Lower than default 60
    min_data_completeness: 0.90,       // Higher than default 0.80
    enforce_confidence_degradation: true,
};

let strict_engine = SafeIntentEngine::new(
    IntentConfig::default(),
    strict_config,
);
```

### More Lenient Thresholds for Development

```rust
let lenient_config = KillSwitchConfig {
    min_regime_confidence: 0.20,
    min_data_quality: 0.40,
    min_overall_confidence: 0.20,
    max_time_ambiguity_seconds: 120,
    min_data_completeness: 0.70,
    enforce_confidence_degradation: false,
};

let dev_engine = SafeIntentEngine::new(
    IntentConfig::default(),
    lenient_config,
);
```

## Monitoring and Alerting

### Tracking Kill Switch Activations

```rust
// In production, wrap engine calls with monitoring
let result = engine.process_safe(signals, data_quality, structural);

match result {
    Ok(intent_result) => {
        // Log successful inference
        metrics::increment_counter("inference.success");
        
        // Check for warnings
        if intent_result.metadata.get("kill_switch_status") == Some(&"warning".to_string()) {
            metrics::increment_counter("inference.warning");
            // Alert on warnings
            for (key, value) in &intent_result.metadata {
                if key.starts_with("kill_switch_warning") {
                    log::warn!("Kill switch warning: {}", value);
                }
            }
        }
    }
    Err(e) => {
        // Log failed inference
        metrics::increment_counter("inference.kill_switch_triggered");
        log::error!("Kill switch triggered: {}", e);
        
        // Alert operations team
        alert::send(AlertLevel::Critical, &format!("Kill switch triggered: {}", e));
        
        // Store incident for analysis
        incident_db::record_kill_switch_activation(&e);
    }
}
```

## Recovery Process

### After Kill Switch Activation

1. **Identify Root Cause**
```rust
// Extract detailed context from error
match result {
    Err(IntentError::KillSwitchTriggered(msg)) => {
        incident_log::record(&msg);
        
        // Parse which condition(s) triggered
        if msg.contains("Data integrity") {
            // Investigate data source
            data_source::run_diagnostics();
        } else if msg.contains("schema") {
            // Check schema versions
            schema_registry::validate_all();
        }
        // ... other conditions
    }
    _ => {}
}
```

2. **Fix Underlying Issue**
```rust
// Once fixed, verify data quality
let verified_data = data_source::fetch_verified();
let validation_result = validate_data_quality(&verified_data);

if validation_result.is_valid() {
    // Retry inference
    let result = engine.process_safe(signals, validation_result.metrics, structural);
    
    if result.is_ok() {
        log::info!("Inference resumed successfully after kill switch recovery");
        incident_log::mark_resolved();
    }
}
```

## Testing Kill Switch Conditions

### Unit Test Example

```rust
#[test]
fn test_kill_switch_on_data_integrity() {
    let engine = SafeIntentEngine::default();
    
    let signals = vec![
        create_test_signal("signal1", 0.5, 0.8),
    ];
    
    let mut data_quality = create_valid_data_quality();
    data_quality.integrity_valid = false;
    
    let result = engine.process_safe(
        signals,
        data_quality,
        create_valid_structural(),
    );
    
    assert!(result.is_err());
    match result {
        Err(IntentError::KillSwitchTriggered(msg)) => {
            assert!(msg.contains("Data integrity"));
        }
        _ => panic!("Expected KillSwitchTriggered error"),
    }
}
```

## Best Practices

1. **Always use SafeIntentEngine in production**
   - Never bypass kill switch in production systems
   - Use IntentEngine directly only in controlled test environments

2. **Monitor kill switch activations**
   - Track all activations as incidents
   - Alert on repeated activations
   - Review patterns monthly

3. **Set appropriate thresholds**
   - Start conservative, relax gradually based on data
   - Document any threshold changes in decision log
   - Review thresholds quarterly

4. **Log all context**
   - Record full data quality metrics on activation
   - Store structural validation results
   - Keep time series of activation rates

5. **Test recovery procedures**
   - Regular drills for kill switch scenarios
   - Validate recovery time objectives
   - Document lessons learned

## Integration with Production Systems

### Example Production Workflow

```rust
pub struct InferenceService {
    engine: SafeIntentEngine,
    data_validator: DataValidator,
    structural_validator: StructuralValidator,
    incident_tracker: IncidentTracker,
}

impl InferenceService {
    pub async fn infer(&self, market_data: MarketData) -> Result<IntentResult, ServiceError> {
        // 1. Validate data quality
        let data_quality = self.data_validator.assess(&market_data)?;
        
        // 2. Validate structural consistency
        let structural = self.structural_validator.check(&market_data)?;
        
        // 3. Extract signals
        let signals = self.extract_signals(&market_data)?;
        
        // 4. Process with kill switch protection
        match self.engine.process_safe(signals, data_quality, structural) {
            Ok(result) => {
                // Log success
                self.incident_tracker.record_success();
                Ok(result)
            }
            Err(e) => {
                // Track incident
                self.incident_tracker.record_kill_switch(&e);
                
                // Alert if needed
                if self.incident_tracker.should_alert() {
                    self.send_alert(&e).await?;
                }
                
                Err(ServiceError::InferenceHalted(e))
            }
        }
    }
}
```
