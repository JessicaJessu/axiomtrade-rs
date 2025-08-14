use crate::auth::{AuthClient, AuthError};
use crate::models::market::{
    ChartTimeframe, MarketStats, PriceData, PriceFeed, TimePeriod, TokenAnalysis, TokenChart,
    TokenInfo, TokenSearch, TokenSearchResult, TrendingToken,
};
use reqwest::StatusCode;
use serde_json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarketDataError {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid token mint: {0}")]
    InvalidTokenMint(String),

    #[error("Token not found: {0}")]
    TokenNotFound(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub struct MarketDataClient {
    auth_client: AuthClient,
    base_url: String,
}

impl MarketDataClient {
    /// Creates a new market data client.
    ///
    /// # Returns
    ///
    /// Result<MarketDataClient, MarketDataError> - A new market data client instance.
    pub fn new() -> Result<Self, MarketDataError> {
        Ok(Self {
            auth_client: AuthClient::new()?,
            base_url: "https://api6.axiom.trade".to_string(),
        })
    }

    /// Creates a market data client with custom base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - String - The base URL for the API.
    ///
    /// # Returns
    ///
    /// Result<MarketDataClient, MarketDataError> - A new market data client instance.
    pub fn with_base_url(base_url: String) -> Result<Self, MarketDataError> {
        Ok(Self {
            auth_client: AuthClient::new()?,
            base_url,
        })
    }

    /// Gets trending meme tokens.
    ///
    /// # Arguments
    ///
    /// * `time_period` - TimePeriod - The time period for trending tokens.
    ///
    /// # Returns
    ///
    /// Result<Vec<TrendingToken>, MarketDataError> - List of trending tokens.
    pub async fn get_trending_tokens(
        &mut self,
        time_period: TimePeriod,
    ) -> Result<Vec<TrendingToken>, MarketDataError> {
        let period_str = match time_period {
            TimePeriod::OneHour => "1h",
            TimePeriod::TwentyFourHours => "24h",
            TimePeriod::SevenDays => "7d",
            TimePeriod::ThirtyDays => "30d",
        };

        let url = format!("{}/meme-trending?timePeriod={}", self.base_url, period_str);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let json_value = response.json::<serde_json::Value>().await?;

                if let Some(array) = json_value.as_array() {
                    let mut tokens = Vec::new();
                    for item in array {
                        let token = TrendingToken {
                            mint_address: item["tokenAddress"].as_str().unwrap_or("").to_string(),
                            symbol: item["tokenTicker"].as_str().unwrap_or("").to_string(),
                            name: item["tokenName"].as_str().unwrap_or("").to_string(),
                            price_usd: item["priceUsd"].as_f64().unwrap_or(0.0),
                            price_change_24h: item["marketCapPercentChange"].as_f64().unwrap_or(0.0),
                            price_change_7d: 0.0,
                            volume_24h: item["volumeSol"].as_f64().unwrap_or(0.0),
                            market_cap: item["marketCapSol"].as_f64().unwrap_or(0.0),
                            holders: item["top10Holders"].as_f64().unwrap_or(0.0) as u64,
                            rank: 0,
                            logo_uri: item["tokenImage"].as_str().map(|s| s.to_string()),
                        };
                        tokens.push(token);
                    }

                    for (i, token) in tokens.iter_mut().enumerate() {
                        token.rank = (i + 1) as u32;
                    }

                    Ok(tokens)
                } else {
                    Err(MarketDataError::ParsingError("Expected array response".to_string()))
                }
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get trending tokens: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets detailed information about a specific token.
    ///
    /// # Arguments
    ///
    /// * `token_symbol` - &str - The token symbol (e.g., "BONK", "SOL", "USDC").
    ///
    /// # Returns
    ///
    /// Result<TokenInfo, MarketDataError> - Detailed token information.
    pub async fn get_token_info(
        &mut self,
        token_symbol: &str,
    ) -> Result<TokenInfo, MarketDataError> {
        if token_symbol.is_empty() {
            return Err(MarketDataError::InvalidTokenMint(
                "Token symbol cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/token-analysis?tokenTicker={}", self.base_url, token_symbol);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let token_info = response.json::<TokenInfo>().await?;
                Ok(token_info)
            }
            StatusCode::NOT_FOUND => {
                Err(MarketDataError::TokenNotFound(token_symbol.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get token info: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets token creator analysis and related tokens.
    ///
    /// # Arguments
    ///
    /// * `token_symbol` - &str - The token symbol (e.g., "BONK", "SOL", "USDC").
    ///
    /// # Returns
    ///
    /// Result<TokenAnalysis, MarketDataError> - Creator analysis and related tokens.
    pub async fn get_token_analysis(
        &mut self,
        token_symbol: &str,
    ) -> Result<TokenAnalysis, MarketDataError> {
        if token_symbol.is_empty() {
            return Err(MarketDataError::InvalidTokenMint(
                "Token symbol cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/token-analysis?tokenTicker={}", self.base_url, token_symbol);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let analysis = response.json::<TokenAnalysis>().await?;
                Ok(analysis)
            }
            StatusCode::NOT_FOUND => {
                Err(MarketDataError::TokenNotFound(token_symbol.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get token analysis: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets quick token information by address (mint or pair address).
    ///
    /// # Arguments
    ///
    /// * `address` - &str - The token mint address or pair address.
    ///
    /// # Returns
    ///
    /// Result<TokenInfo, MarketDataError> - Quick token information.
    pub async fn get_token_info_by_address(
        &mut self,
        address: &str,
    ) -> Result<TokenInfo, MarketDataError> {
        self.validate_token_mint(address)?;

        let url = format!("{}/clipboard-pair-info?address={}", self.base_url, address);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let token_info = response.json::<TokenInfo>().await?;
                Ok(token_info)
            }
            StatusCode::NOT_FOUND => {
                Err(MarketDataError::TokenNotFound(address.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get token info by address: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets current price data for a token.
    ///
    /// # Arguments
    ///
    /// * `token_mint` - &str - The token mint address.
    ///
    /// # Returns
    ///
    /// Result<PriceData, MarketDataError> - Current price data.
    pub async fn get_token_price(
        &mut self,
        token_mint: &str,
    ) -> Result<PriceData, MarketDataError> {
        self.validate_token_mint(token_mint)?;

        let url = format!("{}/price/{}", self.base_url, token_mint);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let price_data = response.json::<PriceData>().await?;
                Ok(price_data)
            }
            StatusCode::NOT_FOUND => {
                Err(MarketDataError::TokenNotFound(token_mint.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get price: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets historical price feed for a token.
    ///
    /// # Arguments
    ///
    /// * `token_mint` - &str - The token mint address.
    /// * `time_period` - TimePeriod - The time period for price history.
    ///
    /// # Returns
    ///
    /// Result<PriceFeed, MarketDataError> - Historical price feed.
    pub async fn get_price_feed(
        &mut self,
        token_mint: &str,
        time_period: TimePeriod,
    ) -> Result<PriceFeed, MarketDataError> {
        self.validate_token_mint(token_mint)?;

        let period_str = match time_period {
            TimePeriod::OneHour => "1h",
            TimePeriod::TwentyFourHours => "24h",
            TimePeriod::SevenDays => "7d",
            TimePeriod::ThirtyDays => "30d",
        };

        let url = format!("{}/price-feed/{}?period={}", self.base_url, token_mint, period_str);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let price_feed = response.json::<PriceFeed>().await?;
                Ok(price_feed)
            }
            StatusCode::NOT_FOUND => {
                Err(MarketDataError::TokenNotFound(token_mint.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get price feed: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets chart data for a token.
    ///
    /// # Arguments
    ///
    /// * `token_mint` - &str - The token mint address.
    /// * `timeframe` - ChartTimeframe - The chart timeframe.
    /// * `limit` - Option<usize> - Maximum number of candles to retrieve.
    ///
    /// # Returns
    ///
    /// Result<TokenChart, MarketDataError> - Chart data with candles.
    pub async fn get_token_chart(
        &mut self,
        token_mint: &str,
        timeframe: ChartTimeframe,
        limit: Option<usize>,
    ) -> Result<TokenChart, MarketDataError> {
        self.validate_token_mint(token_mint)?;

        let timeframe_str = match timeframe {
            ChartTimeframe::OneMinute => "1m",
            ChartTimeframe::FiveMinutes => "5m",
            ChartTimeframe::FifteenMinutes => "15m",
            ChartTimeframe::OneHour => "1h",
            ChartTimeframe::FourHours => "4h",
            ChartTimeframe::OneDay => "1d",
            ChartTimeframe::OneWeek => "1w",
        };

        let mut url = format!("{}/chart/{}?timeframe={}", self.base_url, token_mint, timeframe_str);
        if let Some(limit) = limit {
            url = format!("{}&limit={}", url, limit);
        }

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let chart = response.json::<TokenChart>().await?;
                Ok(chart)
            }
            StatusCode::NOT_FOUND => {
                Err(MarketDataError::TokenNotFound(token_mint.to_string()))
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get chart: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets overall market statistics (via trending endpoint).
    ///
    /// # Returns
    ///
    /// Result<MarketStats, MarketDataError> - Market statistics.
    pub async fn get_market_stats(&mut self) -> Result<MarketStats, MarketDataError> {
        let url = format!("{}/meme-trending?timePeriod=24h", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let json_value = response.json::<serde_json::Value>().await?;

                if let Some(array) = json_value.as_array() {
                    let total_volume = array
                        .iter()
                        .map(|t| t["volumeSol"].as_f64().unwrap_or(0.0))
                        .sum::<f64>();
                    let total_market_cap = array
                        .iter()
                        .map(|t| t["marketCapSol"].as_f64().unwrap_or(0.0))
                        .sum::<f64>();

                    let stats = MarketStats {
                        total_volume_24h: total_volume,
                        total_market_cap,
                        active_traders_24h: array.len() as u64,
                        total_transactions_24h: array.len() as u64,
                        trending_tokens_count: array.len() as u32,
                        new_tokens_24h: array.len() as u32,
                    };
                    Ok(stats)
                } else {
                    Err(MarketDataError::ParsingError("Expected array response".to_string()))
                }
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get market stats: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Searches for tokens by name or symbol.
    ///
    /// # Arguments
    ///
    /// * `query` - &str - The search query.
    /// * `limit` - Option<usize> - Maximum number of results.
    ///
    /// # Returns
    ///
    /// Result<TokenSearch, MarketDataError> - Search results.
    pub async fn search_tokens(
        &mut self,
        query: &str,
        limit: Option<usize>,
    ) -> Result<TokenSearch, MarketDataError> {
        if query.is_empty() {
            return Err(MarketDataError::ApiError(
                "Search query cannot be empty".to_string(),
            ));
        }

        let mut url = format!(
            "{}/search-v3?searchQuery={}",
            self.base_url,
            urlencoding::encode(query)
        );
        if let Some(limit) = limit {
            url = format!("{}&limit={}", url, limit);
        }

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        match response.status() {
            StatusCode::OK => {
                let results = response.json::<Vec<TokenSearchResult>>().await?;
                Ok(TokenSearch {
                    query: query.to_string(),
                    results,
                })
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Search failed: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Gets batch price data for multiple tokens.
    ///
    /// # Arguments
    ///
    /// * `token_mints` - &[String] - Array of token mint addresses.
    ///
    /// # Returns
    ///
    /// Result<Vec<PriceData>, MarketDataError> - Price data for all requested tokens.
    pub async fn get_batch_prices(
        &mut self,
        token_mints: &[String],
    ) -> Result<Vec<PriceData>, MarketDataError> {
        for mint in token_mints {
            self.validate_token_mint(mint)?;
        }

        let url = format!("{}/batch-prices", self.base_url);
        let payload = serde_json::json!({
            "mints": token_mints
        });

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, Some(payload))
            .await?;

        match response.status() {
            StatusCode::OK => {
                let prices = response.json::<Vec<PriceData>>().await?;
                Ok(prices)
            }
            StatusCode::UNAUTHORIZED => Err(MarketDataError::AuthError(AuthError::Unauthorized)),
            StatusCode::BAD_REQUEST => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!("Bad request: {}", error_text)))
            }
            status => {
                let error_text = response.text().await?;
                Err(MarketDataError::ApiError(format!(
                    "Failed to get batch prices: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    /// Validates a token mint address.
    ///
    /// # Arguments
    ///
    /// * `mint` - &str - The token mint address.
    ///
    /// # Returns
    ///
    /// Result<(), MarketDataError> - Ok if valid, error otherwise.
    fn validate_token_mint(&self, mint: &str) -> Result<(), MarketDataError> {
        if mint.is_empty() {
            return Err(MarketDataError::InvalidTokenMint(
                "Token mint cannot be empty".to_string(),
            ));
        }

        if mint.len() < 32 || mint.len() > 44 {
            return Err(MarketDataError::InvalidTokenMint(format!(
                "Invalid mint address length: {}",
                mint
            )));
        }

        if !mint.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(MarketDataError::InvalidTokenMint(format!(
                "Invalid characters in mint address: {}",
                mint
            )));
        }

        Ok(())
    }
}