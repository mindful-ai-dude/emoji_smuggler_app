#!/bin/bash
# Browser Demo Script for Pluralsight Presentation
# Builds WASM and starts a local web server for the demo

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     EMOJI SMUGGLER - BROWSER DEMONSTRATION                 ║"
echo "║     Pluralsight Author Audition                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Change to project directory
cd "$(dirname "$0")/.."

echo "Step 1: Building WASM package..."
echo ""
wasm-pack build --target web

echo ""
echo "✅ WASM build complete!"
echo ""

echo "Step 2: Starting local web server..."
echo ""

# Check if uv is available
if command -v uv &> /dev/null; then
    echo "Using uv to run Python HTTP server..."
    echo "Demo will be available at: http://localhost:8000/www/"
    echo ""
    echo "Press Ctrl+C to stop the server"
    echo ""
    # Use uv run to execute python in the environment without manual activation
    # We use python -m http.server (Python 3)
    uv run python -m http.server 8000
elif command -v python3 &> /dev/null; then
    echo "⚠️  uv not found, falling back to system python3..."
    echo "Demo will be available at: http://localhost:8000/www/"
    echo ""
    python3 -m http.server 8000
else
    echo "❌ Python not found!"
    echo ""
    echo "Please run ./scripts/setup.sh to install uv and Python."
    echo ""
fi
