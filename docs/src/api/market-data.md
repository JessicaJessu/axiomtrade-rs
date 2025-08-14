# Market Data

The Market Data API provides comprehensive access to real-time and historical market information for Solana tokens. This includes trending tokens, price feeds, token analysis, market statistics, and search functionality.

## Overview

The `MarketDataClient` offers a complete suite of market data operations:

- **Trending Tokens**: Get tokens trending across different timeframes
- **Token Information**: Detailed token metadata and analysis
- **Price Feeds**: Real-time and historical price data
- **Market Statistics**: Overall market metrics and trends
- **Token Search**: Find tokens by name, symbol, or address
- **Chart Data**: OHLCV candle data for technical analysis

## Quick Start

```rust
use axiomtrade_rs::api::market_data::MarketDataClient;
use axiomtrade_rs::models::market::TimePeriod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MarketDataClient::new()?;
    
    // Get trending tokens
    let trending = client.get_trending_tokens(TimePeriod::TwentyFourHours).await?;
    
    println!("Top trending token: {} - {}", 
        trending[0].symbol, 
        trending[0].name
    );
    
    Ok(())
}
```

## Trending Tokens

Get tokens that are currently trending based on volume, price movement, and trading activity.

### Get Trending Tokens

```rust
use axiomtrade_rs::models::market::TimePeriod;

// Get 24-hour trending tokens
let trending = client.get_trending_tokens(TimePeriod::TwentyFourHours).await?;

for (i, token) in trending.iter().take(10).enumerate() {
    println!("{}. {} ({}) - ${:.8}", 
        i + 1,
        token.symbol,
        token.name,
        token.price_usd
    );
    println!("   Change: {:.2}% | Volume: ${:.2}", 
        token.price_change_24h,
        token.volume_24h
    );
}
```

### Time Periods

```rust
// Available time periods
TimePeriod::OneHour        // 1h trending
TimePeriod::TwentyFourHours // 24h trending  
TimePeriod::SevenDays      // 7d trending
TimePeriod::ThirtyDays     // 30d trending
```

### Trending Token Data Structure

```rust
pub struct TrendingToken {
    pub mint_address: String,      // Token mint address
    pub symbol: String,            // Token symbol (e.g., "BONK")
    pub name: String,              // Full token name
    pub price_usd: f64,           // Current price in USD
    pub price_change_24h: f64,    // 24h price change percentage
    pub price_change_7d: f64,     // 7d price change percentage  
    pub volume_24h: f64,          // 24h trading volume
    pub market_cap: f64,          // Market capitalization
    pub holders: u64,             // Number of token holders
    pub rank: u32,                // Trending rank
    pub logo_uri: Option<String>, // Token logo URL
}
```

## Token Information

Get detailed information about specific tokens including metadata, liquidity, and protocol details.

### Get Token Info by Symbol

```rust
// Get detailed token information
let token_info = client.get_token_info("BONK").await?;

println!("Token: {} ({})", token_info.name, token_info.symbol);
println!("Decimals: {}", token_info.decimals);
println!("Supply: {:.2}", token_info.supply);
println!("Liquidity SOL: {:.2}", token_info.liquidity_sol);
println!("Protocol: {}", token_info.protocol);
```

### Get Token Info by Address

```rust
// Get token info using mint or pair address
let address = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"; // Example BONK mint
let token_info = client.get_token_info_by_address(address).await?;

println!("Found token: {} at address {}", 
    token_info.symbol, 
    token_info.mint_address
);
```

### Token Information Data Structure

```rust
pub struct TokenInfo {
    pub mint_address: String,         // Token mint address
    pub symbol: String,               // Token symbol
    pub name: String,                 // Token name
    pub decimals: u8,                // Token decimals
    pub supply: f64,                 // Total supply
    pub liquidity_sol: f64,          // SOL liquidity
    pub liquidity_token: f64,        // Token liquidity
    pub pair_address: String,        // Pair contract address
    pub protocol: String,            // DEX protocol (Raydium, etc.)
    pub protocol_details: Option<Value>, // Additional protocol data
    pub created_at: String,          // Creation timestamp
    pub logo_uri: Option<String>,    // Token logo URL
    pub mint_authority: Option<String>, // Mint authority
    pub freeze_authority: Option<String>, // Freeze authority
    pub lp_burned: f64,              // LP tokens burned percentage
}
```

