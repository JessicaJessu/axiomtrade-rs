# WebSocket Streaming

The Axiom Trade Rust client provides comprehensive WebSocket support for real-time data streaming. The WebSocket module handles authentication, connection management, automatic reconnection, and provides a flexible message handling system.

## Overview

The WebSocket implementation consists of three main components:

- **WebSocketClient**: Core client for managing connections and subscriptions
- **MessageHandler**: Trait for processing incoming messages
- **Message Types**: Structured data types for different streaming data

## Basic WebSocket Connection

### Setting Up a Connection

```rust
use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler, WebSocketMessage};
use async_trait::async_trait;
use std::sync::Arc;

// Create a message handler
struct MyMessageHandler;

#[async_trait]
impl MessageHandler for MyMessageHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                println!("Market Update: {} - ${:.6}", update.symbol, update.price_usd);
            }
            _ => {}
        }
    }

    async fn on_connected(&self, session_id: String) {
        println!("Connected with session: {}", session_id);
    }

    async fn on_disconnected(&self, reason: String) {
        println!("Disconnected: {}", reason);
    }

    async fn on_error(&self, error: String) {
        println!("Error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create WebSocket client
    let handler = Arc::new(MyMessageHandler);
    let mut ws_client = WebSocketClient::new(handler)?;
    
    // Connect to WebSocket
    ws_client.connect().await?;
    
    // Subscribe to data feeds
    ws_client.subscribe_new_tokens().await?;
    
    // Keep connection alive
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    
    // Disconnect gracefully
    ws_client.disconnect().await;
    
    Ok(())
}
```

## Regional Connection Options

The WebSocket client supports multiple regional endpoints for optimal latency:

```rust
use axiomtrade_rs::websocket::{WebSocketClient, Region};

// Create client with specific region
let handler = Arc::new(MyMessageHandler);
let mut ws_client = WebSocketClient::with_region(handler, Region::USWest)?;

// Available regions:
// - Region::USWest
// - Region::USCentral  
// - Region::USEast
// - Region::EUWest
// - Region::EUCentral
// - Region::EUEast
// - Region::Asia
// - Region::Australia
// - Region::Global (default)
```

## MessageHandler Trait Implementation

The `MessageHandler` trait defines how your application processes incoming WebSocket messages:

```rust
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handles incoming WebSocket messages
    async fn handle_message(&self, message: WebSocketMessage);
    
    /// Called when connection is established
    async fn on_connected(&self, session_id: String);
    
    /// Called when connection is lost
    async fn on_disconnected(&self, reason: String);
    
    /// Called when an error occurs
    async fn on_error(&self, error: String);
}
```

### Advanced Message Handler Example

```rust
use tokio::sync::RwLock;
use std::collections::HashMap;

struct AdvancedMessageHandler {
    price_data: Arc<RwLock<HashMap<String, f64>>>,
    order_book: Arc<RwLock<HashMap<String, OrderBookData>>>,
}

impl AdvancedMessageHandler {
    fn new() -> Self {
        Self {
            price_data: Arc::new(RwLock::new(HashMap::new())),
            order_book: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    async fn get_latest_price(&self, token: &str) -> Option<f64> {
        self.price_data.read().await.get(token).copied()
    }
}

#[async_trait]
impl MessageHandler for AdvancedMessageHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                // Store price data
                let mut prices = self.price_data.write().await;
                prices.insert(update.symbol.clone(), update.price_usd);
                
                // Process price alerts
                if update.price_change_24h > 10.0 {
                    println!("🚀 {} is up {:.2}% in 24h!", 
                        update.symbol, update.price_change_24h);
                }
            }
            
            WebSocketMessage::OrderUpdate(order) => {
                println!("Order {} status: {:?}", order.order_id, order.status);
                
                if let Some(tx_hash) = &order.transaction_hash {
                    println!("Transaction: {}", tx_hash);
                }
            }
            
            WebSocketMessage::TradeUpdate(trade) => {
                println!("Trade executed: {} {} at ${:.6}",
                    trade.amount, trade.token_mint, trade.price);
            }
            
            WebSocketMessage::BalanceUpdate(balance) => {
                println!("Portfolio value: ${:.2}", balance.total_value_usd);
                for token in &balance.token_balances {
                    println!("  {}: {} (${:.2})", 
                        token.symbol, token.amount, token.value_usd);
                }
            }
            
            WebSocketMessage::Error { code, message } => {
                eprintln!("WebSocket error {}: {}", code, message);
            }
            
            _ => {}
        }
    }
    
    async fn on_connected(&self, session_id: String) {
        println!("✅ WebSocket connected! Session: {}", session_id);
    }
    
    async fn on_disconnected(&self, reason: String) {
        println!("❌ WebSocket disconnected: {}", reason);
        // Implement custom reconnection logic here if needed
    }
    
    async fn on_error(&self, error: String) {
        eprintln!("⚠️ WebSocket error: {}", error);
    }
}
```

