# Trading Examples

This section provides comprehensive examples for using the Axiom Trade API trading functionality, including simple trades, error handling patterns, and advanced trading strategies.

## Simple Trade Example

The `simple_trade.rs` example demonstrates the fundamental trading operations available through the Axiom Trade API. This example serves as a starting point for understanding the trading workflow and best practices.

### Overview

The simple trade example showcases:
- Authentication setup for trading
- Buy and sell operations simulation
- Price quote retrieval
- Parameter validation
- Error handling patterns
- Security best practices

### Example Walkthrough

```rust
use axiomtrade_rs::{AuthClient, Result, AxiomError};
use axiomtrade_rs::api::trading::TradingClient;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials and authenticate
    dotenvy::dotenv().ok();
    let mut trading_client = authenticate().await?;

    // Trading parameters
    let token_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"; // USDC
    let amount_sol = 0.001; // Small demonstration amount
    
    // Execute buy operation
    simulate_buy_trade(&mut trading_client, token_mint, amount_sol).await?;
    
    // Execute sell operation  
    let amount_tokens = 1.0; // 1 USDC
    simulate_sell_trade(&mut trading_client, token_mint, amount_tokens).await?;
    
    Ok(())
}
```

### Authentication Setup

The example begins with proper authentication using environment variables:

```rust
async fn authenticate() -> Result<TradingClient> {
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    let mut auth_client = AuthClient::new()?;
    
    match auth_client.login_full(&email, &password, None).await {
        Ok(login_result) => {
            println!("Authentication successful!");
            println!("Access token obtained: {}", &login_result.tokens.access_token[..20]);
        }
        Err(e) => {
            return Err(AxiomError::Auth(e));
        }
    }

    TradingClient::new().map_err(|e| AxiomError::Api {
        message: format!("Failed to create trading client: {}", e),
    })
}
```

## Trade Parameters

### Essential Parameters

When executing trades, several key parameters must be configured:

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `token_mint` | `&str` | Target token mint address | `"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"` |
| `amount` | `f64` | Trade amount in base units | `0.001` (SOL) or `1.0` (tokens) |
| `slippage` | `Option<f64>` | Maximum acceptable slippage percentage | `Some(1.0)` (1%) |
| `priority_fee` | `Option<f64>` | Additional priority fee in SOL | `Some(0.00001)` |

### Token Mint Addresses

Common token mint addresses used in examples:

```rust
// Native SOL
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

// USDC
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

// USDT  
const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
```

### Slippage Configuration

Slippage tolerance should be set based on token liquidity and market conditions:

```rust
let slippage_scenarios = vec![0.1, 0.5, 1.0, 2.0, 5.0];

for slippage in slippage_scenarios {
    let estimated_impact = calculate_slippage_impact(amount_sol, slippage);
    println!("{}% slippage tolerance - Max cost: {} SOL", 
        slippage, estimated_impact);
}
```

**Recommended slippage settings:**
- **Liquid tokens (SOL, USDC, USDT)**: 0.1% - 0.5%
- **Mid-cap tokens**: 0.5% - 1.0%
- **Low liquidity tokens**: 1.0% - 5.0%
- **Volatile markets**: 2.0% - 5.0%

## Error Handling

The trading system implements comprehensive error handling patterns to ensure robust operation.

### Parameter Validation

All trading parameters undergo validation before execution:

```rust
fn validate_amount(amount: f64, unit: &str) -> Result<()> {
    if amount <= 0.0 {
        return Err(AxiomError::Api {
            message: format!("Amount must be greater than 0, got {} {}", amount, unit),
        });
    }
    
    if amount.is_nan() || amount.is_infinite() {
        return Err(AxiomError::Api {
            message: format!("Invalid amount: {} {}", amount, unit),
        });
    }
    
    Ok(())
}

fn validate_token_mint(mint: &str) -> Result<()> {
    if mint.is_empty() {
        return Err(AxiomError::Api {
            message: "Token mint cannot be empty".to_string(),
        });
    }
    
    if mint.len() < 32 || mint.len() > 44 {
        return Err(AxiomError::Api {
            message: format!("Invalid mint address length: {}", mint),
        });
    }
    
    if !mint.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AxiomError::Api {
            message: format!("Invalid characters in mint address: {}", mint),
        });
    }
    
    Ok(())
}
```

### Trading Limits Verification

Before executing trades, the system checks against trading limits:

