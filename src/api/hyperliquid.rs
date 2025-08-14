use crate::errors::{AxiomError, Result};
use crate::models::hyperliquid::*;
use serde_json::json;

/// Hyperliquid perpetual futures client
/// Direct integration with Hyperliquid API (no authentication required)
pub struct HyperliquidClient {
    client: reqwest::Client,
    base_url: String,
}

impl HyperliquidClient {
    /// Create a new Hyperliquid client.
    ///
    /// Returns:
    ///     Self: New instance of HyperliquidClient.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.hyperliquid.xyz".to_string(),
        }
    }

    /// Get clearinghouse state for a user.
    ///
    /// Args:
    ///     user_address: &str - Ethereum address of the user.
    ///
    /// Returns:
    ///     Result<ClearinghouseState>: User's position and margin information.
    pub async fn get_clearinghouse_state(&self, user_address: &str) -> Result<ClearinghouseState> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "clearinghouseState",
            "user": user_address
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let state: ClearinghouseState = response.json().await?;
            Ok(state)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get user role information.
    ///
    /// Args:
    ///     user_address: &str - Ethereum address of the user.
    ///
    /// Returns:
    ///     Result<UserRole>: User's role and permissions on Hyperliquid.
    pub async fn get_user_role(&self, user_address: &str) -> Result<UserRole> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "userRole",
            "user": user_address
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let role: UserRole = response.json().await?;
            Ok(role)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get metadata for all available markets.
    ///
    /// Returns:
    ///     Result<MarketMetadata>: Information about all trading pairs and their specifications.
    pub async fn get_market_metadata(&self) -> Result<MarketMetadata> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "meta"
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let metadata: MarketMetadata = response.json().await?;
            Ok(metadata)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get current mid prices for all markets.
    ///
    /// Returns:
    ///     Result<AllMids>: Current mid prices for all trading pairs.
    pub async fn get_all_mids(&self) -> Result<AllMids> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "allMids"
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let mids: AllMids = response.json().await?;
            Ok(mids)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get open orders for a user.
    ///
    /// Args:
    ///     user_address: &str - Ethereum address of the user.
    ///
    /// Returns:
    ///     Result<Vec<OpenOrder>>: List of user's open orders.
    pub async fn get_open_orders(&self, user_address: &str) -> Result<Vec<OpenOrder>> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "openOrders",
            "user": user_address
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let orders: Vec<OpenOrder> = response.json().await?;
            Ok(orders)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get user's trade history.
    ///
    /// Args:
    ///     user_address: &str - Ethereum address of the user.
    ///
    /// Returns:
    ///     Result<Vec<UserFill>>: List of user's executed trades.
    pub async fn get_user_fills(&self, user_address: &str) -> Result<Vec<UserFill>> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "userFills",
            "user": user_address
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let fills: Vec<UserFill> = response.json().await?;
            Ok(fills)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get orderbook for a specific market.
    ///
    /// Args:
    ///     coin: &str - The coin symbol (e.g., "BTC", "ETH").
    ///
    /// Returns:
    ///     Result<Orderbook>: Current orderbook data for the market.
    pub async fn get_orderbook(&self, coin: &str) -> Result<Orderbook> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "l2Book",
            "coin": coin
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let orderbook: Orderbook = response.json().await?;
            Ok(orderbook)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get recent trades for a specific market.
    ///
    /// Args:
    ///     coin: &str - The coin symbol (e.g., "BTC", "ETH").
    ///
    /// Returns:
    ///     Result<Vec<RecentTrade>>: List of recent trades for the market.
    pub async fn get_recent_trades(&self, coin: &str) -> Result<Vec<RecentTrade>> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "recentTrades",
            "coin": coin
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let trades: Vec<RecentTrade> = response.json().await?;
            Ok(trades)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get 24-hour statistics for all markets.
    ///
    /// Returns:
    ///     Result<Vec<MarketStats>>: 24-hour statistics for all trading pairs.
    pub async fn get_market_stats(&self) -> Result<Vec<MarketStats>> {
        let url = format!("{}/info", self.base_url);
        let payload = json!({
            "type": "24hrStats"
        });

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let stats: Vec<MarketStats> = response.json().await?;
            Ok(stats)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get user's funding history.
    ///
    /// Args:
    ///     user_address: &str - Ethereum address of the user.
    ///     start_time: Option<u64> - Start timestamp (milliseconds).
    ///     end_time: Option<u64> - End timestamp (milliseconds).
    ///
    /// Returns:
    ///     Result<Vec<FundingPayment>>: List of funding payments.
    pub async fn get_user_funding(
        &self,
        user_address: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<Vec<FundingPayment>> {
        let url = format!("{}/info", self.base_url);
        let mut payload = json!({
            "type": "userFunding",
            "user": user_address
        });

        if let Some(start) = start_time {
            payload["startTime"] = json!(start);
        }
        if let Some(end) = end_time {
            payload["endTime"] = json!(end);
        }

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await?;

        if response.status().is_success() {
            let funding: Vec<FundingPayment> = response.json().await?;
            Ok(funding)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }
}