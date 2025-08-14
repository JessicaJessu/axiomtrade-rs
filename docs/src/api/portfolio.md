# Portfolio Management

The portfolio management system in axiomtrade-rs provides comprehensive access to wallet balances, token holdings, and portfolio analytics. This module allows you to query single wallets, perform batch operations across multiple addresses, and retrieve detailed portfolio summaries.

## Overview

The `PortfolioClient` handles all portfolio-related operations including:

- **Single wallet balance queries** - Get detailed balance information for individual wallets
- **Batch balance operations** - Query multiple wallets simultaneously for efficiency
- **Portfolio summaries** - Comprehensive portfolio analytics with performance metrics
- **Token account analysis** - Detailed token holdings and distribution analysis

## Quick Start

```rust
use axiomtrade_rs::api::portfolio::PortfolioClient;

// Create portfolio client
let mut portfolio_client = PortfolioClient::new()?;

// Get single wallet balance
let balance = portfolio_client.get_balance("DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK").await?;
println!("SOL Balance: {:.6}", balance.sol_balance);
println!("Total Value: ${:.2}", balance.total_value_usd);
```

## Getting Wallet Balances

### Single Wallet Balance

Retrieve the complete balance information for a single Solana wallet address.

```rust
let wallet_address = "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK";
let balance = portfolio_client.get_balance(wallet_address).await?;

println!("SOL Balance: {:.6} SOL", balance.sol_balance);
println!("Total USD Value: ${:.2}", balance.total_value_usd);
println!("Token Count: {}", balance.token_balances.len());

// Access individual token balances
for (mint_address, token) in &balance.token_balances {
    println!("{} ({}): {} tokens (${:.2})", 
        token.symbol, 
        token.name,
        token.ui_amount, 
        token.value_usd
    );
}
```

**Response Structure:**
```rust
WalletBalance {
    sol_balance: f64,                              // SOL amount in native units
    token_balances: HashMap<String, TokenBalance>, // Token holdings by mint address
    total_value_usd: f64,                         // Total portfolio value in USD
}

TokenBalance {
    mint_address: String,     // Token mint address
    symbol: String,           // Token symbol (e.g., "USDC")
    name: String,             // Full token name
    amount: f64,              // Raw token amount
    decimals: u8,             // Token decimal places
    ui_amount: f64,           // Human-readable amount
    value_usd: f64,           // USD value of holding
    price_per_token: f64,     // Current token price
}
```

## Batch Balance Queries

For applications managing multiple wallets, batch queries provide significant performance improvements by fetching all balances in a single API call.

### Basic Batch Query

```rust
let wallet_addresses = vec![
    "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
    "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
    "7xLk17EQQ5KLDLDe44wCmupJKJjTGd8hs3eSVVhCx932".to_string(),
];

let batch_response = portfolio_client.get_batch_balance(&wallet_addresses).await?;

println!("Retrieved {} wallet balances", batch_response.balances.len());

let mut total_value = 0.0;
for (address, balance) in &batch_response.balances {
    println!("Wallet {}: {:.6} SOL (${:.2})", 
        &address[..8], 
        balance.sol_balance, 
        balance.total_value_usd
    );
    total_value += balance.total_value_usd;
}

println!("Combined portfolio value: ${:.2}", total_value);
```

### Advanced Batch Analysis

```rust
let batch_response = portfolio_client.get_batch_balance(&wallet_addresses).await?;

// Analyze token distribution across wallets
let mut token_counts = HashMap::new();
let mut token_values = HashMap::new();

for (address, balance) in &batch_response.balances {
    for token in balance.token_balances.values() {
        *token_counts.entry(token.symbol.clone()).or_insert(0) += 1;
        *token_values.entry(token.symbol.clone()).or_insert(0.0) += token.value_usd;
    }
}

// Find most common tokens
let mut sorted_tokens: Vec<_> = token_counts.iter().collect();
sorted_tokens.sort_by(|a, b| b.1.cmp(a.1));

for (token, count) in sorted_tokens.iter().take(5) {
    let total_value = token_values.get(*token).unwrap_or(&0.0);
    println!("{} - held by {} wallets, total value: ${:.2}", 
        token, count, total_value);
}
```

