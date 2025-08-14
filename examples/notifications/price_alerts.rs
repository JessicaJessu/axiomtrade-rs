/// Price Alert Management Example
/// 
/// This example demonstrates how to create, manage, and receive
/// price-based notifications for portfolio monitoring.

use axiomtrade_rs::{
    api::notifications::NotificationsClient,
    auth::AuthClient,
    errors::Result,
    models::notifications::*,
};
use std::env;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials and authenticate
    dotenvy::dotenv().ok();
    let mut notifications_client = NotificationsClient::new()?;

    println!("Price Alert Management Example");
    println!("Setting up and managing price-based notifications\n");

    // Get some tokens from portfolio for alert setup
    // Note: Portfolio access would typically require authentication
    // For demonstration, we'll use mock data
    let portfolio = simulate_get_portfolio();
    
    if portfolio.wallets.is_empty() {
        println!("No wallets found - using default tokens for demonstration");
    }

    println!("Step 1: Creating price alerts...");

    // Define alert configurations
    let alert_configs = vec![
        PriceAlertConfig {
            token_symbol: "SOL".to_string(),
            token_mint: "So11111111111111111111111111111111111111112".to_string(),
            alert_type: AlertType::PriceAbove,
            threshold: 150.0,
            message: "SOL price exceeded $150!".to_string(),
            enabled: true,
        },
        PriceAlertConfig {
            token_symbol: "SOL".to_string(),
            token_mint: "So11111111111111111111111111111111111111112".to_string(),
            alert_type: AlertType::PriceBelow,
            threshold: 80.0,
            message: "SOL price dropped below $80!".to_string(),
            enabled: true,
        },
        PriceAlertConfig {
            token_symbol: "BONK".to_string(),
            token_mint: "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(),
            alert_type: AlertType::PercentChange,
            threshold: 10.0,
            message: "BONK price changed by more than 10%!".to_string(),
            enabled: true,
        },
        PriceAlertConfig {
            token_symbol: "USDC".to_string(),
            token_mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            alert_type: AlertType::VolumeSpike,
            threshold: 1000000.0,
            message: "USDC volume spike detected!".to_string(),
            enabled: true,
        },
    ];

    let mut created_alerts = Vec::new();

    for config in alert_configs {
        println!("Creating alert for {} ({})", config.token_symbol, config.alert_type);
        
        match simulate_create_price_alert(&config) {
            Ok(alert) => {
                println!("  ✓ Alert created with ID: {}", alert.alert_id);
                println!("    Type: {}", config.alert_type);
                println!("    Threshold: {}", config.threshold);
                println!("    Message: {}", config.message);
                created_alerts.push(alert);
            }
            Err(e) => {
                println!("  ❌ Failed to create alert: {}", e);
            }
        }
    }

    println!("\nStep 2: Listing existing alerts...");
    
    match simulate_get_price_alerts() {
        Ok(alerts) => {
            println!("Found {} total price alerts:", alerts.len());
            
            for alert in &alerts {
                println!("  Alert ID: {}", alert.alert_id);
                println!("    Token: {}", alert.token_symbol);
                println!("    Type: {}", alert.alert_type);
                println!("    Threshold: {}", alert.threshold);
                println!("    Status: {}", if alert.enabled { "Enabled" } else { "Disabled" });
                println!("    Created: {}", alert.created_at.format("%Y-%m-%d %H:%M:%S"));
                
                if let Some(triggered) = &alert.last_triggered {
                    println!("    Last triggered: {}", triggered.format("%Y-%m-%d %H:%M:%S"));
                }
                
                println!();
            }
        }
        Err(e) => {
            println!("Failed to get alerts: {}", e);
        }
    }

    println!("Step 3: Testing alert conditions...");
    
    // Get current prices to test against thresholds
    let test_tokens = vec!["SOL", "BONK", "USDC"];
    
    for symbol in test_tokens {
        match get_current_price(symbol) {
            Ok(price) => {
                println!("Current {} price: ${:.6}", symbol, price);
                
                // Check which alerts would trigger
                for alert in &created_alerts {
                    if alert.token_symbol == symbol {
                        let would_trigger = match alert.alert_type.as_str() {
                            "price_above" => price > alert.threshold,
                            "price_below" => price < alert.threshold,
                            "percent_change" => false, // Would need historical data
                            "volume_spike" => false,   // Would need volume data
                            _ => false,
                        };
                        
                        if would_trigger {
                            println!("  🚨 Alert {} would trigger now!", alert.alert_id);
                        } else {
                            println!("  ✓ Alert {} conditions not met", alert.alert_id);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to get price for {}: {}", symbol, e);
            }
        }
    }

    println!("\nStep 4: Updating alert settings...");
    
    if let Some(alert) = created_alerts.first() {
        println!("Updating alert {} threshold...", alert.alert_id);
        
        let mut updated_config = PriceAlertConfig {
            token_symbol: alert.token_symbol.clone(),
            token_mint: alert.token_mint.clone(),
            alert_type: AlertType::PriceAbove,
            threshold: 200.0, // Updated threshold
            message: "SOL price exceeded $200! (Updated alert)".to_string(),
            enabled: true,
        };

        match simulate_update_price_alert(&alert.alert_id, &updated_config) {
            Ok(updated_alert) => {
                println!("  ✓ Alert updated successfully");
                println!("    New threshold: {}", updated_alert.threshold);
            }
            Err(e) => {
                println!("  ❌ Failed to update alert: {}", e);
            }
        }
    }

    println!("\nStep 5: Testing alert delivery methods...");
    
    // Configure delivery methods
    let delivery_config = AlertDeliveryConfig {
        email_enabled: true,
        email_address: env::var("AXIOM_EMAIL").ok(),
        sms_enabled: false,
        sms_number: None,
        push_enabled: true,
        webhook_url: None,
    };

    match simulate_configure_alert_delivery(&delivery_config) {
        Ok(()) => {
            println!("✓ Alert delivery methods configured");
            println!("  Email: {}", if delivery_config.email_enabled { "Enabled" } else { "Disabled" });
            println!("  SMS: {}", if delivery_config.sms_enabled { "Enabled" } else { "Disabled" });
            println!("  Push: {}", if delivery_config.push_enabled { "Enabled" } else { "Disabled" });
        }
        Err(e) => {
            println!("❌ Failed to configure delivery: {}", e);
        }
    }

    // Send a test alert
    println!("\nSending test alert...");
    match simulate_send_test_alert("This is a test alert from axiomtrade-rs") {
        Ok(()) => {
            println!("✓ Test alert sent successfully");
        }
        Err(e) => {
            println!("❌ Failed to send test alert: {}", e);
        }
    }

    println!("\nStep 6: Managing alert history...");
    
    match simulate_get_alert_history(30) {
        Ok(history) => {
            println!("Alert history for last 30 days:");
            println!("Total alerts triggered: {}", history.len());
            
            let mut by_type = std::collections::HashMap::new();
            for event in &history {
                *by_type.entry(event.alert_type.clone()).or_insert(0) += 1;
            }
            
            for (alert_type, count) in by_type {
                println!("  {}: {} alerts", alert_type, count);
            }
            
            // Show recent alerts
            println!("\nRecent alert events:");
            for event in history.iter().take(5) {
                println!("  {} - {} ({})", 
                    event.triggered_at.format("%Y-%m-%d %H:%M:%S"),
                    event.message,
                    event.token_symbol
                );
            }
        }
        Err(e) => {
            println!("Failed to get alert history: {}", e);
        }
    }

    println!("\nStep 7: Alert performance analytics...");
    
    match simulate_get_alert_analytics() {
        Ok(analytics) => {
            println!("Alert Performance Analytics:");
            println!("  Total alerts created: {}", analytics.total_alerts);
            println!("  Active alerts: {}", analytics.active_alerts);
            println!("  Alerts triggered today: {}", analytics.triggered_today);
            println!("  Average response time: {:.2}s", analytics.avg_response_time);
            println!("  Success rate: {:.1}%", analytics.success_rate * 100.0);
            
            println!("\nMost active tokens:");
            for (symbol, count) in analytics.top_tokens.iter().take(5) {
                println!("    {}: {} alerts", symbol, count);
            }
        }
        Err(e) => {
            println!("Failed to get analytics: {}", e);
        }
    }

    println!("\nStep 8: Cleanup (optional)...");
    
    let cleanup = std::env::var("CLEANUP_ALERTS").unwrap_or_else(|_| "no".to_string());
    
    if cleanup.to_lowercase() == "yes" {
        println!("Cleaning up created alerts...");
        
        for alert in &created_alerts {
            match simulate_delete_price_alert(&alert.alert_id) {
                Ok(()) => {
                    println!("  ✓ Deleted alert {}", alert.alert_id);
                }
                Err(e) => {
                    println!("  ❌ Failed to delete alert {}: {}", alert.alert_id, e);
                }
            }
        }
    } else {
        println!("Keeping created alerts (set CLEANUP_ALERTS=yes to remove)");
    }

    println!("\nPrice alert management example completed successfully!");
    println!("\nKey features demonstrated:");
    println!("- Creating various types of price alerts");
    println!("- Managing alert configurations");
    println!("- Testing alert conditions against current prices");
    println!("- Configuring delivery methods");
    println!("- Viewing alert history and analytics");
    println!("- Managing alert lifecycle");

    Ok(())
}

// Simulation functions for demonstration (would be real API calls in production)
fn simulate_get_portfolio() -> MockPortfolio {
    MockPortfolio {
        wallets: vec![
            MockWallet {
                address: "example1".to_string(),
                tokens: vec!["SOL".to_string(), "BONK".to_string()],
            },
        ],
    }
}

fn simulate_create_price_alert(_config: &PriceAlertConfig) -> Result<MockPriceAlert> {
    Ok(MockPriceAlert {
        alert_id: format!("alert_{}", chrono::Utc::now().timestamp()),
        token_symbol: _config.token_symbol.clone(),
        token_mint: _config.token_mint.clone(),
        alert_type: _config.alert_type.to_string(),
        threshold: _config.threshold,
        enabled: _config.enabled,
        created_at: chrono::Utc::now(),
        last_triggered: None,
    })
}

fn simulate_get_price_alerts() -> Result<Vec<MockPriceAlert>> {
    Ok(vec![
        MockPriceAlert {
            alert_id: "alert_1".to_string(),
            token_symbol: "SOL".to_string(),
            token_mint: "So11111111111111111111111111111111111111112".to_string(),
            alert_type: "price_above".to_string(),
            threshold: 150.0,
            enabled: true,
            created_at: chrono::Utc::now() - chrono::Duration::hours(24),
            last_triggered: None,
        },
        MockPriceAlert {
            alert_id: "alert_2".to_string(),
            token_symbol: "BONK".to_string(),
            token_mint: "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263".to_string(),
            alert_type: "percent_change".to_string(),
            threshold: 10.0,
            enabled: true,
            created_at: chrono::Utc::now() - chrono::Duration::hours(12),
            last_triggered: Some(chrono::Utc::now() - chrono::Duration::hours(2)),
        },
    ])
}

fn simulate_update_price_alert(_alert_id: &str, _config: &PriceAlertConfig) -> Result<MockPriceAlert> {
    Ok(MockPriceAlert {
        alert_id: _alert_id.to_string(),
        token_symbol: _config.token_symbol.clone(),
        token_mint: _config.token_mint.clone(),
        alert_type: _config.alert_type.to_string(),
        threshold: _config.threshold,
        enabled: _config.enabled,
        created_at: chrono::Utc::now() - chrono::Duration::hours(24),
        last_triggered: None,
    })
}

fn simulate_configure_alert_delivery(_config: &AlertDeliveryConfig) -> Result<()> {
    Ok(())
}

fn simulate_send_test_alert(_message: &str) -> Result<()> {
    Ok(())
}

fn simulate_get_alert_history(_days: u32) -> Result<Vec<AlertHistoryEvent>> {
    Ok(vec![
        AlertHistoryEvent {
            alert_type: "price_above".to_string(),
            token_symbol: "SOL".to_string(),
            message: "SOL price exceeded $150!".to_string(),
            triggered_at: chrono::Utc::now() - chrono::Duration::hours(6),
        },
        AlertHistoryEvent {
            alert_type: "percent_change".to_string(),
            token_symbol: "BONK".to_string(),
            message: "BONK price changed by more than 10%!".to_string(),
            triggered_at: chrono::Utc::now() - chrono::Duration::hours(2),
        },
    ])
}

fn simulate_get_alert_analytics() -> Result<AlertPerformanceAnalytics> {
    Ok(AlertPerformanceAnalytics {
        total_alerts: 10,
        active_alerts: 8,
        triggered_today: 3,
        avg_response_time: 1.2,
        success_rate: 0.95,
        top_tokens: vec![
            ("SOL".to_string(), 5),
            ("BONK".to_string(), 3),
            ("USDC".to_string(), 2),
        ],
    })
}

fn simulate_delete_price_alert(_alert_id: &str) -> Result<()> {
    Ok(())
}

// Mock structures for demonstration (these would be defined in the main library)
#[derive(Debug, Clone)]
struct PriceAlertConfig {
    token_symbol: String,
    token_mint: String,
    alert_type: AlertType,
    threshold: f64,
    message: String,
    enabled: bool,
}

#[derive(Debug, Clone)]
enum AlertType {
    PriceAbove,
    PriceBelow,
    PercentChange,
    VolumeSpike,
}

impl std::fmt::Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AlertType::PriceAbove => write!(f, "price_above"),
            AlertType::PriceBelow => write!(f, "price_below"),
            AlertType::PercentChange => write!(f, "percent_change"),
            AlertType::VolumeSpike => write!(f, "volume_spike"),
        }
    }
}

