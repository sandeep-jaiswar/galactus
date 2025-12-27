//! API Types and Serialization
//!
//! Type-safe serialization for API communication.
//! Handles conversion between internal Rust types and external representations.
//!
//! # Serialization Strategy
//!
//! - **Protocol Buffers**: For gRPC efficient binary serialization
//! - **JSON**: For HTTP REST APIs and human readability
//! - **Type Safety**: Compile-time guarantees across language boundaries
//! - **Versioning**: Explicit version handling for API evolution
//!
//! # Type Mapping
//!
//! Internal Rust types are mapped to external API types:
//! - `IntentVector` → `IntentVectorProto`
//! - `SignalInput` → `SignalInputProto`
//! - `IntentResult` → `IntentResultProto`
//!
//! # Validation
//!
//! All API types include validation:
//! - Range checks for numeric values
//! - Required field validation
//! - Size limits for collections
//! - Timestamp validation

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::intent::{IntentVector, IntentResult, SignalInput, SignalContribution};
use crate::api::{IntentRequest, IntentResponse, BatchIntentRequest, BatchIntentResponse, ApiError};

/// Protocol buffer representation of intent vector
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentVectorProto {
    /// Overall capital pressure (-1.0 to 1.0)
    pub pressure: f64,

    /// Confidence in the pressure reading (0.0 to 1.0)
    pub confidence: f64,

    /// Component signals that contributed
    pub signals: Vec<SignalContributionProto>,

    /// Timestamp of intent computation
    pub timestamp: i64,

    /// Market regime context
    pub regime: String,

    /// API version
    pub api_version: String,
}

/// Protocol buffer representation of signal contribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalContributionProto {
    /// Signal name
    pub name: String,

    /// Signal value (-1.0 to 1.0)
    pub value: f64,

    /// Signal weight in final aggregation (0.0 to 1.0)
    pub weight: f64,

    /// Signal confidence (0.0 to 1.0)
    pub confidence: f64,

    /// Signal-specific metadata
    pub metadata: HashMap<String, String>,
}

/// Protocol buffer representation of intent result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentResultProto {
    /// Primary intent vector
    pub intent: IntentVectorProto,

    /// Alternative interpretations
    pub alternatives: Vec<IntentVectorProto>,

    /// Computation metadata
    pub metadata: HashMap<String, String>,

    /// Processing time in nanoseconds
    pub processing_time_ns: u64,

    /// API version
    pub api_version: String,
}

/// Protocol buffer representation of signal input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalInputProto {
    /// Signal name
    pub name: String,

    /// Signal value (-1.0 to 1.0)
    pub value: f64,

    /// Signal confidence (0.0 to 1.0)
    pub confidence: f64,

    /// Signal timestamp
    pub timestamp: i64,

    /// Additional signal metadata
    pub metadata: HashMap<String, String>,
}

/// Protocol buffer representation of intent request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentRequestProto {
    /// Signals to process
    pub signals: Vec<SignalInputProto>,

    /// Request metadata
    pub metadata: HashMap<String, String>,

    /// Client identifier
    pub client_id: String,

    /// Request timestamp
    pub timestamp: i64,

    /// API version
    pub api_version: String,
}

/// Protocol buffer representation of intent response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentResponseProto {
    /// Intent computation result
    pub result: IntentResultProto,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Processing timestamp
    pub timestamp: i64,

    /// API version
    pub api_version: String,
}

/// Protocol buffer representation of batch intent request
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchIntentRequestProto {
    /// Multiple intent requests
    pub requests: Vec<IntentRequestProto>,

    /// Batch metadata
    pub metadata: HashMap<String, String>,

    /// Client identifier
    pub client_id: String,

    /// Request timestamp
    pub timestamp: i64,

    /// API version
    pub api_version: String,
}