```rust
match client.get_trading_limits().await {
    Ok(limits) => {
        if amount_sol < limits.min_sol_amount {
            return Err(AxiomError::Api {
                message: format!("Amount {} SOL is below minimum {}", 
                    amount_sol, limits.min_sol_amount)
            });
        }
        if amount_sol > limits.max_sol_amount {
            return Err(AxiomError::Api {
                message: format!("Amount {} SOL exceeds maximum {}", 
                    amount_sol, limits.max_sol_amount)
            });
        }
    }
    Err(_) => println!("Could not verify trading limits"),
}
```

### Quote Retrieval Error Handling

Price quotes may fail due to network issues or market conditions:

```rust
match client.get_quote(sol_mint, token_mint, amount_sol, Some(1.0)).await {
    Ok(quote) => {
        println!("Current swap quote:");
        println!("  Input: {} SOL", quote.in_amount);
        println!("  Output: {} tokens", quote.out_amount);
        println!("  Price impact: {:.2}%", quote.price_impact);
        println!("  Fee: {} SOL", quote.fee);
    }
    Err(e) => {
        println!("Failed to get quote: {}", e);
        // Implement retry logic or fallback strategy
    }
}
```

## Best Practices

### 1. Pre-Trade Validation

Always validate all parameters before executing trades:

```rust
// Validate trade parameters
validate_amount(amount_sol, "SOL")?;
validate_token_mint(token_mint)?;

// Check trading limits
let limits = client.get_trading_limits().await?;
ensure_within_limits(amount_sol, &limits)?;

// Get current quote for verification
let quote = client.get_quote(from_mint, to_mint, amount, slippage).await?;
verify_quote_acceptable(&quote)?;
```

### 2. Liquidity Assessment

Check token liquidity before executing large trades:

```rust
async fn assess_liquidity(client: &TradingClient, token_mint: &str) -> Result<bool> {
    match client.get_token_info(token_mint).await {
        Ok(info) => {
            println!("Token liquidity: {} SOL", info.liquidity_sol);
            println!("24h volume: {} SOL", info.volume_24h);
            
            // Consider liquid if > 100 SOL liquidity and > 10 SOL daily volume
            Ok(info.liquidity_sol > 100.0 && info.volume_24h > 10.0)
        }
        Err(_) => {
            println!("Could not assess liquidity for {}", token_mint);
            Ok(false)
        }
    }
}
```

### 3. Slippage Management

Implement dynamic slippage based on market conditions:

```rust
fn calculate_optimal_slippage(
    amount: f64, 
    liquidity: f64, 
    volatility: f64
) -> f64 {
    let base_slippage = 0.5; // 0.5% base
    let liquidity_factor = (amount / liquidity).min(0.1); // Cap at 10%
    let volatility_factor = volatility.min(0.05); // Cap at 5%
    
    base_slippage + liquidity_factor + volatility_factor
}
```

### 4. Transaction Monitoring

Monitor transaction status after execution:

```rust
async fn monitor_transaction(
    client: &TradingClient, 
    signature: &str
) -> Result<()> {
    let mut attempts = 0;
    let max_attempts = 30; // 30 seconds timeout
    
    while attempts < max_attempts {
        match client.get_transaction_status(signature).await {
            Ok(status) => {
                match status.as_str() {
                    "confirmed" => {
                        println!("Transaction confirmed: {}", signature);
                        return Ok(());
                    }
                    "failed" => {
                        return Err(AxiomError::Api {
                            message: format!("Transaction failed: {}", signature),
                        });
                    }
                    _ => {
                        println!("Transaction pending... ({})", status);
                    }
                }
            }
            Err(_) => {
                println!("Could not check transaction status");
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        attempts += 1;
    }
    
    Err(AxiomError::Api {
        message: "Transaction timeout".to_string(),
    })
}
```

### 5. Portfolio Impact Analysis

Assess how trades will affect your portfolio:

```rust
async fn analyze_portfolio_impact(
    client: &TradingClient,
    token_mint: &str,
    amount: f64,
    is_buy: bool
) -> Result<()> {
    let portfolio = client.get_portfolio().await?;
    
    if is_buy {
        // Check if we have sufficient SOL
        if portfolio.sol_balance < amount {
            return Err(AxiomError::Api {
                message: format!("Insufficient SOL balance: {} < {}", 
                    portfolio.sol_balance, amount),
            });
        }
    } else {
        // Check if we have sufficient tokens to sell
        let token_balance = portfolio.get_token_balance(token_mint);
        if token_balance < amount {
            return Err(AxiomError::Api {
                message: format!("Insufficient token balance: {} < {}", 
                    token_balance, amount),
            });
        }
    }
    
    println!("Portfolio impact analysis passed");
    Ok(())
}
```

