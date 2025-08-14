use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingToken {
    pub mint_address: String,
    pub symbol: String,
    pub name: String,
    pub price_usd: f64,
    pub price_change_24h: f64,
    pub price_change_7d: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub holders: u64,
    pub rank: u32,
    pub logo_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    #[serde(rename = "tokenAddress")]
    pub mint_address: String,
    #[serde(rename = "tokenTicker")]
    pub symbol: String,
    #[serde(rename = "tokenName")]
    pub name: String,
    #[serde(rename = "tokenDecimals")]
    pub decimals: u8,
    pub supply: f64,
    #[serde(rename = "liquiditySol")]
    pub liquidity_sol: f64,
    #[serde(rename = "liquidityToken")]  
    pub liquidity_token: f64,
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    pub protocol: String,
    #[serde(rename = "protocolDetails")]
    pub protocol_details: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "tokenImage")]
    pub logo_uri: Option<String>,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<String>,
    #[serde(rename = "lpBurned")]
    pub lp_burned: f64,
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAnalysis {
    #[serde(rename = "creatorRiskLevel")]
    pub creator_risk_level: String,
    #[serde(rename = "creatorRugCount")]
    pub creator_rug_count: u32,
    #[serde(rename = "creatorTokenCount")]
    pub creator_token_count: u32,
    #[serde(rename = "topMarketCapCoins")]
    pub top_market_cap_coins: Vec<RelatedToken>,
    #[serde(rename = "topOgCoins")]
    pub top_og_coins: Vec<RelatedToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedToken {
    #[serde(rename = "tokenAddress")]
    pub mint_address: String,
    #[serde(rename = "tokenTicker")]
    pub symbol: String,
    #[serde(rename = "tokenName")]
    pub name: String,
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    #[serde(rename = "marketCap")]
    pub market_cap: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lastTradeTime")]
    pub last_trade_time: String,
    pub image: Option<String>,
    pub migrated: bool,
    #[serde(rename = "bondingCurvePercent")]
    pub bonding_curve_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceData {
    pub mint_address: String,
    pub price_usd: f64,
    pub price_sol: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFeed {
    pub mint_address: String,
    pub prices: Vec<PricePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub timestamp: i64,
    pub price_usd: f64,
    pub price_sol: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketStats {
    pub total_volume_24h: f64,
    pub total_market_cap: f64,
    pub active_traders_24h: u64,
    pub total_transactions_24h: u64,
    pub trending_tokens_count: u32,
    pub new_tokens_24h: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenChart {
    pub mint_address: String,
    pub timeframe: ChartTimeframe,
    pub candles: Vec<Candle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChartTimeframe {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimePeriod {
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "24h")]
    TwentyFourHours,
    #[serde(rename = "7d")]
    SevenDays,
    #[serde(rename = "30d")]
    ThirtyDays,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenSearch {
    pub query: String,
    pub results: Vec<TokenSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSearchResult {
    #[serde(rename = "tokenAddress")]
    pub mint_address: String,
    #[serde(rename = "tokenTicker")]
    pub symbol: String,
    #[serde(rename = "tokenName")]
    pub name: String,
    #[serde(rename = "tokenImage")]
    pub logo_uri: Option<String>,
    #[serde(rename = "tokenDecimals")]
    pub decimals: u8,
    pub supply: f64,
    #[serde(rename = "liquiditySol")]
    pub liquidity_sol: f64,
    #[serde(rename = "marketCapSol")]
    pub market_cap: f64,
    #[serde(rename = "volumeSol")]
    pub volume_sol: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    pub protocol: String,
    pub website: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
}