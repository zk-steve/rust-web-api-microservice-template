#!/bin/bash

# Script to install required dependencies for chaos engineering tests
# Run with: ./scripts/install_dependencies.sh

set -e

echo "Installing dependencies for Rust Web API Microservice Template..."

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    if command -v apt-get &> /dev/null; then
        echo "Detected Ubuntu/Debian system"
        echo "Installing pkg-config and OpenSSL development libraries..."
        sudo apt-get update
        sudo apt-get install -y pkg-config libssl-dev protobuf-compiler
    elif command -v yum &> /dev/null; then
        echo "Detected RHEL/CentOS/Fedora system"
        echo "Installing pkg-config and OpenSSL development libraries..."
        sudo yum install -y pkg-config openssl-devel protobuf-compiler
    elif command -v dnf &> /dev/null; then
        echo "Detected Fedora system with dnf"
        echo "Installing pkg-config and OpenSSL development libraries..."
        sudo dnf install -y pkg-config openssl-devel protobuf-compiler
    elif command -v apk &> /dev/null; then
        echo "Detected Alpine Linux"
        echo "Installing pkg-config and OpenSSL development libraries..."
        sudo apk add pkgconfig openssl-dev protobuf-dev
    else
        echo "Unknown Linux distribution. Please install manually:"
        echo "- pkg-config"
        echo "- libssl-dev (or openssl-devel)"
        echo "- protobuf-compiler"
        exit 1
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    echo "Detected macOS system"
    if command -v brew &> /dev/null; then
        echo "Installing dependencies via Homebrew..."
        brew install pkg-config openssl protobuf
    else
        echo "Homebrew not found. Please install Homebrew first:"
        echo "https://brew.sh/"
        exit 1
    fi
else
    echo "Unsupported operating system: $OSTYPE"
    echo "Please install manually:"
    echo "- pkg-config"
    echo "- OpenSSL development libraries"
    echo "- Protocol Buffers compiler (protoc)"
    exit 1
fi

# Verify installations
echo ""
echo "Verifying installations..."

if command -v pkg-config &> /dev/null; then
    echo "✓ pkg-config is installed: $(pkg-config --version)"
else
    echo "✗ pkg-config is not found"
    exit 1
fi

if command -v protoc &> /dev/null; then
    echo "✓ protoc is installed: $(protoc --version)"
else
    echo "✗ protoc is not found"
    exit 1
fi

if pkg-config --exists openssl; then
    echo "✓ OpenSSL development libraries are available: $(pkg-config --modversion openssl)"
else
    echo "✗ OpenSSL development libraries are not found"
    exit 1
fi

echo ""
echo "All dependencies are installed successfully!"
echo ""
echo "You can now run the chaos engineering tests:"
echo "  cargo test chaos                    # Run all chaos tests"
echo "  cargo test --test chaos_tests_standalone  # Run standalone chaos tests"
echo "  make chaos-tests                    # Run chaos tests via Makefile"
echo ""