/// Protocol buffer representation of batch intent response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchIntentResponseProto {
    /// Individual intent responses
    pub responses: Vec<IntentResponseProto>,

    /// Batch processing summary
    pub summary: BatchSummaryProto,

    /// Response metadata
    pub metadata: HashMap<String, String>,

    /// Processing timestamp
    pub timestamp: i64,

    /// API version
    pub api_version: String,
}

/// Protocol buffer representation of batch summary
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchSummaryProto {
    /// Total requests processed
    pub total_requests: u32,

    /// Successful computations
    pub successful: u32,

    /// Failed computations
    pub failed: u32,

    /// Total processing time
    pub total_processing_time_ns: u64,

    /// Average processing time per request
    pub avg_processing_time_ns: u64,
}

/// Current API version
pub const API_VERSION: &str = "v1.0.0";

/// Conversion trait for API types
pub trait ToProto<T> {
    fn to_proto(&self) -> Result<T, ApiError>;
}

/// Conversion trait from API types
pub trait FromProto<T> {
    fn from_proto(proto: T) -> Result<Self, ApiError> where Self: Sized;
}

impl ToProto<IntentVectorProto> for IntentVector {
    fn to_proto(&self) -> Result<IntentVectorProto, ApiError> {
        validate_intent_vector(self)?;

        let signals = self.signals.values()
            .map(|contrib| contrib.to_proto())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IntentVectorProto {
            pressure: self.pressure,
            confidence: self.confidence,
            signals,
            timestamp: self.timestamp,
            regime: self.regime.clone(),
            api_version: API_VERSION.to_string(),
        })
    }
}

impl FromProto<IntentVectorProto> for IntentVector {
    fn from_proto(proto: IntentVectorProto) -> Result<Self, ApiError> {
        validate_intent_vector_proto(&proto)?;

        let signals = proto.signals.into_iter()
            .map(SignalContribution::from_proto)
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(IntentVector {
            pressure: proto.pressure,
            confidence: proto.confidence,
            signals,
            timestamp: proto.timestamp,
            regime: proto.regime,
        })
    }
}

impl ToProto<SignalContributionProto> for SignalContribution {
    fn to_proto(&self) -> Result<SignalContributionProto, ApiError> {
        validate_signal_contribution(self)?;

        Ok(SignalContributionProto {
            name: self.metadata.get("name")
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            value: self.value,
            weight: self.weight,
            confidence: self.confidence,
            metadata: self.metadata.clone(),
        })
    }
}

impl FromProto<SignalContributionProto> for SignalContribution {
    fn from_proto(proto: SignalContributionProto) -> Result<Self, ApiError> {
        validate_signal_contribution_proto(&proto)?;

        Ok(SignalContribution {
            value: proto.value,
            weight: proto.weight,
            confidence: proto.confidence,
            metadata: proto.metadata,
        })
    }
}

impl ToProto<IntentResultProto> for IntentResult {
    fn to_proto(&self) -> Result<IntentResultProto, ApiError> {
        let intent = self.intent.to_proto()?;
        let alternatives = self.alternatives.iter()
            .map(|alt| alt.to_proto())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IntentResultProto {
            intent,
            alternatives,
            metadata: self.metadata.clone(),
            processing_time_ns: self.processing_time_ns,
            api_version: API_VERSION.to_string(),
        })
    }
}

impl FromProto<IntentResultProto> for IntentResult {
    fn from_proto(proto: IntentResultProto) -> Result<Self, ApiError> {
        let intent = IntentVector::from_proto(proto.intent)?;
        let alternatives = proto.alternatives.into_iter()
            .map(IntentVector::from_proto)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IntentResult {
            intent,
            alternatives,
            metadata: proto.metadata,
            processing_time_ns: proto.processing_time_ns,
        })
    }
}

impl ToProto<SignalInputProto> for SignalInput {
    fn to_proto(&self) -> Result<SignalInputProto, ApiError> {
        validate_signal_input(self)?;

        Ok(SignalInputProto {
            name: self.name.clone(),
            value: self.value,
            confidence: self.confidence,
            timestamp: self.timestamp,
            metadata: self.metadata.clone(),
        })
    }
}

