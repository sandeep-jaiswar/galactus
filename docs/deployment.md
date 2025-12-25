# Deployment Guide

## Prerequisites

### System Requirements
- Docker 20.10+
- Docker Compose 2.0+
- 8GB RAM minimum
- 20GB disk space
- Linux/Windows/Mac

### For Kubernetes Deployment
- kubectl 1.24+
- Kubernetes cluster
- Docker registry access

## Local Development Deployment

### Quick Start
```bash
# Clone repository
git clone <repository-url>
cd datastore

# Start services
docker compose up -d

# Check status
docker compose ps

# Access Airflow
open http://localhost:8081
# Username: admin
# Password: admin
```

### Detailed Setup

1. **Environment Setup**
   ```bash
   # Ensure Docker is running
   docker --version
   docker compose version
   ```

2. **Build and Start**
   ```bash
   # Build images (optional, compose does this automatically)
   docker compose build

   # Start all services
   docker compose up -d

   # Follow logs
   docker compose logs -f
   ```

3. **Initialize Airflow**
   ```bash
   # Create admin user (if not done automatically)
   docker exec datastore-airflow-webserver-1 airflow users create \
     --username admin \
     --password admin \
     --firstname admin \
     --lastname admin \
     --role Admin \
     --email admin@example.com
   ```

4. **Verify Deployment**
   ```bash
   # Check all services are running
   docker compose ps

   # Test Airflow UI
   curl -I http://localhost:8081

   # Test Spark master
   curl -I http://localhost:7077
   ```

### Data Persistence
- PostgreSQL data: `./data/postgres/` (Docker volume)
- Spark warehouse: `./data/spark-warehouse/`
- Airflow logs: `./airflow/logs/`

## Production Deployment

### Docker Registry Setup

1. **Build Images**
   ```bash
   # Build Airflow image
   docker build -f docker/Dockerfile.airflow \
     -t my-registry/bhavcopy-airflow:latest .

   # Build Spark image
   docker build -f docker/Dockerfile.spark \
     -t my-registry/bhavcopy-spark:latest .
   ```

2. **Push Images**
   ```bash
   # Login to registry
   docker login my-registry

   # Push images
   docker push my-registry/bhavcopy-airflow:latest
   docker push my-registry/bhavcopy-spark:latest
   ```

### Kubernetes Deployment

1. **Prerequisites**
   ```bash
   # Install kubectl
   # Configure kubeconfig for your cluster

   # Verify connection
   kubectl cluster-info
   kubectl get nodes
   ```

2. **Update Manifests**
   ```bash
   # Update image references in k8s/*.yaml
   sed -i 's|bhavcopy-airflow:latest|my-registry/bhavcopy-airflow:latest|g' k8s/*.yaml
   sed -i 's|bhavcopy-spark:latest|my-registry/bhavcopy-spark:latest|g' k8s/*.yaml
   ```

3. **Deploy to Kubernetes**
   ```bash
   # Run deployment script
   ./scripts/deploy.sh

   # Or deploy manually
   kubectl apply -f k8s/namespace.yaml
   kubectl apply -f k8s/postgres.yaml
   kubectl apply -f k8s/spark.yaml
   kubectl apply -f k8s/airflow-configmap.yaml
   kubectl apply -f k8s/dags-configmap.yaml
   kubectl apply -f k8s/airflow-scheduler.yaml
   kubectl apply -f k8s/airflow-webserver.yaml
   ```

4. **Verify Deployment**
   ```bash
   # Check pods
   kubectl get pods -n bhavcopy-pipeline

   # Check services
   kubectl get svc -n bhavcopy-pipeline

   # Check logs
   kubectl logs -n bhavcopy-pipeline deployment/airflow-webserver
   ```

5. **Access Services**
   ```bash
   # Get LoadBalancer IP
   kubectl get svc airflow-webserver-service -n bhavcopy-pipeline

   # Port forward for local access
   kubectl port-forward -n bhavcopy-pipeline svc/airflow-webserver-service 8080:8080
   ```

### Persistent Storage

#### Local Development
```yaml
# docker-compose.yml
volumes:
  postgres_data:
  data:
```

#### Kubernetes
```yaml
# PVC for data
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: data-pvc
  namespace: bhavcopy-pipeline
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Gi
```

## Configuration Management

### Environment Variables

#### Development (.env)
```bash
AIRFLOW__CORE__FERNET_KEY=your-fernet-key
POSTGRES_PASSWORD=airflow
SPARK_MASTER_URL=spark://spark:7077
```

#### Production (Kubernetes Secrets)
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: airflow-secrets
  namespace: bhavcopy-pipeline
type: Opaque
data:
  fernet-key: <base64-encoded-key>
  postgres-password: <base64-encoded-password>
```

### Application Configuration

#### Airflow Config
```ini
# airflow/airflow.cfg
[core]
executor = KubernetesExecutor
dags_folder = /opt/airflow/dags