## Token Analysis

Get creator analysis and risk assessment for tokens.

### Get Token Analysis

```rust
// Get creator analysis and related tokens
let analysis = client.get_token_analysis("BONK").await?;

println!("Creator Risk Level: {}", analysis.creator_risk_level);
println!("Creator Rug Count: {}", analysis.creator_rug_count);
println!("Creator Token Count: {}", analysis.creator_token_count);

// Show related tokens from same creator
println!("Related high market cap tokens:");
for token in &analysis.top_market_cap_coins {
    println!("  {} - ${:.2} market cap", 
        token.symbol, 
        token.market_cap
    );
}
```

### Token Analysis Data Structure

```rust
pub struct TokenAnalysis {
    pub creator_risk_level: String,           // Risk assessment
    pub creator_rug_count: u32,              // Number of rugs by creator
    pub creator_token_count: u32,            // Total tokens by creator
    pub top_market_cap_coins: Vec<RelatedToken>, // High cap related tokens
    pub top_og_coins: Vec<RelatedToken>,     // Original related tokens
}

pub struct RelatedToken {
    pub mint_address: String,        // Token mint address
    pub symbol: String,              // Token symbol
    pub name: String,                // Token name
    pub pair_address: String,        // Pair address
    pub market_cap: f64,            // Market capitalization
    pub created_at: String,         // Creation date
    pub last_trade_time: String,    // Last trade timestamp
    pub image: Option<String>,      // Token image URL
    pub migrated: bool,             // Migration status
    pub bonding_curve_percent: f64, // Bonding curve completion
}
```

## Price Feeds

Get current and historical price data for tokens.

### Current Price

```rust
// Get current price for a token
let mint = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";
let price = client.get_token_price(mint).await?;

println!("Current price: ${:.8} USD", price.price_usd);
println!("Price in SOL: {:.6}", price.price_sol);
println!("Last updated: {}", price.timestamp);
```

### Historical Price Feed

```rust
// Get historical price data
let price_feed = client.get_price_feed(mint, TimePeriod::TwentyFourHours).await?;

println!("Price history for {}:", price_feed.mint_address);
for point in price_feed.prices.iter().take(10) {
    println!("  ${:.8} at {} (Volume: ${:.2})", 
        point.price_usd,
        point.timestamp,
        point.volume
    );
}
```

### Batch Prices

```rust
// Get prices for multiple tokens at once
let mints = vec![
    "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(), // BONK
    "So11111111111111111111111111111111111111112".to_string(),      // SOL
];

let prices = client.get_batch_prices(&mints).await?;

for price in prices {
    println!("{}: ${:.6}", price.mint_address, price.price_usd);
}
```

### Price Data Structures

```rust
pub struct PriceData {
    pub mint_address: String,    // Token mint address
    pub price_usd: f64,         // Price in USD
    pub price_sol: f64,         // Price in SOL
    pub timestamp: i64,         // Unix timestamp
}

pub struct PriceFeed {
    pub mint_address: String,       // Token mint address
    pub prices: Vec<PricePoint>,   // Historical price points
}

pub struct PricePoint {
    pub timestamp: i64,    // Unix timestamp
    pub price_usd: f64,   // Price in USD
    pub price_sol: f64,   // Price in SOL
    pub volume: f64,      // Trading volume
}
```

## Chart Data

Get OHLCV candle data for technical analysis and charting.

### Get Token Chart

```rust
use axiomtrade_rs::models::market::ChartTimeframe;

// Get 1-hour candles for a token
let chart = client.get_token_chart(
    mint,
    ChartTimeframe::OneHour,
    Some(100) // Limit to 100 candles
).await?;

println!("Chart data for {} ({})", chart.mint_address, chart.timeframe);

for candle in chart.candles.iter().take(10) {
    println!("  O: {:.6} H: {:.6} L: {:.6} C: {:.6} V: {:.2}",
        candle.open,
        candle.high,
        candle.low,
        candle.close,
        candle.volume
    );
}
```

### Chart Timeframes

```rust
ChartTimeframe::OneMinute      // 1m candles
ChartTimeframe::FiveMinutes    // 5m candles
ChartTimeframe::FifteenMinutes // 15m candles
ChartTimeframe::OneHour        // 1h candles
ChartTimeframe::FourHours      // 4h candles
ChartTimeframe::OneDay         // 1d candles
ChartTimeframe::OneWeek        // 1w candles
```

