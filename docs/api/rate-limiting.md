# Rate Limiting

**Rate limiting policies and best practices for Galactus API**

---

## Overview

The Galactus API implements rate limiting to:
- **Protect service stability** from overload
- **Ensure fair resource allocation** across clients
- **Prevent abuse** and malicious activity
- **Maintain quality of service** for all users

---

## Rate Limit Policies

### Default Limits

| Tier | Requests/Second | Requests/Minute | Requests/Hour | Burst |
|------|-----------------|-----------------|---------------|-------|
| **Free** | 10 | 500 | 10,000 | 20 |
| **Standard** | 50 | 2,500 | 100,000 | 100 |
| **Premium** | 100 | 5,000 | 250,000 | 200 |
| **Enterprise** | Custom | Custom | Custom | Custom |

### Per-Endpoint Limits

Different endpoints may have different limits:

```yaml
# Endpoint-specific limits
/api/v1/intent:
  requests_per_second: 100
  burst: 200

/api/v1/intent/batch:
  requests_per_second: 10  # Lower due to higher cost
  burst: 20

/api/v1/health:
  requests_per_second: 1000  # Higher for monitoring
  burst: 2000

/api/v1/metrics:
  requests_per_second: 100
  burst: 200
```

---

## Rate Limit Headers

### Request Headers

No special headers required for rate limiting.

### Response Headers

Every API response includes rate limit information:

```http
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1703123456
X-RateLimit-Retry-After: 0
```

#### Header Definitions

- **X-RateLimit-Limit**: Maximum requests allowed in current window
- **X-RateLimit-Remaining**: Remaining requests in current window
- **X-RateLimit-Reset**: Unix timestamp when limit resets
- **X-RateLimit-Retry-After**: Seconds to wait before retrying (only when rate limited)

### Rate Limited Response

When rate limit is exceeded:

```http
HTTP/1.1 429 Too Many Requests
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1703123516
Retry-After: 60

{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit of 100 requests per second exceeded",
    "details": {
      "limit": 100,
      "window_seconds": 60,
      "retry_after_seconds": 60,
      "current_usage": 105
    }
  }
}
```

---

## Rate Limiting Algorithm

### Token Bucket Algorithm

Galactus uses the **token bucket algorithm**:

1. Each client has a bucket with a maximum capacity (burst size)
2. Tokens are added to the bucket at a constant rate (requests per second)
3. Each request consumes one token
4. If bucket is empty, request is rate limited
5. Bucket cannot exceed maximum capacity

**Example**:
```
Capacity: 100 tokens (burst size)
Refill rate: 50 tokens/second
Current tokens: 75

Request comes in:
- Consume 1 token → 74 tokens remaining
- Allow request

100 requests in 1 second:
- Consume 100 tokens → 0 tokens remaining
- Next request is rate limited
- Wait 1 second → 50 tokens added
- Can make 50 more requests
```

### Time Windows

Rate limits are enforced using **sliding windows**:

- **Per-second limit**: Rolling 1-second window
- **Per-minute limit**: Rolling 60-second window
- **Per-hour limit**: Rolling 3600-second window

This prevents burst traffic at window boundaries.

---

## Client Implementation

### Checking Rate Limits

```python
import requests

response = requests.post(url, json=data, headers=headers)

# Extract rate limit info
limit = int(response.headers.get('X-RateLimit-Limit', 0))
remaining = int(response.headers.get('X-RateLimit-Remaining', 0))
reset = int(response.headers.get('X-RateLimit-Reset', 0))

print(f"Rate limit: {remaining}/{limit}")
print(f"Resets at: {datetime.fromtimestamp(reset)}")

# Check if approaching limit
if remaining < limit * 0.1:  # Less than 10% remaining
    print("WARNING: Approaching rate limit")
```

### Handling Rate Limits

```python
import time
from datetime import datetime

def make_request_with_rate_limit_handling(url, data, headers):
    """Make request with automatic rate limit handling."""
    while True:
        response = requests.post(url, json=data, headers=headers)
        
        if response.status_code != 429:
            return response
        
        # Rate limited - extract retry information
        retry_after = int(response.headers.get('Retry-After', 60))
        reset_time = int(response.headers.get('X-RateLimit-Reset', 0))
        
        # Calculate wait time
        if reset_time:
            wait_seconds = max(reset_time - int(time.time()), 0)
        else:
            wait_seconds = retry_after
        
        print(f"Rate limited. Waiting {wait_seconds} seconds...")
        time.sleep(wait_seconds)
```

