#!/bin/bash
# Setup script for Emoji Smuggler
# Installs dependencies and prepares the development environment

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║     EMOJI SMUGGLER - ENVIRONMENT SETUP                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# 1. Update Rust Toolchain
echo "Step 1: Updating Rust toolchain..."
if command -v rustup &> /dev/null; then
    rustup update stable
else
    echo "Rustup not found. Please install Rust from https://rustup.rs/"
    exit 1
fi
echo "✅ Rust toolchain updated."
echo ""

# 2. Install wasm-pack
echo "Step 2: Checking wasm-pack..."
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
else
    echo "wasm-pack is already installed."
fi
echo "✅ wasm-pack ready."
echo ""

# 3. Install uv
echo "Step 3: Checking uv (Python package manager)..."
if ! command -v uv &> /dev/null; then
    echo "Installing uv..."
    curl -LsSf https://astral.sh/uv/install.sh | sh
    # Ensure uv is in path for this session if just installed
    source $HOME/.cargo/env 2>/dev/null || true
else
    echo "uv is already installed."
fi
echo "✅ uv ready."
echo ""

# 4. Create Python Virtual Environment
echo "Step 4: Creating Python virtual environment..."
# Create venv if it doesn't exist
if [ ! -d ".venv" ]; then
    uv venv
    echo "Created .venv directory."
else
    echo ".venv already exists."
fi

echo "✅ Virtual environment ready."
echo ""

echo "🎉 Setup complete!"
echo ""
echo "To get started:"
echo "1. Activate the virtual environment: source .venv/bin/activate"
echo "2. Run the tests: cargo test"
echo "3. Run the browser demo: ./scripts/build-and-serve-browser-demo.sh"
