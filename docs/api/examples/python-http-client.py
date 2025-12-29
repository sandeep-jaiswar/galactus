#!/usr/bin/env python3
"""
Galactus HTTP API Client Example
---------------------------------
Demonstrates integration with Galactus intent inference API using HTTP REST.

Requirements:
    pip install requests

Usage:
    python python-http-client.py
    python python-http-client.py --url http://localhost:8080 --api-key YOUR_KEY
"""

import os
import sys
import time
import argparse
import logging
from typing import Dict, List, Any, Optional
import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry


# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class GalactusClient:
    """Client for Galactus intent inference API."""
    
    def __init__(self, base_url: str, api_key: str, timeout: int = 10):
        """
        Initialize Galactus API client.
        
        Args:
            base_url: API base URL (e.g., http://localhost:8080)
            api_key: API authentication key
            timeout: Request timeout in seconds
        """
        self.base_url = base_url.rstrip('/')
        self.api_key = api_key
        self.timeout = timeout
        
        # Create session with connection pooling and retries
        self.session = requests.Session()
        retry_strategy = Retry(
            total=3,
            backoff_factor=1,
            status_forcelist=[429, 500, 502, 503, 504],
            allowed_methods=["GET", "POST"]
        )
        adapter = HTTPAdapter(max_retries=retry_strategy, pool_connections=10, pool_maxsize=20)
        self.session.mount("http://", adapter)
        self.session.mount("https://", adapter)
        
        # Set default headers
        self.session.headers.update({
            'X-API-Key': self.api_key,
            'Content-Type': 'application/json',
            'User-Agent': 'galactus-python-client/0.1.0'
        })
    
    def compute_intent(self, signals: List[Dict[str, Any]], 
                       client_id: str = "python_client",
                       context: Optional[Dict[str, str]] = None,
                       metadata: Optional[Dict[str, str]] = None) -> Dict[str, Any]:
        """
        Compute single intent vector from signals.
        
        Args:
            signals: List of signal inputs
            client_id: Client identifier
            context: Additional context data
            metadata: Request metadata
            
        Returns:
            Intent computation result
            
        Raises:
            requests.exceptions.HTTPError: On API errors
            requests.exceptions.Timeout: On timeout
            requests.exceptions.RequestException: On other request errors
        """
        url = f"{self.base_url}/api/v1/intent"
        
        payload = {
            'signals': signals,
            'client_id': client_id,
            'context': context or {},
            'metadata': metadata or {}
        }
        
        logger.debug(f"Sending request to {url}")
        start_time = time.time()
        
        try:
            response = self.session.post(url, json=payload, timeout=self.timeout)
            response.raise_for_status()
            
            elapsed_ms = (time.time() - start_time) * 1000
            logger.info(f"Intent computed successfully (took {elapsed_ms:.2f}ms)")
            
            return response.json()
            
        except requests.exceptions.Timeout:
            logger.error(f"Request timed out after {self.timeout} seconds")
            raise
        except requests.exceptions.HTTPError as e:
            logger.error(f"HTTP error {e.response.status_code}: {e.response.text}")
            raise
        except requests.exceptions.RequestException as e:
            logger.error(f"Request failed: {e}")
            raise
    
    def compute_batch(self, requests_list: List[Dict[str, Any]],
                      metadata: Optional[Dict[str, str]] = None) -> Dict[str, Any]:
        """
        Compute multiple intent vectors in batch.
        
        Args:
            requests_list: List of intent requests
            metadata: Batch metadata
            
        Returns:
            Batch computation results
        """
        url = f"{self.base_url}/api/v1/intent/batch"
        
        payload = {
            'requests': requests_list,
            'metadata': metadata or {}
        }
        
        logger.debug(f"Sending batch request with {len(requests_list)} items")
        start_time = time.time()
        
        try:
            response = self.session.post(url, json=payload, timeout=self.timeout * 2)
            response.raise_for_status()
            
            elapsed_ms = (time.time() - start_time) * 1000
            logger.info(f"Batch computed successfully (took {elapsed_ms:.2f}ms)")
            
            return response.json()
            
        except requests.exceptions.RequestException as e:
            logger.error(f"Batch request failed: {e}")
            raise
    
    def health_check(self) -> Dict[str, Any]:
        """
        Check API service health.
        
        Returns:
            Health status information
        """
        url = f"{self.base_url}/api/v1/health"
        
        try:
            response = self.session.get(url, timeout=5)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            logger.error(f"Health check failed: {e}")
            raise
    
    def get_metrics(self) -> Dict[str, Any]:
        """
        Get API service metrics.
        
        Returns:
            Service metrics
        """
        url = f"{self.base_url}/api/v1/metrics"
        
        try:
            response = self.session.get(url, timeout=5)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            logger.error(f"Metrics request failed: {e}")
            raise
    
    def close(self):
        """Close the HTTP session."""
        self.session.close()
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


