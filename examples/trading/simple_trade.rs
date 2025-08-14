/// Simple Trading Example
/// 
/// This example demonstrates basic buy and sell operations using the
/// Axiom Trade API with proper error handling and transaction verification.

use axiomtrade_rs::{AuthClient, Result, AxiomError};
use axiomtrade_rs::api::trading::TradingClient;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials and authenticate
    dotenvy::dotenv().ok();
    let mut trading_client = authenticate().await?;

    println!("Simple trading example");
    println!("IMPORTANT: This is a demonstration using minimal amounts");
    println!("Always verify transaction details before executing trades\n");

    // Example trading parameters
    let token_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"; // USDC mint
    let amount_sol = 0.001; // Very small amount for demonstration
    
    println!("Trading setup:");
    println!("  Token: USDC ({})", token_mint);
    println!("  Amount: {} SOL", amount_sol);

    // Demonstrate buy operation
    println!("\n=== BUY OPERATION ===");
    
    println!("Preparing buy transaction...");
    println!("  Trading {} SOL for USDC", amount_sol);
    println!("  Slippage tolerance: 1%");
    println!("  Priority fee: 0.00001 SOL");

    // In a real application, you would execute the trade here
    // For this example, we'll demonstrate the request structure
    simulate_buy_trade(&mut trading_client, token_mint, amount_sol).await?;

    // Demonstrate sell operation
    println!("\n=== SELL OPERATION ===");
    
    let amount_tokens = 1.0; // 1 USDC
    
    println!("Preparing sell transaction...");
    println!("  Trading {} USDC for SOL", amount_tokens);
    println!("  Slippage tolerance: 1%");
    println!("  Priority fee: 0.00001 SOL");

    simulate_sell_trade(&mut trading_client, token_mint, amount_tokens).await?;

    // Demonstrate price quote
    println!("\n=== PRICE CHECKING ===");
    
    let sol_mint = "So11111111111111111111111111111111111111112"; // Native SOL
    
    match trading_client.get_quote(sol_mint, token_mint, amount_sol, Some(1.0)).await {
        Ok(quote) => {
            println!("Current USDC swap quote:");
            println!("  Input: {} SOL", quote.in_amount);
            println!("  Output: {} USDC", quote.out_amount);
            println!("  Price impact: {:.2}%", quote.price_impact);
            println!("  Fee: {} SOL", quote.fee);
        }
        Err(e) => {
            println!("Failed to get quote: {}", e);
        }
    }

    // Demonstrate slippage impact
    println!("\n=== SLIPPAGE ANALYSIS ===");
    
    let slippage_scenarios = vec![0.1, 0.5, 1.0, 2.0, 5.0];
    
    for slippage in slippage_scenarios {
        // Calculate impact (this would normally come from price estimation API)
        let estimated_impact = calculate_slippage_impact(amount_sol, slippage);
        println!("  {}% slippage tolerance - Max cost: {} SOL", 
            slippage, estimated_impact);
    }

    // Best practices demonstration
    println!("\n=== TRADING BEST PRACTICES ===");
    println!("1. Always check token liquidity before large trades");
    println!("2. Use appropriate slippage tolerance (0.1-1% for liquid tokens)");
    println!("3. Monitor priority fees during network congestion");
    println!("4. Verify transaction signatures after execution");
    println!("5. Keep track of all transactions for portfolio management");

    // Security reminders
    println!("\n=== SECURITY REMINDERS ===");
    println!("1. Never share private keys or seed phrases");
    println!("2. Always verify transaction details before signing");
    println!("3. Use hardware wallets for large amounts");
    println!("4. Be aware of MEV (Maximum Extractable Value) attacks");
    println!("5. Consider using MEV protection services");

    println!("\nSimple trading example completed");
    Ok(())
}

