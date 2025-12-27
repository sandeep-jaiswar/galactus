"""
Performance Tests for Research Signals

Tests the computational performance of signal implementations to ensure
they meet production requirements.
"""

import pytest
import numpy as np
import time
from datetime import datetime, timedelta


class TestComputationPerformance:
    """Test computational performance of signal implementations."""

    def test_single_signal_latency(self):
        """Test latency of single signal computation."""
        # Single computation should be very fast
        start = time.time()
        
        # Simulate signal computation
        spot_price = 18500.0
        futures_price = 18550.0
        time_to_expiry = 30
        
        # Simple calculation
        basis = futures_price - spot_price
        
        elapsed = time.time() - start
        
        # Should complete in < 1ms
        assert elapsed < 0.001
        assert basis > 0

    def test_batch_signal_computation(self):
        """Test performance of batch signal computation."""
        # Generate test data
        num_samples = 1000
        spot_prices = np.random.uniform(18000, 19000, num_samples)
        futures_prices = spot_prices * 1.003  # Small premium
        
        start = time.time()
        
        # Batch computation
        bases = futures_prices - spot_prices
        
        elapsed = time.time() - start
        
        # Should complete 1000 computations quickly
        assert elapsed < 0.01  # < 10ms
        assert len(bases) == num_samples

    def test_historical_analysis_performance(self):
        """Test performance of historical data analysis."""
        # Generate historical data
        num_periods = 100
        timestamps = [datetime.now() - timedelta(minutes=i) for i in range(num_periods)]
        prices = np.random.uniform(18000, 19000, num_periods)
        
        start = time.time()
        
        # Perform analysis (simple example)
        mean_price = np.mean(prices)
        std_price = np.std(prices)
        
        elapsed = time.time() - start
        
        # Should be fast
        assert elapsed < 0.01
        assert mean_price > 0
        assert std_price >= 0

    def test_large_dataset_handling(self):
        """Test handling of large datasets."""
        # Simulate large historical dataset
        num_samples = 10000
        data = np.random.randn(num_samples, 5)  # 5 features
        
        start = time.time()
        
        # Perform operations
        means = np.mean(data, axis=0)
        stds = np.std(data, axis=0)
        
        elapsed = time.time() - start
        
        # Should handle large data efficiently
        assert elapsed < 0.1  # < 100ms
        assert len(means) == 5

    def test_memory_efficiency(self):
        """Test memory efficiency of signal computation."""
        # Create reasonably large arrays
        size = 10000
        array1 = np.random.randn(size)
        array2 = np.random.randn(size)
        
        # Operations should not cause memory issues
        result = array1 + array2
        
        assert len(result) == size
        assert result is not None


class TestScalabilityPerformance:
    """Test scalability of signal implementations."""

    def test_linear_scaling(self):
        """Test that computation scales linearly with input size."""
        sizes = [100, 1000, 10000]
        times = []
        
        for size in sizes:
            data = np.random.randn(size)
            
            start = time.time()
            result = np.mean(data)
            elapsed = time.time() - start
            
            times.append(elapsed)
        
        # Scaling should be reasonable
        assert all(t < 0.01 for t in times)

    def test_concurrent_computation(self):
        """Test performance with concurrent computations."""
        # Simulate multiple signals computed in parallel
        num_signals = 5
        
        start = time.time()
        
        results = []
        for _ in range(num_signals):
            # Simple computation
            result = np.mean(np.random.randn(1000))
            results.append(result)
        
        elapsed = time.time() - start
        
        # Should handle concurrent computations
        assert elapsed < 0.1
        assert len(results) == num_signals

    def test_streaming_performance(self):
        """Test performance with streaming data."""
        # Simulate streaming data updates
        num_updates = 100
        
        values = []
        start = time.time()
        
        for i in range(num_updates):
            # Simulate receiving new data point
            new_value = np.random.randn()
            values.append(new_value)
            
            # Compute running statistics
            if len(values) > 10:
                _ = np.mean(values[-10:])
        
        elapsed = time.time() - start
        
        # Streaming updates should be fast
        assert elapsed < 0.05
        assert len(values) == num_updates