### Proactive Rate Limiting

Implement client-side rate limiting to avoid hitting server limits:

```python
import time
from collections import deque

class RateLimiter:
    """Client-side rate limiter."""
    
    def __init__(self, max_requests_per_second):
        self.max_requests = max_requests_per_second
        self.window_seconds = 1.0
        self.requests = deque()
    
    def wait_if_needed(self):
        """Wait if necessary to respect rate limit."""
        now = time.time()
        
        # Remove old requests outside window
        while self.requests and self.requests[0] < now - self.window_seconds:
            self.requests.popleft()
        
        # If at limit, wait
        if len(self.requests) >= self.max_requests:
            sleep_time = self.window_seconds - (now - self.requests[0])
            if sleep_time > 0:
                time.sleep(sleep_time)
            self.requests.popleft()
        
        # Record this request
        self.requests.append(time.time())

# Usage
limiter = RateLimiter(max_requests_per_second=50)

for data in batch_data:
    limiter.wait_if_needed()
    response = requests.post(url, json=data, headers=headers)
```

---

## Best Practices

### 1. Monitor Rate Limit Headers

Always check rate limit headers in responses:

```python
def monitor_rate_limits(response):
    """Monitor and log rate limit status."""
    remaining = int(response.headers.get('X-RateLimit-Remaining', -1))
    limit = int(response.headers.get('X-RateLimit-Limit', -1))
    
    if remaining >= 0:
        usage_percent = ((limit - remaining) / limit) * 100
        
        if usage_percent > 90:
            logger.warning(f"Rate limit usage: {usage_percent:.1f}%")
        
        # Store metrics
        metrics.gauge('rate_limit_remaining', remaining)
        metrics.gauge('rate_limit_usage_percent', usage_percent)
```

### 2. Use Batch Endpoints

Batch multiple requests to reduce API calls:

```python
# ❌ Bad: Multiple single requests
for signal_set in signal_sets:
    response = client.compute_intent([signal_set])

# ✅ Good: Single batch request
batch_request = {
    'requests': [{'signals': sig_set} for sig_set in signal_sets]
}
response = client.compute_batch(batch_request)
```

### 3. Implement Exponential Backoff

When rate limited, use exponential backoff:

```python
def exponential_backoff_retry(func, max_retries=5):
    """Retry with exponential backoff."""
    for attempt in range(max_retries):
        try:
            return func()
        except RateLimitError:
            if attempt == max_retries - 1:
                raise
            
            wait_time = min(2 ** attempt, 60)  # Max 60 seconds
            time.sleep(wait_time)
```

### 4. Distribute Load

Spread requests over time instead of bursts:

```python
import time

def process_with_pacing(items, requests_per_second):
    """Process items with controlled pacing."""
    interval = 1.0 / requests_per_second
    
    for item in items:
        start = time.time()
        
        # Process item
        process_item(item)
        
        # Wait to maintain pace
        elapsed = time.time() - start
        if elapsed < interval:
            time.sleep(interval - elapsed)
```

### 5. Cache Responses

Cache responses to reduce API calls:

```python
from functools import lru_cache
import hashlib
import json

class CachedClient:
    """Client with response caching."""
    
    def __init__(self, client):
        self.client = client
        self.cache = {}
    
    def compute_intent(self, signals, ttl_seconds=300):
        """Compute intent with caching."""
        # Create cache key
        cache_key = self._make_cache_key(signals)
        
        # Check cache
        if cache_key in self.cache:
            cached_time, cached_result = self.cache[cache_key]
            if time.time() - cached_time < ttl_seconds:
                return cached_result
        
        # Call API
        result = self.client.compute_intent(signals)
        
        # Cache result
        self.cache[cache_key] = (time.time(), result)
        
        return result
    
    def _make_cache_key(self, signals):
        """Create cache key from signals."""
        data = json.dumps(signals, sort_keys=True)
        return hashlib.sha256(data.encode()).hexdigest()
```

### 6. Use Connection Pooling

Reuse connections to reduce overhead:

```python
from requests.adapters import HTTPAdapter
from urllib3.poolmanager import PoolManager

class GalactusClient:
    """Client with connection pooling."""
    
    def __init__(self, api_url, api_key):
        self.api_url = api_url
        self.session = requests.Session()
        
        # Configure connection pooling
        adapter = HTTPAdapter(
            pool_connections=20,
            pool_maxsize=50,
            max_retries=3
        )
        self.session.mount('http://', adapter)
        self.session.mount('https://', adapter)
        
        self.session.headers.update({'X-API-Key': api_key})
```