async fn simulate_buy_trade(
    client: &mut TradingClient, 
    token_mint: &str,
    amount_sol: f64,
) -> Result<()> {
    println!("Simulating buy trade execution...");
    
    // Step 1: Validate parameters
    println!("  ✓ Validating trade parameters");
    validate_amount(amount_sol, "SOL")?;
    validate_token_mint(token_mint)?;
    
    // Step 2: Get trading limits
    println!("  ✓ Checking trading limits");
    match client.get_trading_limits().await {
        Ok(limits) => {
            if amount_sol < limits.min_sol_amount {
                return Err(AxiomError::Api {
                    message: format!("Amount {} SOL is below minimum {}", amount_sol, limits.min_sol_amount)
                });
            }
            if amount_sol > limits.max_sol_amount {
                return Err(AxiomError::Api {
                    message: format!("Amount {} SOL exceeds maximum {}", amount_sol, limits.max_sol_amount)
                });
            }
        }
        Err(_) => println!("  ! Could not verify trading limits"),
    }
    
    // Step 3: Get price quote
    println!("  ✓ Getting price quote");
    let sol_mint = "So11111111111111111111111111111111111111112";
    match client.get_quote(sol_mint, token_mint, amount_sol, Some(1.0)).await {
        Ok(quote) => {
            println!("    Expected output: {} tokens", quote.out_amount);
            println!("    Price impact: {:.2}%", quote.price_impact);
        }
        Err(_) => println!("  ! Could not get quote for simulation"),
    }
    
    // Step 4: Simulate the buy (would call buy_token in real scenario)
    println!("  ✓ Ready for buy transaction");
    println!("    Would execute: buy_token({}, {}, Some(1.0))", token_mint, amount_sol);
    
    println!("Buy trade simulation completed successfully");
    Ok(())
}

async fn simulate_sell_trade(
    client: &mut TradingClient,
    token_mint: &str, 
    amount_tokens: f64,
) -> Result<()> {
    println!("Simulating sell trade execution...");
    
    // Step 1: Validate parameters
    println!("  ✓ Validating trade parameters");
    validate_amount(amount_tokens, "tokens")?;
    validate_token_mint(token_mint)?;
    
    // Step 2: Get price quote
    println!("  ✓ Getting price quote");
    let sol_mint = "So11111111111111111111111111111111111111112";
    match client.get_quote(token_mint, sol_mint, amount_tokens, Some(1.0)).await {
        Ok(quote) => {
            println!("    Expected output: {} SOL", quote.out_amount);
            println!("    Price impact: {:.2}%", quote.price_impact);
        }
        Err(_) => println!("  ! Could not get quote for simulation"),
    }
    
    // Step 3: Simulate the sell (would call sell_token in real scenario)
    println!("  ✓ Ready for sell transaction");
    println!("    Would execute: sell_token({}, {}, Some(1.0))", token_mint, amount_tokens);
    
    println!("Sell trade simulation completed successfully");
    Ok(())
}

fn validate_amount(amount: f64, unit: &str) -> Result<()> {
    if amount <= 0.0 {
        return Err(AxiomError::Api {
            message: format!("Amount must be greater than 0, got {} {}", amount, unit),
        });
    }
    
    if amount.is_nan() || amount.is_infinite() {
        return Err(AxiomError::Api {
            message: format!("Invalid amount: {} {}", amount, unit),
        });
    }
    
    Ok(())
}

fn validate_token_mint(mint: &str) -> Result<()> {
    if mint.is_empty() {
        return Err(AxiomError::Api {
            message: "Token mint cannot be empty".to_string(),
        });
    }
    
    if mint.len() < 32 || mint.len() > 44 {
        return Err(AxiomError::Api {
            message: format!("Invalid mint address length: {}", mint),
        });
    }
    
    if !mint.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AxiomError::Api {
            message: format!("Invalid characters in mint address: {}", mint),
        });
    }
    
    Ok(())
}

fn calculate_slippage_impact(amount: f64, slippage: f64) -> f64 {
    // Simple calculation - in reality this would be much more complex
    amount * (1.0 + slippage / 100.0)
}

async fn authenticate() -> Result<TradingClient> {
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    // First authenticate with AuthClient
    let mut auth_client = AuthClient::new()?;
    
    match auth_client.login_full(&email, &password, None).await {
        Ok(login_result) => {
            println!("Authentication successful!");
            // Store the tokens for potential use
            println!("Access token obtained: {}", &login_result.tokens.access_token[..20]);
        }
        Err(e) => {
            return Err(AxiomError::Auth(e));
        }
    }

    // Create trading client (it will use the authenticated session)
    TradingClient::new().map_err(|e| AxiomError::Api {
        message: format!("Failed to create trading client: {}", e),
    })
}