## Subscription Types

The WebSocket client supports various data feed subscriptions:

### New Token Listings

Subscribe to newly launched tokens on the platform:

```rust
// Subscribe to all new token pairs
ws_client.subscribe_new_tokens().await?;
```

### Token Price Updates

Monitor real-time price changes for specific tokens:

```rust
// Subscribe to price updates for a specific token
let bonk_address = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";
ws_client.subscribe_token_price(bonk_address).await?;

// Subscribe to multiple tokens
let tokens = vec![
    "So11111111111111111111111111111111111111112", // SOL
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // BONK
];

for token in tokens {
    ws_client.subscribe_token_price(token).await?;
}
```

### Wallet Transaction Monitoring

Track transactions for specific wallet addresses:

```rust
// Monitor transactions for a wallet
let wallet_address = "YourWalletAddressHere";
ws_client.subscribe_wallet_transactions(wallet_address).await?;
```

## Real-Time Data Handling

### Market Update Structure

```rust
pub struct MarketUpdate {
    pub token_mint: String,      // Token contract address
    pub symbol: String,          // Token symbol (e.g., "BONK")
    pub price_usd: f64,         // Current price in USD
    pub price_sol: f64,         // Current price in SOL
    pub price_change_24h: f64,  // 24-hour price change percentage
    pub volume_24h: f64,        // 24-hour trading volume
    pub market_cap: f64,        // Market capitalization
    pub timestamp: i64,         // Unix timestamp
}
```

### Order Update Structure

```rust
pub struct OrderUpdate {
    pub order_id: String,           // Unique order identifier
    pub status: OrderStatus,        // Order status (Pending, Filled, etc.)
    pub transaction_hash: Option<String>, // Blockchain transaction hash
    pub token_mint: String,         // Token being traded
    pub side: OrderSide,           // Buy or Sell
    pub amount: f64,               // Order amount
    pub price: f64,                // Order price
    pub filled_amount: f64,        // Amount filled so far
    pub timestamp: i64,            // Order timestamp
    pub error_message: Option<String>, // Error message if failed
}

pub enum OrderStatus {
    Pending,
    Processing,
    PartiallyFilled,
    Filled,
    Cancelled,
    Failed,
}

pub enum OrderSide {
    Buy,
    Sell,
}
```

### Price Tracking Example

```rust
use std::collections::HashMap;
use tokio::sync::RwLock;

struct PriceTracker {
    prices: HashMap<String, PriceData>,
    alerts: Vec<PriceAlert>,
}

struct PriceData {
    current_price: f64,
    high_24h: f64,
    low_24h: f64,
    volume_24h: f64,
    last_updated: std::time::Instant,
}

struct PriceAlert {
    token: String,
    target_price: f64,
    condition: AlertCondition, // Above, Below
}

impl PriceTracker {
    async fn update_price(&mut self, symbol: &str, price: f64) {
        // Update price data
        let price_data = self.prices.entry(symbol.to_string())
            .or_insert(PriceData {
                current_price: price,
                high_24h: price,
                low_24h: price,
                volume_24h: 0.0,
                last_updated: std::time::Instant::now(),
            });
        
        price_data.current_price = price;
        price_data.high_24h = price_data.high_24h.max(price);
        price_data.low_24h = price_data.low_24h.min(price);
        price_data.last_updated = std::time::Instant::now();
        
        // Check price alerts
        for alert in &self.alerts {
            if alert.token == symbol {
                match alert.condition {
                    AlertCondition::Above if price > alert.target_price => {
                        println!("🔔 ALERT: {} is above ${:.6}!", symbol, alert.target_price);
                    }
                    AlertCondition::Below if price < alert.target_price => {
                        println!("🔔 ALERT: {} is below ${:.6}!", symbol, alert.target_price);
                    }
                    _ => {}
                }
            }
        }
    }
}
```

## Connection Management and Reconnection Logic

### Automatic Reconnection

The WebSocket client includes built-in reconnection logic:

