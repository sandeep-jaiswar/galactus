# Error Handling

**Comprehensive error handling guide for Galactus API**

---

## Overview

The Galactus API uses standard HTTP status codes and structured error responses to communicate issues to clients.

All error responses follow a consistent format for easy parsing and handling.

---

## Error Response Format

### Structure

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": {
      "field": "additional_context"
    }
  }
}
```

### Fields

- **code**: Machine-readable error identifier (uppercase, underscore-separated)
- **message**: Human-readable description of the error
- **details**: Optional object with additional context

---

## HTTP Status Codes

### 2xx - Success

| Status | Meaning | Usage |
|--------|---------|-------|
| 200 OK | Request successful | All successful requests |

### 4xx - Client Errors

| Status | Meaning | Common Causes |
|--------|---------|---------------|
| 400 Bad Request | Invalid request data | Malformed JSON, missing required fields, invalid values |
| 401 Unauthorized | Authentication failed | Missing API key, invalid API key, expired token |
| 403 Forbidden | Access denied | Insufficient permissions, blocked client |
| 404 Not Found | Resource not found | Invalid endpoint path |
| 429 Too Many Requests | Rate limit exceeded | Too many requests in time window |

### 5xx - Server Errors

| Status | Meaning | Common Causes |
|--------|---------|---------------|
| 500 Internal Server Error | Server error | Unexpected server condition, bug |
| 503 Service Unavailable | Service temporarily unavailable | Maintenance, overload, kill switch active |

---

## Error Codes

### Authentication Errors

#### UNAUTHORIZED

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid or missing API key",
    "details": {
      "header_required": "X-API-Key"
    }
  }
}
```

**Causes**:
- Missing `X-API-Key` header
- Invalid API key format
- Revoked API key

**Resolution**:
- Verify API key is set correctly
- Check for typos in API key
- Request new API key if revoked

#### TOKEN_EXPIRED

```json
{
  "error": {
    "code": "TOKEN_EXPIRED",
    "message": "Authentication token has expired",
    "details": {
      "expired_at": 1703123456,
      "action": "refresh token or obtain new credentials"
    }
  }
}
```

**Causes**:
- JWT token past expiration time
- Session timeout

**Resolution**:
- Refresh token using refresh endpoint
- Obtain new authentication token

### Authorization Errors

#### FORBIDDEN

```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Insufficient permissions for this operation",
    "details": {
      "required_role": "compute",
      "current_roles": ["read"]
    }
  }
}
```

**Causes**:
- API key lacks required permissions
- Client is blocked
- Endpoint requires higher privilege

**Resolution**:
- Request elevated permissions
- Use appropriate API key with correct role
- Contact administrator if blocked

### Validation Errors

#### INVALID_REQUEST

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Request validation failed",
    "details": {
      "field": "signals[0].value",
      "error": "Value 1.5 is outside valid range [-1.0, 1.0]"
    }
  }
}
```

**Causes**:
- Missing required fields
- Invalid field values
- Type mismatches
- Out of range values

**Resolution**:
- Check request against API schema
- Verify all required fields present
- Validate field value ranges
- Ensure correct data types

#### SCHEMA_VALIDATION_FAILED

```json
{
  "error": {
    "code": "SCHEMA_VALIDATION_FAILED",
    "message": "Request does not match expected schema",
    "details": {
      "errors": [
        {
          "path": "signals",
          "message": "Array must contain at least 1 item"
        },
        {
          "path": "client_id",
          "message": "Required field missing"
        }
      ]
    }
  }
}
```

**Causes**:
- Malformed JSON
- Missing required properties
- Extra unexpected properties
- Type mismatches

**Resolution**:
- Validate JSON syntax
- Compare against OpenAPI schema
- Remove unexpected fields
- Ensure all required fields present

### Rate Limiting Errors

#### RATE_LIMIT_EXCEEDED

```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit of 100 requests per second exceeded",
    "details": {
      "limit": 100,
      "window_seconds": 60,
      "retry_after_seconds": 30,
      "current_usage": 105
    }
  }
}
```

**HTTP Headers**:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1703123456
Retry-After: 30
```

