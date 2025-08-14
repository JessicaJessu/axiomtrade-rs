use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    Subscribe {
        action: String,
        room: String,
    },
    Unsubscribe {
        action: String,
        room: String,
    },
    Ping,
    Pong,
    MarketUpdate(MarketUpdate),
    OrderUpdate(OrderUpdate),
    TradeUpdate(TradeUpdate),
    BalanceUpdate(BalanceUpdate),
    Error {
        code: i32,
        message: String,
    },
    Connected {
        session_id: String,
    },
    Disconnected {
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    MarketData,
    OrderBook,
    Trades,
    Portfolio,
    Orders,
    PriceAlerts,
    TrendingTokens,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketUpdate {
    pub token_mint: String,
    pub symbol: String,
    pub price_usd: f64,
    pub price_sol: f64,
    pub price_change_24h: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub order_id: String,
    pub status: OrderStatus,
    pub transaction_hash: Option<String>,
    pub token_mint: String,
    pub side: OrderSide,
    pub amount: f64,
    pub price: f64,
    pub filled_amount: f64,
    pub timestamp: i64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Processing,
    PartiallyFilled,
    Filled,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeUpdate {
    pub token_mint: String,
    pub side: OrderSide,
    pub price: f64,
    pub amount: f64,
    pub total_value: f64,
    pub timestamp: i64,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceUpdate {
    pub wallet_address: String,
    pub sol_balance: f64,
    pub token_balances: Vec<TokenBalance>,
    pub total_value_usd: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub token_mint: String,
    pub symbol: String,
    pub amount: f64,
    pub value_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub token_mint: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: f64,
    pub amount: f64,
}

impl WebSocketMessage {
    /// Creates a subscribe message for a room
    /// 
    /// # Arguments
    /// 
    /// * `room` - String - The room to subscribe to
    /// 
    /// # Returns
    /// 
    /// WebSocketMessage - A subscribe message
    pub fn subscribe(room: String) -> Self {
        WebSocketMessage::Subscribe { 
            action: "join".to_string(),
            room 
        }
    }
    
    /// Creates an unsubscribe message for a room
    /// 
    /// # Arguments
    /// 
    /// * `room` - String - The room to unsubscribe from
    /// 
    /// # Returns
    /// 
    /// WebSocketMessage - An unsubscribe message
    pub fn unsubscribe(room: String) -> Self {
        WebSocketMessage::Unsubscribe { 
            action: "leave".to_string(),
            room 
        }
    }
    
    /// Creates a ping message
    /// 
    /// # Returns
    /// 
    /// WebSocketMessage - A ping message
    pub fn ping() -> Self {
        WebSocketMessage::Ping
    }
}