---

## Rate Limit Monitoring

### Client-Side Monitoring

Track rate limit metrics in your application:

```python
class RateLimitMetrics:
    """Track rate limit metrics."""
    
    def __init__(self):
        self.total_requests = 0
        self.rate_limited_requests = 0
        self.min_remaining = float('inf')
    
    def record_request(self, response):
        """Record request metrics."""
        self.total_requests += 1
        
        if response.status_code == 429:
            self.rate_limited_requests += 1
        
        remaining = int(response.headers.get('X-RateLimit-Remaining', -1))
        if remaining >= 0:
            self.min_remaining = min(self.min_remaining, remaining)
    
    def get_stats(self):
        """Get rate limit statistics."""
        return {
            'total_requests': self.total_requests,
            'rate_limited': self.rate_limited_requests,
            'rate_limited_percent': (self.rate_limited_requests / self.total_requests * 100) 
                                   if self.total_requests > 0 else 0,
            'min_remaining': self.min_remaining
        }
```

### Server-Side Monitoring

Monitor rate limit metrics on server:

```bash
# Check rate limit statistics
curl -s http://localhost:8080/api/v1/metrics | jq '.rate_limiting'

# Example output:
{
  "total_rate_limited": 42,
  "rate_limited_by_client": {
    "client_123": 20,
    "client_456": 15,
    "client_789": 7
  },
  "current_active_clients": 15
}
```

---

## Requesting Limit Increases

### When to Request

Request a limit increase if:
- Consistently hitting limits during normal operations
- Business needs require higher throughput
- Implementing legitimate high-frequency monitoring
- Running batch processing jobs

### How to Request

```bash
# Submit rate limit increase request
galactus-cli support request-limit-increase \
  --client-id your-client-id \
  --current-limit 100 \
  --requested-limit 500 \
  --justification "Need higher throughput for real-time monitoring system"
```

Include in request:
- Current limit and tier
- Desired limit
- Business justification
- Expected request patterns
- Current usage statistics

---

## Rate Limit Testing

### Unit Tests

```python
def test_rate_limit_handling():
    """Test client handles rate limiting."""
    client = GalactusClient(api_url, api_key)
    
    # Mock rate limited response
    with mock.patch('requests.post') as mock_post:
        mock_post.return_value.status_code = 429
        mock_post.return_value.headers = {
            'Retry-After': '5',
            'X-RateLimit-Reset': str(int(time.time()) + 5)
        }
        
        # Should handle gracefully
        with pytest.raises(RateLimitError):
            client.compute_intent(test_data)
```

### Load Testing

```bash
# Test rate limiting behavior
galactus-cli test load \
  --endpoint /api/v1/intent \
  --requests-per-second 150 \
  --duration 60 \
  --expect-rate-limiting

# Verify rate limiting activates correctly
```

---

## Common Issues

### Issue: Frequent Rate Limiting

**Symptoms**: Regular 429 responses

**Solutions**:
- Implement client-side rate limiting
- Use batch endpoints
- Cache responses where appropriate
- Request limit increase if justified

### Issue: Burst Traffic

**Symptoms**: Rate limited during periodic batch jobs

**Solutions**:
- Spread processing over time
- Use batch endpoints efficiently
- Process during off-peak hours
- Request higher burst limit

### Issue: Unfair Rate Distribution

**Symptoms**: Some clients blocked while others have capacity

**Solutions**:
- Check for API key sharing
- Use separate keys for different services
- Implement proper load distribution
- Monitor per-client usage

---

## Enterprise Rate Limiting

### Custom Limits

Enterprise customers can configure custom limits:

```yaml
# Enterprise rate limit configuration
client_id: enterprise_customer_123
limits:
  requests_per_second: 500
  requests_per_minute: 25000
  requests_per_hour: 1000000
  burst: 1000
endpoints:
  /api/v1/intent:
    requests_per_second: 500
  /api/v1/intent/batch:
    requests_per_second: 50
priority: high  # Priority queuing during high load
```

### Dedicated Capacity

Enterprise tiers may include:
- Dedicated service instances
- Reserved capacity guarantees
- Priority request processing
- Custom rate limit policies
- SLA guarantees

---

## Final Statement

**Rate limits protect service quality for everyone.**  
**Respect limits - don't hammer the API.**  
**Cache and batch - reduce unnecessary calls.**
