use crate::auth::AuthClient;
use crate::errors::{AxiomError, Result};
use crate::models::social::*;
use serde_json::json;

pub struct SocialClient {
    auth_client: AuthClient,
    base_url: String,
}

impl SocialClient {
    /// Create a new social trading client.
    ///
    /// Returns:
    ///     Self: New instance of SocialClient.
    pub fn new() -> Result<Self> {
        Ok(Self {
            auth_client: AuthClient::new()?,
            base_url: "https://api8.axiom.trade".to_string(),
        })
    }

    /// Get tracked wallets for the authenticated user.
    ///
    /// Returns:
    ///     Vec<TrackedWallet>: List of tracked wallets with metadata.
    pub async fn get_tracked_wallets(&mut self) -> Result<Vec<TrackedWallet>> {
        let url = format!("{}/tracked-wallets-v2", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let wallets: Vec<TrackedWallet> = response.json().await?;
            Ok(wallets)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get transactions from tracked wallets.
    ///
    /// Args:
    ///     request: TrackedWalletRequest - Filter criteria for transactions.
    ///
    /// Returns:
    ///     Vec<TrackedTransaction>: List of transactions from tracked wallets.
    pub async fn get_tracked_wallet_transactions(
        &mut self,
        request: TrackedWalletRequest,
    ) -> Result<Vec<TrackedTransaction>> {
        let url = format!("{}/tracked-wallet-transactions-v2", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(request)?),
            )
            .await?;

        if response.status().is_success() {
            let transactions: Vec<TrackedTransaction> = response.json().await?;
            Ok(transactions)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get user's watchlist.
    ///
    /// Returns:
    ///     Vec<WatchlistItem>: List of tokens on the user's watchlist.
    pub async fn get_watchlist(&mut self) -> Result<Vec<WatchlistItem>> {
        let url = format!("{}/watchlist", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let watchlist: Vec<WatchlistItem> = response.json().await?;
            Ok(watchlist)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get Twitter/X feed settings.
    ///
    /// Returns:
    ///     TwitterSettings: User's Twitter feed configuration.
    pub async fn get_twitter_settings(&mut self) -> Result<TwitterSettings> {
        let url = format!("{}/twitter-settings", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let settings: TwitterSettings = response.json().await?;
            Ok(settings)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get Twitter/X feed with trading-related content.
    ///
    /// Args:
    ///     include_truth_social: bool - Whether to include Truth Social content.
    ///
    /// Returns:
    ///     Vec<TwitterFeedItem>: List of social media posts related to trading.
    pub async fn get_twitter_feed(
        &mut self,
        include_truth_social: bool,
    ) -> Result<Vec<TwitterFeedItem>> {
        let url = format!(
            "{}/twitter-feed-new-2?includeTruthSocial={}",
            self.base_url, include_truth_social
        );
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let feed: Vec<TwitterFeedItem> = response.json().await?;
            Ok(feed)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Add a wallet to tracking list.
    ///
    /// Args:
    ///     wallet_address: &str - Solana wallet address to track.
    ///     name: Option<&str> - Optional name for the wallet.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn add_tracked_wallet(
        &mut self,
        wallet_address: &str,
        name: Option<&str>,
    ) -> Result<bool> {
        let url = format!("{}/tracked-wallets-v2", self.base_url);
        let payload = json!({
            "address": wallet_address,
            "name": name.unwrap_or(""),
            "action": "add"
        });

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, Some(payload))
            .await?;

        Ok(response.status().is_success())
    }

    /// Remove a wallet from tracking list.
    ///
    /// Args:
    ///     wallet_address: &str - Solana wallet address to stop tracking.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn remove_tracked_wallet(
        &mut self,
        wallet_address: &str,
    ) -> Result<bool> {
        let url = format!("{}/tracked-wallets-v2", self.base_url);
        let payload = json!({
            "address": wallet_address,
            "action": "remove"
        });

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, Some(payload))
            .await?;

        Ok(response.status().is_success())
    }

    /// Add a token to watchlist.
    ///
    /// Args:
    ///     token_address: &str - Token mint address.
    ///     symbol: &str - Token symbol.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn add_to_watchlist(
        &mut self,
        token_address: &str,
        symbol: &str,
    ) -> Result<bool> {
        let url = format!("{}/watchlist", self.base_url);
        let payload = json!({
            "tokenAddress": token_address,
            "symbol": symbol,
            "action": "add"
        });

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, Some(payload))
            .await?;

        Ok(response.status().is_success())
    }

    /// Remove a token from watchlist.
    ///
    /// Args:
    ///     token_address: &str - Token mint address to remove.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn remove_from_watchlist(
        &mut self,
        token_address: &str,
    ) -> Result<bool> {
        let url = format!("{}/watchlist", self.base_url);
        let payload = json!({
            "tokenAddress": token_address,
            "action": "remove"
        });

        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, Some(payload))
            .await?;

        Ok(response.status().is_success())
    }
}