**Causes**:
- Exceeded requests per second limit
- Burst traffic
- Inefficient client implementation

**Resolution**:
- Implement exponential backoff
- Use batch endpoints when possible
- Request higher rate limit if justified
- Implement client-side rate limiting

### Processing Errors

#### INTERNAL_ERROR

```json
{
  "error": {
    "code": "INTERNAL_ERROR",
    "message": "An internal error occurred while processing the request",
    "details": {
      "request_id": "req_abc123",
      "timestamp": 1703123456
    }
  }
}
```

**Causes**:
- Unexpected server condition
- Bug in server code
- Resource unavailable

**Resolution**:
- Retry request with exponential backoff
- Contact support if persists
- Provide `request_id` when reporting

#### SERVICE_UNAVAILABLE

```json
{
  "error": {
    "code": "SERVICE_UNAVAILABLE",
    "message": "Intent computation service is temporarily unavailable",
    "details": {
      "reason": "kill_switch_active",
      "estimated_recovery_seconds": 300,
      "status_url": "http://status.galactus.example.com"
    }
  }
}
```

**Causes**:
- Kill switch activated
- Service maintenance
- System overload
- Data pipeline failure

**Resolution**:
- Wait for service recovery
- Check status page for updates
- Retry with exponential backoff
- Switch to fallback system if available

#### COMPUTATION_FAILED

```json
{
  "error": {
    "code": "COMPUTATION_FAILED",
    "message": "Intent computation failed",
    "details": {
      "reason": "insufficient_confidence",
      "min_confidence_required": 0.5,
      "achieved_confidence": 0.3
    }
  }
}
```

**Causes**:
- Low signal quality
- Insufficient data
- Regime transition
- Confidence threshold not met

**Resolution**:
- Review input signal quality
- Wait for better data availability
- Accept that no confident inference is available
- Don't force inference during uncertainty

---

## Error Handling Best Practices

### 1. Always Check Status Codes

```python
response = requests.post(url, json=data, headers=headers)

if response.status_code == 200:
    result = response.json()
    # Process successful response
elif response.status_code == 400:
    error = response.json()['error']
    # Handle validation error
elif response.status_code == 401:
    # Handle authentication error
elif response.status_code == 429:
    # Handle rate limiting
elif response.status_code >= 500:
    # Handle server error
else:
    # Handle unexpected status
```

### 2. Parse Error Response

```python
def parse_error(response):
    """Parse error response into structured format."""
    try:
        error_data = response.json()
        return {
            'code': error_data['error']['code'],
            'message': error_data['error']['message'],
            'details': error_data['error'].get('details', {})
        }
    except (KeyError, ValueError):
        return {
            'code': 'UNKNOWN_ERROR',
            'message': response.text,
            'details': {}
        }
```

### 3. Implement Retry Logic

```python
import time
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

def create_session_with_retries():
    """Create session with automatic retries."""
    session = requests.Session()
    
    retry_strategy = Retry(
        total=3,
        backoff_factor=1,
        status_forcelist=[429, 500, 502, 503, 504],
        allowed_methods=["GET", "POST"]
    )
    
    adapter = HTTPAdapter(max_retries=retry_strategy)
    session.mount("http://", adapter)
    session.mount("https://", adapter)
    
    return session
```

### 4. Implement Exponential Backoff

```python
def make_request_with_backoff(url, data, headers, max_retries=5):
    """Make request with exponential backoff."""
    for attempt in range(max_retries):
        try:
            response = requests.post(url, json=data, headers=headers, timeout=10)
            
            if response.status_code == 429:
                # Rate limited - use Retry-After header if present
                retry_after = int(response.headers.get('Retry-After', 2 ** attempt))
                time.sleep(retry_after)
                continue
            
            if response.status_code >= 500:
                # Server error - exponential backoff
                wait_time = min(2 ** attempt, 32)  # Max 32 seconds
                time.sleep(wait_time)
                continue
            
            # Success or client error - don't retry
            return response
            
        except requests.exceptions.Timeout:
            # Timeout - retry with backoff
            if attempt < max_retries - 1:
                wait_time = min(2 ** attempt, 32)
                time.sleep(wait_time)
                continue
            raise
    
    raise Exception(f"Failed after {max_retries} attempts")
```

