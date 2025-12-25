#!/bin/bash

# Build Docker images
docker build -f docker/Dockerfile.airflow -t bhavcopy-airflow:latest .
docker build -f docker/Dockerfile.spark -t bhavcopy-spark:latest .

# Apply Kubernetes manifests
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/airflow-configmap.yaml
kubectl apply -f k8s/dags-configmap.yaml
kubectl apply -f k8s/data-pvc.yaml
kubectl apply -f k8s/postgres.yaml
kubectl apply -f k8s/spark.yaml
kubectl apply -f k8s/airflow-scheduler.yaml
kubectl apply -f k8s/airflow-webserver.yaml

echo "Deployment complete. Wait for pods to be ready."
echo "Check status with: kubectl get pods -n bhavcopy-pipeline"
echo "Access Airflow UI via LoadBalancer service"