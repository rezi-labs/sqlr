#!/bin/bash

set -e

INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="sqlc-gen-rust"
DIST_DIR="./dist"

echo "Building $BINARY_NAME..."

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is not installed. Please install Rust first."
    echo "Visit https://rustup.rs/ to install Rust and cargo."
    exit 1
fi

# Build the binary
echo "Running cargo build --release..."
cargo build --release --target x86_64-unknown-linux-gnu

# Create dist directory and copy binary
mkdir -p "$DIST_DIR"
cp target/x86_64-unknown-linux-gnu/release/$BINARY_NAME "$DIST_DIR/"
chmod +x "$DIST_DIR/$BINARY_NAME"

echo "Binary built successfully at $DIST_DIR/$BINARY_NAME"

# Install the binary
echo "Installing $BINARY_NAME to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp "$DIST_DIR/$BINARY_NAME" "$INSTALL_DIR/"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "Warning: $INSTALL_DIR is not in your PATH"
    echo "Add the following to your shell profile (.bashrc, .zshrc, etc.):"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "Successfully installed $BINARY_NAME to $INSTALL_DIR"
echo "You can now run: $BINARY_NAME"