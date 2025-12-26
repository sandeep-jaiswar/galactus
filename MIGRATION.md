# Migration Guide

This guide helps you migrate from the old Galactus codebase to the improved version.

## Overview

The new architecture introduces several important changes:

1. **Bronze → Silver → Gold layers** (was: direct scraping to Hudi)
2. **Centralized configuration** (was: hardcoded paths)
3. **Type-safe financial data** (was: Float → now: Decimal)
4. **Structured logging** (was: basic print statements)
5. **Docker/Kubernetes support** (new)

## Quick Migration Path

### For New Installations

If starting fresh, simply follow the README.md quickstart guide. No migration needed!

### For Existing Installations

Follow these steps to migrate:

#### 1. Backup Your Data

```bash
# Backup existing Hudi data
cp -r /path/to/old/hudi/data /path/to/backup/

# Backup logs
cp -r logs /path/to/backup/logs
```

#### 2. Update Your Repository

```bash
git pull origin main
```

#### 3. Set Up Configuration

```bash
# Copy environment template
cp .env.example .env

# Edit with your settings
nano .env
```

Set these key variables:
```bash
GALACTUS_PROJECT_ROOT=/path/to/galactus
GALACTUS_DATA_ROOT=/path/to/galactus/data
HUDI_BASE_PATH=/path/to/galactus/data/silver
```

#### 4. Update Dependencies

```bash
# Activate your virtual environment
source .venv/bin/activate

# Update packages
pip install -r requirements.txt
```

#### 5. Migrate Data (Optional)

If you want to adopt the new Bronze layer structure:

```bash
# Option A: Keep existing Silver data, add Bronze going forward
# Your existing Hudi tables will continue to work
# New downloads will populate Bronze layer

# Option B: Migrate to full Bronze/Silver architecture
# Re-download and reprocess historical data
# This ensures full Bronze layer coverage
```

For Option B, see "Historical Data Migration" below.

## Breaking Changes

### 1. Path Structure

**Old:**
```
/tmp/bhavcopy/           # Downloaded files
/path/to/hudi/           # Hudi tables directly
```

**New:**
```
data/
├── bronze/              # Raw NSE data (immutable)
│   └── bhavcopy/
│       └── date=2024-01-15/
├── silver/              # Hudi tables (canonical)
│   └── sec_bhavdata/
└── gold/                # Derived datasets
```

### 2. Job Scripts

**Old:**
```bash
spark-submit jobs/ingest_bhavcopy_daily.py 2024-01-15 /path/to/hudi
```

**New (recommended):**
```bash
spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  jobs/ingest_bhavcopy_daily_v2.py \
  2024-01-15 \
  /path/to/hudi
```

**Old script still works** but is deprecated. You'll see a warning message.

### 3. Configuration

**Old:**
Hardcoded paths in each script:
```python
hudi_path = "/media/sandeep/DataDrive/galactus/spark-warehouse"
```

**New:**
Environment variables or config:
```python
from conf.config import config
hudi_path = config.HUDI_BASE_PATH
```

### 4. Data Types

**Old:**
```python
# Prices as Float
OPEN_PRICE: float
```

**New:**
```python
# Prices as Decimal (for precision)
OPEN_PRICE: Decimal(18, 2)
```

**Impact:** Existing Hudi tables with Float types will continue to work. New data will use Decimal.

## Migration Scenarios

### Scenario 1: Keep Everything As-Is

**Goal:** Minimal changes, keep using old scripts

**Steps:**
1. Pull latest code
2. Continue using old job scripts
3. Gradually adopt new features

**Pros:** No disruption
**Cons:** Miss out on improvements

### Scenario 2: Partial Migration

**Goal:** Use new config and scripts, keep existing data

**Steps:**
1. Set up `.env` file
2. Point `HUDI_BASE_PATH` to existing Hudi location
3. Switch to new job scripts (`ingest_bhavcopy_daily_v2.py`)
4. New data goes to Bronze, existing data unchanged

**Pros:** Get new features, no data migration
**Cons:** Mixed architecture (no Bronze for old data)

### Scenario 3: Full Migration

**Goal:** Adopt complete Bronze/Silver/Gold architecture

**Steps:**
1. Set up new directory structure
2. Configure `.env` with new paths
3. Reprocess historical data with new scripts
4. Retire old Hudi location

**Pros:** Full architectural benefits
**Cons:** Requires reprocessing data

## Historical Data Migration

