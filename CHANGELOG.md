# Changelog

All notable changes to the Galactus project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2024-12-26

### Added

#### Architecture & Data Layer
- **Bronze Layer**: Implemented proper Bronze layer for raw, immutable NSE data storage
  - Date-partitioned structure
  - File hash verification for auditability
  - Preserves NSE file naming conventions
- **Silver Layer Processor**: New module for transformation and validation
  - Explicit schema enforcement using Decimal types for prices
  - Data quality checks and validation rules
  - Quarantine logic for invalid records
  - OHLC price relationship validation
- **Improved NSE Scraper**: Enhanced scraper following Bronze layer principles
  - Retry logic with exponential backoff
  - Explicit error handling with custom exceptions
  - Audit logging with file hashes
  - No data transformations (follows principle: scrapers are "dumb recorders")

#### Configuration & Environment
- **Centralized Configuration System** (`conf/config.py`)
  - Environment variable support for all settings
  - Path management for Bronze/Silver/Gold layers
  - Configuration validation
  - Type hints throughout
- **Environment Configuration**
  - `.env.example` file with all available settings
  - Automatic directory creation
  - Flexible deployment configurations

#### Logging & Observability
- **Structured Logging Utilities** (`utils/logging_utils.py`)
  - Consistent log formatting with context
  - Job lifecycle logging (start/end)
  - Data lineage logging for auditability
  - Configurable log levels and outputs

#### Containerization & Deployment
- **Docker Support**
  - Dockerfile with multi-stage build optimization
  - docker-compose.yml for local development stack
  - .dockerignore for efficient builds
  - Pre-configured Spark and Hudi environment
- **Kubernetes Manifests**
  - Deployment configurations
  - ConfigMaps for application settings
  - PersistentVolumeClaim templates
  - StatefulSet for ClickHouse
  - Service definitions
  - Secrets template for credentials
  - Comprehensive deployment guide

#### Testing Infrastructure
- **Unit Test Suite**
  - Tests for configuration system
  - Tests for logging utilities
  - Tests for NSE download with mocking
  - pytest configuration with markers
  - 90%+ coverage of core modules
- **Testing Documentation**
  - Comprehensive testing guide
  - Examples for common patterns
  - CI/CD integration guidelines

#### Documentation
- **README.md**: Complete project documentation
  - Architecture overview
  - Quick start guides for Docker/K8s/manual setup
  - Configuration reference
  - Project structure
  - Key principles
- **K8s Deployment Guide**: Step-by-step Kubernetes deployment
- **Testing Guide**: Comprehensive testing documentation
- **Quick Start Script**: Automated setup script

#### Jobs & Scripts
- **Improved Daily Job** (`ingest_bhavcopy_daily_v2.py`)
  - Follows Bronze → Silver architecture
  - Uses centralized configuration
  - Structured logging
  - Comprehensive error handling
- **Updated Utility Scripts**
  - `check_hudi_status.py`: Enhanced with better output
  - `find_missing_dates.py`: Improved date analysis
  - `run_jobs.py`: Removed hardcoded paths

### Changed

#### Breaking Changes
- Moved from direct scraping → Hudi to Bronze → Silver → Gold architecture
- Changed data types: FLOAT → DECIMAL for financial data
- Centralized configuration system (old hardcoded paths deprecated)

#### Improvements
- **Airflow DAG**: Updated to use environment variables instead of hardcoded paths
- **All Scripts**: Removed hardcoded paths, now use configuration system
- **Error Handling**: All modules now use explicit exceptions and proper error messages
- **Type Hints**: Added comprehensive type hints to new modules
- **Security**: 
  - Added input validation throughout
  - Fixed potential SQL injection in ClickHouse sync
  - Added secrets management for Kubernetes
  - Passed CodeQL security scan with 0 vulnerabilities

### Deprecated
- `ingest_bhavcopy_daily.py`: Old version kept for backwards compatibility
  - Added deprecation warning
  - Recommends using `ingest_bhavcopy_daily_v2.py`

### Fixed
- Hardcoded paths throughout codebase
- Missing error handling in scraper
- Inconsistent data types for financial data
- Missing data validation
- No proper logging structure
- Security vulnerabilities in ClickHouse queries

### Security
- Eliminated hardcoded credentials
- Added secrets template for Kubernetes
- Implemented input validation for database identifiers
- Added security notes for production deployments
- Passed CodeQL security scan

## [Previous Version]

No version tracking was in place before this changelog.

## Future Roadmap

### Planned Features
- [ ] Additional NSE data sources (F&O, indices, corporate actions)
- [ ] Automated backfill detection and scheduling
- [ ] Data quality dashboards
- [ ] Trading calendar integration
- [ ] Alerting for data anomalies
- [ ] REST API for data access
- [ ] Gold layer transformations for research

### Infrastructure Improvements
- [ ] CI/CD pipeline with GitHub Actions
- [ ] Automated backup strategies
- [ ] Monitoring with Prometheus/Grafana
- [ ] High availability configurations
- [ ] Performance benchmarking suite