[database]
sql_alchemy_conn = postgresql+psycopg2://airflow:airflow@postgres-service:5432/airflow
```

#### Spark Config
```bash
# Environment variables in containers
SPARK_MASTER_HOST=spark-master
SPARK_MASTER_PORT=7077
SPARK_WORKER_CORES=2
SPARK_WORKER_MEMORY=4g
```

## Monitoring & Maintenance

### Health Checks

#### Docker Compose
```bash
# Check service health
docker compose ps
docker compose exec postgres pg_isready -U airflow
```

#### Kubernetes
```bash
# Check pod health
kubectl get pods -n bhavcopy-pipeline
kubectl describe pod <pod-name> -n bhavcopy-pipeline
```

### Logs

#### Development
```bash
# View all logs
docker compose logs -f

# View specific service
docker compose logs -f airflow-webserver
```

#### Production
```bash
# Pod logs
kubectl logs -n bhavcopy-pipeline deployment/airflow-webserver
kubectl logs -n bhavcopy-pipeline -l app=spark-master

# Previous logs
kubectl logs -n bhavcopy-pipeline --previous deployment/airflow-scheduler
```

### Backups

#### Database Backup
```bash
# PostgreSQL backup
docker exec datastore-postgres-1 pg_dump -U airflow airflow > backup.sql

# Kubernetes
kubectl exec -n bhavcopy-pipeline postgres-pod -- pg_dump -U airflow airflow > backup.sql
```

#### Data Backup
```bash
# Copy data directory
cp -r data/ backup/

# Kubernetes PVC backup
kubectl cp bhavcopy-pipeline/spark-pod:/opt/spark/data ./backup/
```

## Scaling

### Horizontal Scaling

#### Spark Workers
```bash
# Scale Spark workers
kubectl scale deployment spark-worker -n bhavcopy-pipeline --replicas=3
```

#### Airflow Workers (Future)
```yaml
# Additional worker deployment
apiVersion: apps/v1
kind: Deployment
metadata:
  name: airflow-worker
spec:
  replicas: 2
  # ... worker configuration
```

### Vertical Scaling

#### Resource Limits
```yaml
resources:
  requests:
    memory: "2Gi"
    cpu: "1000m"
  limits:
    memory: "4Gi"
    cpu: "2000m"
```

## Troubleshooting

### Common Issues

#### "ImagePullBackOff"
```bash
# Check image exists
docker pull my-registry/bhavcopy-airflow:latest

# Check registry credentials
kubectl create secret docker-registry regcred \
  --docker-server=my-registry \
  --docker-username=user \
  --docker-password=password
```

#### "Pending" Pods
```bash
# Check node resources
kubectl describe node

# Check PVC status
kubectl get pvc -n bhavcopy-pipeline
```

#### Database Connection Issues
```bash
# Test connection
kubectl exec -n bhavcopy-pipeline postgres-pod -- psql -U airflow -d airflow -c "SELECT 1"

# Check service
kubectl get svc postgres-service -n bhavcopy-pipeline
```

#### DAG Import Errors
```bash
# Check pod logs
kubectl logs -n bhavcopy-pipeline deployment/airflow-scheduler

# Verify ConfigMap
kubectl get configmap dags-config -n bhavcopy-pipeline -o yaml
```

### Performance Tuning

#### Airflow
```ini
# airflow.cfg
[core]
parallelism = 32
dag_concurrency = 16

[scheduler]
max_threads = 4
```

#### Spark
```bash
# Environment variables
SPARK_EXECUTOR_MEMORY=2g
SPARK_EXECUTOR_CORES=2
SPARK_DRIVER_MEMORY=1g
```

## Security

### Network Policies
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: airflow-netpol
  namespace: bhavcopy-pipeline
spec:
  podSelector:
    matchLabels:
      app: airflow-webserver
  policyTypes:
  - Ingress
  ingress:
  - from: []
    ports:
    - protocol: TCP
      port: 8080
```

### Secrets Management
```yaml
# Use external secret management
# - AWS Secrets Manager
# - HashiCorp Vault
# - Kubernetes secrets with encryption
```

### Updates & Rollbacks

#### Rolling Updates
```bash
# Update deployment
kubectl set image deployment/airflow-webserver \
  airflow-webserver=my-registry/bhavcopy-airflow:v2.0.0 \
  -n bhavcopy-pipeline

# Check rollout status
kubectl rollout status deployment/airflow-webserver -n bhavcopy-pipeline
```

#### Rollbacks
```bash
# Rollback deployment
kubectl rollout undo deployment/airflow-webserver -n bhavcopy-pipeline

# Rollback to specific revision
kubectl rollout undo deployment/airflow-webserver \
  --to-revision=2 \
  -n bhavcopy-pipeline
```

## Cost Optimization

### Resource Management
- Set appropriate resource limits
- Use spot instances for workers
- Implement auto-scaling
- Monitor resource utilization

### Storage Optimization
- Use appropriate storage classes
- Implement data lifecycle policies
- Compress old data
- Archive unused data