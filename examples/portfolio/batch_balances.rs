/// Batch Balance Queries Example
/// 
/// This example demonstrates efficient batch querying of wallet balances
/// for multiple addresses simultaneously.

use axiomtrade_rs::api::portfolio::PortfolioClient;
use axiomtrade_rs::auth::AuthClient;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Load credentials and authenticate
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Batch Balance Queries Example");
    println!("=============================\n");

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

    println!("Demonstrating batch balance queries...\n");

    // Example wallet addresses (these could be from user input or database)
    let wallet_addresses = vec![
        "DYw8jCTfwHNRJhhmFcbXvVDTqWMEVFBX6ZKUmG5CNSKK".to_string(),
        "5FHwkrdxntdK24hgQU8qgBjn35Y1zwhz1GZwCkP2UJnM".to_string(),
        "7xLk17EQQ5KLDLDe44wCmupJKJjTGd8hs3eSVVhCx932".to_string(),
        "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
        "3kUvgBCZepkyENe2gqzxXfGmXiJXrKEXqVmCxtMmfEgD".to_string(),
    ];

    println!("Querying balances for {} wallets:", wallet_addresses.len());
    for (i, addr) in wallet_addresses.iter().enumerate() {
        println!("  {}: {}...{}", 
            i + 1, 
            &addr[..6],
            &addr[addr.len()-6..]
        );
    }

    // Perform batch balance query
    println!("\n📊 Performing batch balance query...");
    
    match portfolio_client.get_batch_balance(&wallet_addresses).await {
        Ok(batch_response) => {
            println!("✓ Batch query successful!\n");
            
            // Display results in a table format
            println!("{:<15} {:>15} {:>15} {:>10}", 
                "Address", "SOL Balance", "USD Value", "Tokens");
            println!("{}", "-".repeat(65));
            
            let mut total_sol = 0.0;
            let mut total_usd = 0.0;
            let mut wallet_stats = Vec::new();
            
            for address in &wallet_addresses {
                if let Some(balance) = batch_response.balances.get(address) {
                    println!("{:<15} {:>15.6} {:>15.2} {:>10}", 
                        format!("{}...{}", &address[..6], &address[address.len()-6..]),
                        balance.sol_balance,
                        balance.total_value_usd,
                        balance.token_balances.len()
                    );
                    
                    total_sol += balance.sol_balance;
                    total_usd += balance.total_value_usd;
                    
                    wallet_stats.push((address.clone(), balance.clone()));
                } else {
                    println!("{:<15} {:>15} {:>15} {:>10}", 
                        format!("{}...{}", &address[..6], &address[address.len()-6..]),
                        "N/A",
                        "N/A",
                        "0"
                    );
                }
            }
            
            println!("{}", "-".repeat(65));
            println!("{:<15} {:>15.6} {:>15.2}", "TOTAL", total_sol, total_usd);
            
            // Analyze token distribution
            println!("\n📈 Token Distribution Analysis:");
            let mut token_counts: HashMap<String, usize> = HashMap::new();
            let mut token_values: HashMap<String, f64> = HashMap::new();
            
            for (_address, balance) in wallet_stats.iter() {
                for token in balance.token_balances.values() {
                    *token_counts.entry(token.symbol.clone()).or_insert(0) += 1;
                    *token_values.entry(token.symbol.clone()).or_insert(0.0) += token.value_usd;
                }
            }
            
            if !token_counts.is_empty() {
                println!("  Most common tokens:");
                let mut sorted_tokens: Vec<_> = token_counts.iter().collect();
                sorted_tokens.sort_by(|a, b| b.1.cmp(a.1));
                
                for (token, count) in sorted_tokens.iter().take(5) {
                    let total_value = token_values.get(*token).unwrap_or(&0.0);
                    println!("    {} - held by {} wallet(s), total value: ${:.2}", 
                        token, count, total_value);
                }
            } else {
                println!("  No tokens found in wallets");
            }
            
            // Find richest wallet
            if let Some((richest_addr, richest_balance)) = wallet_stats.iter()
                .max_by(|a, b| a.1.total_value_usd.partial_cmp(&b.1.total_value_usd).unwrap()) {
                println!("\n💰 Richest wallet:");
                println!("  Address: {}", richest_addr);
                println!("  SOL: {:.6}", richest_balance.sol_balance);
                println!("  Total Value: ${:.2}", richest_balance.total_value_usd);
                println!("  Token Count: {}", richest_balance.token_balances.len());
            }
            
            // Performance metrics
            println!("\n⚡ Performance Metrics:");
            println!("  Wallets queried: {}", wallet_addresses.len());
            println!("  Wallets with balance: {}", 
                wallet_stats.iter().filter(|(_, b)| b.total_value_usd > 0.0).count());
            println!("  Average wallet value: ${:.2}", 
                if !wallet_stats.is_empty() { total_usd / wallet_stats.len() as f64 } else { 0.0 });
            
            // Batch efficiency
            println!("\n📦 Batch Efficiency:");
            println!("  Single queries would require: {} API calls", wallet_addresses.len());
            println!("  Batch query used: 1 API call");
            println!("  Efficiency gain: {}x", wallet_addresses.len());
            
        }
        Err(e) => {
            println!("✗ Batch query failed: {}", e);
            println!("\nTrying individual queries as fallback...");
            
            // Fallback to individual queries
            for address in &wallet_addresses {
                match portfolio_client.get_balance(address).await {
                    Ok(balance) => {
                        println!("  {} => {:.6} SOL (${:.2})", 
                            &address[..8],
                            balance.sol_balance,
                            balance.total_value_usd
                        );
                    }
                    Err(e) => {
                        println!("  {} => Error: {}", &address[..8], e);
                    }
                }
            }
        }
    }

    println!("\n✅ Batch balance query example completed!");
}