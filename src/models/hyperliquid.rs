use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Clearinghouse state for a user on Hyperliquid
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearinghouseState {
    pub margin_summary: MarginSummary,
    pub cross_margin_summary: MarginSummary,
    pub cross_maintenance_margin_used: String,
    pub withdrawable: String,
    pub asset_positions: Vec<AssetPosition>,
    pub time: u64,
}

/// Margin summary information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSummary {
    pub account_value: String,
    pub total_ntl_pos: String,
    pub total_raw_usd: String,
    pub total_margin_used: String,
}

/// Asset position information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPosition {
    pub position: Position,
    pub entry: PositionEntry,
    pub unrealized_pnl: String,
    pub roe: String,
    pub margin_used: String,
    pub coin: String,
}

/// Position details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub coin: String,
    pub entry_px: String,
    pub leverage: Leverage,
    pub liquidation_px: Option<String>,
    pub margin_used: String,
    pub max_leverage: String,
    pub position_value: String,
    pub return_on_equity: String,
    pub szi: String,
    pub unrealized_pnl: String,
}

/// Position leverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leverage {
    pub type_: String,
    pub value: u32,
    pub raw_usd: String,
}

/// Position entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionEntry {
    pub entry_px: String,
    pub szi: String,
    pub time: u64,
}

/// User role on Hyperliquid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub role: String,
}

/// Market metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketMetadata {
    pub universe: Vec<MarketInfo>,
    pub margin_tables: Vec<MarginTable>,
}

/// Individual market information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketInfo {
    pub name: String,
    pub sz_decimals: u8,
    pub max_leverage: u32,
    pub margin_table_id: u32,
    pub is_delisted: Option<bool>,
}

/// Margin table information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginTable {
    pub id: u32,
    pub initial_margin_frac: String,
    pub maintenance_margin_frac: String,
    pub max_position_size: String,
}

/// All mid prices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllMids(pub HashMap<String, String>);

/// Open order information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    pub coin: String,
    pub limit_px: String,
    pub oid: u64,
    pub side: String,
    pub sz: String,
    pub timestamp: u64,
    pub cloid: Option<String>,
    pub reduce_only: bool,
    pub order_type: String,
    pub tif: String,
}

/// User fill (executed trade)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFill {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub timestamp: u64,
    pub start_position: String,
    pub dir: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub liquidation: Option<bool>,
}

/// Orderbook data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orderbook {
    pub coin: String,
    pub levels: Vec<Vec<OrderbookLevel>>,
    pub time: u64,
}

/// Orderbook level (bids or asks)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookLevel {
    pub px: String,
    pub sz: String,
    pub n: u32,
}

/// Recent trade information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentTrade {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub timestamp: u64,
    pub hash: String,
}

/// 24-hour market statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketStats {
    pub coin: String,
    pub day_ntl_vlm: String,
    pub day_change: String,
    pub funding: String,
    pub open_interest: String,
    pub prev_day_px: String,
    pub mark_px: String,
    pub mid_px: String,
    pub impact_px: Vec<String>,
    pub premium: String,
}

/// Funding payment information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingPayment {
    pub coin: String,
    pub usdc: String,
    pub szi: String,
    pub funding_rate: String,
    pub timestamp: u64,
}

/// Candle data for charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

/// WebSocket subscription types for Hyperliquid
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HyperliquidSubscription {
    AllMids,
    Notification { user: String },
    WebData { user: String },
    Candle { coin: String, interval: String },
    L2Book { coin: String },
    Trades { coin: String },
    UserEvents { user: String },
    UserFills { user: String },
    UserFundings { user: String },
}

/// WebSocket message from Hyperliquid
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidMessage {
    pub channel: String,
    pub data: serde_json::Value,
}

/// Order side
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Time in force
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "Gtc")]
    GoodTillCanceled,
    #[serde(rename = "Ioc")]
    ImmediateOrCancel,
    #[serde(rename = "Fok")]
    FillOrKill,
}

/// Portfolio summary combining Hyperliquid and Solana data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedPortfolio {
    pub total_value_usd: f64,
    pub hyperliquid_portfolio: HyperliquidPortfolio,
    pub solana_portfolio: SolanaPortfolio,
    pub last_updated: u64,
}

/// Hyperliquid-specific portfolio data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidPortfolio {
    pub account_value: f64,
    pub unrealized_pnl: f64,
    pub margin_used: f64,
    pub available_margin: f64,
    pub positions: Vec<AssetPosition>,
    pub open_orders: Vec<OpenOrder>,
}

/// Solana-specific portfolio data (for unified view)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaPortfolio {
    pub sol_balance: f64,
    pub token_balances: Vec<TokenBalance>,
    pub total_value_usd: f64,
}

/// Token balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub mint: String,
    pub symbol: String,
    pub balance: f64,
    pub value_usd: f64,
    pub decimals: u8,
}