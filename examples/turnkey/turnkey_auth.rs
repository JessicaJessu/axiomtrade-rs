/// Turnkey Authentication Example
/// 
/// This example demonstrates secure authentication with Turnkey using
/// P256 cryptographic signatures and enterprise wallet management.

use axiomtrade_rs::api::turnkey::TurnkeyClient;
use axiomtrade_rs::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from environment variables
    dotenvy::dotenv().ok();
    
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Turnkey Authentication Example");
    println!("Demonstrating enterprise-grade wallet security with P256 signatures\n");

    // Create Turnkey client
    let mut turnkey_client = TurnkeyClient::new();
    
    println!("Step 1: Loading session data...");
    
    // Load existing Turnkey session (this would be created during initial authentication)
    let session_file = ".axiom_turnkey_session.json";
    
    match std::fs::read_to_string(session_file) {
        Ok(session_content) => {
            println!("✓ Session file found");
            
            let session: axiomtrade_rs::auth::types::AuthSession = 
                serde_json::from_str(&session_content)?;
            
            if let Some(turnkey_session) = session.turnkey_session {
                println!("✓ Turnkey session loaded");
                println!("  Organization ID: {}", turnkey_session.organization_id);
                println!("  User ID: {}", turnkey_session.user_id);
                println!("  Username: {}", turnkey_session.username);
                
                // Set up Turnkey client credentials
                // Note: In production, the password should be the raw password, not hashed
                turnkey_client.set_credentials(
                    &turnkey_session.organization_id,
                    &turnkey_session.user_id,
                    &password  // Raw password for Turnkey P256 key derivation
                );
                
                println!("\nStep 2: Testing Turnkey authentication...");
                
                // Test whoami request
                match turnkey_client.whoami(
                    &turnkey_session.organization_id,
                    &turnkey_session.client_secret
                ).await {
                    Ok(whoami) => {
                        println!("🎉 Turnkey authentication successful!");
                        println!("  Organization: {}", whoami.organization_id);
                        println!("  User ID: {}", whoami.user_id);
                        println!("  Username: {}", whoami.username);
                    }
                    Err(e) => {
                        println!("❌ Turnkey authentication failed: {}", e);
                        return handle_auth_failure(e);
                    }
                }
                
                println!("\nStep 3: Testing API key management...");
                
                // Get API keys
                match turnkey_client.get_api_keys(
                    &turnkey_session.user_id,
                    &turnkey_session.organization_id,
                    &turnkey_session.client_secret
                ).await {
                    Ok(api_keys) => {
                        println!("✓ API keys retrieved successfully");
                        println!("  Total API keys: {}", api_keys.api_keys.len());
                        
                        if api_keys.api_keys.is_empty() {
                            println!("  No API keys configured (this is normal for new accounts)");
                        } else {
                            for (i, key) in api_keys.api_keys.iter().enumerate() {
                                println!("  Key {}: {}", i + 1, key.api_key_name);
                                println!("    ID: {}", key.api_key_id);
                                println!("    Public Key: {}", key.credential.public_key);
                                println!("    Type: {}", key.credential.credential_type);
                                println!("    Created: {}", key.created_at.seconds);
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ API key retrieval failed: {}", e);
                    }
                }
                
                println!("\nStep 4: Demonstrating P256 cryptographic operations...");
                
                // Demonstrate key generation and signing
                demonstrate_p256_operations(&password, &turnkey_session.client_secret)?;
                
                println!("\nStep 5: Session management...");
                
                // Parse and display session information
                let whoami = turnkey_client.whoami(
                    &turnkey_session.organization_id,
                    &turnkey_session.client_secret
                ).await?;
                
                let api_keys = turnkey_client.get_api_keys(
                    &turnkey_session.user_id,
                    &turnkey_session.organization_id,
                    &turnkey_session.client_secret
                ).await?;
                
                let parsed_session = turnkey_client.parse_session(
                    &whoami,
                    &api_keys,
                    &turnkey_session.client_secret
                );
                
                println!("Session summary:");
                println!("{}", turnkey_client.session_summary(&parsed_session));
                
                println!("\n✅ Turnkey integration demonstration completed successfully!");
                println!("\nKey features demonstrated:");
                println!("- P256 cryptographic authentication");
                println!("- Secure session management");
                println!("- API key retrieval and management");
                println!("- Enterprise-grade wallet security");
                
            } else {
                println!("❌ No Turnkey session found in session file");
                println!("Please run the authentication flow first to create a Turnkey session");
            }
        }
        Err(_) => {
            println!("❌ No session file found: {}", session_file);
            println!("Please run the authentication flow first to create a session");
            println!("This can be done by running the basic authentication examples");
        }
    }

    Ok(())
}

fn demonstrate_p256_operations(password: &str, client_secret: &str) -> Result<()> {
    println!("Demonstrating P256 cryptographic operations...");
    
    // Generate keypair from password and client secret
    let keypair = axiomtrade_rs::utils::p256_crypto::recreate_keypair_from_client_secret(
        password, 
        client_secret
    )?;
    
    println!("✓ P256 keypair generated");
    println!("  Public key: {}", keypair.public_key);
    println!("  Private key length: {} characters", keypair.private_key.len());
    
    // Demonstrate message signing
    let test_message = b"Hello, Turnkey!";
    
    let signature = axiomtrade_rs::utils::p256_crypto::sign_message(
        test_message,
        &keypair.private_key
    )?;
    
    println!("✓ Message signed with P256");
    println!("  Message: {:?}", std::str::from_utf8(test_message).unwrap());
    println!("  Signature length: {} bytes", signature.len());
    println!("  Signature format: DER (starts with 0x30)");
    
    // Verify signature format
    if signature.len() > 0 && signature[0] == 0x30 {
        println!("✓ Signature is in correct DER format");
    } else {
        println!("⚠️  Signature format may be incorrect");
    }
    
    // Show hex representation
    println!("  Signature (hex): {}", hex::encode(&signature[..16.min(signature.len())]));
    
    Ok(())
}

fn handle_auth_failure(error: axiomtrade_rs::AxiomError) -> Result<()> {
    println!("\nTroubleshooting Turnkey authentication failure:");
    
    match error {
        axiomtrade_rs::AxiomError::Api { message, .. } if message.contains("PUBLIC_KEY_NOT_FOUND") => {
            println!("Issue: Public key not registered with Turnkey");
            println!("Solution: Ensure the password generates the correct public key");
            println!("The public key must be registered in the Turnkey organization");
        }
        axiomtrade_rs::AxiomError::Api { message, .. } if message.contains("unauthorized") => {
            println!("Issue: Authentication credentials are invalid");
            println!("Solution: Check that the password and client secret are correct");
        }
        axiomtrade_rs::AxiomError::Network { .. } => {
            println!("Issue: Network connectivity problem");
            println!("Solution: Check internet connection and try again");
        }
        _ => {
            println!("Issue: Unknown error occurred");
            println!("Error details: {}", error);
        }
    }
    
    println!("\nCommon solutions:");
    println!("1. Verify AXIOM_PASSWORD in .env file is correct");
    println!("2. Ensure session file contains valid Turnkey data");
    println!("3. Check that public key is registered in Turnkey organization");
    println!("4. Try re-authenticating to refresh session data");
    
    Ok(())
}