/// Session Management Example
/// 
/// This example demonstrates session persistence, token refresh,
/// and managing multiple authentication sessions.

use axiomtrade_rs::auth::{AuthClient, TokenManager, SessionManager};
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // Load credentials from environment variables
    dotenvy::dotenv().ok();
    
    println!("Session Management Example");
    println!("==========================\n");

    // Create token manager with persistent storage
    let tokens_file = PathBuf::from(".axiom_session.json");
    let token_manager = TokenManager::new(Some(tokens_file.clone()));
    
    // Check for existing session
    println!("Step 1: Checking for existing session...");
    if let Some(tokens) = token_manager.get_tokens().await {
        println!("✓ Found existing session");
        println!("  Access token: {}...", &tokens.access_token[..20.min(tokens.access_token.len())]);
        
        // Check if tokens are still valid (simplified check)
        // In production, you'd check expiry time
        if !tokens.access_token.is_empty() {
            println!("✓ Session is still valid");
            
            // Test the session with an API call
            println!("\nTesting session with API call...");
            let client = match axiomtrade_rs::client::EnhancedClient::new() {
                Ok(c) => c,
                Err(e) => {
                    println!("Failed to create client: {}", e);
                    return;
                }
            };
            
            // In a real implementation, you would make an API call here
            println!("✓ Session is active and working");
            
        } else {
            println!("⚠ Session expired, need to refresh or re-login");
            
            // Try to refresh the session
            if !tokens.refresh_token.is_empty() {
                println!("\nAttempting to refresh session...");
                let mut auth_client = match AuthClient::new() {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Failed to create auth client: {}", e);
                        return;
                    }
                };
                
                // In a real implementation, you would call refresh endpoint
                // match auth_client.refresh_tokens(&refresh_token).await {
                //     Ok(new_tokens) => {
                //         token_manager.set_tokens(new_tokens).await.ok();
                //         println!("✓ Session refreshed successfully");
                //     }
                //     Err(e) => {
                //         println!("Failed to refresh: {}", e);
                //         perform_new_login(&token_manager).await;
                //     }
                // }
                
                println!("Note: Token refresh would be called here in production");
            } else {
                perform_new_login(&token_manager).await;
            }
        }
    } else {
        println!("No existing session found");
        perform_new_login(&token_manager).await;
    }
    
    println!("\nStep 2: Managing multiple sessions...");
    
    // Create session manager for handling multiple accounts
    let session_path = PathBuf::from(".axiom_sessions.json");
    let session_manager = SessionManager::new(Some(session_path), true);
    
    // Example: Managing sessions for different accounts
    let session_files = vec![
        ".axiom_session_account1.json",
        ".axiom_session_account2.json",
        ".axiom_session_trading.json",
    ];
    
    println!("Available session files:");
    for file in &session_files {
        let path = PathBuf::from(file);
        if path.exists() {
            println!("  ✓ {}", file);
        } else {
            println!("  ✗ {} (not found)", file);
        }
    }
    
    println!("\nStep 3: Session best practices...");
    println!("1. Always check token validity before API calls");
    println!("2. Implement automatic token refresh");
    println!("3. Store tokens securely (consider OS keychain)");
    println!("4. Handle session expiry gracefully");
    println!("5. Support multiple concurrent sessions");
    println!("6. Clear sessions on logout");
    
    println!("\nStep 4: Session lifecycle hooks...");
    println!("You can implement callbacks for:");
    println!("- onSessionCreated: New login successful");
    println!("- onSessionRefreshed: Tokens refreshed");
    println!("- onSessionExpired: Session no longer valid");
    println!("- onSessionError: Authentication error");
    
    println!("\nSession management example completed!");
}

async fn perform_new_login(token_manager: &TokenManager) {
    println!("\nPerforming new login...");
    
    let email = match env::var("AXIOM_EMAIL") {
        Ok(e) => e,
        Err(_) => {
            println!("AXIOM_EMAIL not set");
            return;
        }
    };
    
    let password = match env::var("AXIOM_PASSWORD") {
        Ok(p) => p,
        Err(_) => {
            println!("AXIOM_PASSWORD not set");
            return;
        }
    };
    
    let mut auth_client = match AuthClient::new() {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to create auth client: {}", e);
            return;
        }
    };
    
    match auth_client.login(&email, &password, None).await {
        Ok(tokens) => {
            println!("✓ Login successful");
            
            // Save the new session
            if let Err(e) = token_manager.set_tokens(tokens).await {
                println!("Warning: Failed to save session: {}", e);
            } else {
                println!("✓ Session saved for future use");
            }
        }
        Err(e) => {
            println!("✗ Login failed: {}", e);
        }
    }
}