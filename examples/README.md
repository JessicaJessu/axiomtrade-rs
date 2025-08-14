# axiomtrade-rs Examples

A comprehensive collection of 22 working examples demonstrating how to use the axiomtrade-rs library for Solana trading operations, portfolio management, and wallet integrations.

## Repository Information

- **Repository**: [https://github.com/vibheksoni/axiomtrade-rs](https://github.com/vibheksoni/axiomtrade-rs)
- **Author**: [vibheksoni](https://github.com/vibheksoni/)
- **Language**: Rust
- **Platform**: Cross-platform (Windows, Linux, macOS)

## ✅ All Examples Now Working!

All examples have been updated to use the correct APIs and compile successfully.

## Examples Structure

### Authentication (`authentication/`) - 4 Examples
- **basic_login.rs** - Simple email/password authentication with AuthClient
- **otp_verification.rs** - OTP verification with automatic inbox.lv integration
- **session_management.rs** - Token persistence and session handling
- **cookie_auth.rs** - Cookie-based authentication for web integrations

### Portfolio Management (`portfolio/`) - 4 Examples
- **get_portfolio.rs** - Fetch complete portfolio with PortfolioClient
- **batch_balances.rs** - Query multiple wallet balances efficiently
- **token_accounts.rs** - Analyze token accounts and holdings
- **portfolio_monitoring.rs** - Real-time portfolio tracking and analysis

### Trading Operations (`trading/`) - 1 Example
- **simple_trade.rs** - Execute buy/sell operations with TradingClient

### Market Data (`market_data/`) - 1 Example
- **trending_tokens.rs** - Analyze trending tokens with MarketDataClient

### WebSocket Connections (`websocket/`) - 2 Examples
- **basic_websocket.rs** - WebSocket connection with MessageHandler
- **price_subscriptions.rs** - Real-time price tracking and updates

### Turnkey Wallet Management (`turnkey/`) - 1 Example
- **turnkey_auth.rs** - Hardware wallet authentication via Turnkey API

### Notifications (`notifications/`) - 3 Examples
- **price_alerts.rs** - Price alert management system
- **system_alerts.rs** - System monitoring and alerts
- **email_notifications.rs** - Email notification templates

### Infrastructure Monitoring (`infrastructure/`) - 1 Example
- **health_checks.rs** - API health monitoring with EnhancedClient

### Advanced Integration (`advanced/`) - 3 Examples
- **multi_chain_portfolio.rs** - Multi-chain portfolio management
- **automated_trading_bot.rs** - Complete automated trading strategies
- **high_frequency_trading.rs** - HFT optimizations and strategies

### Setup & Configuration (`setup/`) - 2 Examples
- **environment_setup.rs** - Interactive .env file configuration
- **test_auto_otp.rs** - Test automatic OTP retrieval system

## Getting Started

### Prerequisites

1. **Rust Installation**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Environment Setup**
   Create a `.env` file in the project root:
   ```env
   AXIOM_EMAIL=your_email@domain.com
   AXIOM_PASSWORD=your_password
   
   # Optional: For automatic OTP
   INBOX_LV_EMAIL=your_inbox@inbox.lv
   INBOX_LV_PASSWORD=your_imap_password
   ```

3. **Dependencies**
   The project's `Cargo.toml` already includes all necessary dependencies.

### Running Examples

Execute any example using cargo:

```bash
# Authentication examples
cargo run --example basic_login
cargo run --example otp_verification
cargo run --example session_management
cargo run --example cookie_auth

# Portfolio management
cargo run --example get_portfolio
cargo run --example batch_balances
cargo run --example portfolio_monitoring
cargo run --example token_accounts

# Trading
cargo run --example simple_trade

# Market data
cargo run --example trending_tokens

# WebSocket
cargo run --example basic_websocket
cargo run --example price_subscriptions

# Turnkey
cargo run --example turnkey_auth

# Notifications
cargo run --example price_alerts
cargo run --example system_alerts
cargo run --example email_notifications

# Infrastructure
cargo run --example health_checks

# Advanced
cargo run --example multi_chain_portfolio
cargo run --example automated_trading_bot
cargo run --example high_frequency_trading

# Setup
cargo run --example environment_setup
cargo run --example test_auto_otp
```

## Key APIs and Patterns

### Authentication Pattern
```rust
use axiomtrade_rs::auth::{AuthClient, TokenManager};

let mut auth_client = AuthClient::new()?;
let tokens = auth_client.login(&email, &password, None).await?;

// Tokens are automatically stored and can be retrieved
let token_manager = TokenManager::new(Some(path));
```

### Portfolio Operations
```rust
use axiomtrade_rs::api::portfolio::PortfolioClient;

let mut portfolio_client = PortfolioClient::new()?;
let balance = portfolio_client.get_balance(wallet_address).await?;
let batch = portfolio_client.get_batch_balance(&wallets).await?;
```

### Trading Operations
```rust
use axiomtrade_rs::api::trading::TradingClient;

let mut trading_client = TradingClient::new()?;
let quote = trading_client.get_quote(token_address, amount).await?;
let result = trading_client.buy_token(token_address, amount, slippage).await?;
```

### WebSocket Streaming
```rust
use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler};
use std::sync::Arc;

struct MyHandler;
impl MessageHandler for MyHandler {
    fn handle_message(&self, message: &str) {
        // Process message
    }
    // ... other trait methods
}

let handler = Arc::new(MyHandler);
let mut ws_client = WebSocketClient::new(handler)?;
ws_client.connect().await?;
```

## Key Features Demonstrated

- **Type-safe API interactions** with comprehensive error handling
- **Async/await patterns** for efficient concurrent operations  
- **Real-time data streaming** via WebSocket connections
- **Enterprise-grade security** with P256 cryptographic operations
- **Multi-wallet management** with batch operations
- **Automatic OTP handling** via IMAP integration
- **Session persistence** and automatic token refresh
- **Production-ready patterns** including retry logic and rate limiting

## Error Handling Patterns

All examples demonstrate proper error handling:

```rust
use axiomtrade_rs::auth::AuthClient;
use axiomtrade_rs::AxiomError;

match auth_client.login(&email, &password, None).await {
    Ok(tokens) => {
        println!("Success: Access token received");
    }
    Err(AxiomError::Authentication(msg)) => {
        eprintln!("Authentication failed: {}", msg);
    }
    Err(AxiomError::Network(msg)) => {
        eprintln!("Network error: {}", msg);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Performance Considerations

- **Connection pooling** - Reuse HTTP clients for multiple requests
- **Batch operations** - Use batch endpoints for multiple wallets
- **WebSocket management** - Proper connection lifecycle management
- **Rate limiting** - Respect API rate limits with exponential backoff
- **Memory efficiency** - Stream large datasets instead of loading into memory

## Security Best Practices

- **Environment variables** - Never hardcode credentials
- **Secure session storage** - Use TokenManager for secure token storage
- **Token rotation** - Implement automatic token refresh
- **Input validation** - Validate all user inputs
- **Error sanitization** - Don't expose sensitive data in error messages

## Testing All Examples

Verify all examples compile:

```bash
# Check all examples
cargo check --examples

# Build all examples
cargo build --examples

# Run a specific example
cargo run --example basic_login
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add comprehensive examples with documentation
4. Include error handling and performance considerations
5. Submit a pull request

## Support

- **Issues**: [GitHub Issues](https://github.com/vibheksoni/axiomtrade-rs/issues)
- **Documentation**: See individual example files for detailed usage
- **API Reference**: Check [docs.rs/axiomtrade-rs](https://docs.rs/axiomtrade-rs)

## License

This project is licensed under the MIT License - see the main repository for details.