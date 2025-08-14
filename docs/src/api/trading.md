# Trading Operations

The Axiom Trade trading API provides comprehensive trading functionality for Solana tokens with built-in risk management, slippage protection, and MEV (Maximum Extractable Value) safeguards. This guide covers all trading operations, order types, and best practices for safe and efficient trading.

## Table of Contents

- [Quick Start](#quick-start)
- [Buy Operations](#buy-operations)
- [Sell Operations](#sell-operations)
- [Swap Operations](#swap-operations)
- [Price Quotes](#price-quotes)
- [Slippage and MEV Protection](#slippage-and-mev-protection)
- [Order Types and Parameters](#order-types-and-parameters)
- [Trading Limits](#trading-limits)
- [Transaction Simulation](#transaction-simulation)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)
- [Security Considerations](#security-considerations)

## Quick Start

```rust
use axiomtrade_rs::api::trading::TradingClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize trading client
    let mut trading_client = TradingClient::new()?;
    
    // Example: Buy USDC with SOL
    let usdc_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    let amount_sol = 0.1;
    
    let order = trading_client
        .buy_token(usdc_mint, amount_sol, Some(1.0))
        .await?;
    
    println!("Buy order executed: {}", order.signature);
    Ok(())
}
```

## Buy Operations

Buy operations allow you to purchase tokens using SOL as the base currency. The API handles routing through the best available liquidity sources.

### Basic Buy

```rust
// Buy tokens with SOL
let order_response = trading_client
    .buy_token(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC mint
        0.1,                                              // 0.1 SOL
        Some(1.0)                                         // 1% slippage
    )
    .await?;

println!("Transaction signature: {}", order_response.signature);
println!("Tokens received: {}", order_response.amount_out);
println!("Price per token: ${}", order_response.price_per_token);
```

### Buy with Custom Parameters

```rust
use axiomtrade_rs::models::trading::BuyOrderRequest;

// Get a quote first to check the expected output
let quote = trading_client
    .get_quote(
        "So11111111111111111111111111111111111111112", // Native SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        0.1,
        Some(0.5)  // 0.5% slippage for quote
    )
    .await?;

println!("Expected USDC output: {}", quote.out_amount);
println!("Price impact: {:.2}%", quote.price_impact);

// Execute buy if satisfied with quote
if quote.price_impact < 2.0 {  // Only if price impact is less than 2%
    let order = trading_client
        .buy_token(
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            0.1,
            Some(0.5)  // Conservative slippage
        )
        .await?;
    
    println!("Buy executed successfully!");
}
```

### Buy Order Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `token_mint` | `&str` | Token mint address to buy | Required |
| `amount_sol` | `f64` | Amount of SOL to spend | Required |
| `slippage_percent` | `Option<f64>` | Maximum slippage tolerance | 5.0% |
| `priority_fee` | `Option<f64>` | Priority fee in SOL | Auto-calculated |

## Sell Operations

Sell operations convert your token holdings back to SOL through optimal routing.

### Basic Sell

```rust
// Sell tokens for SOL
let order_response = trading_client
    .sell_token(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC mint
        10.0,                                             // 10 USDC
        Some(1.0)                                         // 1% slippage
    )
    .await?;

println!("SOL received: {}", order_response.amount_out);
println!("Transaction fee: {}", order_response.fee);
```

### Sell with Price Checking

```rust
// Check current price before selling
let quote = trading_client
    .get_quote(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        "So11111111111111111111111111111111111111112",   // Native SOL
        10.0,
        Some(1.0)
    )
    .await?;

println!("Expected SOL output: {}", quote.out_amount);
println!("Current SOL/USDC rate: {:.6}", quote.out_amount / 10.0);

// Proceed with sell if price is acceptable
let min_sol_expected = 0.08; // Minimum SOL we want to receive
if quote.out_amount >= min_sol_expected {
    let order = trading_client
        .sell_token(
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            10.0,
            Some(1.0)
        )
        .await?;
    
    println!("Sell executed: {} SOL received", order.amount_out);
}
```

### Sell Order Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `token_mint` | `&str` | Token mint address to sell | Required |
| `amount_tokens` | `f64` | Amount of tokens to sell | Required |
| `slippage_percent` | `Option<f64>` | Maximum slippage tolerance | 5.0% |
| `priority_fee` | `Option<f64>` | Priority fee in SOL | Auto-calculated |

## Swap Operations

Swap operations allow direct token-to-token exchanges without using SOL as an intermediary.

### Basic Token Swap

```rust
// Swap USDC to BONK
let order_response = trading_client
    .swap_tokens(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // BONK
        5.0,                                              // 5 USDC
        Some(2.0)                                         // 2% slippage
    )
    .await?;

println!("BONK tokens received: {}", order_response.amount_out);
```

### Multi-Route Swap Analysis

```rust
// Get detailed routing information
let quote = trading_client
    .get_quote(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // BONK
        100.0,
        Some(1.0)
    )
    .await?;

println!("Swap route analysis:");
println!("Input: {} USDC", quote.in_amount);
println!("Output: {} BONK", quote.out_amount);
println!("Price impact: {:.2}%", quote.price_impact);
println!("Total fees: {} USDC", quote.fee);

// Analyze routing steps
for (i, step) in quote.route.iter().enumerate() {
    println!("Route step {}: {} -> {} via {}", 
        i + 1, 
        step.input_mint[..8].to_string() + "...",
        step.output_mint[..8].to_string() + "...",
        step.amm
    );
    println!("  Fee: {}", step.fee_amount);
}

// Execute swap if acceptable
if quote.price_impact < 3.0 {
    let order = trading_client
        .swap_tokens(
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263",
            100.0,
            Some(1.0)
        )
        .await?;
    
    println!("Swap completed successfully!");
}
```

### Swap Order Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `from_mint` | `&str` | Source token mint address | Required |
| `to_mint` | `&str` | Destination token mint address | Required |
| `amount` | `f64` | Amount of source tokens | Required |
| `slippage_percent` | `Option<f64>` | Maximum slippage tolerance | 5.0% |
| `priority_fee` | `Option<f64>` | Priority fee in SOL | Auto-calculated |

## Price Quotes

Always get a quote before executing trades to understand pricing, fees, and market impact.

### Getting Accurate Quotes

```rust
// Get a comprehensive quote
let quote = trading_client
    .get_quote(
        "So11111111111111111111111111111111111111112",   // SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
        1.0,        // 1 SOL
        Some(0.5)   // 0.5% slippage
    )
    .await?;

println!("Quote Analysis:");
println!("  Input: {} SOL", quote.in_amount);
println!("  Output: {} USDC", quote.out_amount);
println!("  Exchange rate: {} USDC per SOL", quote.out_amount / quote.in_amount);
println!("  Price impact: {:.3}%", quote.price_impact);
println!("  Platform fee: {} SOL", quote.fee);
```

### Quote Response Structure

```rust
pub struct QuoteResponse {
    pub input_mint: String,     // Source token mint
    pub output_mint: String,    // Destination token mint
    pub in_amount: f64,         // Input amount
    pub out_amount: f64,        // Expected output amount
    pub price_impact: f64,      // Price impact percentage
    pub fee: f64,               // Total fees
    pub route: Vec<RouteStep>,  // Routing information
}

pub struct RouteStep {
    pub amm: String,            // AMM/DEX name (e.g., "Raydium", "Orca")
    pub input_mint: String,     // Input token for this step
    pub output_mint: String,    // Output token for this step
    pub in_amount: f64,         // Input amount for this step
    pub out_amount: f64,        // Output amount for this step
    pub fee_amount: f64,        // Fee for this step
}
```

## Slippage and MEV Protection

Axiom Trade includes built-in protection against slippage and MEV attacks.

### Understanding Slippage

Slippage occurs when the actual execution price differs from the expected price due to market movement or liquidity constraints.

```rust
// Conservative slippage for large trades
let large_trade_slippage = 0.1; // 0.1% for liquid tokens

// Standard slippage for normal trades
let normal_slippage = 0.5; // 0.5%

// Higher slippage for small/illiquid tokens
let high_slippage = 2.0; // 2.0%

// Example with different slippage scenarios
async fn execute_trade_with_slippage_analysis(
    client: &mut TradingClient,
    token_mint: &str,
    amount: f64
) -> Result<(), Box<dyn std::error::Error>> {
    
    // Test different slippage levels
    let slippage_levels = vec![0.1, 0.5, 1.0, 2.0];
    
    for slippage in slippage_levels {
        match client.get_quote(
            "So11111111111111111111111111111111111111112",
            token_mint,
            amount,
            Some(slippage)
        ).await {
            Ok(quote) => {
                println!("Slippage {:.1}%: Output {} tokens, Impact {:.3}%", 
                    slippage, quote.out_amount, quote.price_impact);
            }
            Err(e) => {
                println!("Slippage {:.1}%: Failed - {}", slippage, e);
            }
        }
    }
    
    // Execute with conservative slippage
    let order = client.buy_token(token_mint, amount, Some(0.5)).await?;
    println!("Trade executed with 0.5% slippage tolerance");
    
    Ok(())
}
```

### MEV Protection

Axiom Trade automatically implements MEV protection strategies:

1. **Private Mempool**: Transactions are routed through private mempools
2. **Bundle Protection**: Orders are bundled to prevent front-running
3. **Optimal Timing**: Executes at optimal times to minimize MEV exposure
4. **Route Optimization**: Uses routes that minimize MEV vulnerability

```rust
// MEV protection is automatically enabled
// No additional configuration required

let order = trading_client
    .buy_token(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        1.0,
        Some(0.5)
    )
    .await?;

// The system automatically:
// - Routes through protected channels
// - Uses optimal AMM combinations
// - Implements timing strategies
// - Protects against sandwich attacks
```

## Order Types and Parameters

### Order Status Types

```rust
pub enum OrderStatus {
    Success,    // Order completed successfully
    Failed,     // Order failed to execute
    Pending,    // Order is being processed
    Cancelled,  // Order was cancelled
}
```

### Order Response Structure

```rust
pub struct OrderResponse {
    pub signature: String,          // Transaction signature
    pub status: OrderStatus,        // Order status
    pub transaction_type: OrderType, // Buy, Sell, or Swap
    pub token_mint: String,         // Token mint address
    pub amount_in: f64,             // Input amount
    pub amount_out: f64,            // Output amount received
    pub price_per_token: f64,       // Effective price per token
    pub total_sol: f64,             // Total SOL involved
    pub fee: f64,                   // Transaction fee
    pub timestamp: i64,             // Execution timestamp
}
```

### Priority Fees

Priority fees help ensure faster transaction processing during network congestion.

```rust
// Auto-calculated priority fee (recommended)
let order = trading_client
    .buy_token(token_mint, amount, Some(1.0))
    .await?;

// Priority fees are automatically adjusted based on:
// - Current network congestion
// - Transaction urgency
// - Historical gas patterns
// - Market volatility
```

## Trading Limits

Understanding and respecting trading limits ensures consistent API access.

### Getting Current Limits

```rust
let limits = trading_client.get_trading_limits().await?;

println!("Trading Limits:");
println!("  Minimum SOL per trade: {}", limits.min_sol_amount);
println!("  Maximum SOL per trade: {}", limits.max_sol_amount);
println!("  Maximum slippage: {}%", limits.max_slippage_percent);
println!("  Default slippage: {}%", limits.default_slippage_percent);
println!("  Priority fee: {} lamports", limits.priority_fee_lamports);
```

### Limit Structure

```rust
pub struct TradingLimits {
    pub min_sol_amount: f64,           // Minimum SOL per trade (0.01)
    pub max_sol_amount: f64,           // Maximum SOL per trade (100.0)
    pub max_slippage_percent: f64,     // Maximum allowed slippage (50.0%)
    pub default_slippage_percent: f64, // Default slippage (5.0%)
    pub priority_fee_lamports: u64,    // Default priority fee (5000)
}
```

### Respecting Limits

```rust
async fn safe_trade_execution(
    client: &mut TradingClient,
    token_mint: &str,
    amount: f64,
    slippage: f64
) -> Result<OrderResponse, Box<dyn std::error::Error>> {
    
    // Check limits before trading
    let limits = client.get_trading_limits().await?;
    
    // Validate amount
    if amount < limits.min_sol_amount {
        return Err(format!("Amount {} below minimum {}", 
            amount, limits.min_sol_amount).into());
    }
    
    if amount > limits.max_sol_amount {
        return Err(format!("Amount {} exceeds maximum {}", 
            amount, limits.max_sol_amount).into());
    }
    
    // Validate slippage
    let safe_slippage = if slippage > limits.max_slippage_percent {
        limits.default_slippage_percent
    } else {
        slippage
    };
    
    // Execute trade
    let order = client.buy_token(token_mint, amount, Some(safe_slippage)).await?;
    Ok(order)
}
```

## Transaction Simulation

Simulate transactions before execution to verify expected outcomes.

### Basic Simulation

```rust
// Note: You would typically get the transaction data from a quote or prepare step
let transaction_base64 = "base64_encoded_transaction_data";

let simulation = trading_client
    .simulate_transaction(&transaction_base64)
    .await?;

if simulation.success {
    println!("Simulation successful!");
    println!("Compute units consumed: {}", simulation.units_consumed);
    
    // Analyze logs for detailed information
    for log in &simulation.logs {
        if log.contains("Program log:") {
            println!("  {}", log);
        }
    }
} else {
    println!("Simulation failed: {}", 
        simulation.error.unwrap_or_else(|| "Unknown error".to_string()));
}
```

### Simulation Response Structure

```rust
pub struct TransactionSimulation {
    pub success: bool,              // Whether simulation succeeded
    pub error: Option<String>,      // Error message if failed
    pub logs: Vec<String>,          // Transaction logs
    pub units_consumed: u64,        // Compute units used
}
```

## Error Handling

Comprehensive error handling for robust trading applications.

### Error Types

```rust
pub enum TradingError {
    AuthError(AuthError),           // Authentication issues
    NetworkError(reqwest::Error),   // Network connectivity problems
    InvalidTokenMint(String),       // Invalid token address
    InsufficientBalance(String),    // Not enough tokens/SOL
    SlippageExceeded(String),       // Slippage tolerance exceeded
    TransactionFailed(String),      // Transaction execution failed
    ApiError(String),               // General API errors
    ParsingError(String),           // Data parsing issues
}
```

### Error Handling Examples

```rust
use axiomtrade_rs::api::trading::TradingError;

async fn handle_trading_errors(
    client: &mut TradingClient
) -> Result<(), Box<dyn std::error::Error>> {
    
    match client.buy_token("invalid_mint", 0.1, Some(1.0)).await {
        Ok(order) => {
            println!("Trade successful: {}", order.signature);
        }
        Err(TradingError::InvalidTokenMint(msg)) => {
            println!("Invalid token address: {}", msg);
        }
        Err(TradingError::InsufficientBalance(msg)) => {
            println!("Insufficient balance: {}", msg);
        }
        Err(TradingError::SlippageExceeded(msg)) => {
            println!("Slippage exceeded: {}", msg);
            // Retry with higher slippage tolerance
        }
        Err(TradingError::NetworkError(e)) => {
            println!("Network error: {}", e);
            // Implement retry logic
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
    
    Ok(())
}
```

### Retry Logic

```rust
use tokio::time::{sleep, Duration};

async fn execute_with_retry(
    client: &mut TradingClient,
    token_mint: &str,
    amount: f64,
    max_retries: u32
) -> Result<OrderResponse, TradingError> {
    
    let mut attempts = 0;
    
    loop {
        match client.buy_token(token_mint, amount, Some(1.0)).await {
            Ok(order) => return Ok(order),
            Err(TradingError::NetworkError(_)) if attempts < max_retries => {
                attempts += 1;
                println!("Network error, retrying ({}/{})", attempts, max_retries);
                sleep(Duration::from_millis(1000 * attempts as u64)).await;
            }
            Err(TradingError::SlippageExceeded(_)) if attempts < max_retries => {
                attempts += 1;
                let higher_slippage = 1.0 + (0.5 * attempts as f64);
                println!("Retrying with {}% slippage", higher_slippage);
                
                match client.buy_token(token_mint, amount, Some(higher_slippage)).await {
                    Ok(order) => return Ok(order),
                    Err(e) => {
                        if attempts >= max_retries {
                            return Err(e);
                        }
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Best Practices

### 1. Always Get Quotes First

```rust
// Good practice: Check quote before trading
let quote = trading_client.get_quote(from_mint, to_mint, amount, Some(1.0)).await?;

if quote.price_impact < 2.0 {  // Acceptable price impact
    let order = trading_client.swap_tokens(from_mint, to_mint, amount, Some(1.0)).await?;
}
```

### 2. Use Appropriate Slippage

```rust
// Slippage guidelines by token type
let slippage = match token_type {
    "major" => 0.1,    // BTC, ETH, SOL, USDC - very liquid
    "popular" => 0.5,  // Popular tokens with good liquidity
    "standard" => 1.0, // Standard tokens
    "small" => 2.0,    // Smaller tokens with less liquidity
    _ => 5.0,          // Very small or new tokens
};
```

### 3. Implement Proper Error Handling

```rust
async fn robust_trading_function(
    client: &mut TradingClient,
    token_mint: &str,
    amount: f64
) -> Result<String, String> {
    
    // Validate inputs
    if amount <= 0.0 {
        return Err("Amount must be positive".to_string());
    }
    
    // Check trading limits
    let limits = client.get_trading_limits().await
        .map_err(|e| format!("Failed to get limits: {}", e))?;
    
    if amount < limits.min_sol_amount || amount > limits.max_sol_amount {
        return Err(format!("Amount {} outside limits [{}, {}]", 
            amount, limits.min_sol_amount, limits.max_sol_amount));
    }
    
    // Get quote first
    let quote = client.get_quote(
        "So11111111111111111111111111111111111111112",
        token_mint,
        amount,
        Some(1.0)
    ).await.map_err(|e| format!("Failed to get quote: {}", e))?;
    
    // Check price impact
    if quote.price_impact > 5.0 {
        return Err(format!("Price impact too high: {:.2}%", quote.price_impact));
    }
    
    // Execute trade
    let order = client.buy_token(token_mint, amount, Some(1.0)).await
        .map_err(|e| format!("Trade failed: {}", e))?;
    
    Ok(order.signature)
}
```

### 4. Monitor Transaction Status

```rust
async fn execute_and_monitor(
    client: &mut TradingClient,
    token_mint: &str,
    amount: f64
) -> Result<(), Box<dyn std::error::Error>> {
    
    let order = client.buy_token(token_mint, amount, Some(1.0)).await?;
    
    match order.status {
        OrderStatus::Success => {
            println!("✅ Trade completed successfully");
            println!("   Signature: {}", order.signature);
            println!("   Tokens received: {}", order.amount_out);
            println!("   Fee paid: {}", order.fee);
        }
        OrderStatus::Pending => {
            println!("⏳ Trade is pending confirmation");
            println!("   Monitor signature: {}", order.signature);
        }
        OrderStatus::Failed => {
            println!("❌ Trade failed");
            return Err("Trade execution failed".into());
        }
        OrderStatus::Cancelled => {
            println!("🚫 Trade was cancelled");
        }
    }
    
    Ok(())
}
```

### 5. Implement Rate Limiting

```rust
use tokio::time::{sleep, Duration, Instant};

struct RateLimiter {
    last_request: Instant,
    min_interval: Duration,
}

impl RateLimiter {
    fn new(requests_per_second: f64) -> Self {
        Self {
            last_request: Instant::now(),
            min_interval: Duration::from_secs_f64(1.0 / requests_per_second),
        }
    }
    
    async fn wait_if_needed(&mut self) {
        let elapsed = self.last_request.elapsed();
        if elapsed < self.min_interval {
            sleep(self.min_interval - elapsed).await;
        }
        self.last_request = Instant::now();
    }
}

// Usage
let mut rate_limiter = RateLimiter::new(2.0); // 2 requests per second

for trade in trades {
    rate_limiter.wait_if_needed().await;
    let result = trading_client.buy_token(&trade.mint, trade.amount, Some(1.0)).await;
    // Handle result...
}
```

## Security Considerations

### 1. Token Validation

```rust
fn validate_token_mint(mint: &str) -> Result<(), String> {
    // Check length
    if mint.len() < 32 || mint.len() > 44 {
        return Err("Invalid mint address length".to_string());
    }
    
    // Check characters
    if !mint.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("Invalid characters in mint address".to_string());
    }
    
    // Add known token validation
    let known_scam_tokens = vec![
        // Add known scam token addresses
    ];
    
    if known_scam_tokens.contains(&mint) {
        return Err("Token flagged as potential scam".to_string());
    }
    
    Ok(())
}
```

### 2. Amount Validation

```rust
fn validate_trade_amount(amount: f64, balance: f64) -> Result<(), String> {
    if amount <= 0.0 {
        return Err("Amount must be positive".to_string());
    }
    
    if amount.is_nan() || amount.is_infinite() {
        return Err("Invalid amount value".to_string());
    }
    
    if amount > balance * 0.95 {  // Keep 5% buffer
        return Err("Amount too close to total balance".to_string());
    }
    
    Ok(())
}
```

### 3. Slippage Protection

```rust
fn calculate_safe_slippage(
    amount: f64,
    token_liquidity: f64,
    market_volatility: f64
) -> f64 {
    let base_slippage = 0.5; // 0.5% base
    
    // Adjust for trade size relative to liquidity
    let size_impact = (amount / token_liquidity) * 100.0;
    let size_adjustment = if size_impact > 1.0 { size_impact } else { 0.0 };
    
    // Adjust for market volatility
    let volatility_adjustment = market_volatility * 0.5;
    
    // Cap at reasonable maximum
    let total_slippage = base_slippage + size_adjustment + volatility_adjustment;
    total_slippage.min(5.0) // Never exceed 5%
}
```

This comprehensive guide covers all aspects of trading operations with the Axiom Trade API. Always test with small amounts first and implement proper error handling and security measures in production applications.
