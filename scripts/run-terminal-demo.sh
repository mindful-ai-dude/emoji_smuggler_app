#!/bin/bash
# Terminal Demo Script for Pluralsight Presentation
# Runs the full terminal demonstration

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     EMOJI SMUGGLER - TERMINAL DEMONSTRATION                ║"
echo "║     Pluralsight Author Audition                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Change to project directory
cd "$(dirname "$0")/.."

echo "Step 1: Building project..."
echo ""
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""

echo "Step 2: Running full demonstration..."
echo ""
cargo run --release -- demo

echo ""
echo "✅ Demonstration complete!"
echo ""
