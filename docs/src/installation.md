# Installation

This guide covers all methods for installing axiomtrade-rs, system requirements, and platform-specific setup instructions.

## Prerequisites

### Rust Version Requirements

axiomtrade-rs requires **Rust 1.70 or later** with the 2024 edition. Check your Rust version:

```bash
rustc --version
```

If you need to install or update Rust:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update existing Rust installation
rustup update
```

### System Requirements

**Minimum Requirements:**
- RAM: 2GB available memory
- Storage: 500MB free disk space
- Network: Stable internet connection for API access

**Recommended:**
- RAM: 4GB+ for optimal performance
- Storage: 2GB+ for full development setup
- CPU: Multi-core processor for concurrent operations

### Platform Support

axiomtrade-rs supports all major platforms:
- **Windows** (Windows 10/11, Windows Server 2019+)
- **Linux** (Ubuntu 18.04+, RHEL 8+, Debian 10+)
- **macOS** (macOS 10.15+, both Intel and Apple Silicon)

## Installation Methods

### Option 1: Install from crates.io (Recommended)

The easiest way to use axiomtrade-rs in your project:

```toml
[dependencies]
axiomtrade-rs = "0.1.0"
```

Then run:

```bash
cargo build
```

### Option 2: Install from Source

For the latest development version or contributing:

```bash
# Clone the repository
git clone https://github.com/vibheksoni/axiomtrade-rs.git
cd axiomtrade-rs

# Build the project
cargo build --release

# Run tests to verify installation
cargo test
```

### Option 3: Install Specific Features

axiomtrade-rs uses feature flags for optional functionality:

```toml
[dependencies]
axiomtrade-rs = { version = "0.1.0", features = ["websocket", "auto-otp"] }
```

Available features:
- `websocket` - Real-time WebSocket support
- `auto-otp` - Automatic OTP fetching via IMAP
- `hyperliquid` - Hyperliquid integration
- `notifications` - Email and system notifications

## Dependency Overview

axiomtrade-rs uses carefully selected dependencies for optimal performance and security:

### Core Dependencies
- **tokio** (v1.40) - Async runtime with full features
- **reqwest** (v0.12) - HTTP client with JSON and cookie support
- **serde** (v1.0) - Serialization framework
- **thiserror** (v1.0) - Error handling

### Cryptography & Security
- **pbkdf2** (v0.12) - Password hashing
- **sha2** (v0.10) - SHA256 hashing
- **p256** (v0.13) - ECDSA cryptography for Turnkey
- **hmac** (v0.12) - HMAC signatures

### WebSocket Support
- **tokio-tungstenite** (v0.24) - WebSocket client
- **fastwebsockets** (v0.10) - High-performance WebSocket handling

### Optional Features
- **imap** (v2.4) - Email OTP fetching
- **native-tls** (v0.2) - TLS support
- **regex** (v1.10) - Pattern matching

## Platform-Specific Setup

### Windows

**Prerequisites:**
```powershell
# Install Visual Studio Build Tools or Visual Studio Community
# Download from: https://visualstudio.microsoft.com/downloads/

# Or install via chocolatey
choco install microsoft-build-tools

# Install Git
winget install Git.Git
```

**Installation:**
```powershell
# Open PowerShell as Administrator
git clone https://github.com/vibheksoni/axiomtrade-rs.git
cd axiomtrade-rs
cargo build --release
```

**Common Windows Issues:**
- If you encounter linker errors, ensure Visual Studio Build Tools are installed
- For SSL/TLS issues, you may need to install certificates: `cargo install cargo-update`

### Linux (Ubuntu/Debian)

**Prerequisites:**
```bash
# Update package list
sudo apt update

# Install build essentials
sudo apt install build-essential pkg-config libssl-dev

# Install Git
sudo apt install git

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Installation:**
```bash
git clone https://github.com/vibheksoni/axiomtrade-rs.git
cd axiomtrade-rs
cargo build --release
```

### Linux (RHEL/CentOS/Fedora)