```rust
// Enable automatic reconnection (enabled by default)
ws_client.set_auto_reconnect(true);

// The client will automatically:
// 1. Refresh authentication tokens every 10 minutes
// 2. Reconnect if the connection is lost
// 3. Re-subscribe to all previous subscriptions
```

### Manual Reconnection

```rust
// Check connection status
if !ws_client.is_connected().await {
    println!("Connection lost, reconnecting...");
    
    match ws_client.reconnect().await {
        Ok(()) => {
            println!("Reconnected successfully");
            
            // Re-subscribe to feeds if needed
            ws_client.subscribe_new_tokens().await?;
        }
        Err(e) => {
            eprintln!("Reconnection failed: {}", e);
        }
    }
}
```

### Connection Health Monitoring

```rust
use tokio::time::{interval, Duration};

async fn monitor_connection(ws_client: &mut WebSocketClient) {
    let mut health_check = interval(Duration::from_secs(30));
    
    loop {
        health_check.tick().await;
        
        if !ws_client.is_connected().await {
            println!("⚠️ Connection lost, attempting reconnection...");
            
            match ws_client.reconnect().await {
                Ok(()) => {
                    println!("✅ Reconnection successful");
                }
                Err(e) => {
                    eprintln!("❌ Reconnection failed: {}", e);
                    // Implement exponential backoff or other retry logic
                }
            }
        }
        
        // Display current subscriptions
        let subs = ws_client.get_subscriptions().await;
        println!("Active subscriptions: {:?}", subs);
    }
}
```

## Token Price WebSocket

For dedicated price monitoring, use the token price WebSocket:

```rust
// Connect to the specialized token price stream
let mut price_ws = WebSocketClient::new(handler)?;
price_ws.connect_token_price().await?;

// This uses socket8.axiom.trade specifically for price data
```

## Error Handling

### WebSocket Error Types

```rust
pub enum WebSocketError {
    AuthError(AuthError),           // Authentication failure
    ConnectionError(String),        // Connection issues
    NotConnected,                  // Not connected when operation attempted
    SendError(String),             // Failed to send message
    ReceiveError(String),          // Failed to receive message
    SerializationError(String),    // JSON parsing errors
    WebSocketError(tokio_tungstenite::tungstenite::Error), // Low-level WebSocket errors
    HttpError(http::Error),        // HTTP upgrade errors
}
```

### Robust Error Handling Example

```rust
async fn robust_websocket_connection() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(MyMessageHandler::new());
    let mut ws_client = WebSocketClient::new(handler)?;
    
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 5;
    
    loop {
        match ws_client.connect().await {
            Ok(()) => {
                println!("✅ Connected successfully");
                break;
            }
            Err(e) => {
                retry_count += 1;
                eprintln!("❌ Connection failed (attempt {}): {}", retry_count, e);
                
                if retry_count >= MAX_RETRIES {
                    return Err(format!("Failed to connect after {} attempts", MAX_RETRIES).into());
                }
                
                // Exponential backoff
                let delay = Duration::from_secs(2_u64.pow(retry_count));
                println!("⏳ Retrying in {:?}...", delay);
                tokio::time::sleep(delay).await;
            }
        }
    }
    
    // Subscribe with error handling
    if let Err(e) = ws_client.subscribe_new_tokens().await {
        eprintln!("⚠️ Failed to subscribe to new tokens: {}", e);
    }
    
    Ok(())
}
```

## Best Practices

### 1. Use Arc for Shared Message Handlers

```rust
// Good: Use Arc for shared ownership
let handler = Arc::new(MyMessageHandler::new());
let mut ws_client = WebSocketClient::new(handler.clone())?;

// The handler can be safely shared across async tasks
let handler_clone = handler.clone();
tokio::spawn(async move {
    // Use handler_clone in another task
});
```

### 2. Implement Connection Pooling for Multiple Streams

```rust
struct WebSocketManager {
    connections: HashMap<String, WebSocketClient>,
    handler: Arc<dyn MessageHandler>,
}

impl WebSocketManager {
    async fn create_connection(&mut self, name: &str, region: Region) -> Result<(), WebSocketError> {
        let mut client = WebSocketClient::with_region(self.handler.clone(), region)?;
        client.connect().await?;
        self.connections.insert(name.to_string(), client);
        Ok(())
    }
    
    async fn subscribe_to_token(&mut self, connection_name: &str, token: &str) -> Result<(), WebSocketError> {
        if let Some(client) = self.connections.get_mut(connection_name) {
            client.subscribe_token_price(token).await
        } else {
            Err(WebSocketError::NotConnected)
        }
    }
}
```

