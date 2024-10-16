#!/bin/bash

# setup.sh - Script to set up the blockchain project environment

# Exit immediately if a command exits with a non-zero status
set -e

# Function to print messages
function info() {
    echo "[INFO] $1"
}

info "Starting setup..."

# Install Rust and Cargo if not installed
if ! command -v rustc &> /dev/null; then
    info "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    info "Rust is already installed."
fi

# Install necessary dependencies
info "Building the project..."
cargo build --release

info "Setup completed successfully!"
