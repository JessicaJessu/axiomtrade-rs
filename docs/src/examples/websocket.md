# WebSocket Examples

This section demonstrates how to use WebSocket connections for real-time data streaming from Axiom Trade. WebSocket connections provide low-latency access to market updates, trade notifications, order status changes, and portfolio balance updates.

## Table of Contents

- [Basic WebSocket Connection](#basic-websocket-connection)
- [Price Subscriptions](#price-subscriptions)
- [MessageHandler Implementations](#messagehandler-implementations)
- [Connection Management](#connection-management)
- [Real-World Use Cases](#real-world-use-cases)
- [Error Handling](#error-handling)
- [Performance Considerations](#performance-considerations)

## Basic WebSocket Connection

The simplest way to establish a WebSocket connection and receive real-time updates.

### Example: basic_websocket.rs

```rust
use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler, WebSocketMessage};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Simple message handler that prints incoming messages
struct BasicMessageHandler;

#[async_trait]
impl MessageHandler for BasicMessageHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                println!("Market Update: {} - ${:.6}", update.symbol, update.price_usd);
            }
            WebSocketMessage::OrderUpdate(order) => {
                println!("Order Update: {} - {:?}", order.order_id, order.status);
            }
            WebSocketMessage::TradeUpdate(trade) => {
                println!("Trade Update: {} - ${:.6}", trade.token_mint, trade.price);
            }
            WebSocketMessage::BalanceUpdate(balance) => {
                println!("Balance Update: {} SOL", balance.sol_balance);
            }
            WebSocketMessage::Connected { session_id } => {
                println!("Connected with session: {}", session_id);
            }
            WebSocketMessage::Disconnected { reason } => {
                println!("Disconnected: {}", reason);
            }
            WebSocketMessage::Error { code, message } => {
                println!("WebSocket Error {}: {}", code, message);
            }
            _ => {
                println!("Other message: {:?}", message);
            }
        }
    }

    async fn on_connected(&self, session_id: String) {
        println!("WebSocket connected! Session ID: {}", session_id);
    }

    async fn on_disconnected(&self, reason: String) {
        println!("WebSocket disconnected: {}", reason);
    }

    async fn on_error(&self, error: String) {
        println!("WebSocket error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Create message handler and WebSocket client
    let handler = Arc::new(BasicMessageHandler);
    let mut ws_client = WebSocketClient::new(handler.clone())?;
    
    // Connect to WebSocket (authentication is handled internally)
    ws_client.connect().await?;
    
    // Subscribe to new token listings
    ws_client.subscribe_new_tokens().await?;
    
    // Monitor the connection for 30 seconds
    let monitoring_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitoring_duration {
        if !ws_client.is_connected().await {
            println!("Connection lost, attempting reconnection...");
            ws_client.reconnect().await?;
        }
        sleep(Duration::from_millis(1000)).await;
    }

    // Graceful disconnect
    ws_client.disconnect().await;
    println!("WebSocket disconnected gracefully");

    Ok(())
}
```

**Key Features:**
- Simple connection establishment with automatic authentication
- Basic message handling for different event types
- Connection health monitoring with automatic reconnection
- Graceful disconnection

## Price Subscriptions

Advanced example demonstrating subscription to multiple token price feeds with real-time tracking.

### Example: price_subscriptions.rs

```rust
use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler, WebSocketMessage};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::sleep;
use tokio::sync::RwLock;

/// Price tracking message handler
struct PriceMessageHandler {
    price_tracker: Arc<RwLock<PriceTracker>>,
}

impl PriceMessageHandler {
    fn new(price_tracker: Arc<RwLock<PriceTracker>>) -> Self {
        Self { price_tracker }
    }
}

#[async_trait]
impl MessageHandler for PriceMessageHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                let mut tracker = self.price_tracker.write().await;
                tracker.update_price(
                    &update.symbol, 
                    update.price_usd, 
                    Some(update.volume_24h), 
                    Some(update.price_change_24h)
                ).await;
            }
            WebSocketMessage::TradeUpdate(trade) => {
                println!("Trade: {} - ${:.6}", trade.token_mint, trade.price);
            }
            _ => {}
        }
    }

    async fn on_connected(&self, session_id: String) {
        println!("WebSocket connected! Session ID: {}", session_id);
    }

    async fn on_disconnected(&self, reason: String) {
        println!("WebSocket disconnected: {}", reason);
    }

    async fn on_error(&self, error: String) {
        println!("WebSocket error: {}", error);
    }
}

struct PriceTracker {
    prices: HashMap<String, PriceData>,
    update_count: u64,
}

struct PriceData {
    symbol: String,
    current_price: f64,
    previous_price: f64,
    high_24h: f64,
    low_24h: f64,
    volume_24h: f64,
    change_24h: f64,
    last_updated: std::time::Instant,
    update_count: u32,
}

impl PriceTracker {
    fn new() -> Self {
        Self {
            prices: HashMap::new(),
            update_count: 0,
        }
    }

    async fn update_price(&mut self, symbol: &str, price: f64, volume: Option<f64>, change: Option<f64>) {
        self.update_count += 1;
        
        let price_data = self.prices.entry(symbol.to_string()).or_insert(PriceData {
            symbol: symbol.to_string(),
            current_price: price,
            previous_price: price,
            high_24h: price,
            low_24h: price,
            volume_24h: volume.unwrap_or(0.0),
            change_24h: change.unwrap_or(0.0),
            last_updated: std::time::Instant::now(),
            update_count: 0,
        });

        price_data.previous_price = price_data.current_price;
        price_data.current_price = price;
        
        if price > price_data.high_24h {
            price_data.high_24h = price;
        }
        if price < price_data.low_24h {
            price_data.low_24h = price;
        }
        
        if let Some(vol) = volume {
            price_data.volume_24h = vol;
        }
        if let Some(chg) = change {
            price_data.change_24h = chg;
        }
        
        price_data.last_updated = std::time::Instant::now();
        price_data.update_count += 1;

        // Display real-time update with direction indicator
        let direction = if price > price_data.previous_price {
            "📈"
        } else if price < price_data.previous_price {
            "📉"
        } else {
            "➡️"
        };

        println!("{} {} ${:.6} ({})", 
            chrono::Utc::now().format("%H:%M:%S"),
            symbol,
            price,
            direction
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Create price tracker and message handler
    let price_tracker = Arc::new(RwLock::new(PriceTracker::new()));
    let handler = Arc::new(PriceMessageHandler::new(price_tracker.clone()));
    let mut ws_client = WebSocketClient::new(handler.clone())?;
    
    // Connect to WebSocket
    ws_client.connect().await?;

    // Subscribe to new token listings for general market updates
    ws_client.subscribe_new_tokens().await?;

    // Define tokens to monitor for specific price updates
    let tokens_to_watch = vec![
        ("SOL", "So11111111111111111111111111111111111111112"),
        ("USDC", "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        ("BONK", "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
        ("WIF", "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
        ("PEPE", "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"),
    ];

    // Subscribe to price feeds for multiple tokens
    for (symbol, mint) in &tokens_to_watch {
        match ws_client.subscribe_token_price(mint).await {
            Ok(()) => println!("✓ Subscribed to {} price feed", symbol),
            Err(e) => println!("❌ Failed to subscribe to {}: {}", symbol, e),
        }
        sleep(Duration::from_millis(100)).await;
    }

    // Monitor prices for 2 minutes
    let monitoring_duration = Duration::from_secs(120);
    let start_time = std::time::Instant::now();
    let mut last_summary = std::time::Instant::now();

    while start_time.elapsed() < monitoring_duration {
        // Check connection health
        if !ws_client.is_connected().await {
            println!("Connection lost, reconnecting...");
            ws_client.reconnect().await?;
            
            // Re-subscribe after reconnection
            ws_client.subscribe_new_tokens().await.ok();
            for (_symbol, mint) in &tokens_to_watch {
                ws_client.subscribe_token_price(mint).await.ok();
            }
        }

        // Show periodic summary every 30 seconds
        if last_summary.elapsed() >= Duration::from_secs(30) {
            show_price_summary(&price_tracker).await;
            last_summary = std::time::Instant::now();
        }

        sleep(Duration::from_millis(500)).await;
    }

    // Final summary and disconnect
    show_detailed_summary(&price_tracker).await;
    ws_client.disconnect().await;

    Ok(())
}

async fn show_price_summary(price_tracker: &Arc<RwLock<PriceTracker>>) {
    let tracker = price_tracker.read().await;
    println!("\n📊 Price Summary ({} updates total):", tracker.update_count);
    println!("{:<8} {:>12} {:>12} {:>12} {:>8}", "Symbol", "Price", "24h Change", "Volume", "Updates");
    println!("{}", "-".repeat(60));
    
    for symbol in tracker.get_symbols() {
        if let Some(data) = tracker.get_price_data(&symbol) {
            println!("{:<8} {:>12.6} {:>11.2}% {:>12.0} {:>8}", 
                data.symbol,
                data.current_price,
                data.change_24h,
                data.volume_24h,
                data.update_count
            );
        }
    }
}
```

**Key Features:**
- Multi-token price subscription management
- Real-time price tracking with historical data
- Periodic summary reports
- Automatic re-subscription after reconnection
- Visual price direction indicators

## MessageHandler Implementations

### Default MessageHandler

The library provides a built-in `DefaultMessageHandler` for basic use cases:

```rust
use axiomtrade_rs::websocket::handler::DefaultMessageHandler;

// Create default handler that stores all updates
let handler = Arc::new(DefaultMessageHandler::new());
let ws_client = WebSocketClient::new(handler.clone())?;

// After some time, retrieve stored updates
let market_updates = handler.get_market_updates().await;
let order_updates = handler.get_order_updates().await;
let trade_updates = handler.get_trade_updates().await;
let balance_updates = handler.get_balance_updates().await;

// Clear stored data to manage memory
handler.clear_all().await;
```

### Custom MessageHandler

Implement the `MessageHandler` trait for custom behavior:

```rust
use axiomtrade_rs::websocket::{MessageHandler, WebSocketMessage};
use async_trait::async_trait;

struct CustomHandler {
    // Your custom fields
}

#[async_trait]
impl MessageHandler for CustomHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        // Custom message processing logic
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                // Process market updates
                self.process_market_data(update).await;
            }
            WebSocketMessage::OrderUpdate(order) => {
                // Handle order status changes
                self.update_order_status(order).await;
            }
            WebSocketMessage::TradeUpdate(trade) => {
                // Log trade executions
                self.log_trade(trade).await;
            }
            WebSocketMessage::BalanceUpdate(balance) => {
                // Update portfolio tracking
                self.update_portfolio(balance).await;
            }
            _ => {}
        }
    }

    async fn on_connected(&self, session_id: String) {
        // Connection established logic
        println!("Connected: {}", session_id);
    }

    async fn on_disconnected(&self, reason: String) {
        // Cleanup on disconnection
        println!("Disconnected: {}", reason);
    }

    async fn on_error(&self, error: String) {
        // Error handling
        eprintln!("Error: {}", error);
    }
}
```

## Connection Management

### Regional Selection

Choose optimal regions for better latency:

```rust
use axiomtrade_rs::websocket::{WebSocketClient, Region};

// Use specific region for better performance
let ws_client = WebSocketClient::with_region(handler, Region::USWest)?;

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

### Connection Health Monitoring

```rust
// Check connection status
let is_connected = ws_client.is_connected().await;

// Manual reconnection
if !is_connected {
    ws_client.reconnect().await?;
}

// Get current subscriptions
let subscriptions = ws_client.get_subscriptions().await;
println!("Active subscriptions: {:?}", subscriptions);

// Enable automatic token refresh
let ws_client = WebSocketClient::with_auto_refresh(handler, true)?;
```

### Subscription Management

```rust
// Subscribe to different data feeds
ws_client.subscribe_new_tokens().await?;
ws_client.subscribe_token_price("token_mint_address").await?;
ws_client.subscribe_portfolio_updates().await?;
ws_client.subscribe_order_updates().await?;

// Unsubscribe from feeds
ws_client.unsubscribe_token_price("token_mint_address").await?;
ws_client.unsubscribe_new_tokens().await?;

// Check active subscriptions
let active_subs = ws_client.get_subscriptions().await;
for subscription in active_subs {
    println!("Active: {:?}", subscription);
}
```

## Real-World Use Cases

### Trading Bot with Real-Time Signals

```rust
struct TradingBotHandler {
    trading_client: Arc<TradingClient>,
    strategy: Arc<TradingStrategy>,
}

#[async_trait]
impl MessageHandler for TradingBotHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                // Analyze market data for trading signals
                if let Some(signal) = self.strategy.analyze_market(&update).await {
                    match signal {
                        TradingSignal::Buy { token, amount } => {
                            self.trading_client.execute_buy(&token, amount).await.ok();
                        }
                        TradingSignal::Sell { token, amount } => {
                            self.trading_client.execute_sell(&token, amount).await.ok();
                        }
                    }
                }
            }
            WebSocketMessage::OrderUpdate(order) => {
                // Track order execution
                self.strategy.update_order_status(order).await;
            }
            _ => {}
        }
    }
}
```

### Portfolio Monitoring Dashboard

```rust
struct DashboardHandler {
    portfolio_state: Arc<RwLock<PortfolioState>>,
    alert_system: Arc<AlertSystem>,
}

#[async_trait]
impl MessageHandler for DashboardHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::BalanceUpdate(balance) => {
                let mut portfolio = self.portfolio_state.write().await;
                portfolio.update_balance(balance).await;
                
                // Check for alerts
                if portfolio.total_value_usd < portfolio.stop_loss_threshold {
                    self.alert_system.send_alert(AlertType::StopLoss).await;
                }
            }
            WebSocketMessage::MarketUpdate(update) => {
                // Update portfolio value based on price changes
                let mut portfolio = self.portfolio_state.write().await;
                portfolio.update_token_price(&update.token_mint, update.price_usd).await;
            }
            _ => {}
        }
    }
}
```

### Price Alert System

```rust
struct PriceAlertHandler {
    alert_rules: Arc<RwLock<Vec<PriceAlert>>>,
    notification_service: Arc<NotificationService>,
}

