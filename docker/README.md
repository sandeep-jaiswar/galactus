# Galactus Docker Setup

This directory contains Docker configuration for containerizing the complete Galactus system.

## Architecture

The Galactus system runs in multiple containers:

- **galactus-core**: Rust-based inference engine and APIs
- **galactus-research**: Python research environment (optional)
- **prometheus**: Metrics collection
- **grafana**: Monitoring dashboards
- **postgres**: Data persistence (future)

## Quick Start

```bash
# Build and run everything
docker-compose up --build

# Or run in background
docker-compose up -d --build

# View logs
docker-compose logs -f galactus-core

# Stop everything
docker-compose down
```

## Services

### galactus-core
- **Port**: 8080 (HTTP API), 9090 (gRPC)
- **Health**: http://localhost:8080/api/v1/health
- **Metrics**: http://localhost:8080/api/v1/metrics

### grafana
- **Port**: 3000
- **URL**: http://localhost:3000
- **Credentials**: admin/admin (change on first login)

### prometheus
- **Port**: 9090
- **URL**: http://localhost:9090

## Development

```bash
# Build specific service
docker-compose build galactus-core

# Run with hot reload (Rust)
docker-compose -f docker-compose.dev.yml up

# Run tests in container
docker-compose exec galactus-core cargo test

# Access container shell
docker-compose exec galactus-core bash
```

## Production Deployment

```bash
# Build production images
docker-compose -f docker-compose.prod.yml build

# Deploy
docker-compose -f docker-compose.prod.yml up -d

# Scale services
docker-compose up -d --scale galactus-core=3
```

## Configuration

Environment variables are managed through `.env` file:

```bash
# Copy example
cp .env.example .env

# Edit as needed
nano .env
```

## Monitoring

- **Grafana Dashboard**: Auto-imported from `monitoring/grafana_dashboard.json`
- **Prometheus Metrics**: Available at `/api/v1/metrics`
- **Health Checks**: Available at `/api/v1/health`
- **Logs**: Structured JSON logging to stdout</content>
<parameter name="filePath">/media/sandeep/DataDrive/galactus/docker/README.md