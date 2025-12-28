#!/bin/bash
# Galactus Production Startup Script

set -e

echo "🚀 Starting Galactus Production Environment"
echo "=========================================="

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from template..."
    cp .env.example .env
    echo "⚠️  Please edit .env file with your production settings"
fi

# Build and start services
echo "🏗️  Building and starting services..."
docker-compose up --build -d

# Wait for services to be healthy
echo "⏳ Waiting for services to start..."
sleep 10

# Check health
echo "🏥 Checking system health..."
python scripts/visualize_signals.py --mode health

echo ""
echo "✅ Galactus is running!"
echo ""
echo "🌐 Service URLs:"
echo "   API:        http://localhost:8080"
echo "   gRPC:       localhost:9090"
echo "   Grafana:    http://localhost:3000 (admin/admin)"
echo "   Prometheus: http://localhost:9090"
echo ""
echo "📊 Monitoring:"
echo "   python scripts/visualize_signals.py --mode all"
echo ""
echo "📝 Logs:"
echo "   docker-compose logs -f galactus-core"
echo ""
echo "🛑 Stop: docker-compose down"