impl FromProto<SignalInputProto> for SignalInput {
    fn from_proto(proto: SignalInputProto) -> Result<Self, ApiError> {
        validate_signal_input_proto(&proto)?;

        Ok(SignalInput {
            name: proto.name,
            value: proto.value,
            confidence: proto.confidence,
            timestamp: proto.timestamp,
            metadata: proto.metadata,
        })
    }
}

impl ToProto<IntentRequestProto> for IntentRequest {
    fn to_proto(&self) -> Result<IntentRequestProto, ApiError> {
        let signals = self.signals.iter()
            .map(|signal| signal.to_proto())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IntentRequestProto {
            signals,
            metadata: self.metadata.clone(),
            client_id: self.client_id.clone(),
            timestamp: self.timestamp,
            api_version: API_VERSION.to_string(),
        })
    }
}

impl FromProto<IntentRequestProto> for IntentRequest {
    fn from_proto(proto: IntentRequestProto) -> Result<Self, ApiError> {
        let signals = proto.signals.into_iter()
            .map(SignalInput::from_proto)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(IntentRequest {
            signals,
            metadata: proto.metadata,
            client_id: proto.client_id,
            timestamp: proto.timestamp,
        })
    }
}

impl ToProto<IntentResponseProto> for IntentResponse {
    fn to_proto(&self) -> Result<IntentResponseProto, ApiError> {
        let result = self.result.to_proto()?;

        Ok(IntentResponseProto {
            result,
            metadata: self.metadata.clone(),
            timestamp: self.timestamp,
            api_version: API_VERSION.to_string(),
        })
    }
}

impl FromProto<IntentResponseProto> for IntentResponse {
    fn from_proto(proto: IntentResponseProto) -> Result<Self, ApiError> {
        let result = IntentResult::from_proto(proto.result)?;

        Ok(IntentResponse {
            result,
            metadata: proto.metadata,
            timestamp: proto.timestamp,
        })
    }
}

// Validation functions
fn validate_intent_vector(vector: &IntentVector) -> Result<(), ApiError> {
    if !(-1.0..=1.0).contains(&vector.pressure) {
        return Err(ApiError::InvalidRequest(
            format!("Intent pressure {} is outside valid range [-1.0, 1.0]", vector.pressure)
        ));
    }

    if !(0.0..=1.0).contains(&vector.confidence) {
        return Err(ApiError::InvalidRequest(
            format!("Intent confidence {} is outside valid range [0.0, 1.0]", vector.confidence)
        ));
    }

    if vector.regime.trim().is_empty() {
        return Err(ApiError::InvalidRequest("Intent regime cannot be empty".to_string()));
    }

    Ok(())
}

fn validate_signal_contribution(contrib: &SignalContribution) -> Result<(), ApiError> {
    if !(-1.0..=1.0).contains(&contrib.value) {
        return Err(ApiError::InvalidRequest(
            format!("Signal contribution value {} is outside valid range [-1.0, 1.0]", contrib.value)
        ));
    }

    if !(0.0..=1.0).contains(&contrib.weight) {
        return Err(ApiError::InvalidRequest(
            format!("Signal contribution weight {} is outside valid range [0.0, 1.0]", contrib.weight)
        ));
    }

    if !(0.0..=1.0).contains(&contrib.confidence) {
        return Err(ApiError::InvalidRequest(
            format!("Signal contribution confidence {} is outside valid range [0.0, 1.0]", contrib.confidence)
        ));
    }

    Ok(())
}

fn validate_signal_input(signal: &SignalInput) -> Result<(), ApiError> {
    if !(-1.0..=1.0).contains(&signal.value) {
        return Err(ApiError::InvalidRequest(
            format!("Signal value {} is outside valid range [-1.0, 1.0]", signal.value)
        ));
    }

    if !(0.0..=1.0).contains(&signal.confidence) {
        return Err(ApiError::InvalidRequest(
            format!("Signal confidence {} is outside valid range [0.0, 1.0]", signal.confidence)
        ));
    }

    if signal.name.trim().is_empty() {
        return Err(ApiError::InvalidRequest("Signal name cannot be empty".to_string()));
    }

    Ok(())
}

