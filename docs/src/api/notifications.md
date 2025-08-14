# Notifications API

The Axiom Trade notifications system provides comprehensive alert and messaging capabilities for monitoring trading activities, portfolio changes, and system events. This module supports multiple notification types, delivery methods, and configuration options to keep users informed of important events.

## Overview

The notifications API allows users to:

- Create and manage price alerts for specific tokens
- Monitor wallet activity with customizable filters
- Receive system notifications and announcements
- Configure email notifications with custom templates
- Set up notification preferences and delivery methods
- Track notification history and engagement metrics

## NotificationsClient

The `NotificationsClient` provides access to all notification functionality:

```rust
use axiomtrade_rs::api::notifications::NotificationsClient;

let mut client = NotificationsClient::new()?;
```

## Price Alerts

Price alerts notify users when token prices reach specified thresholds or meet certain conditions.

### Creating Price Alerts

```rust
use axiomtrade_rs::models::notifications::*;

let alert_request = PriceAlertRequest {
    token_address: "So11111111111111111111111111111111111111112".to_string(),
    alert_type: PriceAlertType::AbsolutePrice,
    target_price: 150.0,
    comparison: PriceComparison::Above,
    notification_methods: vec![
        NotificationMethod::Email,
        NotificationMethod::InApp,
    ],
    is_one_time: true,
    expires_at: Some(chrono::Utc::now() + chrono::Duration::days(30)),
};

let alert_id = client.create_price_alert(alert_request).await?;
println!("Created price alert: {}", alert_id);
```

### Price Alert Types

- **AbsolutePrice**: Alert when token reaches a specific price
- **PercentageChange**: Alert on percentage price movement
- **VolumeThreshold**: Alert when trading volume exceeds threshold
- **MarketCapThreshold**: Alert based on market capitalization changes

### Price Comparison Options

- **Above**: Trigger when price goes above target
- **Below**: Trigger when price goes below target
- **Equal**: Trigger when price equals target (within tolerance)
- **PercentageIncrease**: Trigger on percentage increase
- **PercentageDecrease**: Trigger on percentage decrease

### Managing Price Alerts

```rust
// Delete a price alert
let success = client.delete_price_alert("alert_id").await?;

// List all price alerts (via get_notifications)
let notifications = client.get_notifications().await?;
let price_alerts: Vec<_> = notifications
    .into_iter()
    .filter(|n| n.notification_type == NotificationType::PriceAlert)
    .collect();
```

## Wallet Activity Alerts

Monitor specific wallet addresses for trading activity and transactions.

### Creating Wallet Alerts

```rust
let wallet_alert = WalletAlertRequest {
    wallet_address: "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1".to_string(),
    activity_types: vec![
        WalletActivityType::Buy,
        WalletActivityType::Sell,
        WalletActivityType::LargeTransaction,
    ],
    min_transaction_value: Some(1000.0), // Minimum $1000 transactions
    token_filters: vec![
        "So11111111111111111111111111111111111111112".to_string(), // SOL
    ],
    notification_methods: vec![
        NotificationMethod::Email,
        NotificationMethod::InApp,
    ],
    is_active: true,
};

let alert_id = client.create_wallet_alert(wallet_alert).await?;
```

### Wallet Activity Types

- **Buy**: Token purchase transactions
- **Sell**: Token sale transactions
- **Swap**: Token swap operations
- **Transfer**: Token transfer events
- **LargeTransaction**: Transactions above specified value
- **NewToken**: First interaction with new tokens
- **AllActivity**: Monitor all wallet activity

## System Notifications

System notifications provide information about platform status, maintenance, and important updates.

### Retrieving Notifications

```rust
// Get user notifications
let notifications = client.get_notifications().await?;
for notification in notifications {
    println!("{}: {}", notification.title, notification.message);
    if !notification.is_read {
        println!("  Priority: {:?}", notification.priority);
        println!("  Created: {}", notification.created_at);
    }
}

// Get system announcements
let announcements = client.get_announcements().await?;
for announcement in announcements {
    println!("Announcement: {}", announcement.title);
    println!("Type: {:?}", announcement.announcement_type);
    println!("Target: {:?}", announcement.target_audience);
    
    if announcement.action_required {
        println!("Action required: {}", announcement.action_url.unwrap_or_default());
    }
}
```

### Notification Types

- **PriceAlert**: Price-based alerts
- **WalletActivity**: Wallet monitoring alerts
- **TradeExecution**: Trade confirmation notifications
- **SystemUpdate**: Platform updates and changes
- **SecurityAlert**: Security-related notifications
- **MarketNews**: Market information and news
- **SocialMention**: Social media mentions and sentiment

### Notification Priorities

- **Low**: Informational messages
- **Medium**: Standard notifications
- **High**: Important alerts requiring attention
- **Critical**: Urgent notifications requiring immediate action

### Managing Notifications

```rust
// Mark specific notification as read
let success = client.mark_notification_read("notification_id").await?;

// Mark all notifications as read
let success = client.mark_all_notifications_read().await?;
```

## Email Notifications

Configure email delivery for notifications with customizable templates and scheduling.

### Email Notification Settings

```rust
// Get current email settings
let settings = client.get_notification_settings().await?;
println!("Email notifications enabled: {}", settings.email_notifications);

// Update email settings
let mut updated_settings = settings;
updated_settings.email_notifications = true;
updated_settings.preferred_methods = vec![
    NotificationMethod::Email,
    NotificationMethod::InApp,
];

let success = client.update_notification_settings(updated_settings).await?;
```

