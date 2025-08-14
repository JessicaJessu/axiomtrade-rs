/// Real-time Portfolio Monitoring Example
/// 
/// This example demonstrates continuous portfolio monitoring with
/// real-time updates, alerts, and performance tracking.

use axiomtrade_rs::api::portfolio::PortfolioClient;
use axiomtrade_rs::auth::AuthClient;
use axiomtrade_rs::models::portfolio_v5::PortfolioV5Response;
use dotenvy::dotenv;
use std::env;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials and authenticate
    dotenv().ok();
    let mut client = authenticate().await?;

    println!("Starting real-time portfolio monitoring...");
    println!("Press Ctrl+C to stop monitoring");

    // Initial portfolio snapshot - using demo wallets
    let demo_wallets = vec![
        "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
        "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
    ];
    
    let mut last_portfolio = client.get_portfolio_summary(&demo_wallets).await?;
    let mut last_update = Instant::now();
    let mut update_count = 0;

    println!("\nInitial Portfolio State:");
    print_portfolio_summary(&last_portfolio);

    // Initialize tracking variables
    let mut total_value_history = Vec::new();
    let initial_value = last_portfolio.balance_stats.total_value_sol;
    total_value_history.push((Instant::now(), initial_value));

    println!("\nStarting monitoring loop (updates every 30 seconds)...");

    loop {
        // Wait for next update interval
        sleep(Duration::from_secs(30)).await;
        
        println!("\n{}", "=".repeat(60));
        println!("Update #{} - {:.1}s since last update", 
            update_count + 1, 
            last_update.elapsed().as_secs_f64()
        );

        // Fetch current portfolio
        match client.get_portfolio_summary(&demo_wallets).await {
            Ok(current_portfolio) => {
                update_count += 1;
                last_update = Instant::now();

                // Detect changes
                let changes = detect_portfolio_changes(&last_portfolio, &current_portfolio);
                
                // Print current state
                print_portfolio_summary(&current_portfolio);
                
                // Track value history
                total_value_history.push((Instant::now(), current_portfolio.balance_stats.total_value_sol));
                
                // Keep only last 100 data points
                if total_value_history.len() > 100 {
                    total_value_history.remove(0);
                }

                // Report changes
                if !changes.is_empty() {
                    println!("\nDetected Changes:");
                    for change in &changes {
                        println!("  {}", change);
                    }
                } else {
                    println!("No significant changes detected");
                }

                // Performance analysis
                analyze_performance(&total_value_history, initial_value);

                // Check for alerts
                check_alerts(&current_portfolio, &last_portfolio);

                // Update last portfolio
                last_portfolio = current_portfolio;

                // Memory cleanup every 20 updates
                if update_count % 20 == 0 {
                    println!("Performing memory cleanup...");
                    // In a real application, you might save historical data here
                }
            }
            Err(e) => {
                println!("Failed to fetch portfolio: {}", e);
                
                // Check if we need to refresh authentication
                if e.to_string().contains("unauthorized") || e.to_string().contains("401") {
                    println!("Authentication may have expired, you may need to re-authenticate");
                }
            }
        }

        // Exit condition for demo (remove in production)
        if update_count >= 10 {
            println!("\nDemo completed after {} updates", update_count);
            break;
        }
    }

    // Final summary
    println!("\nMonitoring Session Summary:");
    println!("Total updates: {}", update_count);
    println!("Initial value: {:.6} SOL", initial_value);
    println!("Final value: {:.6} SOL", last_portfolio.balance_stats.total_value_sol);
    
    let total_change = last_portfolio.balance_stats.total_value_sol - initial_value;
    let total_change_pct = if initial_value > 0.0 { (total_change / initial_value) * 100.0 } else { 0.0 };
    
    println!("Total change: {:.6} SOL ({:.2}%)", total_change, total_change_pct);

    Ok(())
}

fn print_portfolio_summary(portfolio: &PortfolioV5Response) {
    println!("Portfolio Summary:");
    println!("  Active positions: {}", portfolio.active_positions.len());
    println!("  Total SOL: {:.6}", portfolio.balance_stats.total_value_sol);
    println!("  Available SOL: {:.6}", portfolio.balance_stats.available_balance_sol);
    println!("  Unrealized PnL: {:.6} SOL", portfolio.balance_stats.unrealized_pnl_sol);
    println!("  Total transactions: {}", portfolio.transactions.len());
}

