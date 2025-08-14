use crate::auth::AuthClient;
use crate::errors::{AxiomError, Result};
use crate::models::notifications::*;

pub struct NotificationsClient {
    auth_client: AuthClient,
    base_url: String,
}

impl NotificationsClient {
    /// Create a new notifications client.
    ///
    /// Returns:
    ///     Self: New instance of NotificationsClient.
    pub fn new() -> Result<Self> {
        Ok(Self {
            auth_client: AuthClient::new()?,
            base_url: "https://api8.axiom.trade".to_string(),
        })
    }

    /// Get user notifications.
    ///
    /// Returns:
    ///     Vec<Notification>: List of user notifications.
    pub async fn get_notifications(&mut self) -> Result<Vec<Notification>> {
        let url = format!("{}/get-notifications", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let notifications: Vec<Notification> = response.json().await?;
            Ok(notifications)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get system announcements.
    ///
    /// Returns:
    ///     Vec<Announcement>: List of system announcements.
    pub async fn get_announcements(&mut self) -> Result<Vec<Announcement>> {
        let url = format!("{}/get-announcement", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let announcements: Vec<Announcement> = response.json().await?;
            Ok(announcements)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Mark notification as read.
    ///
    /// Args:
    ///     notification_id: &str - ID of the notification to mark as read.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn mark_notification_read(&mut self, notification_id: &str) -> Result<bool> {
        let url = format!("{}/notifications/{}/read", self.base_url, notification_id);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, None)
            .await?;

        Ok(response.status().is_success())
    }

    /// Mark all notifications as read.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn mark_all_notifications_read(&mut self) -> Result<bool> {
        let url = format!("{}/notifications/read-all", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::POST, &url, None)
            .await?;

        Ok(response.status().is_success())
    }

    /// Create a price alert.
    ///
    /// Args:
    ///     alert: PriceAlertRequest - Alert configuration.
    ///
    /// Returns:
    ///     String: Created alert ID.
    pub async fn create_price_alert(&mut self, alert: PriceAlertRequest) -> Result<String> {
        let url = format!("{}/alerts/price", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(alert)?),
            )
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(id) = result.get("alertId").and_then(|v| v.as_str()) {
                Ok(id.to_string())
            } else {
                Err(AxiomError::InvalidResponse)
            }
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Delete a price alert.
    ///
    /// Args:
    ///     alert_id: &str - ID of the alert to delete.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn delete_price_alert(&mut self, alert_id: &str) -> Result<bool> {
        let url = format!("{}/alerts/price/{}", self.base_url, alert_id);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::DELETE, &url, None)
            .await?;

        Ok(response.status().is_success())
    }

    /// Create a wallet activity alert.
    ///
    /// Args:
    ///     alert: WalletAlertRequest - Wallet alert configuration.
    ///
    /// Returns:
    ///     String: Created alert ID.
    pub async fn create_wallet_alert(&mut self, alert: WalletAlertRequest) -> Result<String> {
        let url = format!("{}/alerts/wallet", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::POST,
                &url,
                Some(serde_json::to_value(alert)?),
            )
            .await?;

        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            if let Some(id) = result.get("alertId").and_then(|v| v.as_str()) {
                Ok(id.to_string())
            } else {
                Err(AxiomError::InvalidResponse)
            }
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Get user's notification settings.
    ///
    /// Returns:
    ///     NotificationSettings: User's notification preferences.
    pub async fn get_notification_settings(&mut self) -> Result<NotificationSettings> {
        let url = format!("{}/notifications/settings", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(reqwest::Method::GET, &url, None)
            .await?;

        if response.status().is_success() {
            let settings: NotificationSettings = response.json().await?;
            Ok(settings)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("{}: {}", status, error_text),
            })
        }
    }

    /// Update notification settings.
    ///
    /// Args:
    ///     settings: NotificationSettings - Updated notification preferences.
    ///
    /// Returns:
    ///     bool: Success status.
    pub async fn update_notification_settings(
        &mut self,
        settings: NotificationSettings,
    ) -> Result<bool> {
        let url = format!("{}/notifications/settings", self.base_url);
        let response = self
            .auth_client
            .make_authenticated_request(
                reqwest::Method::PUT,
                &url,
                Some(serde_json::to_value(settings)?),
            )
            .await?;

        Ok(response.status().is_success())
    }
}