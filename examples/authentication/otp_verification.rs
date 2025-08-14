/// OTP Verification Example
/// 
/// This example demonstrates OTP verification with automatic email fetching
/// from inbox.lv when properly configured.

use axiomtrade_rs::auth::{AuthClient, TokenManager};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // Load credentials from environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Starting OTP verification example...");

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
    
    // Check if automatic OTP fetching is configured
    let inbox_email = env::var("INBOX_LV_EMAIL").ok();
    let inbox_password = env::var("INBOX_LV_PASSWORD").ok();
    
    let auto_otp_configured = inbox_email.is_some() && inbox_password.is_some();
    
    if auto_otp_configured {
        println!("Automatic OTP retrieval is configured");
        println!("The system will automatically fetch OTP from inbox.lv");
    } else {
        println!("Automatic OTP not configured");
        println!("To enable automatic OTP:");
        println!("1. Create an inbox.lv account");
        println!("2. Enable IMAP access in settings");
        println!("3. Set INBOX_LV_EMAIL and INBOX_LV_PASSWORD in .env");
        println!("4. Forward Axiom OTP emails to your inbox.lv address");
        println!("\nFor now, you'll need to enter OTP manually when prompted.");
    }
    
    println!("\nPerforming login...");
    
    // The login method handles OTP automatically if configured
    match auth_client.login(&email, &password, None).await {
        Ok(tokens) => {
            println!("\nLogin successful with OTP verification!");
            
            println!(
                "Access Token: {}...",
                &tokens.access_token[..20.min(tokens.access_token.len())]
            );
            
            // Save tokens
            let tokens_file = PathBuf::from(".axiom_tokens.json");
            let token_manager = TokenManager::new(Some(tokens_file.clone()));
            
            if let Err(e) = token_manager.set_tokens(tokens).await {
                println!("Warning: Failed to save tokens: {}", e);
            } else {
                println!("\nTokens saved to {}", tokens_file.display());
            }
            
            println!("\nOTP verification example completed successfully!");
            
            if auto_otp_configured {
                println!("\nNote: OTP was fetched automatically from inbox.lv");
            } else {
                println!("\nNote: In production, configure auto-OTP for seamless authentication");
            }
        }
        Err(e) => {
            println!("Login failed: {}", e);
            
            if !auto_otp_configured {
                println!("\nHint: The login may have failed because OTP couldn't be retrieved automatically.");
                println!("Configure inbox.lv integration for automatic OTP handling.");
                println!("See: examples/setup/auto_otp_setup.md");
            }
        }
    }
}

/// Helper function to manually get OTP from user (not used with auto-OTP)
fn get_otp_from_user() -> Result<String, io::Error> {
    print!("Enter OTP code: ");
    io::stdout().flush()?;
    
    let mut otp = String::new();
    io::stdin().read_line(&mut otp)?;
    
    Ok(otp.trim().to_string())
}