## Advanced Trading Patterns

### 1. Dollar Cost Averaging (DCA)

Implement systematic buying over time:

```rust
pub struct DCAStrategy {
    token_mint: String,
    amount_per_trade: f64,
    interval_hours: u64,
    total_trades: usize,
    completed_trades: usize,
}

impl DCAStrategy {
    pub async fn execute_next_trade(
        &mut self, 
        client: &mut TradingClient
    ) -> Result<()> {
        if self.completed_trades >= self.total_trades {
            return Err(AxiomError::Api {
                message: "DCA strategy completed".to_string(),
            });
        }
        
        println!("Executing DCA trade {}/{}", 
            self.completed_trades + 1, self.total_trades);
            
        // Execute buy with current market conditions
        let result = client.buy_token(
            &self.token_mint,
            self.amount_per_trade,
            Some(1.0) // 1% slippage tolerance
        ).await?;
        
        self.completed_trades += 1;
        
        println!("DCA trade executed: {}", result.signature);
        Ok(())
    }
}
```

### 2. Grid Trading

Implement grid trading strategy:

```rust
pub struct GridStrategy {
    token_mint: String,
    base_price: f64,
    grid_spacing: f64,
    grid_levels: usize,
    amount_per_level: f64,
    buy_orders: Vec<GridOrder>,
    sell_orders: Vec<GridOrder>,
}

#[derive(Debug)]
pub struct GridOrder {
    price: f64,
    amount: f64,
    executed: bool,
}

impl GridStrategy {
    pub fn new(
        token_mint: String,
        base_price: f64,
        grid_spacing: f64,
        grid_levels: usize,
        amount_per_level: f64,
    ) -> Self {
        let mut buy_orders = Vec::new();
        let mut sell_orders = Vec::new();
        
        // Create buy orders below current price
        for i in 1..=grid_levels {
            let price = base_price * (1.0 - (i as f64 * grid_spacing));
            buy_orders.push(GridOrder {
                price,
                amount: amount_per_level,
                executed: false,
            });
        }
        
        // Create sell orders above current price
        for i in 1..=grid_levels {
            let price = base_price * (1.0 + (i as f64 * grid_spacing));
            sell_orders.push(GridOrder {
                price,
                amount: amount_per_level,
                executed: false,
            });
        }
        
        Self {
            token_mint,
            base_price,
            grid_spacing,
            grid_levels,
            amount_per_level,
            buy_orders,
            sell_orders,
        }
    }
    
    pub async fn check_and_execute(
        &mut self, 
        client: &mut TradingClient,
        current_price: f64
    ) -> Result<()> {
        // Check buy orders
        for order in &mut self.buy_orders {
            if !order.executed && current_price <= order.price {
                println!("Executing grid buy at {}", order.price);
                
                match client.buy_token(&self.token_mint, order.amount, Some(0.5)).await {
                    Ok(_) => {
                        order.executed = true;
                        println!("Grid buy executed successfully");
                    }
                    Err(e) => {
                        println!("Grid buy failed: {}", e);
                    }
                }
            }
        }
        
        // Check sell orders
        for order in &mut self.sell_orders {
            if !order.executed && current_price >= order.price {
                println!("Executing grid sell at {}", order.price);
                
                match client.sell_token(&self.token_mint, order.amount, Some(0.5)).await {
                    Ok(_) => {
                        order.executed = true;
                        println!("Grid sell executed successfully");
                    }
                    Err(e) => {
                        println!("Grid sell failed: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### 3. Stop Loss Implementation

Implement automatic stop loss orders:

```rust
pub struct StopLossManager {
    positions: Vec<StopLossPosition>,
}

#[derive(Debug)]
pub struct StopLossPosition {
    token_mint: String,
    entry_price: f64,
    amount: f64,
    stop_loss_percent: f64,
    trailing: bool,
    highest_price: f64,
}

