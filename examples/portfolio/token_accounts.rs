/// Token Accounts Analysis Example
/// 
/// This example demonstrates detailed analysis of token accounts,
/// including position sizing, risk assessment, and portfolio optimization.

use axiomtrade_rs::api::portfolio::PortfolioClient;
use axiomtrade_rs::auth::AuthClient;
use dotenvy::dotenv;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials and authenticate
    dotenv().ok();
    let mut client = authenticate().await?;

    println!("Starting token accounts analysis...");

    // Demo wallets for analysis
    let demo_wallets = vec![
        "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
        "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
    ];
    
    // Get batch balances for the wallets
    let batch_response = client.get_batch_balance(&demo_wallets).await?;
    
    if batch_response.balances.is_empty() {
        println!("No wallet balances found");
        return Ok(());
    }

    // Analyze token distribution across all wallets
    let mut token_summary: HashMap<String, TokenSummary> = HashMap::new();
    let mut total_positions = 0;
    let mut total_value = 0.0;

    println!("Analyzing token accounts across {} wallets...", batch_response.balances.len());

    for (wallet_address, wallet_balance) in &batch_response.balances {
        println!("\nWallet: {} ({} tokens)", 
            &wallet_address[..8], wallet_balance.token_balances.len());
        
        // Add SOL balance as a "token"
        if wallet_balance.sol_balance > 0.0 {
            let sol_value_usd = wallet_balance.sol_balance * 100.0; // Rough SOL price estimate
            total_positions += 1;
            total_value += sol_value_usd;
            
            let entry = token_summary.entry("SOL".to_string()).or_insert(TokenSummary {
                symbol: "SOL".to_string(),
                mint: "So11111111111111111111111111111111111111112".to_string(),
                total_balance: 0.0,
                total_usd_value: 0.0,
                wallet_count: 0,
                positions: Vec::new(),
            });
            
            entry.total_balance += wallet_balance.sol_balance;
            entry.total_usd_value += sol_value_usd;
            entry.wallet_count += 1;
            entry.positions.push(TokenPosition {
                wallet_address: wallet_address.clone(),
                balance: wallet_balance.sol_balance,
                usd_value: sol_value_usd,
            });
            
            if sol_value_usd > 10.0 {
                println!("  SOL - {:.6} (${:.2})", wallet_balance.sol_balance, sol_value_usd);
            }
        }
        
        // Process token balances
        for (mint_address, token_balance) in &wallet_balance.token_balances {
            total_positions += 1;
            
            let symbol = &token_balance.symbol;
            let balance = token_balance.ui_amount;
            let usd_value = token_balance.value_usd;
            
            total_value += usd_value;
            
            // Add to summary
            let entry = token_summary.entry(symbol.clone()).or_insert(TokenSummary {
                symbol: symbol.clone(),
                mint: mint_address.clone(),
                total_balance: 0.0,
                total_usd_value: 0.0,
                wallet_count: 0,
                positions: Vec::new(),
            });
            
            entry.total_balance += balance;
            entry.total_usd_value += usd_value;
            entry.wallet_count += 1;
            entry.positions.push(TokenPosition {
                wallet_address: wallet_address.clone(),
                balance,
                usd_value,
            });
            
            // Show individual position if significant
            if usd_value > 10.0 {
                println!("  {} - {:.6} (${:.2})", symbol, balance, usd_value);
            }
        }
    }

    println!("\nToken Portfolio Summary:");
    println!("Total positions: {}", total_positions);
    println!("Total portfolio value: ${:.2}", total_value);
    println!("Unique tokens: {}", token_summary.len());

    // Sort tokens by total USD value
    let mut sorted_tokens: Vec<_> = token_summary.values().collect();
    sorted_tokens.sort_by(|a, b| b.total_usd_value.partial_cmp(&a.total_usd_value).unwrap());

    // Top 10 token holdings
    println!("\nTop 10 Token Holdings:");
    println!("{:<15} {:>15} {:>15} {:>10} {:>12}", 
        "Symbol", "Total Balance", "USD Value", "Wallets", "Avg Position");
    println!("{}", "-".repeat(80));

    for token in sorted_tokens.iter().take(10) {
        let avg_position = token.total_usd_value / token.wallet_count as f64;
        println!("{:<15} {:>15.6} {:>15.2} {:>10} {:>12.2}", 
            token.symbol,
            token.total_balance,
            token.total_usd_value,
            token.wallet_count,
            avg_position
        );
    }

    // Risk analysis
    println!("\nRisk Analysis:");
    
    // Concentration risk
    let top_5_value: f64 = sorted_tokens.iter()
        .take(5)
        .map(|t| t.total_usd_value)
        .sum();
    let concentration_ratio = (top_5_value / total_value) * 100.0;
    println!("Top 5 tokens concentration: {:.1}%", concentration_ratio);
    
    if concentration_ratio > 70.0 {
        println!("  WARNING: High concentration risk detected");
    }

    // Small position analysis
    let small_positions = sorted_tokens.iter()
        .filter(|t| t.total_usd_value < 5.0)
        .count();
    println!("Positions under $5: {} ({:.1}%)", 
        small_positions, 
        (small_positions as f64 / sorted_tokens.len() as f64) * 100.0
    );

    // Distribution analysis
    let mut distribution_buckets = [0; 5]; // <$1, $1-10, $10-100, $100-1000, >$1000
    
    for token in &sorted_tokens {
        let value = token.total_usd_value;
        let bucket = if value < 1.0 { 0 }
        else if value < 10.0 { 1 }
        else if value < 100.0 { 2 }
        else if value < 1000.0 { 3 }
        else { 4 };
        
        distribution_buckets[bucket] += 1;
    }

    println!("\nPosition Size Distribution:");
    let bucket_labels = ["<$1", "$1-10", "$10-100", "$100-1K", ">$1K"];
    for (i, &count) in distribution_buckets.iter().enumerate() {
        let percentage = (count as f64 / sorted_tokens.len() as f64) * 100.0;
        println!("  {}: {} positions ({:.1}%)", bucket_labels[i], count, percentage);
    }

    // Wallet diversity analysis
    println!("\nWallet Diversity Analysis:");
    for token in sorted_tokens.iter().take(5) {
        println!("{}:", token.symbol);
        println!("  Held in {} wallets", token.wallet_count);
        
        // Find largest position
        if let Some(largest) = token.positions.iter()
            .max_by(|a, b| a.usd_value.partial_cmp(&b.usd_value).unwrap()) {
            let largest_percentage = (largest.usd_value / token.total_usd_value) * 100.0;
            println!("  Largest position: ${:.2} ({:.1}%) in wallet {}", 
                largest.usd_value, largest_percentage, &largest.wallet_address[..8]);
        }
        
        // Check if well distributed
        let positions_over_20pct = token.positions.iter()
            .filter(|p| (p.usd_value / token.total_usd_value) > 0.20)
            .count();
        
        if positions_over_20pct > 1 {
            println!("  WARNING: {} positions over 20% of total holding", positions_over_20pct);
        }
    }

    // Optimization suggestions
    println!("\nOptimization Suggestions:");
    
    if concentration_ratio > 50.0 {
        println!("- Consider reducing concentration in top holdings");
    }
    
    if small_positions > 20 {
        println!("- Consider consolidating or closing positions under $5");
    }
    
    let dust_positions = sorted_tokens.iter()
        .filter(|t| t.total_usd_value < 1.0)
        .count();
    
    if dust_positions > 10 {
        println!("- Consider cleaning up {} dust positions", dust_positions);
    }

    // Most distributed tokens
    let well_distributed: Vec<_> = sorted_tokens.iter()
        .filter(|t| t.wallet_count > 2 && t.total_usd_value > 50.0)
        .collect();
    
    if !well_distributed.is_empty() {
        println!("\nWell Distributed Holdings:");
        for token in well_distributed {
            println!("  {} - {} wallets, ${:.2}", 
                token.symbol, token.wallet_count, token.total_usd_value);
        }
    }

    println!("\nToken accounts analysis completed");
    Ok(())
}

#[derive(Debug)]
struct TokenSummary {
    symbol: String,
    mint: String,
    total_balance: f64,
    total_usd_value: f64,
    wallet_count: usize,
    positions: Vec<TokenPosition>,
}

#[derive(Debug)]
struct TokenPosition {
    wallet_address: String,
    balance: f64,
    usd_value: f64,
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