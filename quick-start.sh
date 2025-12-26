#!/bin/bash
# Quick Start Script for Galactus
# This script sets up the Galactus environment and runs a test ingestion

set -e  # Exit on error

echo "======================================"
echo "Galactus Quick Start"
echo "======================================"

# Check prerequisites
echo ""
echo "Checking prerequisites..."

# Check Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed."
    exit 1
fi
echo "✓ Python 3 found"

# Check Java
if ! command -v java &> /dev/null; then
    echo "❌ Java is required but not installed."
    echo "   Please install Java 17 or later."
    exit 1
fi
echo "✓ Java found"

# Create virtual environment
echo ""
echo "Setting up virtual environment..."
if [ ! -d ".venv" ]; then
    python3 -m venv .venv
    echo "✓ Virtual environment created"
else
    echo "✓ Virtual environment already exists"
fi

# Activate virtual environment
source .venv/bin/activate
echo "✓ Virtual environment activated"

# Install dependencies
echo ""
echo "Installing dependencies..."
pip install --quiet --upgrade pip
pip install --quiet -r requirements.txt
echo "✓ Dependencies installed"

# Set environment variables
echo ""
echo "Setting up environment..."
export GALACTUS_PROJECT_ROOT=$(pwd)
export GALACTUS_DATA_ROOT=$(pwd)/data
export HUDI_BASE_PATH=$(pwd)/data/silver
export PYTHONPATH=$(pwd)
echo "✓ Environment configured"

# Create directories
echo ""
echo "Creating data directories..."
mkdir -p data/bronze data/silver data/gold logs
echo "✓ Data directories created"

# Copy .env.example if .env doesn't exist
if [ ! -f ".env" ]; then
    echo ""
    echo "Creating .env file..."
    cp .env.example .env
    echo "✓ .env file created (please customize if needed)"
fi

# Run tests
echo ""
echo "Running tests..."
if pytest -q tests/; then
    echo "✓ Tests passed"
else
    echo "⚠ Some tests failed (this may be expected in some environments)"
fi

# Success message
echo ""
echo "======================================"
echo "✓ Setup Complete!"
echo "======================================"
echo ""
echo "Next steps:"
echo ""
echo "1. Activate the environment:"
echo "   source .venv/bin/activate"
echo ""
echo "2. Customize configuration (optional):"
echo "   nano .env"
echo ""
echo "3. Run a test ingestion:"
echo "   spark-submit \\"
echo "     --packages org.apache.hudi:hudi-spark3.4-bundle_2.12:0.15.0 \\"
echo "     --master local[*] \\"
echo "     jobs/ingest_bhavcopy_daily_v2.py \\"
echo "     2024-01-15 \\"
echo "     \$HUDI_BASE_PATH"
echo ""
echo "4. Or use Docker:"
echo "   docker-compose up -d"
echo ""
echo "5. Read the documentation:"
echo "   - README.md - Project overview"
echo "   - k8s/README.md - Kubernetes deployment"
echo "   - tests/README.md - Testing guide"
echo ""
echo "======================================"
