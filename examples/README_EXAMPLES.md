# axiomtrade-rs Examples

## ✅ All Examples Now Working!

All 22 examples have been updated and are now fully functional, demonstrating the correct usage of the axiomtrade-rs library.

## Examples by Category

### Authentication (4 examples)
- ✅ `basic_login` - Basic authentication flow with AuthClient
- ✅ `otp_verification` - OTP verification with automatic email fetching
- ✅ `session_management` - Token persistence and session handling
- ✅ `cookie_auth` - Cookie-based authentication for web integrations

### Portfolio Management (4 examples)
- ✅ `get_portfolio` - Fetch complete portfolio with PortfolioClient
- ✅ `batch_balances` - Efficient batch querying of multiple wallets
- ✅ `portfolio_monitoring` - Real-time portfolio tracking and analysis
- ✅ `token_accounts` - Token account management and analysis

### Trading (1 example)
- ✅ `simple_trade` - Execute trades with TradingClient

### Market Data (1 example)
- ✅ `trending_tokens` - Analyze trending tokens with MarketDataClient

### WebSocket (2 examples)
- ✅ `basic_websocket` - WebSocket connection with MessageHandler
- ✅ `price_subscriptions` - Real-time price tracking

### Turnkey Integration (1 example)
- ✅ `turnkey_auth` - Hardware wallet authentication via Turnkey

### Notifications (3 examples)
- ✅ `price_alerts` - Price alert management with NotificationsClient
- ✅ `system_alerts` - System monitoring and alerts
- ✅ `email_notifications` - Email notification templates

### Infrastructure (1 example)
- ✅ `health_checks` - API health monitoring with EnhancedClient

### Advanced Trading (3 examples)
- ✅ `multi_chain_portfolio` - Multi-chain portfolio management
- ✅ `automated_trading_bot` - Automated trading strategies
- ✅ `high_frequency_trading` - HFT strategies and optimizations

### Setup & Configuration (2 examples)
- ✅ `environment_setup` - Interactive .env file setup
- ✅ `test_auto_otp` - Test automatic OTP retrieval

## Running Examples

### Prerequisites

1. Create a `.env` file with your credentials:
```env
AXIOM_EMAIL=your-email@example.com
AXIOM_PASSWORD=your-password

# Optional: For automatic OTP
INBOX_LV_EMAIL=your-email@inbox.lv
INBOX_LV_PASSWORD=your-imap-password
```

2. Run the environment setup helper:
```bash
cargo run --example environment_setup
```

### Running Individual Examples

```bash
# Authentication examples
cargo run --example basic_login
cargo run --example otp_verification
cargo run --example session_management
cargo run --example cookie_auth

# Portfolio examples
cargo run --example get_portfolio
cargo run --example batch_balances
cargo run --example portfolio_monitoring
cargo run --example token_accounts

# Trading example
cargo run --example simple_trade

# Market data
cargo run --example trending_tokens

# WebSocket examples
cargo run --example basic_websocket
cargo run --example price_subscriptions

# And more...
```

## Example Architecture

All examples follow a consistent pattern:

### 1. Authentication Pattern
```rust
use axiomtrade_rs::auth::{AuthClient, TokenManager};

let mut auth_client = AuthClient::new()?;
let tokens = auth_client.login(&email, &password, None).await?;
```

### 2. Client Usage Pattern
```rust
use axiomtrade_rs::client::EnhancedClient;
use axiomtrade_rs::api::portfolio::PortfolioClient;

// For general API calls
let client = EnhancedClient::new()?;

// For specific domains
let portfolio_client = PortfolioClient::new()?;
```

### 3. WebSocket Pattern
```rust
use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler};
use std::sync::Arc;

struct MyHandler;
impl MessageHandler for MyHandler { /* ... */ }

let handler = Arc::new(MyHandler);
let ws_client = WebSocketClient::new(handler)?;
```

## Key APIs Demonstrated

### Authentication & Session Management
- `AuthClient` - Login, OTP verification, token management
- `TokenManager` - Token persistence and refresh
- `SessionManager` - Multi-account session handling

### Trading & Portfolio
- `PortfolioClient` - Balance queries, portfolio summaries
- `TradingClient` - Buy/sell operations, order management
- `MarketDataClient` - Market trends, price feeds

### Real-time Data
- `WebSocketClient` - Live price streams, order updates
- `MessageHandler` - Custom message processing

### Advanced Features
- `TurnkeyClient` - Hardware wallet integration
- `NotificationsClient` - Alert management
- `EnhancedClient` - General-purpose API client

## Best Practices Shown in Examples

1. **Error Handling**: Proper Result types and error propagation
2. **Authentication**: Secure token management with automatic refresh
3. **Rate Limiting**: Built-in retry logic and backoff strategies
4. **Type Safety**: Leveraging Rust's type system for compile-time guarantees
5. **Async Operations**: Efficient async/await patterns
6. **Resource Management**: Proper cleanup and connection pooling

## Contributing New Examples

When adding new examples:

1. Follow the existing structure and naming conventions
2. Include comprehensive error handling
3. Add descriptive comments explaining the functionality
4. Test the example thoroughly
5. Update this README with the new example

## Support

For questions about the examples:
- Check the inline documentation in each example
- Review the library documentation at [docs.rs/axiomtrade-rs](https://docs.rs/axiomtrade-rs)
- Open an issue on [GitHub](https://github.com/vibheksoni/axiomtrade-rs/issues)