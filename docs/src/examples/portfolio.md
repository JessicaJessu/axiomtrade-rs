# Portfolio Examples

This section demonstrates comprehensive portfolio management capabilities using the Axiom Trade Rust client. These examples show how to retrieve, monitor, and analyze portfolio data across multiple wallets.

## Overview

The portfolio examples showcase four key areas of functionality:

1. **Basic Portfolio Retrieval** - Getting complete portfolio summaries with performance metrics
2. **Batch Balance Operations** - Efficiently querying multiple wallets simultaneously  
3. **Real-time Monitoring** - Continuous portfolio tracking with alerts and change detection
4. **Token Account Analysis** - Deep analysis of token holdings, distribution, and risk assessment

All examples require proper authentication and environment setup as described in the [Authentication Examples](authentication.md).

## 1. Basic Portfolio Retrieval (`get_portfolio.rs`)

### Overview

The basic portfolio retrieval example demonstrates how to fetch comprehensive portfolio information including balance statistics, performance metrics, top positions, and recent transactions.

### Key Features

- Complete portfolio summary with SOL values
- Performance metrics (1-day, 7-day, 30-day, all-time PnL)
- Top position analysis with profit/loss percentages
- Recent transaction history
- Individual wallet balance breakdowns
- Token holding details with market values

### Example Usage

```rust
use axiomtrade_rs::api::portfolio::PortfolioClient;
use axiomtrade_rs::auth::AuthClient;

// Authenticate and create portfolio client
let mut auth_client = AuthClient::new()?;
auth_client.login(&email, &password, None).await?;
let mut portfolio_client = PortfolioClient::new()?;

// Define wallets to analyze
let wallet_addresses = vec![
    "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
    "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
];

// Get comprehensive portfolio summary
let portfolio = portfolio_client.get_portfolio_summary(&wallet_addresses).await?;

// Access portfolio data
println!("Total Value: {:.4} SOL", portfolio.balance_stats.total_value_sol);
println!("Available Balance: {:.4} SOL", portfolio.balance_stats.available_balance_sol);
println!("Unrealized PnL: {:.4} SOL", portfolio.balance_stats.unrealized_pnl_sol);
```

### Output Example

```
📊 Portfolio Summary
====================
  Total Value SOL: 125.4567 SOL
  Available SOL: 98.7654 SOL
  Unrealized PnL: 12.3456 SOL

📈 Performance Metrics:
  1 Day PnL: 2.3456 SOL
  7 Day PnL: 8.9012 SOL
  30 Day PnL: 15.6789 SOL
  All Time PnL: 45.2341 SOL

💎 Top Positions:
  1. BONK (Bonk Inu)
     Amount: 1234567.8900
     Value: $567.89
     PnL: +12.34%
```

## 2. Batch Balance Queries (`batch_balances.rs`)

### Overview

This example demonstrates efficient batch querying capabilities for retrieving balance information across multiple wallets in a single API call, providing significant performance improvements over individual queries.

### Key Features

- Single API call for multiple wallet balances
- Comprehensive balance statistics and aggregation
- Token distribution analysis across wallets
- Performance metrics and efficiency calculations
- Fallback to individual queries on batch failure
- Rich portfolio analytics and reporting

### Efficiency Benefits

- **API Calls**: 1 batch call vs N individual calls
- **Network Overhead**: Reduced latency and bandwidth usage
- **Rate Limiting**: More efficient use of API quotas
- **Data Consistency**: Snapshot of all balances at the same time

### Example Usage

```rust
// Define multiple wallet addresses
let wallet_addresses = vec![
    "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
    "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
    "7xLk17EQQ5KLDLDe44wCmupJKJjTGd8hs3eSVVhCx932".to_string(),
    // ... more wallets
];

// Perform batch balance query
let batch_response = portfolio_client.get_batch_balance(&wallet_addresses).await?;

// Analyze results
for (address, balance) in batch_response.balances.iter() {
    println!("Wallet: {}...", &address[..8]);
    println!("  SOL Balance: {:.6} SOL", balance.sol_balance);
    println!("  Total Value: ${:.2}", balance.total_value_usd);
    println!("  Token Count: {}", balance.token_balances.len());
}
```

### Output Example