### 5. Handle Rate Limiting

```python
class RateLimitedClient:
    """Client with rate limit handling."""
    
    def __init__(self, api_url, api_key):
        self.api_url = api_url
        self.api_key = api_key
        self.session = requests.Session()
        self.session.headers.update({'X-API-Key': api_key})
        
    def make_request(self, endpoint, data):
        """Make request with rate limit handling."""
        while True:
            response = self.session.post(
                f"{self.api_url}{endpoint}",
                json=data,
                timeout=10
            )
            
            if response.status_code != 429:
                return response
            
            # Rate limited - respect Retry-After
            retry_after = int(response.headers.get('Retry-After', 60))
            print(f"Rate limited. Waiting {retry_after} seconds...")
            time.sleep(retry_after)
```

### 6. Log Errors Properly

```python
import logging

logger = logging.getLogger(__name__)

def handle_api_error(response):
    """Log and handle API error."""
    error = parse_error(response)
    
    logger.error(
        f"API error: {error['code']} - {error['message']}",
        extra={
            'status_code': response.status_code,
            'error_code': error['code'],
            'error_details': error['details'],
            'request_id': response.headers.get('X-Request-ID')
        }
    )
    
    # Return structured error for caller
    return error
```

### 7. Handle Network Errors

```python
def make_robust_request(url, data, headers):
    """Make request with network error handling."""
    try:
        response = requests.post(
            url,
            json=data,
            headers=headers,
            timeout=10
        )
        response.raise_for_status()
        return response.json()
        
    except requests.exceptions.ConnectionError:
        logger.error("Connection error - service may be down")
        raise
        
    except requests.exceptions.Timeout:
        logger.error("Request timed out")
        raise
        
    except requests.exceptions.HTTPError as e:
        logger.error(f"HTTP error: {e.response.status_code}")
        error = handle_api_error(e.response)
        raise
        
    except requests.exceptions.RequestException as e:
        logger.error(f"Request failed: {e}")
        raise
```

---

## Testing Error Handling

### Unit Tests

```python
def test_validation_error_handling():
    """Test handling of validation errors."""
    client = GalactusClient(api_url, api_key)
    
    # Invalid signal value
    invalid_data = {
        'signals': [{'name': 'test', 'value': 1.5, 'confidence': 0.8}],
        'client_id': 'test'
    }
    
    with pytest.raises(requests.exceptions.HTTPError) as exc:
        client.compute_intent(invalid_data)
    
    assert exc.value.response.status_code == 400
    error = exc.value.response.json()['error']
    assert error['code'] == 'INVALID_REQUEST'
```

### Integration Tests

```python
def test_rate_limit_handling():
    """Test rate limit handling."""
    client = RateLimitedClient(api_url, api_key)
    
    # Send many requests
    results = []
    for i in range(150):  # Exceed limit of 100
        try:
            result = client.make_request('/api/v1/intent', test_data)
            results.append(result)
        except Exception as e:
            pytest.fail(f"Should handle rate limiting: {e}")
    
    # Should succeed eventually despite rate limiting
    assert len(results) == 150
```

---

## Monitoring and Alerting

### Track Error Rates

```bash
# Monitor error distribution
curl -s http://localhost:8080/api/v1/metrics | jq '.error_distribution'

# Alert on high error rates
galactus-cli alerts create \
  --name "HighAPIErrorRate" \
  --condition "error_rate > 0.05" \
  --severity critical
```

### Error Rate Metrics

```python
# Prometheus metrics
api_errors_total{code="INVALID_REQUEST"} 42
api_errors_total{code="UNAUTHORIZED"} 12
api_errors_total{code="RATE_LIMIT_EXCEEDED"} 8
api_errors_total{code="INTERNAL_ERROR"} 2
```

---

## Final Statement

**Errors are expected - handle them gracefully.**  
**Retry intelligently - don't hammer the service.**  
**Log everything - errors are debugging gold.**
