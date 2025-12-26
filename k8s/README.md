# Kubernetes Deployment Guide

This guide explains how to deploy Galactus on Kubernetes.

## Prerequisites

- Kubernetes cluster (1.20+)
- kubectl configured
- Storage provisioner for PersistentVolumes

## Quick Deploy

```bash
# Apply all manifests in order
kubectl apply -f k8s/clickhouse.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/pvc.yaml
kubectl apply -f k8s/deployment.yaml
```

## Step-by-Step Deployment

### 1. Create Namespace and ClickHouse

```bash
kubectl apply -f k8s/clickhouse.yaml
```

This creates:
- `galactus` namespace
- ClickHouse StatefulSet with persistent storage
- ClickHouse Service

Verify:
```bash
kubectl get pods -n galactus
kubectl logs -n galactus clickhouse-0
```

### 2. Configure Application

```bash
kubectl apply -f k8s/configmap.yaml
```

Edit the ConfigMap to customize:
- Log levels
- Spark configuration
- ClickHouse connection settings

### 3. Create Storage

```bash
kubectl apply -f k8s/pvc.yaml
```

This creates PersistentVolumeClaims for:
- Data storage (100GB)
- Log storage (10GB)

Adjust sizes in `k8s/pvc.yaml` based on your needs.

### 4. Deploy Application

```bash
kubectl apply -f k8s/deployment.yaml
```

This creates:
- Galactus Deployment
- Galactus Service (ClusterIP)

## Verify Deployment

```bash
# Check all resources
kubectl get all -n galactus

# Check pod logs
kubectl logs -n galactus deployment/galactus-app

# Access application shell
kubectl exec -it -n galactus deployment/galactus-app -- bash
```

## Running Jobs

### Daily Ingestion

```bash
kubectl exec -it -n galactus deployment/galactus-app -- \
  spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  --master local[*] \
  jobs/ingest_bhavcopy_daily_v2.py \
  $(date +%Y-%m-%d)
```

### Historical Backfill

```bash
kubectl exec -it -n galactus deployment/galactus-app -- \
  spark-submit \
  --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \
  --master local[*] \
  jobs/ingest_bhavcopy_historical.py \
  2024-01-01 \
  2024-12-31 \
  /app/data/silver
```

## Access ClickHouse

### Port Forward

```bash
kubectl port-forward -n galactus svc/clickhouse-service 8123:8123
```

Then access at http://localhost:8123

### From within cluster

```bash
kubectl exec -it -n galactus deployment/galactus-app -- \
  python utils/hudi_clickhouse_sync.py \
  --table sec_bhavdata \
  --hudi-path /app/data/silver
```

## Scaling

### Scale Application

```bash
kubectl scale -n galactus deployment/galactus-app --replicas=3
```

### Update Resources

Edit `k8s/deployment.yaml` and adjust:
```yaml
resources:
  requests:
    memory: "8Gi"
    cpu: "4"
  limits:
    memory: "16Gi"
    cpu: "8"
```

Then apply:
```bash
kubectl apply -f k8s/deployment.yaml
```

## Monitoring

### View Logs

```bash
# Application logs
kubectl logs -n galactus -f deployment/galactus-app

# ClickHouse logs
kubectl logs -n galactus -f statefulset/clickhouse
```

### Check Resource Usage

```bash
kubectl top pods -n galactus
```

## Persistence

Data is stored in PersistentVolumes:
- `/app/data` - Bronze, Silver, Gold layers
- `/app/logs` - Application logs

These survive pod restarts and deletions.

## Backup

### Backup Hudi Data

```bash
kubectl exec -n galactus deployment/galactus-app -- \
  tar czf /tmp/hudi-backup.tar.gz /app/data/silver

kubectl cp galactus/galactus-app-xxx:/tmp/hudi-backup.tar.gz ./hudi-backup.tar.gz
```

### Backup ClickHouse

```bash
kubectl exec -n galactus clickhouse-0 -- \
  clickhouse-client --query "BACKUP DATABASE galactus TO Disk('default', 'backup/galactus')"
```

## Troubleshooting

### Pod not starting

```bash
kubectl describe pod -n galactus <pod-name>
kubectl logs -n galactus <pod-name>
```

### Storage issues

```bash
kubectl get pvc -n galactus
kubectl describe pvc -n galactus <pvc-name>
```

### Network issues

```bash
# Test ClickHouse connectivity
kubectl exec -it -n galactus deployment/galactus-app -- \
  curl http://clickhouse-service:8123

# Check services
kubectl get svc -n galactus
```

## Cleanup

Remove all resources:

```bash
kubectl delete namespace galactus
```

Or selectively:

```bash
kubectl delete -f k8s/deployment.yaml
kubectl delete -f k8s/pvc.yaml
kubectl delete -f k8s/configmap.yaml
kubectl delete -f k8s/clickhouse.yaml
```

## Production Considerations

1. **Storage**: Use production-grade storage classes (e.g., AWS EBS, GCE PD)
2. **Secrets**: Use Kubernetes Secrets for sensitive data
3. **Ingress**: Add Ingress for external access
4. **Monitoring**: Integrate Prometheus/Grafana
5. **Backup**: Set up automated backup schedules
6. **High Availability**: Run multiple ClickHouse replicas
7. **Resource Limits**: Set appropriate CPU/memory limits
8. **Network Policies**: Restrict pod-to-pod communication
