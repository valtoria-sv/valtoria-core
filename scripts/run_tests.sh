#!/bin/bash

# run_tests.sh - Script to run tests for the blockchain project

# Exit immediately if a command exits with a non-zero status
set -e

# Function to print messages
function info() {
    echo "[INFO] $1"
}

info "Starting tests..."

# Run unit tests
info "Running unit tests..."
cargo test

# Run benchmarks
info "Running benchmarks..."
cargo bench

info "All tests completed successfully!"
