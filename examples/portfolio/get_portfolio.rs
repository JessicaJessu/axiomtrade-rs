/// Portfolio Retrieval Example
/// 
/// This example demonstrates how to fetch and analyze a complete portfolio
/// including all wallets, balances, and token positions.

use axiomtrade_rs::api::portfolio::PortfolioClient;
use axiomtrade_rs::auth::AuthClient;
use std::env;

#[tokio::main]
async fn main() {
    // Load credentials and authenticate
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Portfolio Retrieval Example");
    println!("===========================\n");

    // Create auth client and login
    let mut auth_client = match AuthClient::new() {
        Ok(client) => client,
        Err(e) => {
            println!("Failed to create auth client: {}", e);
            return;
        }
    };
    
    match auth_client.login(&email, &password, None).await {
        Ok(_tokens) => println!("✓ Logged in successfully\n"),
        Err(e) => {
            println!("✗ Login failed: {}", e);
            return;
        }
    }
    
    // Create portfolio client
    let mut portfolio_client = match PortfolioClient::new() {
        Ok(client) => client,
        Err(e) => {
            println!("Failed to create portfolio client: {}", e);
            return;
        }
    };

    println!("Fetching portfolio information...\n");

    // Example wallet addresses (in production, these would be user's actual wallets)
    let example_wallets = vec![
        "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
        "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
    ];

    // Get portfolio summary
    match portfolio_client.get_portfolio_summary(&example_wallets).await {
        Ok(portfolio) => {
            println!("📊 Portfolio Summary");
            println!("====================");
            println!("  Total Value SOL: {:.4} SOL", portfolio.balance_stats.total_value_sol);
            println!("  Available SOL: {:.4} SOL", portfolio.balance_stats.available_balance_sol);
            println!("  Unrealized PnL: {:.4} SOL", portfolio.balance_stats.unrealized_pnl_sol);
            
            // Performance metrics
            println!("\n📈 Performance Metrics:");
            println!("  1 Day PnL: {:.4} SOL", portfolio.performance_metrics.one_day.total_pnl);
            println!("  7 Day PnL: {:.4} SOL", portfolio.performance_metrics.seven_day.total_pnl);
            println!("  30 Day PnL: {:.4} SOL", portfolio.performance_metrics.thirty_day.total_pnl);
            println!("  All Time PnL: {:.4} SOL", portfolio.performance_metrics.all_time.total_pnl);
            
            // Top positions
            if !portfolio.top_positions.is_empty() {
                println!("\n💎 Top Positions:");
                for (i, position) in portfolio.top_positions.iter().take(5).enumerate() {
                    println!("  {}. {} ({})", 
                        i + 1, 
                        position.symbol.as_deref().unwrap_or("Unknown"), 
                        position.name.as_deref().unwrap_or("Unknown")
                    );
                    if let Some(amount) = position.amount {
                        println!("     Amount: {:.4}", amount);
                    }
                    if let Some(value_usd) = position.value_usd {
                        println!("     Value: ${:.2}", value_usd);
                    }
                    if let Some(pnl_percent) = position.pnl_percent {
                        println!("     PnL: {}{:.2}%", 
                            if pnl_percent >= 0.0 { "+" } else { "" },
                            pnl_percent
                        );
                    }
                }
            }
            
            // Recent transactions
            if !portfolio.transactions.is_empty() {
                println!("\n📝 Recent Transactions:");
                for tx in portfolio.transactions.iter().take(3) {
                    if let (Some(symbol), Some(tx_type), Some(amount), Some(value_usd)) = 
                        (&tx.symbol, &tx.transaction_type, tx.amount, tx.value_usd) {
                        println!("  • {} {} {} for ${:.2}", tx_type, amount, symbol, value_usd);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error getting portfolio summary: {}", e);
        }
    }

    // Get batch balances for all wallets
    println!("\n📈 Wallet Balances");
    println!("==================");
    
    match portfolio_client.get_batch_balance(&example_wallets).await {
        Ok(batch_response) => {
            println!("Retrieved {} wallet balances\n", batch_response.balances.len());
            
            let mut total_sol = 0.0;
            let mut total_usd = 0.0;
            let mut total_tokens = 0;
            
            for (address, balance) in batch_response.balances.iter() {
                println!("Wallet: {}...", &address[..8]);
                println!("  SOL Balance: {:.6} SOL", balance.sol_balance);
                println!("  Total Value: ${:.2}", balance.total_value_usd);
                println!("  Token Count: {}", balance.token_balances.len());
                
                total_sol += balance.sol_balance;
                total_usd += balance.total_value_usd;
                total_tokens += balance.token_balances.len();
                
                if !balance.token_balances.is_empty() {
                    println!("  Top Tokens:");
                    let mut tokens: Vec<_> = balance.token_balances.values().collect();
                    tokens.sort_by(|a, b| b.value_usd.partial_cmp(&a.value_usd).unwrap());
                    
                    for token in tokens.iter().take(3) {
                        println!("    • {} - {} (${:.2})", 
                            token.symbol,
                            token.ui_amount,
                            token.value_usd
                        );
                    }
                }
                println!();
            }
            
            println!("Portfolio Statistics:");
            println!("  Total SOL: {:.6} SOL", total_sol);
            println!("  Total USD Value: ${:.2}", total_usd);
            println!("  Unique Tokens: {}", total_tokens);
            println!("  Average Wallet Value: ${:.2}", 
                total_usd / batch_response.balances.len() as f64);
        }
        Err(e) => println!("Error getting batch balances: {}", e)
    }

    // Get individual wallet details
    println!("\n💎 Detailed Wallet Analysis");
    println!("===========================");
    
    for wallet in example_wallets.iter().take(1) {
        match portfolio_client.get_balance(wallet).await {
            Ok(balance) => {
                println!("Wallet: {}", wallet);
                println!("  SOL: {} SOL", balance.sol_balance);
                println!("  Total Value: ${:.2}", balance.total_value_usd);
                
                if !balance.token_balances.is_empty() {
                    println!("\n  Token Holdings:");
                    let mut tokens: Vec<_> = balance.token_balances.values().collect();
                    tokens.sort_by(|a, b| b.value_usd.partial_cmp(&a.value_usd).unwrap());
                    
                    for token in tokens.iter() {
                        if token.value_usd > 1.0 {
                            println!("    {} ({})", token.symbol, token.name);
                            println!("      Amount: {}", token.ui_amount);
                            println!("      Value: ${:.2}", token.value_usd);
                            println!("      Price: ${:.6}/token", token.price_per_token);
                        }
                    }
                }
            }
            Err(e) => println!("Error getting wallet balance: {}", e)
        }
    }

    println!("\n✅ Portfolio analysis completed!");
}