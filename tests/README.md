# Testing Guide

This guide explains how to run tests for the Galactus project.

## Setup

Install testing dependencies:

```bash
pip install -r requirements.txt
```

Or install only test dependencies:

```bash
pip install pytest pytest-cov pytest-mock
```

## Running Tests

### Run all tests

```bash
pytest
```

### Run with coverage

```bash
pytest --cov=conf --cov=utils --cov=jobs --cov-report=html --cov-report=term
```

View coverage report:

```bash
# Open in browser
open htmlcov/index.html  # macOS
xdg-open htmlcov/index.html  # Linux
start htmlcov/index.html  # Windows
```

### Run specific test files

```bash
pytest tests/test_config.py
pytest tests/test_logging.py
pytest tests/test_nse_download.py
```

### Run specific tests

```bash
# Run a specific test function
pytest tests/test_config.py::test_config_import

# Run tests matching a pattern
pytest -k "config"
```

### Run with verbose output

```bash
pytest -v
```

### Run with detailed output on failures

```bash
pytest -vv
```

## Test Organization

Tests are organized by module:

- `tests/test_config.py` - Configuration system tests
- `tests/test_logging.py` - Logging utilities tests
- `tests/test_nse_download.py` - NSE scraper tests
- `tests/test_silver_processor.py` - Silver layer processor tests (to be added)

## Test Markers

Tests can be marked with custom markers for selective execution:

```bash
# Run only unit tests
pytest -m unit

# Run only integration tests
pytest -m integration

# Run all except slow tests
pytest -m "not slow"

# Run tests that don't require network
pytest -m "not requires_network"
```

## Writing Tests

### Test Structure

```python
def test_feature_name():
    """Test description"""
    # Arrange - set up test data
    
    # Act - execute the code being tested
    
    # Assert - verify the results
    assert expected == actual
```

### Using Fixtures

```python
import pytest

@pytest.fixture
def sample_config():
    """Fixture that provides test configuration"""
    from conf.config import GalactusConfig
    return GalactusConfig()

def test_with_fixture(sample_config):
    """Test using fixture"""
    assert sample_config is not None
```

### Mocking External Dependencies

```python
from unittest.mock import Mock, patch

@patch('utils.nse_download.requests.get')
def test_download(mock_get):
    """Test with mocked HTTP request"""
    mock_response = Mock()
    mock_response.status_code = 200
    mock_response.content = b"test data"
    mock_get.return_value = mock_response
    
    # Your test code here
```

## Continuous Integration

Tests are run automatically on:
- Every commit
- Every pull request
- Scheduled daily runs

CI configuration is in `.github/workflows/` (to be added).

## Test Coverage Goals

- **Configuration**: >90% coverage
- **Utilities**: >80% coverage
- **Jobs**: >70% coverage
- **Overall**: >80% coverage

## Testing Best Practices

1. **Write tests first** (TDD) or immediately after implementing features
2. **Test edge cases** - null values, empty data, boundary conditions
3. **Mock external dependencies** - databases, APIs, file systems
4. **Keep tests fast** - use mocks to avoid slow operations
5. **Make tests independent** - each test should run in isolation
6. **Use descriptive names** - test names should explain what is being tested
7. **One assertion per test** - makes failures easier to diagnose

## Common Testing Patterns

### Testing with temporary files

```python
import tempfile
from pathlib import Path

def test_file_operation():
    with tempfile.TemporaryDirectory() as tmpdir:
        test_file = Path(tmpdir) / "test.txt"
        # Use test_file for testing
        assert test_file.parent.exists()
```

### Testing exceptions

```python
import pytest

def test_raises_exception():
    with pytest.raises(ValueError) as exc_info:
        # Code that should raise ValueError
        raise ValueError("test error")
    
    assert "test error" in str(exc_info.value)
```

### Parametrized tests

```python
import pytest

@pytest.mark.parametrize("input,expected", [
    ("2024-01-15", True),
    ("invalid", False),
    ("2024/01/15", False),
])
def test_date_validation(input, expected):
    result = validate_date(input)
    assert result == expected
```

## Troubleshooting

### Import errors

If you see `ModuleNotFoundError`, ensure:
1. You're in the project root directory
2. `PYTHONPATH` includes project root
3. Virtual environment is activated

### Spark tests failing

For tests requiring Spark:
1. Ensure Java is installed
2. Check Spark version compatibility
3. Consider marking as integration tests and skipping in CI

### Slow tests

Mark slow tests to skip during development:

```python
@pytest.mark.slow
def test_slow_operation():
    # Long-running test
    pass
```

Run without slow tests:

```bash
pytest -m "not slow"
```
