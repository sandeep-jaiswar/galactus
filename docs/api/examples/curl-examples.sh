#!/bin/bash

# Galactus API cURL Examples
# ---------------------------
# Demonstrates various API operations using cURL

set -e

# Configuration
API_URL="${GALACTUS_API_URL:-http://localhost:8080}"
API_KEY="${GALACTUS_API_KEY:-test-api-key}"

echo "=========================================="
echo "Galactus API cURL Examples"
echo "=========================================="
echo "API URL: $API_URL"
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Helper function to print section headers
section() {
    echo ""
    echo -e "${BLUE}=== $1 ===${NC}"
    echo ""
}

# Helper function to print success
success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Helper function to print error
error() {
    echo -e "${RED}✗ $1${NC}"
}

# 1. Health Check
section "1. Health Check"
echo "GET $API_URL/api/v1/health"
echo ""

curl -s -X GET "$API_URL/api/v1/health" | jq '.'
success "Health check completed"

# 2. Service Metrics
section "2. Service Metrics"
echo "GET $API_URL/api/v1/metrics"
echo ""

curl -s -X GET "$API_URL/api/v1/metrics" \
  -H "X-API-Key: $API_KEY" | jq '.'
success "Metrics retrieved"

# 3. Single Intent Computation
section "3. Single Intent Computation"
echo "POST $API_URL/api/v1/intent"
echo ""

cat > /tmp/intent_request.json <<EOF
{
  "signals": [
    {
      "name": "oi_decay",
      "value": -0.4,
      "confidence": 0.85,
      "timestamp": $(date +%s),
      "metadata": {}
    }
  ],
  "client_id": "curl_client",
  "context": {},
  "metadata": {}
}
EOF

echo "Request payload:"
cat /tmp/intent_request.json | jq '.'
echo ""

curl -s -X POST "$API_URL/api/v1/intent" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d @/tmp/intent_request.json | jq '.'
success "Intent computed"

# 4. Batch Intent Computation
section "4. Batch Intent Computation"
echo "POST $API_URL/api/v1/intent/batch"
echo ""

cat > /tmp/batch_request.json <<EOF
{
  "requests": [
    {
      "signals": [
        {
          "name": "oi_decay",
          "value": -0.3,
          "confidence": 0.80,
          "timestamp": $(date +%s),
          "metadata": {}
        }
      ],
      "client_id": "batch_client_1"
    },
    {
      "signals": [
        {
          "name": "oi_decay",
          "value": -0.5,
          "confidence": 0.90,
          "timestamp": $(date +%s),
          "metadata": {}
        }
      ],
      "client_id": "batch_client_2"
    }
  ],
  "metadata": {
    "batch_id": "batch_001"
  }
}
EOF

echo "Batch request payload:"
cat /tmp/batch_request.json | jq '.'
echo ""

curl -s -X POST "$API_URL/api/v1/intent/batch" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d @/tmp/batch_request.json | jq '.'
success "Batch computed"

# 5. Error Handling - Missing API Key
section "5. Error Handling - Missing API Key"
echo "POST $API_URL/api/v1/intent (without API key)"
echo ""

HTTP_CODE=$(curl -s -o /tmp/error_response.json -w "%{http_code}" \
  -X POST "$API_URL/api/v1/intent" \
  -H "Content-Type: application/json" \
  -d @/tmp/intent_request.json)

echo "HTTP Status: $HTTP_CODE"
cat /tmp/error_response.json | jq '.'
success "Error handling demonstrated"

# 6. Error Handling - Invalid Signal Value
section "6. Error Handling - Invalid Signal Value"
echo "POST $API_URL/api/v1/intent (with invalid value)"
echo ""

cat > /tmp/invalid_request.json <<EOF
{
  "signals": [
    {
      "name": "oi_decay",
      "value": 1.5,
      "confidence": 0.85,
      "timestamp": $(date +%s),
      "metadata": {}
    }
  ],
  "client_id": "curl_client"
}
EOF

HTTP_CODE=$(curl -s -o /tmp/error_response.json -w "%{http_code}" \
  -X POST "$API_URL/api/v1/intent" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d @/tmp/invalid_request.json)

echo "HTTP Status: $HTTP_CODE"
cat /tmp/error_response.json | jq '.'
success "Validation error demonstrated"

# 7. Rate Limit Headers
section "7. Rate Limit Headers"
echo "Checking rate limit headers"
echo ""

curl -v -X POST "$API_URL/api/v1/intent" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d @/tmp/intent_request.json \
  2>&1 | grep -i "x-ratelimit" || echo "Rate limit headers not present"
success "Rate limit check completed"

# 8. Multiple Signals
section "8. Multiple Signals"
echo "POST $API_URL/api/v1/intent (with multiple signals)"
echo ""

cat > /tmp/multi_signal_request.json <<EOF
{
  "signals": [
    {
      "name": "oi_decay",
      "value": -0.4,
      "confidence": 0.85,
      "timestamp": $(date +%s),
      "metadata": {}
    },
    {
      "name": "hedge_pressure",
      "value": -0.3,
      "confidence": 0.75,
      "timestamp": $(date +%s),
      "metadata": {}
    }
  ],
  "client_id": "multi_signal_client"
}
EOF

echo "Request with multiple signals:"
cat /tmp/multi_signal_request.json | jq '.'
echo ""

curl -s -X POST "$API_URL/api/v1/intent" \
  -H "X-API-Key: $API_KEY" \
  -H "Content-Type: application/json" \
  -d @/tmp/multi_signal_request.json | jq '.'
success "Multiple signals processed"

# Cleanup
rm -f /tmp/intent_request.json /tmp/batch_request.json /tmp/invalid_request.json
rm -f /tmp/multi_signal_request.json /tmp/error_response.json

echo ""
echo "=========================================="
echo "All examples completed!"
echo "=========================================="
echo ""
