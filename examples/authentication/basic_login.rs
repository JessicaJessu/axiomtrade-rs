/// Basic Authentication Example
/// 
/// This example demonstrates the fundamental authentication flow with Axiom Trade,
/// including email/password login and token management.

use axiomtrade_rs::auth::{AuthClient, TokenManager};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // Load credentials from environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Starting basic authentication example...");
    println!("Email: {}", email);

    // Create a new auth client
    let mut auth_client = match AuthClient::new() {
        Ok(client) => {
            println!("Auth client initialized");
            client
        }
        Err(e) => {
            println!("Failed to create auth client: {}", e);
            return;
        }
    };
    
    // Check if OTP auto-fetching is configured
    let inbox_configured = env::var("INBOX_LV_EMAIL").is_ok()
        && env::var("INBOX_LV_PASSWORD").is_ok();
    
    if inbox_configured {
        println!("OTP auto-fetching is configured");
    } else {
        println!("OTP auto-fetching not configured - will require manual entry");
    }
    
    // Perform login
    println!("\nAttempting login...");
    match auth_client.login(&email, &password, None).await {
        Ok(tokens) => {
            println!("Login successful!");
            
            println!(
                "Access Token: {}...",
                &tokens.access_token[..20.min(tokens.access_token.len())]
            );
            println!(
                "Refresh Token: {}...",
                &tokens.refresh_token[..20.min(tokens.refresh_token.len())]
            );
            
            // Save tokens for future use
            let tokens_file = PathBuf::from(".axiom_tokens.json");
            let token_manager = TokenManager::new(Some(tokens_file.clone()));
            
            if let Err(e) = token_manager.set_tokens(tokens).await {
                println!("Warning: Failed to save tokens: {}", e);
            } else {
                println!("\nTokens saved to {}", tokens_file.display());
                println!("You can now use these tokens for API calls without re-logging in");
            }
            
            // Test authenticated request using EnhancedClient
            println!("\nTesting authenticated request...");
            let enhanced_client = match axiomtrade_rs::client::EnhancedClient::new() {
                Ok(client) => client,
                Err(e) => {
                    println!("Failed to create enhanced client: {}", e);
                    return;
                }
            };
            
            // Example: Get portfolio would go here
            // match enhanced_client.get_portfolio().await {
            //     Ok(portfolio) => {
            //         println!("Portfolio access successful!");
            //     }
            //     Err(e) => {
            //         println!("Failed to get portfolio: {}", e);
            //     }
            // }
            
            println!("\nAuthentication example completed successfully!");
        }
        Err(e) => {
            println!("Login failed: {}", e);
            println!("\nPossible reasons:");
            println!("1. Invalid credentials");
            println!("2. Network connectivity issues");
            println!("3. API service unavailable");
            
            if !inbox_configured {
                println!("4. OTP required but auto-fetching not configured");
                println!("   Run: cargo run --example setup_env");
                println!("   Or see: examples/setup/auto_otp_setup.md");
            }
        }
    }
}