use std::collections::HashMap;

use reqwest::StatusCode;
use serde_json::Value;
use thiserror::Error;

use crate::auth::{AuthClient, AuthError};
use crate::models::portfolio::{BatchBalanceRequest, BatchBalanceResponse, TokenBalance, WalletBalance};
use crate::models::portfolio_v5::PortfolioV5Response;

#[derive(Error, Debug)]
pub enum PortfolioError {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid wallet address: {0}")]
    InvalidWalletAddress(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Parsing error: {0}")]
    ParsingError(String),
}

pub struct PortfolioClient {
    auth_client: AuthClient,
}

impl PortfolioClient {
    ///
    /// Creates a new portfolio client.
    ///
    /// # Returns
    ///
    /// Result<PortfolioClient, PortfolioError> - A new portfolio client instance
    ///
    pub fn new() -> Result<Self, PortfolioError> {
        Ok(Self {
            auth_client: AuthClient::new()?,
        })
    }

    ///
    /// Gets the balance for a single wallet address.
    ///
    /// # Arguments
    ///
    /// * `wallet_address` - &str - The Solana wallet address
    ///
    /// # Returns
    ///
    /// Result<WalletBalance, PortfolioError> - The wallet balance information
    ///
    pub async fn get_balance(
        &mut self,
        wallet_address: &str,
    ) -> Result<WalletBalance, PortfolioError> {
        self.validate_wallet_address(wallet_address)?;
        let mut balances = self.get_batch_balance(&[wallet_address.to_string()]).await?;
        balances
            .balances
            .remove(wallet_address)
            .ok_or_else(|| PortfolioError::ApiError("Balance not found in response".to_string()))
    }

    ///
    /// Gets balances for multiple wallet addresses.
    ///
    /// # Arguments
    ///
    /// * `wallet_addresses` - &[String] - Array of Solana wallet addresses
    ///
    /// # Returns
    ///
    /// Result<BatchBalanceResponse, PortfolioError> - The batch balance response
    ///
    pub async fn get_batch_balance(
        &mut self,
        wallet_addresses: &[String],
    ) -> Result<BatchBalanceResponse, PortfolioError> {
        for address in wallet_addresses {
            self.validate_wallet_address(address)?;
        }

        let request = BatchBalanceRequest {
            public_keys: wallet_addresses.to_vec(),
        };

        self.auth_client.ensure_valid_authentication().await?;
        
        // Note: This endpoint might only be available on main domain
        // Let's try the main domain first, then fall back to API servers
        let url = "https://axiom.trade/api/batched-sol-balance".to_string();
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(request).map_err(|e| {
                    PortfolioError::ParsingError(format!("Failed to serialize request: {}", e))
                })?),
            )
            .await?;

