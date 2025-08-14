# Introduction

**axiomtrade-rs** is a high-performance Rust SDK for interacting with the Axiom Trade platform, providing a type-safe, async-first interface for Solana trading operations.

## Key Features

- **High Performance**: Built with Rust for zero-cost abstractions and maximum throughput
- **Type Safety**: Comprehensive type system prevents runtime errors
- **Async/Await**: Non-blocking operations for concurrent trading
- **Secure Authentication**: Modern PBKDF2 password hashing with 600,000 iterations
- **Automatic OTP**: Optional IMAP integration for seamless OTP retrieval
- **WebSocket Streaming**: Real-time market data and portfolio updates
- **Portfolio Management**: Comprehensive balance and position tracking
- **Trading Operations**: Full support for buy, sell, and swap operations
- **Turnkey Integration**: Hardware wallet support for institutional trading

## Why Choose axiomtrade-rs?

### Performance First
Built in Rust, axiomtrade-rs delivers exceptional performance with:
- Sub-50ms API response times
- Support for 1000+ concurrent WebSocket connections
- Batch operations for 1000+ wallets
- Memory usage under 50MB for typical operations

### Developer Experience
- **Clear Documentation**: Comprehensive guides and examples
- **Type Safety**: Catch errors at compile time, not runtime
- **Modern Async**: Built on tokio for excellent concurrency
- **Rich Examples**: 22+ working examples covering all use cases

### Security & Reliability
- Modern password hashing with PBKDF2-SHA256
- Secure token management with automatic refresh
- Rate limiting and retry logic built-in
- Cross-platform compatibility (Windows, Linux, macOS)

## Architecture Overview

axiomtrade-rs is organized into several key modules:

```
axiomtrade-rs/
├── auth/           # Authentication and session management
├── api/            # API endpoints (portfolio, trading, market data)
├── websocket/      # Real-time data streaming
├── models/         # Data structures and types
├── utils/          # Utilities (password hashing, rate limiting)
└── examples/       # Comprehensive examples for all features
```

## Getting Started

Ready to start trading with axiomtrade-rs? Jump to our [Quick Start Guide](./quick-start.md) to get up and running in minutes.

For detailed setup instructions including automatic OTP configuration, see our [Environment Setup Guide](./environment-setup.md).

## Community & Support

- **Documentation**: You're reading it!
- **Issues**: [GitHub Issues](https://github.com/vibheksoni/axiomtrade-rs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/vibheksoni/axiomtrade-rs/discussions)
- **Support Development**: [Buy me a coffee](https://buymeacoffee.com/vibheksoni)

## License

axiomtrade-rs is licensed under the [MIT License](https://github.com/vibheksoni/axiomtrade-rs/blob/master/LICENSE) with an optional attribution request to help support the project.