# Authentication and Authorization

**Secure access to Galactus intent inference API**

---

## Overview

The Galactus API supports multiple authentication methods:

1. **API Key Authentication** (HTTP header)
2. **JWT Token Authentication** (Bearer token)
3. **Mutual TLS** (Client certificates)

---

## API Key Authentication

### HTTP REST API

Include your API key in the `X-API-Key` header:

```bash
curl -X POST http://localhost:8080/api/v1/intent \
  -H "X-API-Key: your-api-key-here" \
  -H "Content-Type: application/json" \
  -d @request.json
```

### Python Example

```python
import requests

API_KEY = "your-api-key-here"
API_URL = "http://localhost:8080/api/v1/intent"

headers = {
    "X-API-Key": API_KEY,
    "Content-Type": "application/json"
}

response = requests.post(API_URL, headers=headers, json=request_data)
```

### JavaScript Example

```javascript
const API_KEY = 'your-api-key-here';
const API_URL = 'http://localhost:8080/api/v1/intent';

const response = await fetch(API_URL, {
  method: 'POST',
  headers: {
    'X-API-Key': API_KEY,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify(requestData)
});
```

---

## JWT Token Authentication

### Obtaining a Token

```bash
# Request JWT token
curl -X POST http://localhost:8080/api/v1/auth/token \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "your-client-id",
    "client_secret": "your-client-secret"
  }'
```

Response:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600
}
```

### Using JWT Token

```bash
curl -X POST http://localhost:8080/api/v1/intent \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d @request.json
```

### Token Refresh

```bash
curl -X POST http://localhost:8080/api/v1/auth/refresh \
  -H "Authorization: Bearer your-refresh-token"
```

---

## gRPC Authentication

### Metadata-based Authentication

```rust
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

// Create channel
let channel = Channel::from_static("http://localhost:50051")
    .connect()
    .await?;

let mut client = IntentServiceClient::new(channel);

// Add authentication metadata
let mut request = tonic::Request::new(intent_request);
request.metadata_mut().insert(
    "authorization",
    MetadataValue::from_str(&format!("Bearer {}", token))?
);

// Make authenticated request
let response = client.get_intent(request).await?;
```

### Python gRPC Authentication

```python
import grpc
from galactus_pb2_grpc import IntentServiceStub

# Create channel with credentials
credentials = grpc.ssl_channel_credentials()
channel = grpc.secure_channel('localhost:50051', credentials)
stub = IntentServiceStub(channel)

# Add authentication metadata
metadata = [('authorization', f'Bearer {token}')]

# Make authenticated request
response = stub.GetIntent(request, metadata=metadata)
```

---

## Mutual TLS (mTLS)

### Server Configuration

```rust
use tonic::transport::{Server, ServerTlsConfig};

let tls_config = ServerTlsConfig::new()
    .identity(cert, key)
    .client_ca_root(ca_cert);

Server::builder()
    .tls_config(tls_config)?
    .add_service(intent_service)
    .serve(addr)
    .await?;
```

### Client Configuration

```rust
use tonic::transport::{Channel, ClientTlsConfig};

let tls_config = ClientTlsConfig::new()
    .domain_name("galactus.example.com")
    .ca_certificate(ca_cert)
    .identity(client_cert, client_key);

let channel = Channel::from_static("https://localhost:50051")
    .tls_config(tls_config)?
    .connect()
    .await?;
```

---

## Authorization

### Role-Based Access Control (RBAC)

API keys and tokens can have different permission levels:

| Role | Permissions |
|------|-------------|
| **read** | Read-only access to health and metrics |
| **compute** | Intent computation access |
| **admin** | Full API access including configuration |

### Permission Checks

The API performs authorization checks on each request:

```json
{
  "client_id": "client_123",
  "roles": ["compute"],
  "rate_limit": 100,
  "allowed_endpoints": ["/api/v1/intent", "/api/v1/intent/batch"]
}
```

### Example: Restricted Client

```bash
# Client with read-only access
curl -X GET http://localhost:8080/api/v1/health \
  -H "X-API-Key: read-only-key"