**Batch Response Structure:**
```rust
BatchBalanceResponse {
    balances: HashMap<String, WalletBalance>, // Wallet address -> balance data
    timestamp: i64,                          // Query timestamp
}
```

## Portfolio Summary

The portfolio summary provides comprehensive analytics including performance metrics, top positions, and transaction history.

### Getting Portfolio Summary

```rust
let wallet_addresses = vec![
    "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
    "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
];

let portfolio = portfolio_client.get_portfolio_summary(&wallet_addresses).await?;

// Overall balance statistics
println!("Total Value: {:.4} SOL", portfolio.balance_stats.total_value_sol);
println!("Available Balance: {:.4} SOL", portfolio.balance_stats.available_balance_sol);
println!("Unrealized PnL: {:.4} SOL", portfolio.balance_stats.unrealized_pnl_sol);

// Performance metrics
println!("1 Day PnL: {:.4} SOL", portfolio.performance_metrics.one_day.total_pnl);
println!("7 Day PnL: {:.4} SOL", portfolio.performance_metrics.seven_day.total_pnl);
println!("30 Day PnL: {:.4} SOL", portfolio.performance_metrics.thirty_day.total_pnl);
println!("All Time PnL: {:.4} SOL", portfolio.performance_metrics.all_time.total_pnl);
```

### Analyzing Top Positions

```rust
// Display top holdings
for (i, position) in portfolio.top_positions.iter().take(10).enumerate() {
    println!("{}. {} ({})", 
        i + 1, 
        position.symbol.as_deref().unwrap_or("Unknown"), 
        position.name.as_deref().unwrap_or("Unknown")
    );
    
    if let Some(amount) = position.amount {
        println!("   Amount: {:.4}", amount);
    }
    if let Some(value_usd) = position.value_usd {
        println!("   Value: ${:.2}", value_usd);
    }
    if let Some(pnl_percent) = position.pnl_percent {
        println!("   PnL: {}{:.2}%", 
            if pnl_percent >= 0.0 { "+" } else { "" },
            pnl_percent
        );
    }
}
```

### Transaction History

```rust
// Recent transactions
for tx in portfolio.transactions.iter().take(10) {
    if let (Some(symbol), Some(tx_type), Some(amount), Some(value_usd)) = 
        (&tx.symbol, &tx.transaction_type, tx.amount, tx.value_usd) {
        println!("{} {} {} for ${:.2}", tx_type, amount, symbol, value_usd);
        
        if let Some(timestamp) = tx.timestamp {
            let datetime = chrono::DateTime::from_timestamp(timestamp / 1000, 0)
                .unwrap_or_default();
            println!("   Time: {}", datetime.format("%Y-%m-%d %H:%M:%S UTC"));
        }
    }
}
```

## Portfolio Monitoring

For real-time portfolio tracking, you can implement continuous monitoring with change detection.

### Basic Monitoring Loop

```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

let mut last_portfolio = portfolio_client.get_portfolio_summary(&wallet_addresses).await?;
let initial_value = last_portfolio.balance_stats.total_value_sol;

loop {
    sleep(Duration::from_secs(30)).await;
    
    match portfolio_client.get_portfolio_summary(&wallet_addresses).await {
        Ok(current_portfolio) => {
            // Detect changes
            let value_change = current_portfolio.balance_stats.total_value_sol - 
                              last_portfolio.balance_stats.total_value_sol;
            
            if value_change.abs() > 0.001 {
                let change_pct = (value_change / last_portfolio.balance_stats.total_value_sol) * 100.0;
                println!("Portfolio value changed: {:+.6} SOL ({:+.2}%)", 
                    value_change, change_pct);
            }
            
            // Check for new positions
            if current_portfolio.active_positions.len() != last_portfolio.active_positions.len() {
                let pos_diff = current_portfolio.active_positions.len() as i32 - 
                              last_portfolio.active_positions.len() as i32;
                println!("Position count changed: {:+} positions", pos_diff);
            }
            
            last_portfolio = current_portfolio;
        }
        Err(e) => println!("Error fetching portfolio: {}", e),
    }
}
```