impl StopLossManager {
    pub async fn check_positions(
        &mut self,
        client: &mut TradingClient
    ) -> Result<()> {
        for position in &mut self.positions {
            // Get current price
            let current_price = self.get_current_price(client, &position.token_mint).await?;
            
            // Update trailing stop
            if position.trailing && current_price > position.highest_price {
                position.highest_price = current_price;
            }
            
            // Calculate stop loss price
            let reference_price = if position.trailing {
                position.highest_price
            } else {
                position.entry_price
            };
            
            let stop_price = reference_price * (1.0 - position.stop_loss_percent / 100.0);
            
            // Execute stop loss if triggered
            if current_price <= stop_price {
                println!("Stop loss triggered for {} at price {}", 
                    position.token_mint, current_price);
                    
                match client.sell_token(&position.token_mint, position.amount, Some(2.0)).await {
                    Ok(result) => {
                        println!("Stop loss executed: {}", result.signature);
                        // Remove position after execution
                    }
                    Err(e) => {
                        println!("Stop loss execution failed: {}", e);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn get_current_price(
        &self,
        client: &TradingClient,
        token_mint: &str
    ) -> Result<f64> {
        let sol_mint = "So11111111111111111111111111111111111111112";
        let quote = client.get_quote(token_mint, sol_mint, 1.0, Some(0.5)).await?;
        Ok(quote.out_amount)
    }
}
```

### 4. Arbitrage Detection

Detect arbitrage opportunities across different markets:

```rust
pub struct ArbitrageScanner {
    min_profit_percent: f64,
    max_trade_amount: f64,
}

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    token_mint: String,
    buy_market: String,
    sell_market: String,
    buy_price: f64,
    sell_price: f64,
    profit_percent: f64,
    max_amount: f64,
}

impl ArbitrageScanner {
    pub async fn scan_opportunities(
        &self,
        client: &TradingClient,
        tokens: &[String]
    ) -> Result<Vec<ArbitrageOpportunity>> {
        let mut opportunities = Vec::new();
        
        for token in tokens {
            // Get prices from different markets/DEXs
            let prices = self.get_multi_market_prices(client, token).await?;
            
            if prices.len() < 2 {
                continue;
            }
            
            // Find min and max prices
            let min_price_market = prices.iter().min_by(|a, b| 
                a.price.partial_cmp(&b.price).unwrap()).unwrap();
            let max_price_market = prices.iter().max_by(|a, b| 
                a.price.partial_cmp(&b.price).unwrap()).unwrap();
            
            let profit_percent = 
                (max_price_market.price - min_price_market.price) / 
                min_price_market.price * 100.0;
            
            if profit_percent >= self.min_profit_percent {
                opportunities.push(ArbitrageOpportunity {
                    token_mint: token.clone(),
                    buy_market: min_price_market.market.clone(),
                    sell_market: max_price_market.market.clone(),
                    buy_price: min_price_market.price,
                    sell_price: max_price_market.price,
                    profit_percent,
                    max_amount: self.max_trade_amount.min(min_price_market.liquidity),
                });
            }
        }
        
        Ok(opportunities)
    }
    
    async fn get_multi_market_prices(
        &self,
        client: &TradingClient,
        token: &str
    ) -> Result<Vec<MarketPrice>> {
        // Implementation would query multiple DEXs/markets
        // This is a simplified example
        Ok(vec![])
    }
}

#[derive(Debug)]
struct MarketPrice {
    market: String,
    price: f64,
    liquidity: f64,
}
```

## Running the Examples

To run the trading examples:

1. **Set up environment variables:**
   ```bash
   cp .env.example .env
   # Edit .env with your credentials
   ```

2. **Run the simple trade example:**
   ```bash
   cargo run --example simple_trade
   ```

3. **Run with debug output:**
   ```bash
   RUST_LOG=debug cargo run --example simple_trade
   ```

4. **Run specific trading strategy:**
   ```bash
   cargo run --example grid_trading
   cargo run --example dca_strategy  
   cargo run --example stop_loss
   ```

## Security Considerations

### Never share sensitive information:
- Private keys or seed phrases
- Access tokens or refresh tokens  
- API credentials
- Trading strategies or positions

### Always verify before execution:
- Transaction details and amounts
- Recipient addresses and token mints
- Slippage tolerance and fees
- Market conditions and liquidity

### Use appropriate security measures:
- Hardware wallets for large amounts
- Multi-signature wallets for institutional use
- MEV protection services during high volatility
- Regular security audits of trading strategies

### Monitor for suspicious activity:
- Unexpected price movements
- Failed transactions or errors
- Unusual network latency
- Account access from unknown locations

The trading examples provide a comprehensive foundation for building sophisticated trading applications while maintaining security and reliability standards.