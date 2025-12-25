# Development Guide

## Prerequisites

- Docker and Docker Compose
- Git
- Python 3.8+ (for local development)
- kubectl (for Kubernetes deployment)

## Local Development Setup

### 1. Clone and Setup
```bash
git clone <repository-url>
cd datastore
```

### 2. Start Services
```bash
# Start all services
docker compose up -d

# Check status
docker compose ps
```

### 3. Access Services
- **Airflow UI**: http://localhost:8081 (admin/admin)
- **Spark Master**: http://localhost:7077
- **PostgreSQL**: localhost:5433

### 4. Development Workflow

#### Code Changes
1. Edit files in `src/`, `config/`, or `airflow/dags/`
2. Restart services if needed:
   ```bash
   docker compose restart airflow-webserver airflow-scheduler
   ```

#### Testing DAGs
1. Access Airflow UI
2. Enable the DAG
3. Trigger manual run
4. Monitor logs

#### Testing Spark Jobs
```bash
# Submit job manually (from container)
docker exec -it datastore-spark-1 bash
cd /opt/spark
spark-submit --master spark://spark-master:7077 \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  /opt/spark/src/jobs/ingest_bhavcopy_daily.py 2025-12-25 /opt/spark/data/spark-warehouse
```

## Project Structure

```
├── src/                    # Source code
│   ├── jobs/              # Spark jobs
│   └── utils/             # Utility functions
├── config/                # Configuration files
├── data/                  # Persistent data
├── airflow/dags/          # Airflow DAGs
├── docker/                # Docker files
├── k8s/                   # Kubernetes manifests
├── scripts/               # Deployment scripts
├── docs/                  # Documentation
└── requirements.txt       # Python dependencies
```

## Code Quality

### Python Standards
- Follow PEP 8 style guide
- Use type hints
- Add docstrings to functions
- Keep functions small and focused

### Testing
```bash
# Run tests (when implemented)
pytest

# Check code quality
flake8 src/
black src/
```

### Pre-commit Hooks
```bash
# Install pre-commit
pip install pre-commit
pre-commit install

# Run on all files
pre-commit run --all-files
```

## Adding New Features

### 1. New DAG
1. Create DAG file in `airflow/dags/`
2. Follow existing patterns
3. Test locally
4. Update documentation

### 2. New Spark Job
1. Add job in `src/jobs/`
2. Update imports in DAG
3. Test with sample data
4. Add configuration if needed

### 3. New Utility
1. Add to `src/utils/`
2. Write tests
3. Update imports
4. Document usage

## Configuration Management

### Environment Variables
- Use `.env` files for local development
- Never commit secrets
- Use Kubernetes secrets for production

### Application Config
- Hudi settings in `config/hudi.py`
- Airflow config in `airflow/airflow.cfg`
- Docker config in `docker-compose.yml`

## Database Management

### Local Development
- PostgreSQL runs in Docker
- Data persists in `data/` directory
- Reset with: `docker compose down -v`

### Production
- Use managed PostgreSQL
- Configure connection strings
- Set up backups

## Debugging

### Airflow Issues
```bash
# Check scheduler logs
docker compose logs airflow-scheduler

# Check webserver logs
docker compose logs airflow-webserver

# Test DAG parsing
docker exec datastore-airflow-scheduler-1 python -c "from airflow import DAG; print('DAG syntax OK')"
```

### Spark Issues
```bash
# Check Spark master logs
docker compose logs spark

# Access Spark UI
open http://localhost:4040

# Debug job submission
docker exec datastore-spark-1 spark-submit --help
```

### Data Issues
```bash
# Check data directory
ls -la data/spark-warehouse/

# Query Hudi table (if metastore available)
# Use Spark SQL or Presto
```

## Deployment

### Docker Images
```bash
# Build images
docker build -f docker/Dockerfile.airflow -t my-registry/bhavcopy-airflow:latest .
docker build -f docker/Dockerfile.spark -t my-registry/bhavcopy-spark:latest .

# Push to registry
docker push my-registry/bhavcopy-airflow:latest
docker push my-registry/bhavcopy-spark:latest
```

### Kubernetes
```bash
# Update image references in k8s/*.yaml
# Deploy
./scripts/deploy.sh

# Check status
kubectl get pods -n bhavcopy-pipeline
```

## Monitoring & Logging

### Application Logs
- Airflow: Web UI and container logs
- Spark: Spark UI and driver logs
- Application: Structured logging in jobs

### Health Checks
```bash
# Service health
curl http://localhost:8081/health

# Database connectivity
docker exec datastore-postgres-1 pg_isready -U airflow
```

### Metrics
- Airflow metrics endpoint
- Spark metrics
- Custom application metrics (future)

## Contributing

1. Create feature branch
2. Make changes
3. Test thoroughly
4. Update documentation
5. Create pull request

### Commit Messages
```
feat: add new data validation
fix: resolve Spark job timeout
docs: update deployment guide
refactor: simplify DAG structure
```

## Troubleshooting Common Issues

### "Container already in use"
```bash
docker compose down
docker system prune
```

### "Port already in use"
```bash
# Find process
lsof -i :8081
# Kill process or change port
```

### "DAG import errors"
- Check PYTHONPATH
- Verify file syntax
- Check dependencies

### "Spark job fails"
- Check Spark master connectivity
- Verify Hudi package version
- Check data permissions

### "Database connection fails"
- Verify PostgreSQL is running
- Check connection string
- Test with psql client