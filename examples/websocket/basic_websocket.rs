/// Basic WebSocket Connection Example
/// 
/// This example demonstrates how to establish a WebSocket connection
/// to Axiom Trade for real-time data streaming.

use axiomtrade_rs::websocket::{WebSocketClient, MessageHandler, WebSocketMessage};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Simple message handler that prints incoming messages
struct BasicMessageHandler;

#[async_trait]
impl MessageHandler for BasicMessageHandler {
    /// Handles incoming WebSocket messages
    /// 
    /// # Arguments
    /// 
    /// * `message` - WebSocketMessage - The message to handle
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                println!("📊 Market Update: {} - ${:.6}", update.symbol, update.price_usd);
            }
            WebSocketMessage::OrderUpdate(order) => {
                println!("📋 Order Update: {} - {:?}", order.order_id, order.status);
            }
            WebSocketMessage::TradeUpdate(trade) => {
                println!("💹 Trade Update: {} - ${:.6}", trade.token_mint, trade.price);
            }
            WebSocketMessage::BalanceUpdate(balance) => {
                println!("💰 Balance Update: {} SOL", balance.sol_balance);
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
            _ => {
                println!("📨 Other message: {:?}", message);
            }
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

    println!("Basic WebSocket Connection Example");
    println!("Connecting to Axiom Trade WebSocket for real-time data\n");

    // Create message handler and WebSocket client
    let handler = Arc::new(BasicMessageHandler);
    let mut ws_client = WebSocketClient::new(handler.clone())?;
    
    println!("Step 1: Connecting to WebSocket...");
    
    // Connect to WebSocket (authentication is handled internally)
    match ws_client.connect().await {
        Ok(()) => {
            println!("✓ WebSocket connection established");
            println!("Connection status: Connected");
        }
        Err(e) => {
            println!("❌ WebSocket connection failed: {}", e);
            return Err(e.into());
        }
    }

    println!("\nStep 2: Testing subscription to new tokens...");
    
    // Subscribe to new token listings
    match ws_client.subscribe_new_tokens().await {
        Ok(()) => {
            println!("✓ Subscribed to new token listings");
        }
        Err(e) => {
            println!("⚠️ Failed to subscribe to new tokens: {}", e);
        }
    }

    println!("\nStep 3: Testing subscription to price updates...");
    
    // Subscribe to price updates for a popular token (BONK)
    let bonk_address = "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263";
    match ws_client.subscribe_token_price(bonk_address).await {
        Ok(()) => {
            println!("✓ Subscribed to BONK price updates");
        }
        Err(e) => {
            println!("⚠️ Failed to subscribe to BONK prices: {}", e);
        }
    }

    println!("\nStep 4: Monitoring connection for 30 seconds...");
    println!("Watching for incoming messages and connection events...");

    // Monitor the connection for a period
    let monitoring_duration = Duration::from_secs(30);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitoring_duration {
        // Check connection status
        if !ws_client.is_connected().await {
            println!("⚠️ Connection lost, attempting reconnection...");
            
            match ws_client.reconnect().await {
                Ok(()) => {
                    println!("✓ Reconnection successful");
                }
                Err(e) => {
                    println!("❌ Reconnection failed: {}", e);
                    break;
                }
            }
        }

        // Brief pause before next check
        sleep(Duration::from_millis(1000)).await;
        
        // Show progress every 10 seconds
        let elapsed = start_time.elapsed().as_secs();
        if elapsed > 0 && elapsed % 10 == 0 {
            println!("Monitoring... {}s elapsed", elapsed);
        }
    }

    println!("\nStep 5: Current subscriptions...");
    let subscriptions = ws_client.get_subscriptions().await;
    println!("Active subscriptions: {:?}", subscriptions);

    println!("\nStep 6: Graceful disconnect...");
    
    ws_client.disconnect().await;
    println!("✓ WebSocket disconnected gracefully");

    println!("\nBasic WebSocket example completed successfully");
    println!("\nKey concepts demonstrated:");
    println!("- WebSocket connection establishment");
    println!("- Authentication handled internally");
    println!("- Message handling via custom MessageHandler trait");
    println!("- Subscription to market data feeds");
    println!("- Connection health monitoring");
    println!("- Automatic reconnection handling");
    println!("- Graceful disconnection");

    Ok(())
}