```
📊 Performing batch balance query...
✓ Batch query successful!

Address         SOL Balance     USD Value    Tokens
-----------------------------------------------------------------
DYw8jC...          12.345678      1,234.56        15
5FHwkr...          23.456789      2,345.67         8
7xLk17...           5.678901        567.89        22
-----------------------------------------------------------------
TOTAL              41.481368      4,148.12

📈 Token Distribution Analysis:
  Most common tokens:
    USDC - held by 3 wallet(s), total value: $1,234.56
    BONK - held by 2 wallet(s), total value: $567.89
    SOL - held by 3 wallet(s), total value: $2,345.67

📦 Batch Efficiency:
  Single queries would require: 3 API calls
  Batch query used: 1 API call
  Efficiency gain: 3x
```

## 3. Real-time Portfolio Monitoring (`portfolio_monitoring.rs`)

### Overview

This example implements continuous real-time monitoring of portfolio changes with configurable update intervals, change detection algorithms, performance tracking, and alert systems.

### Key Features

- **Continuous Monitoring**: Configurable update intervals (default 30 seconds)
- **Change Detection**: Automatic detection of value, balance, and position changes
- **Performance Analysis**: Historical tracking with volatility calculations
- **Alert System**: Configurable thresholds for significant changes
- **Memory Management**: Efficient historical data storage with cleanup
- **Error Handling**: Robust error recovery and authentication refresh

### Monitoring Capabilities

#### Change Detection
- Total portfolio value changes (SOL and percentage)
- Available balance fluctuations
- Active position count changes
- New transaction detection

#### Performance Analysis
- Total performance since monitoring start
- Recent trend analysis (last 5 data points)
- Volatility calculation using standard deviation
- Real-time profit/loss tracking

#### Alert System
- Portfolio value change alerts (>5% threshold)
- New position alerts
- New transaction alerts
- Custom threshold configuration

### Example Usage

```rust
// Initialize monitoring
let demo_wallets = vec![
    "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
    "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
];

let mut last_portfolio = client.get_portfolio_summary(&demo_wallets).await?;
let initial_value = last_portfolio.balance_stats.total_value_sol;

// Monitoring loop
loop {
    sleep(Duration::from_secs(30)).await;
    
    let current_portfolio = client.get_portfolio_summary(&demo_wallets).await?;
    let changes = detect_portfolio_changes(&last_portfolio, &current_portfolio);
    
    // Process changes and generate alerts
    if !changes.is_empty() {
        for change in &changes {
            println!("  {}", change);
        }
    }
    
    // Performance and alert analysis
    analyze_performance(&total_value_history, initial_value);
    check_alerts(&current_portfolio, &last_portfolio);
    
    last_portfolio = current_portfolio;
}
```

### Output Example

```
============================================================
Update #3 - 30.2s since last update

Portfolio Summary:
  Active positions: 15
  Total SOL: 125.456789
  Available SOL: 98.765432
  Unrealized PnL: 12.345678 SOL
  Total transactions: 45

Detected Changes:
  Total value: +2.345678 SOL (+1.90%)
  New transactions: +2

Performance Analysis:
  Since start: $45.67 (+2.34%)
  Recent trend: $12.34 (+1.12%)
  Volatility: 3.45%

ALERT: 2 new transactions detected
```

## 4. Token Account Analysis (`token_accounts.rs`)

### Overview

This sophisticated example provides comprehensive analysis of token holdings across multiple wallets, including position sizing, risk assessment, portfolio optimization suggestions, and distribution analysis.

### Key Features

- **Comprehensive Token Analysis**: Aggregated view across all wallets
- **Risk Assessment**: Concentration risk and position size analysis  
- **Portfolio Optimization**: Automated suggestions for improvement
- **Distribution Analysis**: Position size buckets and diversity metrics
- **Wallet Diversity**: Token distribution across wallet addresses
- **Performance Metrics**: Position sizing and value calculations

### Analysis Capabilities

#### Token Aggregation
- Total balance and USD value per token across all wallets
- Wallet count per token (distribution analysis)
- Average position size calculations
- Comprehensive position tracking

#### Risk Analysis
- **Concentration Risk**: Top 5 holdings percentage of total portfolio
- **Small Position Analysis**: Identification of positions under $5
- **Distribution Buckets**: Position categorization (<$1, $1-10, $10-100, $100-1K, >$1K)
- **Wallet Diversity**: Analysis of token distribution across wallets

#### Optimization Suggestions
- Concentration reduction recommendations
- Small position consolidation advice
- Dust position cleanup suggestions
- Well-distributed holdings identification

### Example Usage

