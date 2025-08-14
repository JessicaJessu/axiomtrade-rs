# Quick Start

Get up and running with axiomtrade-rs in under 5 minutes.

## Installation

Add axiomtrade-rs to your `Cargo.toml`:

```toml
[dependencies]
axiomtrade-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Basic Usage

### 1. Authentication

```rust
use axiomtrade_rs::auth::AuthClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()?;
    
    let email = "your-email@example.com";
    let password = "your-password";
    
    // Login and get tokens
    let tokens = auth_client.login(&email, &password, None).await?;
    println!("Login successful! Access token: {}", tokens.access_token);
    
    Ok(())
}
```

### 2. Portfolio Management

```rust
use axiomtrade_rs::api::portfolio::PortfolioClient;

// Get wallet balance
let mut portfolio_client = PortfolioClient::new()?;
let wallet = "your-wallet-address";
let balance = portfolio_client.get_balance(wallet).await?;
println!("SOL Balance: {} SOL", balance.sol_balance);
```

### 3. Trading Operations

```rust
use axiomtrade_rs::api::trading::TradingClient;

// Execute a simple trade
let mut trading_client = TradingClient::new()?;
let result = trading_client.buy_token(
    "token-mint-address",
    1.0, // Amount in SOL
    None, // Use default slippage
).await?;
println!("Trade executed: {:?}", result);
```

### 4. WebSocket Streaming

```rust
use axiomtrade_rs::websocket::{client::WebSocketClient, handler::MessageHandler};

struct MyHandler;

impl MessageHandler for MyHandler {
    async fn handle_message(&self, message: String) {
        println!("Received: {}", message);
    }
}

// Connect to WebSocket
let mut ws_client = WebSocketClient::new("wss://api.axiom.trade/ws").await?;
ws_client.set_handler(Box::new(MyHandler)).await;
ws_client.connect().await?;
```

## Environment Setup

For automatic OTP and other features, create a `.env` file:

```env
# Axiom Trade credentials
AXIOM_EMAIL=your-email@example.com
AXIOM_PASSWORD=your-password

# Optional: Automatic OTP via inbox.lv
INBOX_LV_EMAIL=your-otp-email@inbox.lv
INBOX_LV_PASSWORD=your-imap-password
```

## Running Examples

The repository includes 22+ working examples:

```bash
# Authentication examples
cargo run --example basic_login
cargo run --example otp_verification

# Portfolio examples
cargo run --example get_portfolio
cargo run --example batch_balances

# Trading examples
cargo run --example simple_trade

# WebSocket examples
cargo run --example basic_websocket
```

## Next Steps

- **Authentication**: Learn about [session management](./auth/sessions.md) and [automatic OTP](./automatic-otp.md)
- **Trading**: Explore [trading operations](./api/trading.md) and [risk management](./best-practices/security.md)
- **WebSocket**: Set up [real-time data streaming](./api/websocket.md)
- **Examples**: Browse all [working examples](./examples/authentication.md)

## Need Help?

- Check the [troubleshooting guide](./troubleshooting/common-issues.md)
- Browse [frequently asked questions](./troubleshooting/faq.md)
- Open an issue on [GitHub](https://github.com/vibheksoni/axiomtrade-rs/issues)