# Bhavcopy Data Pipeline Documentation

## Overview

This project implements a containerized data pipeline for ingesting NSE (National Stock Exchange) bhavcopy data into Apache Hudi tables using Apache Airflow and Apache Spark.

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   NSE Website   │───▶│   Airflow DAG   │───▶│   Spark Job     │───▶│   Apache Hudi   │
│                 │    │                 │    │                 │    │   Tables        │
│  CSV Downloads  │    └─────────────────┘    │ Hudi Ingestion  │    └─────────────────┘
└─────────────────┘                           │                 │             │
                                              │ Apache Hudi     │             ▼
                                              │ Tables          │    ┌─────────────────┐
                                              └─────────────────┘    │ ClickHouse DB   │
                                                       │             │                 │
                                                       ▼             │ Analytics       │
                                              ┌─────────────────┐    │ Queries         │
                                              │   Data Lake     │    └─────────────────┘
                                              │                 │
                                              │ Parquet Files   │
                                              └─────────────────┘
```

## Components

### Data Source
- **NSE Bhavcopy**: Daily trading data from National Stock Exchange of India
- **Format**: CSV files containing trade data (symbol, date, price, volume, etc.)
- **Source**: https://nsearchives.nseindia.com/products/content/sec_bhavdata_full_*.csv

### Orchestration
- **Apache Airflow**: Workflow orchestration and scheduling
- **DAG**: `bhavcopy_ingestion` - runs daily at midnight
- **Tasks**: Single task that downloads data and processes it via Spark

### Processing
- **Apache Spark**: Distributed data processing
- **Libraries**: PySpark, Hudi-Spark integration
- **Operations**: Data cleaning, schema validation, Hudi table updates

### Storage
- **Apache Hudi**: Data lakehouse format for incremental updates
- **ClickHouse**: Analytical database for fast queries and analytics
- **Table Type**: Copy-on-Write (COW) for Hudi, ReplacingMergeTree for ClickHouse
- **Keys**: Record key (symbol + date), Precombine key (date)
- **Partitioning**: By trade_date in both systems

### Infrastructure
- **Docker**: Containerization for all services
- **PostgreSQL**: Airflow metadata database
- **Kubernetes**: Production deployment (optional)

## Data Flow

1. **Daily Schedule**: Airflow DAG triggers at midnight
2. **Data Download**: Fetch latest bhavcopy CSV from NSE
3. **Data Processing**:
   - Parse CSV with proper schema
   - Clean column names
   - Add Hudi-specific columns (record_key, trade_date)
4. **Hudi Ingestion**: Upsert data into Hudi table
5. **ClickHouse Sync**: Sync updated data from Hudi to ClickHouse
6. **Storage**: Save as partitioned Parquet files in Hudi and analytical format in ClickHouse

## Data Schema

### Source CSV Schema
```
SYMBOL, SERIES, DATE1, PREV_CLOSE, OPEN_PRICE, HIGH_PRICE, LOW_PRICE, LAST_PRICE, CLOSE_PRICE, AVG_PRICE, TTL_TRD_QNTY, TURNOVER_LACS, NO_OF_TRDS, DELIV_QTY, DELIV_PER
```

### Processed Schema
```
SYMBOL, SERIES, DATE1, PREV_CLOSE, OPEN_PRICE, HIGH_PRICE, LOW_PRICE, LAST_PRICE, CLOSE_PRICE, AVG_PRICE, TTL_TRD_QNTY, TURNOVER_LACS, NO_OF_TRDS, DELIV_QTY, DELIV_PER, record_key, trade_date
```

## DAGs

- `bhavcopy_ingestion`: Daily ingestion of NSE bhavcopy data
  - **ingest_bhavcopy**: Downloads and processes CSV into Hudi
  - **sync_to_clickhouse**: Syncs data from Hudi to ClickHouse for analytics

## Configuration

### Hudi Configuration
- **Table Name**: `sec_bhavdata`
- **Table Type**: COPY_ON_WRITE
- **Operation**: UPSERT
- **Record Key**: `record_key` (SYMBOL + DATE1)
- **Precombine Key**: `trade_date`
- **Partition Key**: `trade_date`

### Airflow Configuration
- **Schedule**: `@daily` (midnight)
- **Retries**: 1
- **Retry Delay**: 5 minutes
- **Timeout**: None

## Deployment

### Development
```bash
docker compose up -d
```

### Production
```bash
./scripts/deploy.sh
```

## Monitoring

### Airflow UI
- DAG status and logs
- Task execution history
- Trigger manual runs

### Spark UI
- Job execution details
- Resource utilization
- Task metrics

### Data Validation
- Row counts per partition
- Data quality checks
- Schema evolution tracking

## Troubleshooting

### Common Issues

1. **DAG not running**: Check Airflow scheduler logs
2. **Spark job failures**: Check Spark UI for error details
3. **Data download failures**: Verify NSE website availability
4. **Hudi table issues**: Check metastore connectivity

### Logs Location
- Airflow: `airflow/logs/`
- Spark: Container logs
- Application: `data/` directory

## Performance Considerations

- **Partitioning**: Daily partitions for efficient querying
- **File Size**: Hudi manages file sizes automatically
- **Indexing**: Hudi metadata table for fast lookups
- **Caching**: Spark caching for iterative operations

## Security

- Containerized environment
- Network isolation
- No sensitive data in logs
- Secure API access to NSE

## Future Enhancements

- Real-time data ingestion
- Multiple data sources
- Advanced analytics
- Data quality monitoring
- Alerting system