### Notification Delivery Methods

- **InApp**: Display within the application interface
- **Email**: Send via email to configured address
- **Push**: Push notifications to mobile devices
- **Webhook**: HTTP POST to configured webhook URL
- **Telegram**: Send to Telegram bot/channel
- **Discord**: Send to Discord webhook

## Notification Configuration

### Notification Settings Structure

```rust
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
```

### Quiet Hours Configuration

Set times when notifications should be suppressed:

```rust
let quiet_hours = QuietHours {
    enabled: true,
    start_time: "22:00".to_string(), // 10 PM
    end_time: "07:00".to_string(),   // 7 AM
    timezone: "UTC".to_string(),
    days: vec![
        "monday".to_string(),
        "tuesday".to_string(),
        "wednesday".to_string(),
        "thursday".to_string(),
        "friday".to_string(),
    ],
};
```

### Frequency Limits

Control notification frequency to prevent spam:

```rust
let frequency_limits = FrequencyLimits {
    max_price_alerts_per_hour: 10,
    max_wallet_alerts_per_hour: 20,
    max_social_alerts_per_hour: 5,
    batch_similar_notifications: true,
    batch_delay_minutes: 15,
};
```

## Alert Management

### Best Practices

1. **Set Appropriate Thresholds**: Avoid too many low-value alerts
2. **Use Expiration Dates**: Set expiration for temporary alerts
3. **Configure Quiet Hours**: Respect user sleep schedules
4. **Batch Similar Notifications**: Reduce notification fatigue
5. **Monitor Delivery Success**: Track notification effectiveness

### Performance Considerations

- **Rate Limiting**: Respect API rate limits when creating multiple alerts
- **Batch Operations**: Use batch endpoints for multiple alerts
- **Efficient Filtering**: Use appropriate filters to reduce unnecessary notifications
- **Cleanup**: Remove expired or unnecessary alerts regularly

### Error Handling

```rust
use axiomtrade_rs::errors::AxiomError;

match client.create_price_alert(alert_request).await {
    Ok(alert_id) => {
        println!("Alert created successfully: {}", alert_id);
    }
    Err(AxiomError::Api { message }) => {
        eprintln!("API error creating alert: {}", message);
    }
    Err(AxiomError::Network(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Example: Complete Notification Setup

```rust
use axiomtrade_rs::{
    api::notifications::NotificationsClient,
    models::notifications::*,
    errors::Result,
};

async fn setup_comprehensive_notifications() -> Result<()> {
    let mut client = NotificationsClient::new()?;
    
    // Configure notification preferences
    let settings = NotificationSettings {
        email_notifications: true,
        push_notifications: true,
        in_app_notifications: true,
        price_alerts_enabled: true,
        wallet_alerts_enabled: true,
        social_mentions_enabled: false,
        market_news_enabled: true,
        trade_confirmations_enabled: true,
        security_alerts_enabled: true,
        quiet_hours: Some(QuietHours {
            enabled: true,
            start_time: "22:00".to_string(),
            end_time: "07:00".to_string(),
            timezone: "UTC".to_string(),
            days: vec!["saturday".to_string(), "sunday".to_string()],
        }),
        preferred_methods: vec![
            NotificationMethod::Email,
            NotificationMethod::InApp,
        ],
        frequency_limits: FrequencyLimits {
            max_price_alerts_per_hour: 5,
            max_wallet_alerts_per_hour: 10,
            max_social_alerts_per_hour: 3,
            batch_similar_notifications: true,
            batch_delay_minutes: 10,
        },
    };
    
    client.update_notification_settings(settings).await?;
    
    // Create price alert for SOL
    let sol_alert = PriceAlertRequest {
        token_address: "So11111111111111111111111111111111111111112".to_string(),
        alert_type: PriceAlertType::AbsolutePrice,
        target_price: 200.0,
        comparison: PriceComparison::Above,
        notification_methods: vec![
            NotificationMethod::Email,
            NotificationMethod::InApp,
        ],
        is_one_time: false, // Recurring alert
        expires_at: None,   // No expiration
    };
    
    let alert_id = client.create_price_alert(sol_alert).await?;
    println!("Created SOL price alert: {}", alert_id);
    
    // Create wallet monitoring alert
    let wallet_alert = WalletAlertRequest {
        wallet_address: "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1".to_string(),
        activity_types: vec![
            WalletActivityType::Buy,
            WalletActivityType::Sell,
        ],
        min_transaction_value: Some(500.0),
        token_filters: vec![], // Monitor all tokens
        notification_methods: vec![NotificationMethod::InApp],
        is_active: true,
    };
    
    let wallet_alert_id = client.create_wallet_alert(wallet_alert).await?;
    println!("Created wallet monitoring alert: {}", wallet_alert_id);
    
    // Check for new notifications
    let notifications = client.get_notifications().await?;
    println!("Current unread notifications: {}", 
        notifications.iter().filter(|n| !n.is_read).count()
    );
    
    Ok(())
}
```

## Security Considerations

- **Webhook Security**: Use HTTPS endpoints and validate webhook signatures
- **Email Privacy**: Be cautious with sensitive information in email notifications
- **Rate Limiting**: Implement client-side rate limiting to prevent abuse
- **Data Validation**: Validate all notification data before processing
- **Access Control**: Ensure users can only access their own notifications

The notifications API provides a comprehensive system for keeping users informed about their trading activities and platform updates. Proper configuration and management of notifications can significantly enhance the user experience while maintaining security and preventing notification fatigue.
