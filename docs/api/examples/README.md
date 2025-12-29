# API Usage Examples

**Practical examples for integrating with Galactus API**

---

## Overview

This directory contains complete, working examples for integrating with the Galactus API in multiple programming languages.

---

## Available Examples

### HTTP REST API
- **[`python-http-client.py`](python-http-client.py)** — Python requests client
- **[`javascript-fetch-client.js`](javascript-fetch-client.js)** — JavaScript/TypeScript fetch client
- **[`rust-http-client.rs`](rust-http-client.rs)** — Rust reqwest client
- **[`go-http-client.go`](go-http-client.go)** — Go net/http client

### gRPC API
- **[`python-grpc-client.py`](python-grpc-client.py)** — Python gRPC client
- **[`rust-grpc-client.rs`](rust-grpc-client.rs)** — Rust tonic client
- **[`go-grpc-client.go`](go-grpc-client.go)** — Go gRPC client

### Shell Scripts
- **[`curl-examples.sh`](curl-examples.sh)** — cURL examples for testing

---

## Quick Start

### Python HTTP Client

```bash
# Install dependencies
pip install requests

# Run example
python python-http-client.py
```

### JavaScript Client

```bash
# Install dependencies
npm install node-fetch

# Run example
node javascript-fetch-client.js
```

### Rust Client

```bash
# Add dependencies to Cargo.toml
cargo add reqwest tokio serde serde_json

# Run example
cargo run --example http-client
```

---

## Common Patterns

### Single Intent Computation

All clients follow this pattern:

1. **Prepare signal data**
```json
{
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
}
```

2. **Send authenticated request**
```python
headers = {"X-API-Key": api_key}
response = requests.post(url, json=data, headers=headers)
```

3. **Process intent result**
```python
result = response.json()
pressure = result["result"]["intent"]["pressure"]
confidence = result["result"]["intent"]["confidence"]
```

### Batch Processing

For processing multiple intents efficiently:

```python
batch_request = {
    "requests": [
        {"signals": [...], "client_id": "client_123"},
        {"signals": [...], "client_id": "client_123"},
        # ... more requests
    ]
}

response = requests.post(f"{url}/batch", json=batch_request, headers=headers)
```

### Error Handling

All examples include proper error handling:

```python
try:
    response = requests.post(url, json=data, headers=headers, timeout=5)
    response.raise_for_status()
    result = response.json()
except requests.exceptions.Timeout:
    print("Request timed out")
except requests.exceptions.HTTPError as e:
    if e.response.status_code == 401:
        print("Authentication failed")
    elif e.response.status_code == 429:
        print("Rate limit exceeded")
except requests.exceptions.RequestException as e:
    print(f"Request failed: {e}")
```

---

## Testing Examples

Each example includes test data and expected outputs:

```bash
# Test Python client
python python-http-client.py --test

# Test with custom endpoint
python python-http-client.py --url http://localhost:8080

# Enable debug logging
python python-http-client.py --debug
```

---

## Environment Variables

All examples support environment variables for configuration:

```bash
export GALACTUS_API_URL=http://localhost:8080
export GALACTUS_API_KEY=your-api-key-here

# Run examples
python python-http-client.py
```

---

## Dependencies

### Python
```bash
pip install requests grpcio grpcio-tools
```

### JavaScript/TypeScript
```bash
npm install node-fetch @grpc/grpc-js @grpc/proto-loader
```

### Rust
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tonic = "0.12"
```

### Go
```bash
go get google.golang.org/grpc
go get google.golang.org/protobuf
```

---

## Performance Considerations

### Connection Pooling

Reuse HTTP connections for better performance:

```python
import requests

# Create session with connection pooling
session = requests.Session()
session.headers.update({"X-API-Key": api_key})

# Reuse session for multiple requests
for data in batch_data:
    response = session.post(url, json=data)
```

### Parallel Requests

For high throughput:

```python
import asyncio
import aiohttp

async def compute_intent(session, data):
    async with session.post(url, json=data) as response:
        return await response.json()

async def main():
    async with aiohttp.ClientSession(headers=headers) as session:
        tasks = [compute_intent(session, data) for data in batch_data]
        results = await asyncio.gather(*tasks)
```

---

## Production Best Practices

1. **Use connection pooling** for HTTP clients
2. **Implement retry logic** with exponential backoff
3. **Set appropriate timeouts** (5-10 seconds recommended)
4. **Handle rate limits** gracefully
5. **Log requests and responses** for debugging
6. **Validate responses** before processing

---

## Final Statement

**Examples demonstrate integration.**  
**Examples show best practices.**  
**Examples are production-ready.**
