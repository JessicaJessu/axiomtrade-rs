use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyOrderRequest {
    pub token_mint: String,
    pub amount_sol: f64,
    pub slippage_percent: f64,
    pub priority_fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellOrderRequest {
    pub token_mint: String,
    pub amount_tokens: f64,
    pub slippage_percent: f64,
    pub priority_fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapOrderRequest {
    pub from_mint: String,
    pub to_mint: String,
    pub amount: f64,
    pub slippage_percent: f64,
    pub priority_fee: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub signature: String,
    pub status: OrderStatus,
    pub transaction_type: OrderType,
    pub token_mint: String,
    pub amount_in: f64,
    pub amount_out: f64,
    pub price_per_token: f64,
    pub total_sol: f64,
    pub fee: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Success,
    Failed,
    Pending,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Buy,
    Sell,
    Swap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: f64,
    pub slippage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: f64,
    pub out_amount: f64,
    pub price_impact: f64,
    pub fee: f64,
    pub route: Vec<RouteStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStep {
    pub amm: String,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: f64,
    pub out_amount: f64,
    pub fee_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSimulation {
    pub success: bool,
    pub error: Option<String>,
    pub logs: Vec<String>,
    pub units_consumed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingLimits {
    pub min_sol_amount: f64,
    pub max_sol_amount: f64,
    pub max_slippage_percent: f64,
    pub default_slippage_percent: f64,
    pub priority_fee_lamports: u64,
}