### Chart Data Structures

```rust
pub struct TokenChart {
    pub mint_address: String,         // Token mint address
    pub timeframe: ChartTimeframe,    // Chart timeframe
    pub candles: Vec<Candle>,        // OHLCV candles
}

pub struct Candle {
    pub timestamp: i64,    // Candle timestamp
    pub open: f64,        // Opening price
    pub high: f64,        // Highest price
    pub low: f64,         // Lowest price
    pub close: f64,       // Closing price
    pub volume: f64,      // Trading volume
}
```

## Market Statistics

Get overall market metrics and trends.

### Get Market Stats

```rust
// Get overall market statistics
let stats = client.get_market_stats().await?;

println!("Market Statistics:");
println!("  Total 24h Volume: ${:.2}", stats.total_volume_24h);
println!("  Total Market Cap: ${:.2}", stats.total_market_cap);
println!("  Active Traders: {}", stats.active_traders_24h);
println!("  Total Transactions: {}", stats.total_transactions_24h);
println!("  Trending Tokens: {}", stats.trending_tokens_count);
println!("  New Tokens (24h): {}", stats.new_tokens_24h);
```

### Market Statistics Data Structure

```rust
pub struct MarketStats {
    pub total_volume_24h: f64,         // Total 24h trading volume
    pub total_market_cap: f64,         // Total market capitalization
    pub active_traders_24h: u64,       // Active traders in 24h
    pub total_transactions_24h: u64,   // Total transactions in 24h
    pub trending_tokens_count: u32,    // Number of trending tokens
    pub new_tokens_24h: u32,          // New tokens launched in 24h
}
```

## Token Search

Search for tokens by name, symbol, or partial matches.

### Search Tokens

```rust
// Search for tokens
let results = client.search_tokens("bonk", Some(10)).await?;

println!("Search results for 'bonk':");
for (i, token) in results.results.iter().enumerate() {
    println!("{}. {} ({}) - ${:.8}", 
        i + 1,
        token.symbol,
        token.name,
        token.market_cap
    );
    
    if let Some(website) = &token.website {
        println!("   Website: {}", website);
    }
    
    if let Some(twitter) = &token.twitter {
        println!("   Twitter: {}", twitter);
    }
}
```

### Search Data Structures

```rust
pub struct TokenSearch {
    pub query: String,                      // Original search query
    pub results: Vec<TokenSearchResult>,   // Search results
}

pub struct TokenSearchResult {
    pub mint_address: String,        // Token mint address
    pub symbol: String,              // Token symbol
    pub name: String,                // Token name
    pub logo_uri: Option<String>,    // Token logo URL
    pub decimals: u8,               // Token decimals
    pub supply: f64,                // Total supply
    pub liquidity_sol: f64,         // SOL liquidity
    pub market_cap: f64,            // Market capitalization
    pub volume_sol: f64,            // SOL volume
    pub created_at: String,         // Creation timestamp
    pub pair_address: String,       // Pair address
    pub protocol: String,           // DEX protocol
    pub website: Option<String>,    // Project website
    pub twitter: Option<String>,    // Twitter handle
    pub telegram: Option<String>,   // Telegram channel
}
```

## Error Handling

The Market Data API uses comprehensive error handling for robust applications.

### Error Types

```rust
use axiomtrade_rs::api::market_data::MarketDataError;

match client.get_trending_tokens(TimePeriod::TwentyFourHours).await {
    Ok(tokens) => {
        println!("Found {} trending tokens", tokens.len());
    }
    Err(MarketDataError::AuthError(_)) => {
        println!("Authentication failed - check credentials");
    }
    Err(MarketDataError::TokenNotFound(symbol)) => {
        println!("Token not found: {}", symbol);
    }
    Err(MarketDataError::InvalidTokenMint(mint)) => {
        println!("Invalid mint address: {}", mint);
    }
    Err(MarketDataError::NetworkError(_)) => {
        println!("Network error - check connection");
    }
    Err(MarketDataError::ApiError(msg)) => {
        println!("API error: {}", msg);
    }
    Err(e) => {
        println!("Unexpected error: {}", e);
    }
}
```

## Complete Example

Here's a comprehensive example showing various market data operations:

```rust
use axiomtrade_rs::api::market_data::MarketDataClient;
use axiomtrade_rs::models::market::{TimePeriod, ChartTimeframe};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let mut client = MarketDataClient::new()?;
    
    // Get trending tokens
    println!("=== TRENDING TOKENS ===");
    let trending = client.get_trending_tokens(TimePeriod::TwentyFourHours).await?;
    
    for (i, token) in trending.iter().take(5).enumerate() {
        println!("{}. {} - ${:.8} ({:.2}%)", 
            i + 1,
            token.symbol,
            token.price_usd,
            token.price_change_24h
        );
    }
    
    // Get detailed info for top token
    if let Some(top_token) = trending.first() {
        println!("\n=== TOKEN DETAILS ===");
        let token_info = client.get_token_info(&top_token.symbol).await?;
        
        println!("Name: {}", token_info.name);
        println!("Symbol: {}", token_info.symbol);
        println!("Supply: {:.2}", token_info.supply);
        println!("Liquidity: {:.2} SOL", token_info.liquidity_sol);
        
        // Get price history
        println!("\n=== PRICE HISTORY ===");
        let price_feed = client.get_price_feed(
            &token_info.mint_address, 
            TimePeriod::TwentyFourHours
        ).await?;
        
        println!("Last 5 price points:");
        for point in price_feed.prices.iter().rev().take(5) {
            println!("  ${:.8} (Volume: {:.2})", 
                point.price_usd, 
                point.volume
            );
        }
        
        // Get chart data
        println!("\n=== CHART DATA ===");
        let chart = client.get_token_chart(
            &token_info.mint_address,
            ChartTimeframe::OneHour,
            Some(5)
        ).await?;
        
        println!("Recent candles:");
        for candle in &chart.candles {
            println!("  OHLC: {:.6}/{:.6}/{:.6}/{:.6}", 
                candle.open, 
                candle.high, 
                candle.low, 
                candle.close
            );
        }
    }
    
    // Get market statistics
    println!("\n=== MARKET STATS ===");
    let stats = client.get_market_stats().await?;
    println!("Total Volume: ${:.2}", stats.total_volume_24h);
    println!("Active Traders: {}", stats.active_traders_24h);
    
    // Search for tokens
    println!("\n=== TOKEN SEARCH ===");
    let search_results = client.search_tokens("sol", Some(3)).await?;
    
    for result in &search_results.results {
        println!("{} ({}) - ${:.2} market cap", 
            result.symbol,
            result.name,
            result.market_cap
        );
    }
    
    Ok(())
}
```

## Best Practices

### Rate Limiting

The API includes built-in rate limiting. For high-frequency applications:

```rust
use tokio::time::{sleep, Duration};

// Add delays between requests for bulk operations
for symbol in token_symbols {
    let info = client.get_token_info(&symbol).await?;
    // Process info...
    
    sleep(Duration::from_millis(100)).await; // Prevent rate limiting
}
```

### Error Recovery

Implement retry logic for network failures:

```rust
use tokio::time::{sleep, Duration};

async fn get_trending_with_retry(
    client: &mut MarketDataClient,
    period: TimePeriod,
    max_retries: u32
) -> Result<Vec<TrendingToken>, MarketDataError> {
    for attempt in 0..max_retries {
        match client.get_trending_tokens(period.clone()).await {
            Ok(tokens) => return Ok(tokens),
            Err(MarketDataError::NetworkError(_)) if attempt < max_retries - 1 => {
                sleep(Duration::from_millis(1000 * (attempt + 1) as u64)).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    
    unreachable!()
}
```

### Batch Operations

Use batch endpoints when possible for better performance:

```rust
// Instead of multiple individual calls
// let price1 = client.get_token_price(mint1).await?;
// let price2 = client.get_token_price(mint2).await?;

// Use batch endpoint
let mints = vec![mint1.to_string(), mint2.to_string()];
let prices = client.get_batch_prices(&mints).await?;
```

### Data Validation

Always validate mint addresses before API calls:

```rust
fn is_valid_mint_address(address: &str) -> bool {
    address.len() >= 32 && 
    address.len() <= 44 && 
    address.chars().all(|c| c.is_ascii_alphanumeric())
}

if is_valid_mint_address(&mint_address) {
    let price = client.get_token_price(&mint_address).await?;
} else {
    println!("Invalid mint address format");
}
```

The Market Data API provides comprehensive access to Solana token market information, enabling developers to build sophisticated trading applications, market analysis tools, and portfolio management systems.