### Performance Tracking

```rust
let mut value_history = Vec::new();

// Track performance over time
value_history.push((Instant::now(), portfolio.balance_stats.total_value_sol));

// Calculate volatility (after collecting sufficient data points)
if value_history.len() >= 10 {
    let recent_values: Vec<f64> = value_history.iter()
        .rev()
        .take(10)
        .map(|(_, value)| *value)
        .collect();
    
    let mean = recent_values.iter().sum::<f64>() / recent_values.len() as f64;
    let variance = recent_values.iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>() / recent_values.len() as f64;
    
    let volatility = (variance.sqrt() / mean) * 100.0;
    println!("Portfolio volatility: {:.2}%", volatility);
}
```

## Token Account Analysis

Analyze token distribution and concentration across your portfolio.

### Position Size Analysis

```rust
let batch_response = portfolio_client.get_batch_balance(&wallet_addresses).await?;

// Aggregate all token positions
let mut token_summary = HashMap::new();

for (wallet_address, balance) in &batch_response.balances {
    for (mint, token) in &balance.token_balances {
        let entry = token_summary.entry(token.symbol.clone()).or_insert_with(|| {
            (0.0, 0.0, 0) // (total_balance, total_usd_value, wallet_count)
        });
        
        entry.0 += token.ui_amount;
        entry.1 += token.value_usd;
        entry.2 += 1;
    }
}

// Sort by total USD value
let mut sorted_tokens: Vec<_> = token_summary.iter().collect();
sorted_tokens.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());

// Display top holdings
println!("{:<15} {:>15} {:>15} {:>10}", "Symbol", "Total Balance", "USD Value", "Wallets");
println!("{}", "-".repeat(65));

for (symbol, (balance, usd_value, wallet_count)) in sorted_tokens.iter().take(10) {
    println!("{:<15} {:>15.6} {:>15.2} {:>10}", 
        symbol, balance, usd_value, wallet_count);
}
```

### Risk Analysis

```rust
// Calculate concentration risk
let total_value: f64 = sorted_tokens.iter().map(|(_, (_, usd_value, _))| usd_value).sum();
let top_5_value: f64 = sorted_tokens.iter()
    .take(5)
    .map(|(_, (_, usd_value, _))| usd_value)
    .sum();

let concentration_ratio = (top_5_value / total_value) * 100.0;
println!("Top 5 token concentration: {:.1}%", concentration_ratio);

if concentration_ratio > 70.0 {
    println!("WARNING: High concentration risk detected");
}

// Identify small positions
let small_positions = sorted_tokens.iter()
    .filter(|(_, (_, usd_value, _))| **usd_value < 5.0)
    .count();

println!("Positions under $5: {} ({:.1}%)", 
    small_positions, 
    (small_positions as f64 / sorted_tokens.len() as f64) * 100.0
);
```

## Complete API Reference

### PortfolioClient Methods

#### `new() -> Result<Self, PortfolioError>`
Creates a new portfolio client instance.

**Returns:** `Result<PortfolioClient, PortfolioError>`

#### `get_balance(wallet_address: &str) -> Result<WalletBalance, PortfolioError>`
Gets the balance for a single wallet address.

**Parameters:**
- `wallet_address: &str` - The Solana wallet address to query

**Returns:** `Result<WalletBalance, PortfolioError>`

#### `get_batch_balance(wallet_addresses: &[String]) -> Result<BatchBalanceResponse, PortfolioError>`
Gets balances for multiple wallet addresses in a single request.

