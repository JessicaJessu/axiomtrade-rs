use serde::{Deserialize, Serialize};

/// Response from the /portfolio-v5 endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioV5Response {
    pub active_positions: Vec<Position>,
    pub history_positions: Vec<Position>,
    pub top_positions: Vec<Position>,
    pub transactions: Vec<Transaction>,
    pub balance_stats: BalanceStats,
    pub performance_metrics: PerformanceMetrics,
    pub chart_data: Vec<ChartDataPoint>,
    pub calendar_data: Vec<CalendarDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub token_address: Option<String>,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub amount: Option<f64>,
    pub value_sol: Option<f64>,
    pub value_usd: Option<f64>,
    pub pnl: Option<f64>,
    pub pnl_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub signature: Option<String>,
    pub timestamp: Option<i64>,
    pub token_address: Option<String>,
    pub symbol: Option<String>,
    pub transaction_type: Option<String>,
    pub amount: Option<f64>,
    pub price: Option<f64>,
    pub value_sol: Option<f64>,
    pub value_usd: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceStats {
    pub total_value_sol: f64,
    pub available_balance_sol: f64,
    pub unrealized_pnl_sol: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceMetrics {
    pub one_day: PeriodMetrics,
    pub seven_day: PeriodMetrics,
    pub thirty_day: PeriodMetrics,
    pub all_time: PeriodMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodMetrics {
    pub total_pnl: f64,
    pub buy_count: u32,
    pub sell_count: u32,
    pub pnl_breakdown: PnlBreakdown,
    pub usd_bought: f64,
    pub usd_sold: f64,
    pub sol_bought: f64,
    pub sol_sold: f64,
    pub realized_sol_pnl: f64,
    pub realized_sol_bought: f64,
    pub realized_sol_sold: f64,
    pub realized_usd_pnl: f64,
    pub realized_usd_bought: f64,
    pub realized_usd_sold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PnlBreakdown {
    pub over500_percent: u32,
    pub between200_and500_percent: u32,
    pub between0_and200_percent: u32,
    pub between0_and_neg50_percent: u32,
    pub under_neg50_percent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartDataPoint {
    pub timestamp: Option<i64>,
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarDataPoint {
    pub date: Option<String>,
    pub pnl: Option<f64>,
    pub trades: Option<u32>,
}