#[derive(Debug)]
struct AlertDeliveryConfig {
    email_enabled: bool,
    email_address: Option<String>,
    sms_enabled: bool,
    sms_number: Option<String>,
    push_enabled: bool,
    webhook_url: Option<String>,
}

#[derive(Debug)]
struct MockPortfolio {
    wallets: Vec<MockWallet>,
}

#[derive(Debug)]
struct MockWallet {
    address: String,
    tokens: Vec<String>,
}

#[derive(Debug, Clone)]
struct MockPriceAlert {
    alert_id: String,
    token_symbol: String,
    token_mint: String,
    alert_type: String,
    threshold: f64,
    enabled: bool,
    created_at: DateTime<Utc>,
    last_triggered: Option<DateTime<Utc>>,
}

#[derive(Debug)]
struct AlertHistoryEvent {
    alert_type: String,
    token_symbol: String,
    message: String,
    triggered_at: DateTime<Utc>,
}

#[derive(Debug)]
struct AlertPerformanceAnalytics {
    total_alerts: u32,
    active_alerts: u32,
    triggered_today: u32,
    avg_response_time: f64,
    success_rate: f64,
    top_tokens: Vec<(String, u32)>,
}

fn get_current_price(symbol: &str) -> Result<f64> {
    // In a real implementation, this would fetch current price
    // For demonstration, return mock prices
    let mock_prices = match symbol {
        "SOL" => 120.50,
        "BONK" => 0.000025,
        "USDC" => 1.0001,
        _ => 1.0,
    };
    
    Ok(mock_prices)
}