To fully adopt Bronze/Silver architecture for historical data:

### Step 1: Configure New Paths

```bash
# .env
GALACTUS_DATA_ROOT=/new/data/location
HUDI_BASE_PATH=/new/data/location/silver
```

### Step 2: Backfill Historical Data

```bash
# This will download to Bronze and process to Silver
spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  jobs/ingest_bhavcopy_historical.py \
  2024-01-01 \
  2024-12-31 \
  $HUDI_BASE_PATH
```

### Step 3: Validate Migration

```bash
# Check new Hudi table
python check_hudi_status.py

# Compare record counts with old table
# Should match (or be close, accounting for weekends/holidays)
```

### Step 4: Update Schedulers

Update any scheduled jobs (cron, systemd timers, etc.) to use new scripts and paths.

Example cron job:
```bash
# Daily ingestion at 6 PM
0 18 * * * cd /path/to/galactus && ./run_jobs.py $(date +\%Y-\%m-\%d)
```

### Step 5: Retire Old Data (Optional)

Once validated:
```bash
# Archive old data
tar -czf old_hudi_backup.tar.gz /old/hudi/path

# Remove after verification period
rm -rf /old/hudi/path
```

## Scheduling & Automation

For scheduling jobs, you can use:

**Option 1: Cron Jobs (Linux/Mac)**
```bash
# Edit crontab
crontab -e

# Add daily job at 6 PM
0 18 * * * cd /path/to/galactus && source .venv/bin/activate && ./run_jobs.py $(date +\%Y-\%m-\%d) >> /path/to/galactus/logs/cron.log 2>&1
```

**Option 2: systemd Timers (Linux)**
Create a service file and timer for more robust scheduling.

**Option 3: Kubernetes CronJob**
See `k8s/README.md` for Kubernetes-based scheduling examples.

## Docker/Kubernetes Migration

### Moving to Docker

If currently running on bare metal, migrate to Docker:

```bash
# Build image
docker build -t galactus:v1 .

# Run with docker-compose
docker-compose up -d

# Migrate data to Docker volumes
docker cp /old/data/path galactus-app:/app/data/
```

### Moving to Kubernetes

```bash
# Create namespace
kubectl create namespace galactus

# Apply configurations
kubectl apply -f k8s/

# Migrate data to PVC
kubectl cp /old/data/path galactus/galactus-app-xxx:/app/data/
```

See `k8s/README.md` for detailed deployment guide.

## Rollback Plan

If you need to rollback:

### Rollback Configuration

```bash
# Restore old environment
git checkout <previous-commit>

# Use old paths
export PYTHONPATH=/old/path
```

### Rollback Data

```bash
# Restore from backup
cp -r /path/to/backup/hudi/* /original/hudi/path/
```

### Rollback Schedulers

Remove any cron jobs or systemd timers you configured.

## Testing After Migration

Run these checks after migration:

```bash
# 1. Test configuration
python -c "from conf.config import config; print(config.HUDI_BASE_PATH)"

# 2. Run tests
pytest

# 3. Check Hudi table
python check_hudi_status.py

# 4. Test a single day ingestion
spark-submit jobs/ingest_bhavcopy_daily_v2.py $(date +%Y-%m-%d)

# 5. Verify Bronze layer
ls -la data/bronze/bhavcopy/

# 6. Verify Silver layer (Hudi)
ls -la $HUDI_BASE_PATH/sec_bhavdata/
```

## Getting Help

If you encounter issues during migration:

1. Check `CHANGELOG.md` for breaking changes
2. Review logs in `logs/` directory
3. Run with `GALACTUS_LOG_LEVEL=DEBUG` for detailed output
4. Open an issue on GitHub with:
   - Migration scenario attempted
   - Error messages
   - Log snippets
   - Environment details

## Best Practices Post-Migration

1. **Monitor Initial Runs**: Watch first few job executions closely
2. **Validate Data**: Compare record counts and data quality
3. **Gradual Adoption**: Don't migrate everything at once
4. **Keep Backups**: Maintain backups until confident
5. **Update Documentation**: Document your specific configuration
6. **Train Team**: Ensure team understands new architecture

## Timeline Recommendation

- **Week 1**: Set up new environment, run in parallel
- **Week 2**: Switch to new scripts, validate outputs
- **Week 3**: Set up scheduling (cron/K8s CronJob)
- **Week 4**: Full cutover, retire old system
- **Week 5+**: Monitor and optimize
