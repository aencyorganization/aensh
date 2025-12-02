#!/bin/bash

# Aensh Shell Installation Script
# This script builds and installs the Aensh shell system-wide

set -e

echo "🦀 Installing Aensh Shell..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the shell
echo "🔨 Building Aensh..."
cargo build --release

# Create installation directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Install the binary
echo "📦 Installing to $INSTALL_DIR..."
cp target/release/aensh "$INSTALL_DIR/aensh"

# Make executable
chmod +x "$INSTALL_DIR/aensh"

# Check if installation directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "⚠️  $INSTALL_DIR is not in your PATH"
    echo "   Add this to your ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
fi

# Create symlink for easy access (optional)
if [ -w "/usr/local/bin" ]; then
    echo "🔗 Creating system-wide symlink..."
    sudo ln -sf "$INSTALL_DIR/aensh" "/usr/local/bin/aensh"
else
    echo "ℹ️  Cannot create system-wide symlink (need sudo rights)"
fi

echo "✅ Aensh Shell installed successfully!"
echo ""
echo "🚀 Run with:"
echo "   aensh"
echo ""
echo "📚 For help, run:"
echo "   aensh"
echo "   then type: help"
echo ""
echo "🗑️  To uninstall:"
echo "   rm -f $INSTALL_DIR/aensh"
echo "   rm -f /usr/local/bin/aensh"
