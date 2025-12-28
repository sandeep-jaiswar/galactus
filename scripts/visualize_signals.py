#!/usr/bin/env python3
"""
Galactus Production Signal Visualization Tool

This script provides production-ready visualization for Galactus signals and monitoring:

1. Real-time API monitoring and metrics
2. Grafana dashboard setup and validation
3. System health visualization
4. Production signal analysis

Usage:
    python visualize_signals.py --mode [api|grafana|health|production|all]
"""

import argparse
import json
import time
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, List, Optional
import sys
import os

try:
    import requests
    import matplotlib.pyplot as plt
    import seaborn as sns
    import numpy as np
except ImportError as e:
    print(f"Missing dependencies: {e}")
    print("Install with: pip install requests matplotlib seaborn numpy")
    sys.exit(1)

# Set plotting style
plt.style.use("default")
sns.set_palette("husl")


class GalactusVisualizer:
    """Production visualization class for Galactus signals"""

    def __init__(self, api_url: str = "http://localhost:8080"):
        self.api_url = api_url
        self.colors = {
            "oi_decay": "#FF6B6B",
            "hedge_pressure": "#4ECDC4",
            "basis_pressure": "#45B7D1",
            "confidence": "#96CEB4",
            "stability": "#FFEAA7",
            "health": "#DDA0DD",
        }

    def check_api_health(self) -> bool:
        """Check if the API is running"""
        try:
            response = requests.get(f"{self.api_url}/api/v1/health", timeout=5)
            return response.status_code == 200
        except (requests.RequestException, ConnectionError):
            return False

    def visualize_api_metrics(self) -> None:
        """Visualize API metrics if available"""
        if not self.check_api_health():
            print("❌ API not available at", self.api_url)
            print("💡 Start the API server first:")
            print("   docker-compose up galactus-core")
            return

        print("🔍 Fetching API metrics...")

        try:
            # Get health status
            health_response = requests.get(f"{self.api_url}/api/v1/health", timeout=5)
            health_response.raise_for_status()
            health_data = health_response.json()

            # Get metrics
            metrics_response = requests.get(f"{self.api_url}/api/v1/metrics", timeout=5)
            metrics_response.raise_for_status()
            metrics_text = metrics_response.text

            print("\n✅ API Status:")
            print(f"   Service: {health_data.get('status', 'unknown')}")
            print(f"   Uptime: {health_data.get('uptime_seconds', 0)} seconds")
            print(f"   Components: {len(health_data.get('components', {}))}")

            # Parse Prometheus metrics
            metrics = self._parse_prometheus_metrics(metrics_text)

            # Create metrics visualization
            self._plot_api_metrics(metrics)

        except (requests.RequestException, ValueError, KeyError) as e:
            print(f"❌ Error fetching metrics: {e}")

    def _parse_prometheus_metrics(self, metrics_text: str) -> Dict[str, float]:
        """Parse Prometheus metrics text format"""
        metrics = {}
        for line in metrics_text.split("\n"):
            if line.startswith("#") or not line.strip():
                continue
            if "{" in line:  # Has labels
                metric_name = line.split("{")[0]
                value = float(line.split("} ")[1])
            else:  # Simple metric
                parts = line.split(" ")
                if len(parts) >= 2:
                    metric_name = parts[0]
                    try:
                        value = float(parts[1])
                    except ValueError:
                        continue
            metrics[metric_name] = value
        return metrics

    def _plot_api_metrics(self, metrics: Dict[str, float]) -> None:
        """Plot API metrics"""
        fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(15, 10))
        fig.suptitle("Galactus API Metrics Dashboard", fontsize=16)

        # Inference operations
        ops_total = metrics.get("galactus_inference_operations_total", 0)
        ax1.bar(["Inference Operations"], [ops_total], color=self.colors["oi_decay"])
        ax1.set_title("Total Inference Operations")
        ax1.set_ylabel("Count")

        # Quality scores
        scores = [
            metrics.get("galactus_data_quality_score", 0),
            metrics.get("galactus_confidence_score", 0),
            metrics.get("galactus_stability_score", 0),
        ]
        score_names = ["Data Quality", "Confidence", "Stability"]

        bars = ax2.bar(
            score_names,
            scores,
            color=[
                self.colors["oi_decay"],
                self.colors["confidence"],
                self.colors["stability"],
            ],
        )
        ax2.set_title("Quality Scores")
        ax2.set_ylabel("Score (0-1)")
        ax2.set_ylim(0, 1)

        # Add value labels on bars
        for bar, score in zip(bars, scores, strict=True):
            height = bar.get_height()
            ax2.text(
                bar.get_x() + bar.get_width() / 2.0,
                height + 0.01,
                f"{score:.2f}",
                ha="center",
                va="bottom",
            )

        # Active signals
        active_signals = metrics.get("galactus_active_signals", 0)
        ax3.pie(
            [active_signals, 3 - active_signals],
            labels=["Active", "Inactive"],
            colors=[self.colors["oi_decay"], "#E0E0E0"],
            autopct="%1.0f%%",
        )
        ax3.set_title(f"Active Signals ({active_signals}/3)")

        # Silence suppressions
        suppressions = metrics.get("galactus_silence_suppressions_total", 0)
        ax4.bar(
            ["Silence Suppressions"],
            [suppressions],
            color=self.colors["hedge_pressure"],
        )
        ax4.set_title("Safety Suppressions")
        ax4.set_ylabel("Count")

        plt.tight_layout()
        plt.show()

    def show_grafana_setup(self) -> None:
        """Show Grafana dashboard setup instructions"""
        print("📊 Grafana Dashboard Setup:")
        print("=" * 50)
        print("1. Grafana is available at: http://localhost:3000")
        print("2. Dashboard is auto-imported from: monitoring/grafana_dashboard.json")
        print("3. Prometheus data source is auto-configured")
        print("4. Dashboard includes:")
        print("   ✅ System health indicators")
        print("   ✅ Data quality gauges")
        print("   ✅ Inference performance graphs")
        print("   ✅ Resource utilization")
        print("   ✅ Alert status")

        print("\n🚀 Quick start with Docker:")
        print("   docker-compose up grafana")
        print("   # Dashboard will be automatically available")

    def check_system_health(self) -> None:
        """Check overall system health"""
        print("🏥 Galactus System Health Check")
        print("=" * 50)

        services = {
            "galactus-core": f"{self.api_url}/api/v1/health",
            "prometheus": "http://localhost:9090/-/healthy",
            "grafana": "http://localhost:3000/api/health",
        }

        all_healthy = True

        for service, url in services.items():
            try:
                if service == "grafana":
                    # Grafana health check
                    response = requests.get(url, auth=("admin", "admin"), timeout=5)
                else:
                    response = requests.get(url, timeout=5)

                if response.status_code == 200:
                    print(f"   ✅ {service}: Healthy")
                else:
                    print(f"   ❌ {service}: HTTP {response.status_code}")
                    all_healthy = False
            except (requests.RequestException, ConnectionError, TimeoutError) as e:
                print(f"   ❌ {service}: Unavailable ({str(e)[:50]})")
                all_healthy = False

        if all_healthy:
            print("\n🎉 All services are healthy!")
        else:
            print("\n⚠️  Some services are unhealthy. Check docker-compose logs.")

    def show_production_info(self) -> None:
        """Show production deployment information"""
        print("🏭 Galactus Production Deployment")
        print("=" * 50)
        print("Services:")
        print("   galactus-core    : http://localhost:8080 (HTTP API)")
        print("                     localhost:9090 (gRPC API)")
        print("   prometheus       : http://localhost:9090")
        print("   grafana          : http://localhost:3000")
        print("")
        print("Monitoring:")
        print("   Health Check     : GET /api/v1/health")
        print("   Metrics          : GET /api/v1/metrics")
        print("   Logs             : docker-compose logs -f galactus-core")
        print("")
        print("Scaling:")
        print("   docker-compose up -d --scale galactus-core=3")
        print("")
        print("Backup:")
        print("   docker-compose exec galactus-core tar czf /backup.tar.gz /app/logs")
        print(
            "   docker cp $(docker-compose ps -q galactus-core):/backup.tar.gz ./backup.tar.gz"
        )


def main():
    parser = argparse.ArgumentParser(
        description="Galactus Production Signal Visualization Tool"
    )
    parser.add_argument(
        "--mode",
        choices=["api", "grafana", "health", "production", "all"],
        default="health",
        help="Visualization mode",
    )
    parser.add_argument(
        "--api-url", default="http://localhost:8080", help="API server URL"
    )

    args = parser.parse_args()

    visualizer = GalactusVisualizer(args.api_url)

    print("🎯 Galactus Production Visualizer")
    print("=" * 50)

    if args.mode == "health" or args.mode == "all":
        visualizer.check_system_health()

    if args.mode == "api" or args.mode == "all":
        visualizer.visualize_api_metrics()

    if args.mode == "grafana" or args.mode == "all":
        visualizer.show_grafana_setup()

    if args.mode == "production" or args.mode == "all":
        visualizer.show_production_info()


if __name__ == "__main__":
    main()