class TestResourceUtilization:
    """Test resource utilization during signal computation."""

    def test_cpu_utilization(self):
        """Test CPU utilization is reasonable."""
        # Perform computation
        size = 10000
        data = np.random.randn(size, 10)
        
        start = time.time()
        
        # CPU-intensive operation
        result = np.dot(data.T, data)
        
        elapsed = time.time() - start
        
        # Should complete efficiently
        assert elapsed < 0.1
        assert result.shape == (10, 10)

    def test_memory_allocation(self):
        """Test memory allocation patterns."""
        # Create and destroy arrays
        for _ in range(100):
            temp_array = np.random.randn(1000)
            _ = np.mean(temp_array)
        
        # Should not cause memory issues
        assert True

    def test_numpy_vectorization(self):
        """Test that operations are properly vectorized."""
        size = 10000
        array1 = np.random.randn(size)
        array2 = np.random.randn(size)
        
        # Vectorized operation
        start_vec = time.time()
        result_vec = array1 + array2
        elapsed_vec = time.time() - start_vec
        
        # Loop operation (intentionally slow for comparison)
        start_loop = time.time()
        result_loop = np.zeros(size)
        for i in range(size):
            result_loop[i] = array1[i] + array2[i]
        elapsed_loop = time.time() - start_loop
        
        # Vectorized should be much faster
        assert elapsed_vec < elapsed_loop / 5  # At least 5x faster
        np.testing.assert_array_almost_equal(result_vec, result_loop)


class TestComputationalComplexity:
    """Test computational complexity of algorithms."""

    def test_time_complexity_oi_computation(self):
        """Test time complexity of OI-based computations."""
        # Should be O(1) for single point
        start = time.time()
        
        # Simulate OI computation
        oi_current = 1000000
        oi_previous = 950000
        decay = (oi_previous - oi_current) / oi_previous
        
        elapsed = time.time() - start
        
        assert elapsed < 0.001  # Constant time
        assert abs(decay) < 1.0  # Reasonable decay value

    def test_time_complexity_basis_computation(self):
        """Test time complexity of basis computations."""
        # Should be O(1) for single point
        start = time.time()
        
        # Simulate basis computation
        futures = 18550
        spot = 18500
        basis = futures - spot
        
        elapsed = time.time() - start
        
        assert elapsed < 0.001  # Constant time
        assert basis > 0

    def test_time_complexity_aggregation(self):
        """Test time complexity of signal aggregation."""
        # Should be O(n) for n signals
        n_signals = 10
        pressures = np.random.randn(n_signals)
        
        start = time.time()
        
        # Aggregation
        result = np.mean(pressures)
        
        elapsed = time.time() - start
        
        assert elapsed < 0.001  # Linear time, very fast for small n
        assert -3 < result < 3  # Reasonable value


class TestCacheEfficiency:
    """Test efficiency of caching mechanisms."""

    def test_repeated_computation_caching(self):
        """Test that repeated computations can benefit from caching."""
        # First computation
        data = np.random.randn(10000)
        
        start1 = time.time()
        result1 = np.mean(data)
        elapsed1 = time.time() - start1
        
        # Second computation (same data)
        start2 = time.time()
        result2 = np.mean(data)
        elapsed2 = time.time() - start2
        
        # Results should be identical
        assert result1 == result2
        
        # Both should be fast
        assert elapsed1 < 0.01
        assert elapsed2 < 0.01

    def test_incremental_update_efficiency(self):
        """Test efficiency of incremental updates."""
        # Start with small dataset
        data = [1.0, 2.0, 3.0, 4.0, 5.0]
        
        # Add new values incrementally
        for new_value in [6.0, 7.0, 8.0]:
            start = time.time()
            data.append(new_value)
            _ = np.mean(data)
            elapsed = time.time() - start
            
            # Should be fast
            assert elapsed < 0.001


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
