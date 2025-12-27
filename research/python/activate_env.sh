#!/bin/bash
# Galactus Research Python Environment Activator

echo "Activating Galactus Python virtual environment..."
cd "$(dirname "$0")"

# Check if .venv exists
if [ ! -d ".venv" ]; then
    echo "Virtual environment not found. Creating one..."
    python3 -m venv .venv
fi

# Activate the virtual environment
source .venv/bin/activate

# Check if dependencies are installed
python3 -c "import jugaad_data, pandas, numpy" 2>/dev/null
if [ $? -ne 0 ]; then
    echo "Installing dependencies..."
    pip install jugaad-data pandas numpy requests
fi

echo "Environment activated! You can now run Python scripts."
echo "Example: python data_provider_enhanced.py"
echo ""
echo "To deactivate, run: deactivate"