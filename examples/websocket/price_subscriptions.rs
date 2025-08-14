/// Price Subscription WebSocket Example
/// 
/// This example demonstrates subscribing to real-time price updates
/// for multiple tokens via WebSocket connections.

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
    /// Creates a new price message handler
    /// 
    /// # Arguments
    /// 
    /// * `price_tracker` - Arc<RwLock<PriceTracker>> - Shared price tracker
    /// 
    /// # Returns
    /// 
    /// PriceMessageHandler - A new handler instance
    fn new(price_tracker: Arc<RwLock<PriceTracker>>) -> Self {
        Self { price_tracker }
    }
}

#[async_trait]
impl MessageHandler for PriceMessageHandler {
    /// Handles incoming WebSocket messages
    /// 
    /// # Arguments
    /// 
    /// * `message` - WebSocketMessage - The message to handle
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                let mut tracker = self.price_tracker.write().await;
                tracker.update_price(&update.symbol, update.price_usd, Some(update.volume_24h), Some(update.price_change_24h)).await;
            }
            WebSocketMessage::TradeUpdate(trade) => {
                println!("💹 Trade: {} - ${:.6}", trade.token_mint, trade.price);
            }
            WebSocketMessage::Connected { session_id } => {
                println!("🔗 Connected with session: {}", session_id);
            }
            WebSocketMessage::Disconnected { reason } => {
                println!("🔌 Disconnected: {}", reason);
            }
            WebSocketMessage::Error { code, message } => {
                println!("❌ WebSocket Error {}: {}", code, message);
            }
            _ => {}
        }
    }

    /// Called when connection is established
    /// 
    /// # Arguments
    /// 
    /// * `session_id` - String - The session identifier
    async fn on_connected(&self, session_id: String) {
        println!("✅ WebSocket connected! Session ID: {}", session_id);
    }

    /// Called when connection is lost
    /// 
    /// # Arguments
    /// 
    /// * `reason` - String - The disconnection reason
    async fn on_disconnected(&self, reason: String) {
        println!("❌ WebSocket disconnected: {}", reason);
    }

    /// Called when an error occurs
    /// 
    /// # Arguments
    /// 
    /// * `error` - String - The error message
    async fn on_error(&self, error: String) {
        println!("⚠️ WebSocket error: {}", error);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    println!("Price Subscription WebSocket Example");
    println!("Subscribing to real-time price feeds for multiple tokens\n");

    // Create price tracker and message handler
    let price_tracker = Arc::new(RwLock::new(PriceTracker::new()));
    let handler = Arc::new(PriceMessageHandler::new(price_tracker.clone()));
    let mut ws_client = WebSocketClient::new(handler.clone())?;
    
    println!("Connecting to WebSocket for price data...");
    ws_client.connect().await?;

    println!("✓ Connected to WebSocket");

    // Subscribe to new token listings for general market updates
    println!("Subscribing to new token listings...");
    match ws_client.subscribe_new_tokens().await {
        Ok(()) => {
            println!("✓ Subscribed to new token listings");
        }
        Err(e) => {
            println!("❌ Failed to subscribe to new tokens: {}", e);
        }
    }

    // Define tokens to monitor for specific price updates
    let tokens_to_watch = vec![
        ("SOL", "So11111111111111111111111111111111111111112"),
        ("USDC", "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        ("BONK", "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
        ("WIF", "EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
        ("PEPE", "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"),
    ];

    println!("Subscribing to price feeds for {} tokens:", tokens_to_watch.len());
    
    for (symbol, mint) in &tokens_to_watch {
        println!("  {} ({})", symbol, &mint[..8]);
        
        match ws_client.subscribe_token_price(mint).await {
            Ok(()) => {
                println!("    ✓ Subscribed to {} price feed", symbol);
            }
            Err(e) => {
                println!("    ❌ Failed to subscribe to {}: {}", symbol, e);
            }
        }
        
        // Small delay between subscriptions
        sleep(Duration::from_millis(100)).await;
    }

    println!("\nMonitoring price feeds for 2 minutes...");
    println!("Real-time price updates will be displayed below:");
    println!("{}", "-".repeat(80));

    // Monitor prices for a period
    let monitoring_duration = Duration::from_secs(120);
    let start_time = std::time::Instant::now();
    let mut last_summary = std::time::Instant::now();

    while start_time.elapsed() < monitoring_duration {
        // Check connection health
        if !ws_client.is_connected().await {
            println!("🔄 Connection lost, reconnecting...");
            ws_client.reconnect().await?;
            
            // Re-subscribe after reconnection
            ws_client.subscribe_new_tokens().await.ok();
            for (_symbol, mint) in &tokens_to_watch {
                ws_client.subscribe_token_price(mint).await.ok();
            }
        }

        // Show periodic summary
        if last_summary.elapsed() >= Duration::from_secs(30) {
            show_price_summary(&price_tracker).await;
            last_summary = std::time::Instant::now();
        }

        sleep(Duration::from_millis(500)).await;
    }

    println!("\n{}", "=".repeat(80));
    println!("Final Price Summary:");
    show_detailed_summary(&price_tracker).await;

    // Disconnect
    ws_client.disconnect().await;
    println!("✓ WebSocket disconnected");

    println!("\nPrice subscription example completed");
    Ok(())
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

        // Display real-time update
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

    fn get_symbols(&self) -> Vec<String> {
        self.prices.keys().cloned().collect()
    }

    fn get_price_data(&self, symbol: &str) -> Option<&PriceData> {
        self.prices.get(symbol)
    }
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
    println!();
}

async fn show_detailed_summary(price_tracker: &Arc<RwLock<PriceTracker>>) {
    let tracker = price_tracker.read().await;
    for symbol in tracker.get_symbols() {
        if let Some(data) = tracker.get_price_data(&symbol) {
            println!("\n{} Summary:", data.symbol);
            println!("  Current Price: ${:.6}", data.current_price);
            println!("  24h High: ${:.6}", data.high_24h);
            println!("  24h Low: ${:.6}", data.low_24h);
            println!("  24h Change: {:.2}%", data.change_24h);
            println!("  24h Volume: ${:.0}", data.volume_24h);
            println!("  Updates Received: {}", data.update_count);
            
            let price_change = data.current_price - data.previous_price;
            let price_change_pct = (price_change / data.previous_price) * 100.0;
            
            if price_change_pct.abs() > 0.01 {
                println!("  Recent Change: {:.2}% (${:.6})", price_change_pct, price_change);
            }
        }
    }
}