def example_single_intent():
    """Example: Compute single intent vector."""
    print("\n=== Example: Single Intent Computation ===\n")
    
    # Get configuration from environment
    api_url = os.getenv('GALACTUS_API_URL', 'http://localhost:8080')
    api_key = os.getenv('GALACTUS_API_KEY', 'test-api-key')
    
    with GalactusClient(api_url, api_key) as client:
        # Check service health
        try:
            health = client.health_check()
            print(f"Service status: {health.get('status')}")
            print(f"Service version: {health.get('version')}")
        except requests.exceptions.RequestException:
            print("Warning: Health check failed, continuing anyway...")
        
        # Prepare signal inputs
        signals = [
            {
                'name': 'oi_decay',
                'value': -0.4,
                'confidence': 0.85,
                'timestamp': int(time.time()),
                'metadata': {}
            }
        ]
        
        # Compute intent
        try:
            result = client.compute_intent(signals, client_id='example_client')
            
            # Extract intent vector
            intent = result['result']['intent']
            print(f"\nIntent Vector:")
            print(f"  Pressure: {intent['pressure']:.4f}")
            print(f"  Confidence: {intent['confidence']:.4f}")
            print(f"  Regime: {intent.get('regime', 'unknown')}")
            print(f"  Timestamp: {intent['timestamp']}")
            
            # Show signal contributions
            print(f"\nSignal Contributions:")
            for signal_name, contribution in intent.get('signals', {}).items():
                print(f"  {signal_name}:")
                print(f"    Value: {contribution['value']:.4f}")
                print(f"    Weight: {contribution['weight']:.4f}")
                print(f"    Confidence: {contribution['confidence']:.4f}")
            
        except requests.exceptions.RequestException as e:
            print(f"Error computing intent: {e}")
            sys.exit(1)


def example_batch_computation():
    """Example: Batch intent computation."""
    print("\n=== Example: Batch Intent Computation ===\n")
    
    api_url = os.getenv('GALACTUS_API_URL', 'http://localhost:8080')
    api_key = os.getenv('GALACTUS_API_KEY', 'test-api-key')
    
    with GalactusClient(api_url, api_key) as client:
        # Prepare multiple requests
        requests_list = []
        for i in range(3):
            requests_list.append({
                'signals': [
                    {
                        'name': 'oi_decay',
                        'value': -0.3 - (i * 0.1),
                        'confidence': 0.8 + (i * 0.05),
                        'timestamp': int(time.time()),
                        'metadata': {}
                    }
                ],
                'client_id': f'batch_client_{i}'
            })
        
        try:
            result = client.compute_batch(requests_list)
            
            print(f"Batch computation completed:")
            print(f"  Requests processed: {len(result['responses'])}")
            
            for idx, response in enumerate(result['responses']):
                intent = response['result']['intent']
                print(f"\n  Request {idx + 1}:")
                print(f"    Pressure: {intent['pressure']:.4f}")
                print(f"    Confidence: {intent['confidence']:.4f}")
                
        except requests.exceptions.RequestException as e:
            print(f"Error in batch computation: {e}")
            sys.exit(1)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description='Galactus API Client Example')
    parser.add_argument('--url', default=os.getenv('GALACTUS_API_URL', 'http://localhost:8080'),
                       help='API base URL')
    parser.add_argument('--api-key', default=os.getenv('GALACTUS_API_KEY', 'test-api-key'),
                       help='API key')
    parser.add_argument('--debug', action='store_true',
                       help='Enable debug logging')
    args = parser.parse_args()
    
    if args.debug:
        logging.getLogger().setLevel(logging.DEBUG)
    
    # Set environment variables for examples
    os.environ['GALACTUS_API_URL'] = args.url
    os.environ['GALACTUS_API_KEY'] = args.api_key
    
    print("=" * 60)
    print("Galactus API Client Examples")
    print("=" * 60)
    
    try:
        # Run examples
        example_single_intent()
        example_batch_computation()
        
        print("\n" + "=" * 60)
        print("All examples completed successfully!")
        print("=" * 60 + "\n")
        
    except KeyboardInterrupt:
        print("\nExecution interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.exception(f"Unexpected error: {e}")
        sys.exit(1)


if __name__ == '__main__':
    main()
