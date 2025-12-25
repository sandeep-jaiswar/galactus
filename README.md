# Bhavcopy Data Pipeline

A containerized data pipeline for ingesting NSE bhavcopy data into Apache Hudi tables using Apache Airflow and Apache Spark.

## Quick Start

### Local Development
```bash
# Start services
docker compose up -d

# Access Airflow UI
open http://localhost:8081
# Username: admin
# Password: admin
```

### Production Deployment
```bash
# Build and deploy to Kubernetes
./scripts/deploy.sh
```

## Documentation

- **[Architecture](docs/architecture.md)** - System design and data flow
- **[Development](docs/development.md)** - Setup and development guide
- **[API](docs/api.md)** - Code documentation and interfaces
- **[Deployment](docs/deployment.md)** - Production deployment guide

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

## Features

- **Automated Data Ingestion**: Daily NSE bhavcopy data collection
- **Containerized**: Docker + Kubernetes deployment
- **Scalable Processing**: Apache Spark for distributed computing
- **Data Lakehouse**: Apache Hudi for efficient data management
- **Workflow Orchestration**: Apache Airflow for scheduling and monitoring
- **Production Ready**: PostgreSQL metadata store, persistent storage

## Technology Stack

- **Orchestration**: Apache Airflow
- **Processing**: Apache Spark + PySpark
- **Storage**: Apache Hudi + Parquet
- **Database**: PostgreSQL
- **Containerization**: Docker + Docker Compose
- **Orchestration**: Kubernetes
- **Language**: Python 3.8+

## Contributing

1. Read the [development guide](docs/development.md)
2. Follow coding standards
3. Add tests for new features
4. Update documentation
5. Create pull request

## License

[Add your license here]