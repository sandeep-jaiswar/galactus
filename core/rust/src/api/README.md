# API Layer

The API Layer provides external interfaces for Galactus intent inference, offering both gRPC and HTTP APIs for production clients.

## Overview

The API layer enables:

- **gRPC Interface**: High-performance RPC for low-latency clients
- **HTTP REST API**: Web-compatible interface for broad compatibility
- **Type Safety**: Compile-time guarantees across language boundaries
- **Access Control**: Authentication and rate limiting
- **Monitoring**: Health checks and metrics endpoints

## Architecture

### Core Components

1. **gRPC Service** (`grpc.rs`): Protocol buffer-based RPC interface
2. **HTTP Service** (`http.rs`): RESTful JSON API
3. **Type Serialization** (`types.rs`): Safe type conversion and validation
4. **Configuration**: Security and performance settings

### Service Endpoints

#### gRPC Service
- `GetIntent`: Single intent vector computation
- `BatchIntent`: Multiple intent vectors in batch
- `StreamIntent`: Real-time intent stream (future)
- `HealthCheck`: Service health monitoring

#### HTTP REST API
- `POST /api/v1/intent`: Single intent computation
- `POST /api/v1/intent/batch`: Batch intent computation
- `GET /api/v1/health`: Health check
- `GET /api/v1/metrics`: Service metrics

## Usage

### gRPC Client

```rust
use galactus_core::api::{IntentServiceGrpc, IntentRequestProto};
use tonic::transport::Channel;

// Connect to gRPC server
let channel = Channel::from_static("http://localhost:50051").connect().await?;
let mut client = IntentServiceClient::new(channel);

// Create request
let request = tonic::Request::new(IntentRequestProto {
    signals: vec![/* signal data */],
    client_id: "client_123".to_string(),
    // ... other fields
});

// Call service
let response = client.get_intent(request).await?;
println!("Intent pressure: {}", response.into_inner().result.intent.pressure);
```

### HTTP Client

```bash
# Single intent computation
curl -X POST http://localhost:8080/api/v1/intent \
  -H "Content-Type: application/json" \
  -d '{
    "signals": [
      {
        "name": "oi_decay",
        "value": -0.4,
        "confidence": 0.85,
        "timestamp": 1703123456,
        "metadata": {}
      }
    ],
    "client_id": "client_123"
  }'
```

### Response Format

```json
{
  "result": {
    "intent": {
      "pressure": -0.25,
      "confidence": 0.82,
      "regime": "bear",
      "signals": {
        "oi_decay": {
          "value": -0.4,
          "weight": 0.8,
          "confidence": 0.85,
          "metadata": {}
        }
      },
      "timestamp": 1703123456
    },
    "alternatives": [],
    "metadata": {
      "server_version": "0.1.0"
    },
    "processing_time_ns": 150000
  },
  "metadata": {},
  "timestamp": 1703123456
}
```

## Configuration

```rust
use galactus_core::api::ApiConfig;

let config = ApiConfig {
    max_signals_per_request: 10,
    max_batch_size: 100,
    request_timeout_ms: 5000,
    max_request_size_bytes: 1024 * 1024,
    enable_auth: true,
    rate_limit_rps: 100,
    enable_logging: false,
    cors_allowed_origins: vec!["https://example.com".to_string()],
};
```

## Authentication

### API Key Authentication

Include API key in request headers:

```bash
curl -H "X-API-Key: your-api-key" \
     -H "Content-Type: application/json" \
     -d '{"signals": [...], "client_id": "client_123"}' \
     http://localhost:8080/api/v1/intent
```

### gRPC Authentication

```rust
use tonic::metadata::MetadataValue;

let mut request = tonic::Request::new(intent_request);
request.metadata_mut().insert(
    "authorization",
    MetadataValue::from_static("Bearer your-token")
);
```

## Rate Limiting

The API implements configurable rate limiting:

- **Per Client**: Requests per second per client ID
- **Global Limits**: Overall service protection
- **Burst Handling**: Token bucket algorithm

Rate limit headers in HTTP responses:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1703123456
```

## Error Handling

### HTTP Status Codes

- `200 OK`: Successful computation
- `400 Bad Request`: Invalid input data
- `401 Unauthorized`: Authentication failed
- `403 Forbidden`: Authorization failed
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error

### Error Response Format

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Signal value 1.5 is outside valid range [-1.0, 1.0]",
    "details": {}
  }
}
```

### gRPC Status Codes