#[async_trait]
impl MessageHandler for PriceAlertHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        if let WebSocketMessage::MarketUpdate(update) = message {
            let alerts = self.alert_rules.read().await;
            
            for alert in alerts.iter() {
                if alert.token_mint == update.token_mint {
                    let triggered = match alert.condition {
                        AlertCondition::PriceAbove(price) => update.price_usd > price,
                        AlertCondition::PriceBelow(price) => update.price_usd < price,
                        AlertCondition::PriceChange(change) => update.price_change_24h.abs() > change,
                    };
                    
                    if triggered {
                        self.notification_service.send_alert(&alert, &update).await;
                    }
                }
            }
        }
    }
}
```

## Error Handling

### Connection Error Recovery

```rust
async fn robust_websocket_connection() -> Result<(), WebSocketError> {
    let handler = Arc::new(MyHandler::new());
    let mut ws_client = WebSocketClient::new(handler)?;
    
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 5;
    
    loop {
        match ws_client.connect().await {
            Ok(()) => {
                println!("Connected successfully");
                break;
            }
            Err(WebSocketError::AuthError(_)) => {
                // Authentication failed - check credentials
                eprintln!("Authentication failed - check credentials");
                return Err(WebSocketError::AuthError(AuthError::InvalidCredentials));
            }
            Err(e) if retry_count < MAX_RETRIES => {
                retry_count += 1;
                eprintln!("Connection failed (attempt {}): {}", retry_count, e);
                tokio::time::sleep(Duration::from_secs(2_u64.pow(retry_count))).await;
            }
            Err(e) => {
                eprintln!("Failed to connect after {} attempts: {}", MAX_RETRIES, e);
                return Err(e);
            }
        }
    }
    
    Ok(())
}
```

### Message Processing Error Handling

```rust
#[async_trait]
impl MessageHandler for RobustHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        // Wrap message processing in error handling
        if let Err(e) = self.process_message_safe(message).await {
            eprintln!("Failed to process message: {}", e);
            
            // Optionally log error or send to monitoring system
            self.error_logger.log_error(e).await;
        }
    }
    
    async fn on_error(&self, error: String) {
        // Implement sophisticated error handling
        if error.contains("token_expired") {
            // Trigger token refresh
            self.auth_client.refresh_token().await.ok();
        } else if error.contains("rate_limit") {
            // Implement backoff strategy
            self.apply_rate_limit_backoff().await;
        }
    }
}