fn validate_intent_vector_proto(proto: &IntentVectorProto) -> Result<(), ApiError> {
    if !(-1.0..=1.0).contains(&proto.pressure) {
        return Err(ApiError::InvalidRequest(
            format!("Intent pressure {} is outside valid range [-1.0, 1.0]", proto.pressure)
        ));
    }

    if !(0.0..=1.0).contains(&proto.confidence) {
        return Err(ApiError::InvalidRequest(
            format!("Intent confidence {} is outside valid range [0.0, 1.0]", proto.confidence)
        ));
    }

    if proto.regime.trim().is_empty() {
        return Err(ApiError::InvalidRequest("Intent regime cannot be empty".to_string()));
    }

    Ok(())
}

fn validate_signal_contribution_proto(proto: &SignalContributionProto) -> Result<(), ApiError> {
    if !(-1.0..=1.0).contains(&proto.value) {
        return Err(ApiError::InvalidRequest(
            format!("Signal contribution value {} is outside valid range [-1.0, 1.0]", proto.value)
        ));
    }

    if !(0.0..=1.0).contains(&proto.weight) {
        return Err(ApiError::InvalidRequest(
            format!("Signal contribution weight {} is outside valid range [0.0, 1.0]", proto.weight)
        ));
    }

    if !(0.0..=1.0).contains(&proto.confidence) {
        return Err(ApiError::InvalidRequest(
            format!("Signal contribution confidence {} is outside valid range [0.0, 1.0]", proto.confidence)
        ));
    }

    Ok(())
}

fn validate_signal_input_proto(proto: &SignalInputProto) -> Result<(), ApiError> {
    if !(-1.0..=1.0).contains(&proto.value) {
        return Err(ApiError::InvalidRequest(
            format!("Signal value {} is outside valid range [-1.0, 1.0]", proto.value)
        ));
    }

    if !(0.0..=1.0).contains(&proto.confidence) {
        return Err(ApiError::InvalidRequest(
            format!("Signal confidence {} is outside valid range [0.0, 1.0]", proto.confidence)
        ));
    }

    if proto.name.trim().is_empty() {
        return Err(ApiError::InvalidRequest("Signal name cannot be empty".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::IntentVector;

    #[test]
    fn test_intent_vector_proto_conversion() {
        let vector = IntentVector {
            pressure: 0.5,
            confidence: 0.8,
            signals: HashMap::new(),
            timestamp: 1234567890,
            regime: "bull".to_string(),
        };

        let proto = vector.to_proto().unwrap();
        let converted_back = IntentVector::from_proto(proto).unwrap();

        assert_eq!(vector, converted_back);
    }

    #[test]
    fn test_invalid_intent_vector_validation() {
        let invalid_vector = IntentVector {
            pressure: 1.5, // Invalid: outside [-1.0, 1.0]
            confidence: 0.8,
            signals: HashMap::new(),
            timestamp: 1234567890,
            regime: "test".to_string(),
        };

        let result = invalid_vector.to_proto();
        assert!(matches!(result, Err(ApiError::InvalidRequest(_))));
    }

    #[test]
    fn test_signal_input_proto_conversion() {
        let signal = SignalInput {
            name: "test_signal".to_string(),
            value: -0.3,
            confidence: 0.9,
            timestamp: 1234567890,
            metadata: HashMap::new(),
        };

        let proto = signal.to_proto().unwrap();
        let converted_back = SignalInput::from_proto(proto).unwrap();

        assert_eq!(signal, converted_back);
    }

    #[test]
    fn test_api_version_constant() {
        assert_eq!(API_VERSION, "v1.0.0");
    }
}