- `OK`: Success
- `INVALID_ARGUMENT`: Bad request data
- `UNAUTHENTICATED`: Authentication failed
- `PERMISSION_DENIED`: Authorization failed
- `RESOURCE_EXHAUSTED`: Rate limit exceeded
- `INTERNAL`: Server error

## Type Safety

### Protocol Buffers

The gRPC API uses protocol buffers for type-safe communication:

```protobuf
message IntentRequest {
  repeated SignalInput signals = 1;
  string client_id = 2;
  map<string, string> metadata = 3;
  int64 timestamp = 4;
  string api_version = 5;
}

message IntentResponse {
  IntentResult result = 1;
  map<string, string> metadata = 2;
  int64 timestamp = 3;
  string api_version = 4;
}
```

### JSON Schema

HTTP API follows JSON Schema validation:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "signals": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": {"type": "string"},
          "value": {"type": "number", "minimum": -1.0, "maximum": 1.0},
          "confidence": {"type": "number", "minimum": 0.0, "maximum": 1.0},
          "timestamp": {"type": "integer"},
          "metadata": {"type": "object"}
        },
        "required": ["name", "value", "confidence", "timestamp"]
      }
    },
    "client_id": {"type": "string"}
  },
  "required": ["signals", "client_id"]
}
```

## Performance

### gRPC Performance
- **Latency**: Sub-millisecond for local calls
- **Throughput**: 10,000+ requests/second
- **Bandwidth**: Efficient binary serialization
- **Streaming**: Real-time data capabilities

### HTTP Performance
- **Latency**: 1-5ms for local calls
- **Throughput**: 1,000+ requests/second
- **Compression**: Automatic gzip compression
- **Connection Reuse**: Keep-alive connections

## Monitoring

### Health Checks

```bash
curl http://localhost:8080/api/v1/health
```

Response:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "metrics": {
    "total_requests": "15000",
    "active_connections": "5"
  }
}
```

### Metrics

```bash
curl http://localhost:8080/api/v1/metrics
```

Response:
```json
{
  "requests_total": 15000,
  "requests_per_second": 4.2,
  "average_latency_ms": 2.1,
  "error_rate": 0.001,
  "active_connections": 5
}
```

## Security

### Transport Security
- **TLS 1.3**: End-to-end encryption
- **Certificate Validation**: Client and server certificates
- **Perfect Forward Secrecy**: Ephemeral key exchange

### Application Security
- **Input Validation**: Comprehensive request validation
- **Output Encoding**: Safe response serialization
- **CORS Protection**: Configurable cross-origin policies
- **Security Headers**: OWASP recommended headers

## Deployment

### Docker Configuration

```dockerfile
FROM rust:1.70-slim as builder
# Build gRPC service
COPY . .
RUN cargo build --release --bin galactus-grpc

FROM debian:bullseye-slim
COPY --from=builder /target/release/galactus-grpc /usr/local/bin/
EXPOSE 50051
CMD ["galactus-grpc"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: galactus-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: galactus-api
  template:
    spec:
      containers:
      - name: grpc
        image: galactus-core:latest
        ports:
        - containerPort: 50051
        env:
        - name: RUST_LOG
          value: info
      - name: http
        image: galactus-core:latest
        ports:
        - containerPort: 8080
        env:
        - name: PORT
          value: "8080"
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_intent_endpoint() {
        let api = IntentApi::new(engine, config);
        let filter = api.routes();

        let response = request()
            .method("POST")
            .path("/api/v1/intent")
            .json(&intent_request)
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use tonic::transport::Channel;

    #[tokio::test]
    async fn test_grpc_integration() {
        let channel = Channel::from_static("http://localhost:50051").connect().await.unwrap();
        let mut client = IntentServiceClient::new(channel);

        // Test full request/response cycle
        let response = client.get_intent(request).await.unwrap();
        assert!(response.get_ref().result.intent.confidence > 0.0);
    }
}
```

## Examples

See `examples/` directory for complete client implementations in multiple languages.

## Dependencies

The API layer requires additional dependencies:

```toml
[dependencies]
tonic = "0.9"          # gRPC framework
prost = "0.11"         # Protocol buffer serialization
warp = "0.3"           # HTTP framework
serde = { version = "1.0", features = ["derive"] }  # JSON serialization
tokio = { version = "1.0", features = ["full"] }    # Async runtime
```

## Future Enhancements

- **WebSocket Streaming**: Real-time intent updates
- **GraphQL API**: Flexible query interface
- **API Versioning**: Backward compatibility
- **Advanced Rate Limiting**: Custom policies per endpoint
- **Request Tracing**: Distributed tracing support