**Parameters:**
- `wallet_addresses: &[String]` - Array of Solana wallet addresses

**Returns:** `Result<BatchBalanceResponse, PortfolioError>`

#### `get_portfolio_summary(wallet_addresses: &[String]) -> Result<PortfolioV5Response, PortfolioError>`
Gets comprehensive portfolio summary with performance metrics.

**Parameters:**
- `wallet_addresses: &[String]` - Array of wallet addresses to include in portfolio

**Returns:** `Result<PortfolioV5Response, PortfolioError>`

### Error Handling

All portfolio operations return `Result<T, PortfolioError>`. Handle errors appropriately:

```rust
match portfolio_client.get_balance(wallet_address).await {
    Ok(balance) => {
        // Process successful response
        println!("Balance: {:.6} SOL", balance.sol_balance);
    }
    Err(PortfolioError::InvalidWalletAddress(addr)) => {
        println!("Invalid wallet address: {}", addr);
    }
    Err(PortfolioError::AuthError(auth_err)) => {
        println!("Authentication error: {}", auth_err);
        // May need to re-authenticate
    }
    Err(PortfolioError::NetworkError(net_err)) => {
        println!("Network error: {}", net_err);
        // Retry with backoff
    }
    Err(PortfolioError::ApiError(api_err)) => {
        println!("API error: {}", api_err);
    }
    Err(e) => {
        println!("Unexpected error: {}", e);
    }
}
```

### Data Models

#### WalletBalance
```rust
pub struct WalletBalance {
    pub sol_balance: f64,                              // SOL balance in native units
    pub token_balances: HashMap<String, TokenBalance>, // Token holdings by mint
    pub total_value_usd: f64,                          // Total USD value
}
```

#### TokenBalance
```rust
pub struct TokenBalance {
    pub mint_address: String,     // Token mint address  
    pub symbol: String,           // Token symbol (e.g., "USDC")
    pub name: String,             // Full token name
    pub amount: f64,              // Raw token amount
    pub decimals: u8,             // Token decimal places
    pub ui_amount: f64,           // Human-readable amount
    pub value_usd: f64,           // USD value of holding
    pub price_per_token: f64,     // Current token price
}
```

#### PortfolioV5Response
```rust
pub struct PortfolioV5Response {
    pub active_positions: Vec<Position>,          // Current holdings
    pub history_positions: Vec<Position>,         // Historical positions
    pub top_positions: Vec<Position>,             // Top holdings by value
    pub transactions: Vec<Transaction>,           // Recent transactions
    pub balance_stats: BalanceStats,              // Balance summary
    pub performance_metrics: PerformanceMetrics, // Performance data
    pub chart_data: Vec<ChartDataPoint>,          // Chart data points
    pub calendar_data: Vec<CalendarDataPoint>,    // Calendar view data
}
```

#### BalanceStats
```rust
pub struct BalanceStats {
    pub total_value_sol: f64,       // Total portfolio value in SOL
    pub available_balance_sol: f64, // Available (liquid) balance
    pub unrealized_pnl_sol: f64,    // Unrealized profit/loss
}
```

## Best Practices

### Efficient Batch Operations
- Use batch queries for multiple wallets to reduce API calls
- Limit batch size to reasonable numbers (typically 50-100 wallets)
- Implement proper error handling for partial failures

### Performance Optimization
- Cache balance data when possible to reduce API load
- Use appropriate update intervals for monitoring (30-60 seconds minimum)
- Implement exponential backoff for failed requests

### Error Recovery
- Handle authentication errors by refreshing tokens
- Implement retry logic for transient network errors
- Validate wallet addresses before making API calls

### Memory Management
- Clean up historical data periodically in monitoring applications
- Avoid storing excessive amounts of portfolio history in memory
- Use streaming or pagination for large datasets

This portfolio management system provides comprehensive tools for tracking and analyzing Solana wallet portfolios, from simple balance queries to advanced risk analysis and real-time monitoring capabilities.