# ✅ Success

curl -X POST http://localhost:8080/api/v1/intent \
  -H "X-API-Key: read-only-key" \
  -d @request.json
# ❌ 403 Forbidden
```

---

## API Key Management

### Generating API Keys

```bash
# Generate new API key
galactus-cli api-key generate \
  --client-id client_123 \
  --roles compute,read \
  --rate-limit 100

# Output:
# API Key: gal_live_abc123def456...
# Client ID: client_123
# Roles: compute, read
# Rate Limit: 100 req/sec
```

### Revoking API Keys

```bash
# Revoke API key
galactus-cli api-key revoke \
  --api-key gal_live_abc123def456...

# List active keys
galactus-cli api-key list \
  --client-id client_123
```

---

## Security Best Practices

### 1. Secure Storage

**DO NOT** commit API keys to source control:

```bash
# ❌ NEVER DO THIS
API_KEY=gal_live_abc123...  # In version control

# ✅ Use environment variables
export GALACTUS_API_KEY=gal_live_abc123...

# ✅ Use secrets management
aws secretsmanager get-secret-value --secret-id galactus/api-key
```

### 2. Key Rotation

Rotate API keys regularly:

```bash
# Rotate every 90 days
galactus-cli api-key rotate \
  --client-id client_123 \
  --grace-period 24h
```

### 3. Use HTTPS in Production

Always use TLS in production environments:

```bash
# ❌ Insecure
curl http://api.galactus.example.com/api/v1/intent

# ✅ Secure
curl https://api.galactus.example.com/api/v1/intent
```

### 4. Least Privilege

Grant minimum required permissions:

```bash
# ✅ Specific permissions
--roles compute

# ❌ Overly broad permissions
--roles admin
```

### 5. Monitor Usage

Track API key usage and detect anomalies:

```bash
# View usage statistics
galactus-cli api-key stats \
  --api-key gal_live_abc123... \
  --period 7d
```

---

## Error Responses

### Authentication Failures

#### Missing API Key

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Missing authentication credentials",
    "details": {
      "header_required": "X-API-Key"
    }
  }
}
```

#### Invalid API Key

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid API key",
    "details": {}
  }
}
```

#### Expired Token

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Token has expired",
    "details": {
      "expired_at": 1703123456,
      "action": "refresh token or obtain new credentials"
    }
  }
}
```

### Authorization Failures

#### Insufficient Permissions

```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Insufficient permissions",
    "details": {
      "required_role": "compute",
      "current_roles": ["read"]
    }
  }
}
```

---

## Production Configuration

### Docker Environment

```yaml
# docker-compose.yml
services:
  galactus-api:
    image: galactus-core:latest
    environment:
      - API_AUTH_ENABLED=true
      - API_KEY_SECRET=${API_KEY_SECRET}
      - JWT_SECRET=${JWT_SECRET}
      - TLS_CERT_PATH=/certs/server.crt
      - TLS_KEY_PATH=/certs/server.key
    volumes:
      - ./certs:/certs:ro
```

### Kubernetes Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: galactus-api-credentials
type: Opaque
stringData:
  api-key-secret: your-secret-here
  jwt-secret: your-jwt-secret-here
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: galactus-api
spec:
  template:
    spec:
      containers:
      - name: galactus
        env:
        - name: API_KEY_SECRET
          valueFrom:
            secretKeyRef:
              name: galactus-api-credentials
              key: api-key-secret
```

---

## Testing Authentication

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_valid_api_key() {
        let api = create_test_api();
        let request = create_authenticated_request("valid-key");
        
        let response = api.handle_request(request).await;
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_invalid_api_key() {
        let api = create_test_api();
        let request = create_authenticated_request("invalid-key");
        
        let response = api.handle_request(request).await;
        assert_eq!(response.status(), 401);
    }
}
```

---

## Final Statement

**Authentication protects the API.**  
**Authorization controls access.**  
**Security is mandatory, not optional.**