        match response.status() {
            StatusCode::OK => {
                let data = response.json::<Value>().await?;
                self.parse_batch_balance_response(data)
            }
            StatusCode::UNAUTHORIZED => Err(PortfolioError::AuthError(AuthError::Unauthorized)),
            StatusCode::BAD_REQUEST => {
                let error_text = response.text().await?;
                Err(PortfolioError::ApiError(format!("Bad request: {}", error_text)))
            }
            status => {
                let error_text = response.text().await?;
                Err(PortfolioError::ApiError(format!(
                    "Unexpected status {}: {}",
                    status, error_text
                )))
            }
        }
    }

    ///
    /// Gets the user's portfolio summary.
    ///
    /// # Arguments
    ///
    /// * `wallet_addresses` - &[String] - Array of Solana wallet addresses to get portfolio for
    ///
    /// # Returns
    ///
    /// Result<PortfolioV5Response, PortfolioError> - The portfolio summary
    ///
    pub async fn get_portfolio_summary(
        &mut self,
        wallet_addresses: &[String],
    ) -> Result<PortfolioV5Response, PortfolioError> {
        for address in wallet_addresses {
            self.validate_wallet_address(address)?;
        }

        self.auth_client.ensure_valid_authentication().await?;
        let base_url = self.auth_client.get_current_endpoint();
        let url = format!("{}/portfolio-v5", base_url);
        
        // IMPORTANT: Wallet addresses MUST be sorted alphabetically before joining
        // This is verified from the JavaScript implementation: o.sort().join(",")
        let mut sorted_addresses = wallet_addresses.to_vec();
        sorted_addresses.sort();
        let wallet_address_raw = sorted_addresses.join(",");
        
        // The portfolio-v5 endpoint requires specific fields (VERIFIED from JS code)
        // TODO: totalSolBalance and tokenAddressToAmountMap should be calculated from:
        // 1. /batched-sol-balance response
        // 2. /batched-wallet-token-accounts response
        let request_body = serde_json::json!({
            "walletAddressRaw": wallet_address_raw,
            "isOtherWallet": false,  // false for own wallets
            "totalSolBalance": 0,    // Sum of balanceSol from all wallets
            "tokenAddressToAmountMap": {},  // Map of tokenAddress -> sum of balanceRaw
            "timeOffset": chrono::Local::now().offset().local_minus_utc() / 60
        });
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, Some(request_body))
            .await?;

        match response.status() {
            StatusCode::OK => {
                let portfolio_response = response.json::<PortfolioV5Response>().await?;
                Ok(portfolio_response)
            }
            StatusCode::UNAUTHORIZED => Err(PortfolioError::AuthError(AuthError::Unauthorized)),
            status => {
                let error_text = response.text().await?;
                Err(PortfolioError::ApiError(format!(
                    "Failed to get portfolio: {} - {}",
                    status, error_text
                )))
            }
        }
    }

    ///
    /// Validates a Solana wallet address format.
    ///
    /// # Arguments
    ///
    /// * `address` - &str - The wallet address to validate
    ///
    /// # Returns
    ///
    /// Result<(), PortfolioError> - Ok if valid, error otherwise
    ///
    fn validate_wallet_address(&self, address: &str) -> Result<(), PortfolioError> {
        if address.len() < 32 || address.len() > 44 {
            return Err(PortfolioError::InvalidWalletAddress(format!(
                "Invalid address length: {}",
                address
            )));
        }

        if !address.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(PortfolioError::InvalidWalletAddress(format!(
                "Invalid characters in address: {}",
                address
            )));
        }

        Ok(())
    }

    ///
    /// Parses the batch balance response from the API.
    ///
    /// # Arguments
    ///
    /// * `data` - Value - The JSON response from the API
    ///
    /// # Returns
    ///
    /// Result<BatchBalanceResponse, PortfolioError> - The parsed batch balance response
    ///
    fn parse_batch_balance_response(
        &self,
        data: Value,
    ) -> Result<BatchBalanceResponse, PortfolioError> {
        let mut balances = HashMap::new();

        if let Some(balances_obj) = data.as_object() {
            for (wallet_address, balance_data) in balances_obj {
                if let Ok(wallet_balance) =
                    serde_json::from_value::<WalletBalance>(balance_data.clone())
                {
                    balances.insert(wallet_address.clone(), wallet_balance);
                } else {
                    let sol_balance = balance_data["sol_balance"].as_f64().unwrap_or(0.0);

                    let mut token_balances = HashMap::new();
                    if let Some(tokens) = balance_data["tokens"].as_array() {
                        for token in tokens {
                            if let Ok(token_balance) =
                                serde_json::from_value::<TokenBalance>(token.clone())
                            {
                                token_balances
                                    .insert(token_balance.mint_address.clone(), token_balance);
                            }
                        }
                    }

                    let total_value_usd = balance_data["total_value_usd"]
                        .as_f64()
                        .unwrap_or(sol_balance * 100.0);

                    balances.insert(
                        wallet_address.clone(),
                        WalletBalance {
                            sol_balance,
                            token_balances,
                            total_value_usd,
                        },
                    );
                }
            }
        }

        Ok(BatchBalanceResponse {
            balances,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }
}