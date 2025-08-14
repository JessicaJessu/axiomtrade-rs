use crate::auth::{AuthClient, AuthError};
use crate::models::trading::{
    BuyOrderRequest,
    OrderResponse,
    OrderStatus,
    QuoteRequest,
    QuoteResponse,
    SellOrderRequest,
    SwapOrderRequest,
    TransactionSimulation,
    TradingLimits,
};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid token mint: {0}")]
    InvalidTokenMint(String),

    #[error("Insufficient balance: {0}")]
    InsufficientBalance(String),

    #[error("Slippage exceeded: {0}")]
    SlippageExceeded(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub struct TradingClient {
    auth_client: AuthClient,
    base_url: String,
    default_slippage: f64,
}

impl TradingClient {
    ///
    /// Creates a new trading client.
    ///
    /// # Returns
    /// * Result<TradingClient, TradingError> - A new trading client instance.
    ///
    pub fn new() -> Result<Self, TradingError> {
        Ok(Self {
            auth_client: AuthClient::new()?,
            base_url: "https://axiom.trade/api".to_string(),
            default_slippage: 5.0,
        })
    }

    ///
    /// Creates a trading client with custom settings.
    ///
    /// # Arguments
    /// * base_url: String - The base URL for the API.
    /// * default_slippage: f64 - Default slippage percentage.
    ///
    /// # Returns
    /// * Result<TradingClient, TradingError> - A new trading client instance.
    ///
    pub fn with_settings(base_url: String, default_slippage: f64) -> Result<Self, TradingError> {
        Ok(Self {
            auth_client: AuthClient::new()?,
            base_url,
            default_slippage,
        })
    }

    ///
    /// Buys a token with SOL.
    ///
    /// # Arguments
    /// * token_mint: &str - The token mint address.
    /// * amount_sol: f64 - Amount of SOL to spend.
    /// * slippage_percent: Option<f64> - Slippage tolerance percentage.
    ///
    /// # Returns
    /// * Result<OrderResponse, TradingError> - The order response.
    ///
    pub async fn buy_token(
        &mut self,
        token_mint: &str,
        amount_sol: f64,
        slippage_percent: Option<f64>,
    ) -> Result<OrderResponse, TradingError> {
        self.validate_token_mint(token_mint)?;
        self.validate_amount(amount_sol, "SOL")?;

        let request = BuyOrderRequest {
            token_mint: token_mint.to_string(),
            amount_sol,
            slippage_percent: slippage_percent.unwrap_or(self.default_slippage),
            priority_fee: None,
        };

        let url = format!("{}/batched-send-tx-v2", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(request).map_err(|e| {
                    TradingError::ParsingError(format!("Failed to serialize request: {}", e))
                })?),
            )
            .await?;

        self.handle_order_response(response).await
    }

    ///
    /// Sells a token for SOL.
    ///
    /// # Arguments
    /// * token_mint: &str - The token mint address.
    /// * amount_tokens: f64 - Amount of tokens to sell.
    /// * slippage_percent: Option<f64> - Slippage tolerance percentage.
    ///
    /// # Returns
    /// * Result<OrderResponse, TradingError> - The order response.
    ///
    pub async fn sell_token(
        &mut self,
        token_mint: &str,
        amount_tokens: f64,
        slippage_percent: Option<f64>,
    ) -> Result<OrderResponse, TradingError> {
        self.validate_token_mint(token_mint)?;
        self.validate_amount(amount_tokens, "tokens")?;

        let request = SellOrderRequest {
            token_mint: token_mint.to_string(),
            amount_tokens,
            slippage_percent: slippage_percent.unwrap_or(self.default_slippage),
            priority_fee: None,
        };

        let url = format!("{}/batched-send-tx-v2", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(request).map_err(|e| {
                    TradingError::ParsingError(format!("Failed to serialize request: {}", e))
                })?),
            )
            .await?;

        self.handle_order_response(response).await
    }

    ///
    /// Swaps one token for another.
    ///
    /// # Arguments
    /// * from_mint: &str - The source token mint address.
    /// * to_mint: &str - The destination token mint address.
    /// * amount: f64 - Amount of source tokens to swap.
    /// * slippage_percent: Option<f64> - Slippage tolerance percentage.
    ///
    /// # Returns
    /// * Result<OrderResponse, TradingError> - The order response.
    ///
    pub async fn swap_tokens(
        &mut self,
        from_mint: &str,
        to_mint: &str,
        amount: f64,
        slippage_percent: Option<f64>,
    ) -> Result<OrderResponse, TradingError> {
        self.validate_token_mint(from_mint)?;
        self.validate_token_mint(to_mint)?;
        self.validate_amount(amount, "tokens")?;

        if from_mint == to_mint {
            return Err(TradingError::ApiError(
                "Cannot swap token to itself".to_string(),
            ));
        }

        let request = SwapOrderRequest {
            from_mint: from_mint.to_string(),
            to_mint: to_mint.to_string(),
            amount,
            slippage_percent: slippage_percent.unwrap_or(self.default_slippage),
            priority_fee: None,
        };

        let url = format!("{}/batched-send-tx-v2", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(request).map_err(|e| {
                    TradingError::ParsingError(format!("Failed to serialize request: {}", e))
                })?),
            )
            .await?;

        self.handle_order_response(response).await
    }

    ///
    /// Gets a quote for a swap.
    ///
    /// # Arguments
    /// * input_mint: &str - The input token mint address.
    /// * output_mint: &str - The output token mint address.
    /// * amount: f64 - Amount of input tokens.
    /// * slippage_percent: Option<f64> - Slippage tolerance percentage.
    ///
    /// # Returns
    /// * Result<QuoteResponse, TradingError> - The quote response.
    ///
    pub async fn get_quote(
        &mut self,
        input_mint: &str,
        output_mint: &str,
        amount: f64,
        slippage_percent: Option<f64>,
    ) -> Result<QuoteResponse, TradingError> {
        self.validate_token_mint(input_mint)?;
        self.validate_token_mint(output_mint)?;
        self.validate_amount(amount, "tokens")?;

        let request = QuoteRequest {
            input_mint: input_mint.to_string(),
            output_mint: output_mint.to_string(),
            amount,
            slippage_percent: slippage_percent.unwrap_or(self.default_slippage),
        };

        let url = format!("{}/quote", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(request).map_err(|e| {
                    TradingError::ParsingError(format!("Failed to serialize request: {}", e))
                })?),
            )
            .await?;

        match response.status() {
            StatusCode::OK => {
                let quote = response.json::<QuoteResponse>().await?;
                Ok(quote)
            }
            StatusCode::UNAUTHORIZED => {
                Err(TradingError::AuthError(AuthError::Unauthorized))
            }
            StatusCode::BAD_REQUEST => {
                let error_text = response.text().await?;
                Err(TradingError::ApiError(format!("Bad request: {}", error_text)))
            }
            status => {
                let error_text = response.text().await?;
                Err(TradingError::ApiError(format!(
                    "Failed to get quote: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    ///
    /// Simulates a transaction before execution.
    ///
    /// # Arguments
    /// * transaction: &str - The base64 encoded transaction.
    ///
    /// # Returns
    /// * Result<TransactionSimulation, TradingError> - The simulation result.
    ///
    pub async fn simulate_transaction(
        &mut self,
        transaction: &str,
    ) -> Result<TransactionSimulation, TradingError> {
        let url = format!("{}/simulate", self.base_url);
        let payload = serde_json::json!({
            "transaction": transaction
        });

        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(payload),
            )
            .await?;

        match response.status() {
            StatusCode::OK => {
                let simulation = response.json::<TransactionSimulation>().await?;
                Ok(simulation)
            }
            StatusCode::UNAUTHORIZED => {
                Err(TradingError::AuthError(AuthError::Unauthorized))
            }
            status => {
                let error_text = response.text().await?;
                Err(TradingError::ApiError(format!(
                    "Simulation failed: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    ///
    /// Gets trading limits for the current user.
    ///
    /// # Returns
    /// * Result<TradingLimits, TradingError> - The trading limits.
    ///
    pub async fn get_trading_limits(&mut self) -> Result<TradingLimits, TradingError> {
        Ok(TradingLimits {
            min_sol_amount: 0.01,
            max_sol_amount: 100.0,
            max_slippage_percent: 50.0,
            default_slippage_percent: 5.0,
            priority_fee_lamports: 5000,
        })
    }

    ///
    /// Handles the order response from the API.
    ///
    /// # Arguments
    /// * response: reqwest::Response - The HTTP response.
    ///
    /// # Returns
    /// * Result<OrderResponse, TradingError> - The parsed order response.
    ///
    async fn handle_order_response(
        &self,
        response: reqwest::Response,
    ) -> Result<OrderResponse, TradingError> {
        match response.status() {
            StatusCode::OK => {
                let order = response.json::<OrderResponse>().await?;
                match order.status {
                    OrderStatus::Success => Ok(order),
                    OrderStatus::Failed => {
                        Err(TradingError::TransactionFailed(format!(
                            "Transaction failed: {}",
                            order.signature
                        )))
                    }
                    _ => Ok(order),
                }
            }
            StatusCode::UNAUTHORIZED => {
                Err(TradingError::AuthError(AuthError::Unauthorized))
            }
            StatusCode::BAD_REQUEST => {
                let error_text = response.text().await?;
                if error_text.contains("insufficient") {
                    Err(TradingError::InsufficientBalance(error_text))
                } else if error_text.contains("slippage") {
                    Err(TradingError::SlippageExceeded(error_text))
                } else {
                    Err(TradingError::ApiError(error_text))
                }
            }
            status => {
                let error_text = response.text().await?;
                Err(TradingError::ApiError(format!(
                    "Order failed: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    ///
    /// Validates a token mint address.
    ///
    /// # Arguments
    /// * mint: &str - The token mint address.
    ///
    /// # Returns
    /// * Result<(), TradingError> - Ok if valid, error otherwise.
    ///
    fn validate_token_mint(&self, mint: &str) -> Result<(), TradingError> {
        if mint.is_empty() {
            return Err(TradingError::InvalidTokenMint(
                "Token mint cannot be empty".to_string(),
            ));
        }

        if mint.len() < 32 || mint.len() > 44 {
            return Err(TradingError::InvalidTokenMint(format!(
                "Invalid mint address length: {}",
                mint
            )));
        }

        if !mint.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(TradingError::InvalidTokenMint(format!(
                "Invalid characters in mint address: {}",
                mint
            )));
        }

        Ok(())
    }

    ///
    /// Validates an amount.
    ///
    /// # Arguments
    /// * amount: f64 - The amount to validate.
    /// * unit: &str - The unit name for error messages.
    ///
    /// # Returns
    /// * Result<(), TradingError> - Ok if valid, error otherwise.
    ///
    fn validate_amount(&self, amount: f64, unit: &str) -> Result<(), TradingError> {
        if amount <= 0.0 {
            return Err(TradingError::ApiError(format!(
                "Amount must be positive, got {} {}",
                amount, unit
            )));
        }

        if amount.is_nan() || amount.is_infinite() {
            return Err(TradingError::ApiError(format!(
                "Invalid amount: {} {}",
                amount, unit
            )));
        }

        Ok(())
    }
}