#!/bin/bash

# Flappy Bird Build Script
# This script helps build the game in different configurations

set -e  # Exit on error

echo "🐦 Flappy Bird - Rust Edition Build Script"
echo "=========================================="
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo is not installed!"
    echo "Please install Rust from: https://rustup.rs/"
    exit 1
fi

echo "✅ Cargo found: $(cargo --version)"
echo ""

# Parse command line arguments
BUILD_TYPE=${1:-debug}

case "$BUILD_TYPE" in
    debug)
        echo "🔨 Building in DEBUG mode..."
        cargo build
        echo ""
        echo "✅ Build complete!"
        echo "📦 Binary location: target/debug/flappy_bird"
        echo "🚀 Run with: cargo run"
        ;;
    release)
        echo "🔨 Building in RELEASE mode (optimized)..."
        cargo build --release
        echo ""
        echo "✅ Build complete!"
        echo "📦 Binary location: target/release/flappy_bird"
        echo "💪 This version is optimized for performance"
        ;;
    clean)
        echo "🧹 Cleaning build artifacts..."
        cargo clean
        echo "✅ Clean complete!"
        ;;
    run)
        echo "🚀 Building and running in DEBUG mode..."
        cargo run
        ;;
    run-release)
        echo "🚀 Building and running in RELEASE mode..."
        cargo run --release
        ;;
    test)
        echo "🧪 Running tests..."
        cargo test
        ;;
    check)
        echo "🔍 Checking code (fast compile check)..."
        cargo check
        ;;
    *)
        echo "Usage: $0 [debug|release|clean|run|run-release|test|check]"
        echo ""
        echo "Options:"
        echo "  debug        - Build in debug mode (default)"
        echo "  release      - Build optimized release version"
        echo "  clean        - Remove build artifacts"
        echo "  run          - Build and run in debug mode"
        echo "  run-release  - Build and run in release mode"
        echo "  test         - Run tests"
        echo "  check        - Fast compilation check"
        exit 1
        ;;
esac

echo ""
echo "🎮 Happy gaming!"