```rust
// Get batch balances for analysis
let batch_response = client.get_batch_balance(&demo_wallets).await?;

// Analyze token distribution
let mut token_summary: HashMap<String, TokenSummary> = HashMap::new();

for (wallet_address, wallet_balance) in &batch_response.balances {
    // Process SOL balance
    if wallet_balance.sol_balance > 0.0 {
        let sol_value_usd = wallet_balance.sol_balance * 100.0; // SOL price estimate
        // Add to token summary...
    }
    
    // Process token balances
    for (mint_address, token_balance) in &wallet_balance.token_balances {
        // Aggregate token data across wallets...
    }
}

// Generate analysis and optimization suggestions
analyze_concentration_risk(&sorted_tokens, total_value);
analyze_position_distribution(&sorted_tokens);
generate_optimization_suggestions(&sorted_tokens);
```

### Output Example

```
Token Portfolio Summary:
Total positions: 147
Total portfolio value: $12,345.67
Unique tokens: 23

Top 10 Token Holdings:
Symbol          Total Balance       USD Value    Wallets  Avg Position
--------------------------------------------------------------------------------
SOL                    45.678900      4,567.89         3      1,522.63
USDC                2,345.123400      2,345.12         2      1,172.56
BONK           12,345,678.900000      1,234.57         1      1,234.57

Risk Analysis:
Top 5 tokens concentration: 78.9%
  WARNING: High concentration risk detected
Positions under $5: 8 (34.8%)

Position Size Distribution:
  <$1: 5 positions (21.7%)
  $1-10: 8 positions (34.8%)
  $10-100: 6 positions (26.1%)
  $100-1K: 3 positions (13.0%)
  >$1K: 1 positions (4.3%)

Optimization Suggestions:
- Consider reducing concentration in top holdings
- Consider consolidating or closing positions under $5
- Consider cleaning up 5 dust positions

Well Distributed Holdings:
  USDC - 2 wallets, $2,345.12
  SOL - 3 wallets, $4,567.89
```

## Practical Use Cases

### 1. Portfolio Dashboards
Use the portfolio retrieval and monitoring examples to build real-time dashboard applications showing:
- Current portfolio values and performance
- Position changes and alerts
- Historical performance tracking
- Risk metrics and recommendations

### 2. Automated Rebalancing
Combine token account analysis with trading capabilities to:
- Identify overweight positions requiring rebalancing
- Detect underweight allocations needing increases
- Automate portfolio rebalancing based on target allocations
- Implement risk management rules

### 3. Multi-Wallet Management
Use batch balance queries for:
- Managing institutional portfolios across multiple wallets
- Consolidated reporting for multiple trading strategies
- Risk assessment across wallet boundaries
- Efficient monitoring of large wallet collections

### 4. Risk Management Systems
Implement automated risk monitoring using:
- Real-time position monitoring with configurable alerts
- Concentration risk detection and warnings
- Position size limit enforcement
- Automated stop-loss and profit-taking triggers

### 5. Performance Analytics
Build comprehensive analytics systems featuring:
- Historical performance tracking and reporting
- Volatility analysis and risk-adjusted returns
- Benchmark comparisons and relative performance
- Attribution analysis across tokens and time periods

### 6. Compliance and Reporting
Generate regulatory and internal reports using:
- Complete transaction history with timestamps
- Position statements at specific points in time
- Profit/loss calculations for tax reporting
- Audit trails for trading activity

## Running the Examples

### Prerequisites

1. **Environment Setup**: Ensure your `.env` file contains:
   ```
   AXIOM_EMAIL=your_email@example.com
   AXIOM_PASSWORD=your_password
   ```

2. **Dependencies**: Make sure all required dependencies are installed:
   ```bash
   cargo build
   ```

### Execution

Run individual examples using:

```bash
# Basic portfolio retrieval
cargo run --example get_portfolio

# Batch balance queries  
cargo run --example batch_balances

# Real-time monitoring
cargo run --example portfolio_monitoring

# Token account analysis
cargo run --example token_accounts
```

### Customization

Each example can be customized by:
- Modifying wallet addresses to use your actual wallets
- Adjusting monitoring intervals and alert thresholds
- Changing analysis parameters and risk thresholds
- Adding additional metrics and reporting features

### Error Handling

All examples include comprehensive error handling for:
- Authentication failures and token expiration
- Network connectivity issues
- API rate limiting and throttling
- Invalid wallet addresses or missing data
- Graceful degradation with fallback mechanisms

These examples provide a solid foundation for building sophisticated portfolio management applications using the Axiom Trade Rust client.