impl RobustHandler {
    async fn process_message_safe(&self, message: WebSocketMessage) -> Result<(), ProcessingError> {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                self.validate_market_update(&update)?;
                self.store_market_update(update).await?;
            }
            WebSocketMessage::Error { code, message } => {
                self.handle_websocket_error(code, &message).await?;
            }
            _ => {}
        }
        Ok(())
    }
}
```

## Performance Considerations

### Memory Management

```rust
// Use bounded channels for message queuing
struct MemoryEfficientHandler {
    message_queue: Arc<RwLock<VecDeque<WebSocketMessage>>>,
    max_queue_size: usize,
}

#[async_trait]
impl MessageHandler for MemoryEfficientHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        let mut queue = self.message_queue.write().await;
        
        // Prevent memory leaks by limiting queue size
        if queue.len() >= self.max_queue_size {
            queue.pop_front(); // Remove oldest message
        }
        
        queue.push_back(message);
    }
}
```

### Batch Processing

```rust
struct BatchProcessor {
    batch_buffer: Arc<RwLock<Vec<MarketUpdate>>>,
    batch_size: usize,
}

#[async_trait]
impl MessageHandler for BatchProcessor {
    async fn handle_message(&self, message: WebSocketMessage) {
        if let WebSocketMessage::MarketUpdate(update) = message {
            let mut buffer = self.batch_buffer.write().await;
            buffer.push(update);
            
            // Process in batches for efficiency
            if buffer.len() >= self.batch_size {
                let batch = buffer.drain(..).collect::<Vec<_>>();
                drop(buffer); // Release lock before processing
                
                self.process_batch(batch).await;
            }
        }
    }
}
```

### Connection Optimization

```rust
// Use connection pooling for high-frequency applications
struct OptimizedWebSocketClient {
    primary_connection: WebSocketClient,
    backup_connections: Vec<WebSocketClient>,
    load_balancer: LoadBalancer,
}

impl OptimizedWebSocketClient {
    async fn send_with_failover(&mut self, message: &str) -> Result<(), WebSocketError> {
        // Try primary connection first
        if let Err(_) = self.primary_connection.send(message).await {
            // Failover to backup connections
            for backup in &mut self.backup_connections {
                if backup.is_connected().await {
                    return backup.send(message).await;
                }
            }
        }
        
        Err(WebSocketError::NotConnected)
    }
}
```

This comprehensive guide covers all aspects of WebSocket usage in the Axiom Trade Rust library, from basic connections to advanced real-world implementations. The examples demonstrate proper error handling, performance optimization, and various use cases for different trading scenarios.
