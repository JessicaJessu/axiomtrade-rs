use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletBalance {
    pub sol_balance: f64,
    pub token_balances: HashMap<String, TokenBalance>,
    pub total_value_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub mint_address: String,
    pub symbol: String,
    pub name: String,
    pub amount: f64,
    pub decimals: u8,
    pub ui_amount: f64,
    pub value_usd: f64,
    pub price_per_token: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchBalanceRequest {
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchBalanceResponse {
    pub balances: HashMap<String, WalletBalance>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub total_value_usd: f64,
    pub sol_balance: f64,
    pub token_count: usize,
    pub top_holdings: Vec<TokenHolding>,
    pub recent_transactions: Vec<TransactionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenHolding {
    pub mint_address: String,
    pub symbol: String,
    pub name: String,
    pub amount: f64,
    pub value_usd: f64,
    pub percentage_of_portfolio: f64,
    pub price_change_24h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionSummary {
    pub signature: String,
    pub timestamp: i64,
    pub transaction_type: TransactionType,
    pub token_mint: String,
    pub token_symbol: String,
    pub amount: f64,
    pub sol_amount: f64,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Buy,
    Sell,
    Swap,
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}