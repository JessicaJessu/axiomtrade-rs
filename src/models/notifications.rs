use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// User notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
    pub is_read: bool,
    pub priority: NotificationPriority,
    pub data: Option<serde_json::Value>,
    pub action_url: Option<String>,
}

/// Types of notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    PriceAlert,
    WalletActivity,
    TradeExecution,
    SystemUpdate,
    SecurityAlert,
    MarketNews,
    SocialMention,
}

/// Notification priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// System announcement
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub content: String,
    pub announcement_type: AnnouncementType,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub target_audience: TargetAudience,
    pub action_required: bool,
    pub action_url: Option<String>,
}

/// Types of system announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnouncementType {
    Maintenance,
    NewFeature,
    SecurityUpdate,
    MarketUpdate,
    GeneralInfo,
    Emergency,
}

/// Target audience for announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetAudience {
    AllUsers,
    PremiumUsers,
    BetaUsers,
    Developers,
    Traders,
}

/// Price alert request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceAlertRequest {
    pub token_address: String,
    pub alert_type: PriceAlertType,
    pub target_price: f64,
    pub comparison: PriceComparison,
    pub notification_methods: Vec<NotificationMethod>,
    pub is_one_time: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Types of price alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceAlertType {
    AbsolutePrice,
    PercentageChange,
    VolumeThreshold,
    MarketCapThreshold,
}

/// Price comparison operators
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceComparison {
    Above,
    Below,
    Equal,
    PercentageIncrease,
    PercentageDecrease,
}

/// Notification delivery methods
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationMethod {
    InApp,
    Email,
    Push,
    Webhook,
    Telegram,
    Discord,
}

/// Wallet activity alert request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletAlertRequest {
    pub wallet_address: String,
    pub activity_types: Vec<WalletActivityType>,
    pub min_transaction_value: Option<f64>,
    pub token_filters: Vec<String>,
    pub notification_methods: Vec<NotificationMethod>,
    pub is_active: bool,
}

/// Types of wallet activities to monitor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WalletActivityType {
    Buy,
    Sell,
    Swap,
    Transfer,
    LargeTransaction,
    NewToken,
    AllActivity,
}

/// User notification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub in_app_notifications: bool,
    pub price_alerts_enabled: bool,
    pub wallet_alerts_enabled: bool,
    pub social_mentions_enabled: bool,
    pub market_news_enabled: bool,
    pub trade_confirmations_enabled: bool,
    pub security_alerts_enabled: bool,
    pub quiet_hours: Option<QuietHours>,
    pub preferred_methods: Vec<NotificationMethod>,
    pub frequency_limits: FrequencyLimits,
}

/// Quiet hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuietHours {
    pub enabled: bool,
    pub start_time: String, // HH:MM format
    pub end_time: String,   // HH:MM format
    pub timezone: String,
    pub days: Vec<String>,  // ["monday", "tuesday", ...]
}

/// Frequency limits for notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrequencyLimits {
    pub max_price_alerts_per_hour: u32,
    pub max_wallet_alerts_per_hour: u32,
    pub max_social_alerts_per_hour: u32,
    pub batch_similar_notifications: bool,
    pub batch_delay_minutes: u32,
}