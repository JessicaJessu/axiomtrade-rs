use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Tracked wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedWallet {
    pub address: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tracked_since: DateTime<Utc>,
    pub total_pnl: Option<f64>,
    pub win_rate: Option<f64>,
    pub total_trades: u32,
    pub is_active: bool,
    pub tags: Vec<String>,
}

/// Request for tracked wallet transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedWalletRequest {
    pub wallet_addresses: Option<Vec<String>>,
    pub time_range: Option<TimeRange>,
    pub token_filter: Option<String>,
    pub min_amount_sol: Option<f64>,
    pub transaction_type: Option<TransactionType>,
    pub limit: Option<u32>,
}

/// Time range filter for queries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Transaction type filter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Buy,
    Sell,
    Swap,
    All,
}

/// Tracked transaction from followed wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedTransaction {
    pub id: String,
    pub wallet_address: String,
    pub wallet_name: Option<String>,
    pub transaction_hash: String,
    pub block_time: DateTime<Utc>,
    pub transaction_type: String,
    pub input_token: TokenInfo,
    pub output_token: TokenInfo,
    pub input_amount: f64,
    pub output_amount: f64,
    pub sol_amount: f64,
    pub price_impact: Option<f64>,
    pub slippage: Option<f64>,
    pub profit_loss: Option<f64>,
    pub is_profitable: Option<bool>,
    pub platform: String,
}

/// Token information in transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub logo_uri: Option<String>,
}

/// Watchlist item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistItem {
    pub token_address: String,
    pub symbol: String,
    pub name: String,
    pub added_at: DateTime<Utc>,
    pub current_price: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub market_cap: Option<f64>,
    pub volume_24h: Option<f64>,
    pub alerts_enabled: bool,
    pub price_alerts: Vec<PriceAlert>,
}

/// Price alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAlert {
    pub id: String,
    pub alert_type: AlertType,
    pub target_price: f64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Price alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertType {
    Above,
    Below,
    PercentageChange,
}

/// Twitter/X settings for social feed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitterSettings {
    pub enabled: bool,
    pub followed_accounts: Vec<String>,
    pub keywords: Vec<String>,
    pub sentiment_filter: Option<SentimentFilter>,
    pub min_followers: Option<u32>,
    pub include_retweets: bool,
    pub language_filter: Option<String>,
}

/// Sentiment filter for social content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SentimentFilter {
    Positive,
    Negative,
    Neutral,
    All,
}

/// Twitter/X feed item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwitterFeedItem {
    pub id: String,
    pub author: String,
    pub author_handle: String,
    pub author_followers: u32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub likes: u32,
    pub retweets: u32,
    pub replies: u32,
    pub sentiment_score: Option<f64>,
    pub mentioned_tokens: Vec<String>,
    pub platform: String,
    pub url: String,
    pub is_verified: bool,
}