**Prerequisites:**
```bash
# RHEL/CentOS
sudo yum groupinstall "Development Tools"
sudo yum install openssl-devel pkg-config

# Fedora
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel pkg-config
```

### macOS

**Prerequisites:**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (optional but recommended)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Git (if not already available)
brew install git
```

**Installation:**
```bash
git clone https://github.com/vibheksoni/axiomtrade-rs.git
cd axiomtrade-rs
cargo build --release
```

**Apple Silicon (M1/M2) Notes:**
- No special configuration needed
- All dependencies are compatible with ARM64
- Performance is excellent on Apple Silicon

## Environment Configuration

### Required Environment Variables

Create a `.env` file in your project root:

```bash
# Axiom Trade credentials
AXIOM_EMAIL=your_email@example.com
AXIOM_PASSWORD=your_password

# Optional: Automated OTP (requires inbox.lv setup)
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_imap_password

# Optional: API configuration
AXIOM_API_BASE_URL=https://axiom.trade
AXIOM_TIMEOUT_SECONDS=30
```

### Auto-OTP Setup (Optional)

For automated OTP fetching:

1. **Create inbox.lv account**: Visit https://www.inbox.lv/
2. **Enable IMAP**: Go to Settings → "Outlook, email programs" → Enable
3. **Get IMAP password**: Save the special password provided (not your web login)
4. **Configure forwarding**: Forward Axiom OTP emails to your inbox.lv address
5. **Set environment variables** as shown above

## Verification

### Basic Verification

```bash
# Verify compilation
cargo check

# Run all tests
cargo test

# Check specific features
cargo test --features websocket
cargo test --features auto-otp
```

### Quick Start Test

Create `test_installation.rs`:

```rust
use axiomtrade_rs::AxiomClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test basic client creation
    let client = AxiomClient::new();
    println!("axiomtrade-rs installed successfully!");
    Ok(())
}
```

Run with:
```bash
cargo run --bin test_installation
```

### Performance Benchmarks

Run performance tests:

```bash
# Basic performance test
cargo test --release performance_tests

# WebSocket performance
cargo run --example basic_websocket --release

# Memory usage test
cargo run --example portfolio_monitoring --release
```

Expected performance metrics:
- API response time: <50ms
- WebSocket latency: <10ms
- Memory usage: <50MB for typical operations
- Concurrent connections: 1000+ supported

## Troubleshooting

### Common Issues

**Build Failures:**
```bash
# Clear cargo cache
cargo clean

# Update dependencies
cargo update

# Rebuild from scratch
rm -rf target/
cargo build
```

**SSL/TLS Errors:**
```bash
# Update certificates (Linux)
sudo apt update ca-certificates

# macOS
brew install ca-certificates

# Windows: Update Windows or install latest Visual Studio
```

**Permission Errors:**
```bash
# Linux/macOS: Ensure cargo directory permissions
chmod -R 755 ~/.cargo

# Windows: Run PowerShell as Administrator
```

### Getting Help

- **Documentation**: https://docs.rs/axiomtrade-rs
- **Examples**: Check the `examples/` directory
- **Issues**: https://github.com/vibheksoni/axiomtrade-rs/issues
- **Discussions**: https://github.com/vibheksoni/axiomtrade-rs/discussions

## Next Steps

After successful installation:

1. **Read the [Quick Start Guide](./quick-start.md)** for basic usage
2. **Review [Examples](./examples/authentication.md)** for common patterns
3. **Set up [Environment Configuration](./environment-setup.md)**
4. **Explore [API Documentation](./api/portfolio.md)** for detailed features

## Development Setup

For contributing or advanced development:

```bash
# Install additional development tools
cargo install cargo-watch cargo-tarpaulin cargo-audit

# Set up pre-commit hooks
cargo install pre-commit
pre-commit install

# Run development server with auto-reload
cargo watch -x "run --example portfolio_monitoring"
```

The installation is now complete and verified. You can proceed to the Quick Start guide to begin using axiomtrade-rs in your projects.
