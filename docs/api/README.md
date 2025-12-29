# Galactus API Documentation

**Capital-pressure inference engine external interfaces**

---

## Overview

Galactus provides two production-ready API interfaces for intent inference:

1. **HTTP REST API**: Web-compatible JSON interface
2. **gRPC API**: High-performance protocol buffer interface

Both APIs provide identical functionality with different performance characteristics and integration requirements.

---

## Quick Start

### HTTP REST API

```bash
# Single intent computation
curl -X POST http://localhost:8080/api/v1/intent \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d @request.json
```

### gRPC API

```bash
# Using grpcurl
grpcurl -plaintext \
  -d @ localhost:50051 \
  galactus.IntentService/GetIntent < request.json
```

---

## Documentation Structure

- **[`openapi.yaml`](openapi.yaml)** — OpenAPI 3.0 specification for HTTP API
- **[`authentication.md`](authentication.md)** — Authentication and authorization guide
- **[`examples/`](examples/)** — Usage examples for multiple languages
- **[`error-handling.md`](error-handling.md)** — Error codes and handling strategies
- **[`rate-limiting.md`](rate-limiting.md)** — Rate limiting policies and headers

---

## API Endpoints

### HTTP REST API

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/intent` | Single intent computation |
| POST | `/api/v1/intent/batch` | Batch intent computation |
| GET | `/api/v1/health` | Health check |
| GET | `/api/v1/metrics` | Service metrics |

### gRPC API

| RPC Method | Description |
|------------|-------------|
| `GetIntent` | Single intent computation |
| `BatchIntent` | Batch intent computation |
| `HealthCheck` | Service health monitoring |

---

## Core Concepts

### Intent Vector

The primary output of the API is an **Intent Vector** representing capital pressure:

```json
{
  "pressure": -0.25,
  "confidence": 0.82,
  "regime": "bear",
  "signals": {
    "oi_decay": {
      "value": -0.4,
      "weight": 0.8,
      "confidence": 0.85
    }
  },
  "timestamp": 1703123456
}
```

- **pressure**: Capital pressure direction (-1.0 to 1.0)
- **confidence**: Confidence in the inference (0.0 to 1.0)
- **regime**: Market regime classification
- **signals**: Contributing signals with their weights
- **timestamp**: Computation time (Unix epoch)

### Signal Input

Intent is computed from multiple **signal inputs**:

```json
{
  "name": "oi_decay",
  "value": -0.4,
  "confidence": 0.85,
  "timestamp": 1703123456,
  "metadata": {}
}
```

- **name**: Signal identifier (e.g., "oi_decay", "hedge_pressure")
- **value**: Signal value (-1.0 to 1.0)
- **confidence**: Signal confidence (0.0 to 1.0)
- **timestamp**: Signal timestamp (Unix epoch)
- **metadata**: Optional signal-specific data

---

## Performance

### HTTP REST API
- **Latency**: 1-5ms for local calls
- **Throughput**: 1,000+ requests/second
- **Protocol**: HTTP/1.1 or HTTP/2

### gRPC API
- **Latency**: Sub-millisecond for local calls
- **Throughput**: 10,000+ requests/second
- **Protocol**: HTTP/2 with protocol buffers

---

## Security

- **Transport**: TLS 1.3 encryption (production)
- **Authentication**: API key or JWT token
- **Rate Limiting**: Per-client and global limits
- **Input Validation**: Comprehensive request validation

See [authentication.md](authentication.md) for detailed security documentation.

---

## Deployment

### Docker

```bash
# Start HTTP service
docker run -p 8080:8080 galactus-core:latest galactus-http-server

# Start gRPC service
docker run -p 50051:50051 galactus-core:latest galactus-grpc-server
```

### Service URLs

- **HTTP API**: `http://localhost:8080`
- **gRPC API**: `localhost:50051`
- **Health Check**: `http://localhost:8080/api/v1/health`
- **Metrics**: `http://localhost:8080/api/v1/metrics`

---

## Language Support

Client examples are provided for:
- **Rust**: Type-safe native client
- **Python**: Requests and gRPC clients
- **JavaScript/TypeScript**: Fetch and gRPC-web clients
- **Go**: Standard net/http and gRPC clients

See [`examples/`](examples/) for complete implementations.

---

## Final Statement

**The API provides inference, not advice.**  
**The API computes pressure, not predictions.**  
**The API waits for confidence, not guesses.**
