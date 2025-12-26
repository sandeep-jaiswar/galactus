# Galactus - NSE Market Intelligence Platform

A comprehensive, auditable, and research-grade market data platform for capturing and analyzing NSE (National Stock Exchange of India) data using open-source tools and lakehouse architecture.

## 🎯 Mission

Build a financial research system that prioritizes **correctness, auditability, and reproducibility** over speed.

## 🏗️ Architecture

Galactus follows a strict layered data architecture:

```
Scraping → Bronze (Raw) → Silver (Hudi) → Gold (Analytics/ClickHouse)
```

### Data Layers

- **Bronze Layer**: Immutable raw data exactly as published by NSE
  - No transformations, no cleaning
  - Date-partitioned
  - Preserves NSE file naming conventions
  - Audit trail with file hashes

- **Silver Layer**: Clean, validated, schema-enforced data in Apache Hudi
  - Explicit schemas with financial data types (Decimal for prices)
  - Data quality validation
  - Point-in-time correctness
  - System of record

- **Gold Layer**: Derived datasets and aggregations
  - Synced to ClickHouse for low-latency analytics
  - Feature tables for research
  - Backtest-ready datasets

## 🚀 Quick Start

### Using Docker Compose (Recommended for Development)

```bash
# Clone the repository
git clone https://github.com/sandeep-jaiswar/galactus.git
cd galactus

# Build and start services
docker-compose up -d

# Access the application container
docker-compose exec galactus bash

# Run a daily ingestion job
spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  --master local[*] \
  jobs/ingest_bhavcopy_daily.py \
  2024-01-15 \
  /app/data/silver
```

### Using Kubernetes (Recommended for Production)

```bash
# Create namespace and apply configurations
kubectl apply -f k8s/clickhouse.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/pvc.yaml
kubectl apply -f k8s/deployment.yaml

# Check pod status
kubectl get pods -n galactus

# Access application pod
kubectl exec -it -n galactus galactus-app-<pod-id> -- bash
```

### Manual Setup

#### Prerequisites

- Python 3.10+
- Java 17+
- Apache Spark 3.4+
- Apache Hudi 0.15.0+

#### Installation

```bash
# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Set environment variables
export GALACTUS_PROJECT_ROOT=$(pwd)
export GALACTUS_DATA_ROOT=$(pwd)/data
export HUDI_BASE_PATH=$(pwd)/data/silver
```

## 📊 Data Ingestion

### Daily Ingestion

Process today's NSE data:

```bash
spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  --master local[*] \
  jobs/ingest_bhavcopy_daily.py \
  $(date +%Y-%m-%d) \
  /path/to/hudi/base
```

### Historical Backfill

Backfill historical data:

```bash
spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  --master local[*] \
  jobs/ingest_bhavcopy_historical.py \
  2024-01-01 \
  2024-12-31 \
  /path/to/hudi/base
```

### ClickHouse Sync

Sync Silver (Hudi) data to ClickHouse:

```bash
python utils/hudi_clickhouse_sync.py \
  --table sec_bhavdata \
  --hudi-path /path/to/hudi/base
```

## 🔧 Configuration

Galactus uses environment variables for configuration. Key variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `GALACTUS_PROJECT_ROOT` | Project root directory | Auto-detected |
| `GALACTUS_DATA_ROOT` | Base path for all data | `{PROJECT_ROOT}/data` |
| `HUDI_BASE_PATH` | Hudi tables location | `{DATA_ROOT}/silver` |
| `GALACTUS_LOG_LEVEL` | Logging level | `INFO` |
| `CLICKHOUSE_HOST` | ClickHouse server host | `localhost` |
| `CLICKHOUSE_PORT` | ClickHouse server port | `8123` |
| `CLICKHOUSE_DATABASE` | ClickHouse database name | `galactus` |

See `conf/config.py` for complete configuration options.

## 📁 Project Structure

```
galactus/
├── conf/                   # Configuration files
│   ├── config.py          # Centralized configuration
│   └── hudi.py            # Hudi write options
├── jobs/                   # Data ingestion jobs
│   ├── ingest_bhavcopy_daily.py
│   └── ingest_bhavcopy_historical.py
├── utils/                  # Utility modules
│   ├── nse_download.py    # NSE data scraper (Bronze layer)
│   ├── silver_processor.py # Silver layer transformations
│   ├── hudi_clickhouse_sync.py
│   └── logging_utils.py   # Logging utilities
├── airflow/                # Apache Airflow DAGs
├── k8s/                    # Kubernetes manifests
├── data/                   # Data storage (gitignored)
│   ├── bronze/            # Raw NSE data
│   ├── silver/            # Hudi tables
│   └── gold/              # Derived datasets
├── Dockerfile             # Docker image definition
├── docker-compose.yml     # Local development setup
└── requirements.txt       # Python dependencies
```

## 🎓 Key Principles

1. **Correctness First**: Wrong data is worse than slow data
2. **Immutable Bronze**: Raw data is never modified
3. **Explicit Schemas**: No schema inference in production
4. **Financial Types**: Use Decimal for prices, never Float
5. **Fail Explicitly**: Silent failures are unacceptable
6. **Audit Everything**: Log lineage, hashes, and commit IDs
7. **Deterministic**: Same input always produces same output

## 🔒 Security

- No hardcoded credentials
- SQL injection prevention in ClickHouse queries
- Input validation on all user-provided data
- Secure file permissions on sensitive data

## 📝 Logging

Structured logging with context:

```python
from utils.logging_utils import setup_logger, log_job_start, log_job_end

logger = setup_logger(__name__)
log_job_start(logger, "bhavcopy_ingestion", date="2024-01-15")
# ... job logic ...
log_job_end(logger, "bhavcopy_ingestion", success=True, records=1000)
```

## 🧪 Testing

```bash
# Run unit tests (when available)
pytest tests/

# Validate Hudi table status
spark-submit check_hudi_status.py

# Find missing dates
spark-submit find_missing_dates.py
```

## 📚 Documentation

- [Copilot Instructions](.github/copilot-instructions.md) - Development guidelines
- [Airflow Setup](AIRFLOW_README.md) - Airflow configuration
- [Airflow Usage](AIRFLOW_USAGE.md) - Running scheduled jobs

## 🤝 Contributing

1. Follow the architectural principles in `.github/copilot-instructions.md`
2. Never bypass the Bronze layer
3. Always use explicit schemas
4. Add comprehensive logging
5. Write tests for new functionality

## 📄 License

[Add your license here]

## 🙏 Acknowledgments

Built with open-source tools:
- Apache Spark
- Apache Hudi
- ClickHouse
- Apache Airflow

## 📞 Contact

[Add contact information]