fn detect_portfolio_changes(
    old: &PortfolioV5Response, 
    new: &PortfolioV5Response
) -> Vec<String> {
    let mut changes = Vec::new();
    
    // Total SOL value changes
    let value_diff = new.balance_stats.total_value_sol - old.balance_stats.total_value_sol;
    if value_diff.abs() > 0.001 {
        let change_pct = if old.balance_stats.total_value_sol > 0.0 {
            (value_diff / old.balance_stats.total_value_sol) * 100.0
        } else { 0.0 };
        changes.push(format!("Total value: {:+.6} SOL ({:+.2}%)", value_diff, change_pct));
    }
    
    // Available balance changes
    let balance_diff = new.balance_stats.available_balance_sol - old.balance_stats.available_balance_sol;
    if balance_diff.abs() > 0.001 {
        changes.push(format!("Available balance: {:+.6} SOL", balance_diff));
    }
    
    // Position count changes
    if new.active_positions.len() != old.active_positions.len() {
        let pos_diff = new.active_positions.len() as i32 - old.active_positions.len() as i32;
        changes.push(format!("Active positions: {:+} positions", pos_diff));
    }
    
    // Transaction count changes
    if new.transactions.len() != old.transactions.len() {
        let tx_diff = new.transactions.len() as i32 - old.transactions.len() as i32;
        changes.push(format!("New transactions: {:+}", tx_diff));
    }
    
    changes
}

fn analyze_performance(history: &[(Instant, f64)], initial_value: f64) {
    if history.len() < 2 {
        return;
    }
    
    let current_value = history.last().unwrap().1;
    let total_change = current_value - initial_value;
    let total_change_pct = (total_change / initial_value) * 100.0;
    
    println!("\nPerformance Analysis:");
    println!("  Since start: ${:.2} ({:+.2}%)", total_change, total_change_pct);
    
    // Recent performance (last 5 data points)
    if history.len() >= 5 {
        let recent_start = history[history.len() - 5].1;
        let recent_change = current_value - recent_start;
        let recent_change_pct = (recent_change / recent_start) * 100.0;
        
        println!("  Recent trend: ${:.2} ({:+.2}%)", recent_change, recent_change_pct);
    }
    
    // Volatility (standard deviation of recent changes)
    if history.len() >= 10 {
        let recent_values: Vec<f64> = history.iter()
            .rev()
            .take(10)
            .map(|(_, value)| *value)
            .collect();
        
        let mean = recent_values.iter().sum::<f64>() / recent_values.len() as f64;
        let variance = recent_values.iter()
            .map(|value| {
                let diff = value - mean;
                diff * diff
            })
            .sum::<f64>() / recent_values.len() as f64;
        
        let std_dev = variance.sqrt();
        let volatility_pct = (std_dev / mean) * 100.0;
        
        println!("  Volatility: {:.2}%", volatility_pct);
    }
}

fn check_alerts(
    current: &PortfolioV5Response,
    previous: &PortfolioV5Response
) {
    let value_change_pct = if previous.balance_stats.total_value_sol > 0.0 {
        ((current.balance_stats.total_value_sol - previous.balance_stats.total_value_sol) 
            / previous.balance_stats.total_value_sol) * 100.0
    } else { 0.0 };
    
    // Alert thresholds
    if value_change_pct > 5.0 {
        println!("\nALERT: Portfolio value increased by {:.2}%!", value_change_pct);
    } else if value_change_pct < -5.0 {
        println!("\nALERT: Portfolio value decreased by {:.2}%!", value_change_pct.abs());
    }
    
    // New position alerts
    if current.active_positions.len() > previous.active_positions.len() {
        let new_positions = current.active_positions.len() - previous.active_positions.len();
        println!("ALERT: {} new active positions detected", new_positions);
    }
    
    // New transaction alerts
    if current.transactions.len() > previous.transactions.len() {
        let new_transactions = current.transactions.len() - previous.transactions.len();
        println!("ALERT: {} new transactions detected", new_transactions);
    }
}

async fn authenticate() -> Result<PortfolioClient, Box<dyn std::error::Error>> {
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    // Create and login with auth client
    let mut auth_client = AuthClient::new()?;
    auth_client.login(&email, &password, None).await?;
    
    // Create portfolio client (uses same auth internally)
    let portfolio_client = PortfolioClient::new()?;
    
    Ok(portfolio_client)
}