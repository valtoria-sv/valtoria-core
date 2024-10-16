
### 3. development_guide.md

This document provides guidance for developers on how to set up, build, and contribute to the project.

```markdown
# Development Guide

## Introduction

This guide provides instructions for developers looking to contribute to the blockchain application. It covers the setup, building process, and contribution guidelines.

## Prerequisites

- **Rust**: Ensure that you have Rust installed. You can install Rust using [rustup](https://rustup.rs/).
- **Cargo**: Cargo is the Rust package manager and is included with Rust installation.
- **Dependencies**: The project uses several external libraries; ensure they are included in `Cargo.toml`.

## Setting Up the Project

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your_username/your_blockchain_project.git
   cd your_blockchain_project
   cargo build --release