### 3. Rate Limiting Subscriptions

```rust
use tokio::time::{interval, Duration};

async fn subscribe_with_rate_limit(
    ws_client: &mut WebSocketClient,
    tokens: Vec<&str>
) -> Result<(), WebSocketError> {
    let mut rate_limiter = interval(Duration::from_millis(100));
    
    for token in tokens {
        rate_limiter.tick().await;
        
        match ws_client.subscribe_token_price(token).await {
            Ok(()) => {
                println!("✅ Subscribed to {}", token);
            }
            Err(e) => {
                eprintln!("❌ Failed to subscribe to {}: {}", token, e);
            }
        }
    }
    
    Ok(())
}
```

### 4. Graceful Shutdown

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(MyMessageHandler::new());
    let mut ws_client = WebSocketClient::new(handler)?;
    
    ws_client.connect().await?;
    ws_client.subscribe_new_tokens().await?;
    
    // Set up graceful shutdown
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("🛑 Shutdown signal received");
        }
        _ = monitor_websocket(&mut ws_client) => {
            println!("WebSocket monitoring ended");
        }
    }
    
    // Graceful disconnect
    println!("Disconnecting...");
    ws_client.disconnect().await;
    println!("✅ Disconnected gracefully");
    
    Ok(())
}

async fn monitor_websocket(ws_client: &mut WebSocketClient) {
    loop {
        if !ws_client.is_connected().await {
            if let Err(e) = ws_client.reconnect().await {
                eprintln!("Failed to reconnect: {}", e);
                break;
            }
        }
        
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
```

## Complete Example: Multi-Token Price Monitor

```rust
use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler, WebSocketMessage};
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

struct PriceMonitorHandler {
    prices: Arc<RwLock<HashMap<String, f64>>>,
    alerts: Arc<RwLock<Vec<PriceAlert>>>,
}

struct PriceAlert {
    symbol: String,
    target_price: f64,
    triggered: bool,
}

#[async_trait]
impl MessageHandler for PriceMonitorHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        if let WebSocketMessage::MarketUpdate(update) = message {
            // Update price
            {
                let mut prices = self.prices.write().await;
                prices.insert(update.symbol.clone(), update.price_usd);
            }
            
            // Check alerts
            {
                let mut alerts = self.alerts.write().await;
                for alert in alerts.iter_mut() {
                    if alert.symbol == update.symbol && !alert.triggered {
                        if update.price_usd >= alert.target_price {
                            println!("🚨 PRICE ALERT: {} reached ${:.6}!", 
                                update.symbol, update.price_usd);
                            alert.triggered = true;
                        }
                    }
                }
            }
            
            println!("💰 {} ${:.6} ({:+.2}%)", 
                update.symbol, update.price_usd, update.price_change_24h);
        }
    }
    
    async fn on_connected(&self, session_id: String) {
        println!("🔗 Connected: {}", session_id);
    }
    
    async fn on_disconnected(&self, reason: String) {
        println!("🔌 Disconnected: {}", reason);
    }
    
    async fn on_error(&self, error: String) {
        eprintln!("❌ Error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let handler = Arc::new(PriceMonitorHandler {
        prices: Arc::new(RwLock::new(HashMap::new())),
        alerts: Arc::new(RwLock::new(vec![
            PriceAlert {
                symbol: "BONK".to_string(),
                target_price: 0.00003,
                triggered: false,
            }
        ])),
    });
    
    let mut ws_client = WebSocketClient::new(handler.clone())?;
    
    // Connect and subscribe
    ws_client.connect().await?;
    ws_client.subscribe_new_tokens().await?;
    
    // Subscribe to specific tokens
    let popular_tokens = vec![
        "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // BONK
        "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm", // WIF
    ];
    
    for token in popular_tokens {
        ws_client.subscribe_token_price(token).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    // Monitor for 5 minutes
    println!("🚀 Monitoring prices for 5 minutes...");
    tokio::time::sleep(Duration::from_secs(300)).await;
    
    // Show final summary
    let prices = handler.prices.read().await;
    println!("\n📊 Final Price Summary:");
    for (symbol, price) in prices.iter() {
        println!("  {}: ${:.6}", symbol, price);
    }
    
    ws_client.disconnect().await;
    Ok(())
}
```

This comprehensive WebSocket documentation covers all aspects of the streaming functionality, from basic connections to advanced real-time data processing and error handling patterns.
