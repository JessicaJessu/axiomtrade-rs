use crate::websocket::messages::{
    WebSocketMessage, MarketUpdate, OrderUpdate, TradeUpdate, BalanceUpdate,
};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handles incoming WebSocket messages
    /// 
    /// # Arguments
    /// 
    /// * `message` - WebSocketMessage - The message to handle
    async fn handle_message(&self, message: WebSocketMessage);
    
    /// Called when connection is established
    /// 
    /// # Arguments
    /// 
    /// * `session_id` - String - The session identifier
    async fn on_connected(&self, session_id: String);
    
    /// Called when connection is lost
    /// 
    /// # Arguments
    /// 
    /// * `reason` - String - The disconnection reason
    async fn on_disconnected(&self, reason: String);
    
    /// Called when an error occurs
    /// 
    /// # Arguments
    /// 
    /// * `error` - String - The error message
    async fn on_error(&self, error: String);
}

pub struct DefaultMessageHandler {
    market_updates: Arc<RwLock<Vec<MarketUpdate>>>,
    order_updates: Arc<RwLock<Vec<OrderUpdate>>>,
    trade_updates: Arc<RwLock<Vec<TradeUpdate>>>,
    balance_updates: Arc<RwLock<Vec<BalanceUpdate>>>,
}

impl DefaultMessageHandler {
    /// Creates a new default message handler
    /// 
    /// # Returns
    /// 
    /// DefaultMessageHandler - A new handler instance
    pub fn new() -> Self {
        Self {
            market_updates: Arc::new(RwLock::new(Vec::new())),
            order_updates: Arc::new(RwLock::new(Vec::new())),
            trade_updates: Arc::new(RwLock::new(Vec::new())),
            balance_updates: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Gets stored market updates
    /// 
    /// # Returns
    /// 
    /// Vec<MarketUpdate> - List of market updates
    pub async fn get_market_updates(&self) -> Vec<MarketUpdate> {
        self.market_updates.read().await.clone()
    }
    
    /// Gets stored order updates
    /// 
    /// # Returns
    /// 
    /// Vec<OrderUpdate> - List of order updates
    pub async fn get_order_updates(&self) -> Vec<OrderUpdate> {
        self.order_updates.read().await.clone()
    }
    
    /// Gets stored trade updates
    /// 
    /// # Returns
    /// 
    /// Vec<TradeUpdate> - List of trade updates
    pub async fn get_trade_updates(&self) -> Vec<TradeUpdate> {
        self.trade_updates.read().await.clone()
    }
    
    /// Gets stored balance updates
    /// 
    /// # Returns
    /// 
    /// Vec<BalanceUpdate> - List of balance updates
    pub async fn get_balance_updates(&self) -> Vec<BalanceUpdate> {
        self.balance_updates.read().await.clone()
    }
    
    /// Clears all stored updates
    pub async fn clear_all(&self) {
        self.market_updates.write().await.clear();
        self.order_updates.write().await.clear();
        self.trade_updates.write().await.clear();
        self.balance_updates.write().await.clear();
    }
}

#[async_trait]
impl MessageHandler for DefaultMessageHandler {
    async fn handle_message(&self, message: WebSocketMessage) {
        match message {
            WebSocketMessage::MarketUpdate(update) => {
                let mut updates = self.market_updates.write().await;
                updates.push(update);
                if updates.len() > 1000 {
                    updates.drain(0..500);
                }
            }
            WebSocketMessage::OrderUpdate(update) => {
                let mut updates = self.order_updates.write().await;
                updates.push(update);
                if updates.len() > 100 {
                    updates.drain(0..50);
                }
            }
            WebSocketMessage::TradeUpdate(update) => {
                let mut updates = self.trade_updates.write().await;
                updates.push(update);
                if updates.len() > 500 {
                    updates.drain(0..250);
                }
            }
            WebSocketMessage::BalanceUpdate(update) => {
                let mut updates = self.balance_updates.write().await;
                updates.push(update);
                if updates.len() > 50 {
                    updates.drain(0..25);
                }
            }
            WebSocketMessage::Error { code, message } => {
                println!("WebSocket error {}: {}", code, message);
            }
            _ => {}
        }
    }
    
    async fn on_connected(&self, session_id: String) {
        println!("WebSocket connected with session: {}", session_id);
    }
    
    async fn on_disconnected(&self, reason: String) {
        println!("WebSocket disconnected: {}", reason);
    }
    
    async fn on_error(&self, error: String) {
        println!("WebSocket error: {}", error);
    }
}