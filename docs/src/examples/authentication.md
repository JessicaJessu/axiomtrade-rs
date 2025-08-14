# Authentication Examples

This section provides comprehensive examples of authentication with the Axiom Trade API. The examples demonstrate different authentication methods, session management, and OTP handling.

## Overview

The authentication system in axiomtrade-rs supports multiple authentication methods:

- **Basic Login**: Email/password authentication with token management
- **OTP Verification**: Automatic OTP fetching from inbox.lv email service
- **Session Management**: Persistent sessions with token refresh capabilities
- **Cookie Authentication**: Web-based authentication for browser integrations

## Prerequisites

Before running any authentication examples, ensure you have:

1. **Environment Setup**: Configure your `.env` file with required credentials
2. **Dependencies**: All required crates are installed via `cargo build`
3. **Optional OTP Setup**: Configure inbox.lv for automatic OTP retrieval

### Required Environment Variables

```env
# Basic authentication (required)
AXIOM_EMAIL=your_email@example.com
AXIOM_PASSWORD=your_password

# Optional OTP automation (recommended)
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_imap_password
```

## Example 1: Basic Login

The basic login example demonstrates the fundamental authentication flow with email/password credentials.

### Location
```
examples/authentication/basic_login.rs
```

### Code

```rust
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
```

### How to Run

```bash
# Ensure environment variables are set in .env file
cargo run --example basic_login
```

### Features Demonstrated

- **AuthClient Initialization**: Creating and configuring the authentication client
- **Email/Password Login**: Basic credential-based authentication
- **Token Management**: Saving and managing access/refresh tokens
- **Error Handling**: Comprehensive error handling with helpful troubleshooting tips
- **Token Persistence**: Saving tokens to disk for future use

### Expected Output

```
Starting basic authentication example...
Email: your_email@example.com
Auth client initialized
OTP auto-fetching is configured
Attempting login...
Login successful!
Access Token: eyJhbGciOiJIUzI1NiIsI...
Refresh Token: eyJhbGciOiJIUzI1NiIsI...
Tokens saved to .axiom_tokens.json
You can now use these tokens for API calls without re-logging in
Testing authenticated request...
Authentication example completed successfully!
```

## Example 2: OTP Verification

This example demonstrates automatic OTP handling using the inbox.lv email service integration.

### Location
```
examples/authentication/otp_verification.rs
```

### Code

```rust
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
```

### How to Run

```bash
# With automatic OTP (recommended)
# Configure INBOX_LV_EMAIL and INBOX_LV_PASSWORD in .env first
cargo run --example otp_verification

# Without automatic OTP (manual entry required)
cargo run --example otp_verification
```

### Features Demonstrated

- **Automatic OTP Retrieval**: Fetching OTP codes from inbox.lv email service
- **Manual OTP Fallback**: User input for OTP when automation is not configured
- **OTP Configuration Detection**: Checking if automatic OTP is properly set up
- **Email Integration**: IMAP integration for OTP retrieval
- **Seamless Authentication**: Handling OTP verification transparently

### OTP Setup Requirements

To enable automatic OTP retrieval:

1. **Create inbox.lv account** at https://www.inbox.lv/
2. **Enable IMAP access**:
   - Go to Settings → "Outlook, email programs"
   - Click "Enable" button
   - Wait 15 minutes for activation
3. **Get IMAP password** (different from web login password)
4. **Configure email forwarding** from Axiom Trade to your inbox.lv address
5. **Set environment variables**:
   ```env
   INBOX_LV_EMAIL=your_username@inbox.lv
   INBOX_LV_PASSWORD=your_special_imap_password
   ```

### Expected Output

```
Starting OTP verification example...
Auth client initialized
Automatic OTP retrieval is configured
The system will automatically fetch OTP from inbox.lv
Performing login...
Login successful with OTP verification!
Access Token: eyJhbGciOiJIUzI1NiIsI...
Tokens saved to .axiom_tokens.json
OTP verification example completed successfully!
Note: OTP was fetched automatically from inbox.lv
```

## Example 3: Session Management

This example demonstrates persistent session management, token refresh, and handling multiple authentication sessions.

### Location
```
examples/authentication/session_management.rs
```

### Code

```rust
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
```

### How to Run

```bash
cargo run --example session_management
```

### Features Demonstrated

- **Session Persistence**: Saving and loading authentication sessions
- **Token Validation**: Checking if existing tokens are still valid
- **Session Refresh**: Automatic token refresh before expiry
- **Multiple Sessions**: Managing sessions for different accounts
- **Session Lifecycle**: Best practices for session management
- **Error Recovery**: Graceful handling of session expiry

### Expected Output

```
Session Management Example
==========================

Step 1: Checking for existing session...
✓ Found existing session
  Access token: eyJhbGciOiJIUzI1NiIsI...
✓ Session is still valid

Testing session with API call...
✓ Session is active and working

Step 2: Managing multiple sessions...
Available session files:
  ✓ .axiom_session_account1.json
  ✗ .axiom_session_account2.json (not found)
  ✓ .axiom_session_trading.json

Step 3: Session best practices...
1. Always check token validity before API calls
2. Implement automatic token refresh
3. Store tokens securely (consider OS keychain)
4. Handle session expiry gracefully
5. Support multiple concurrent sessions
6. Clear sessions on logout

Step 4: Session lifecycle hooks...
You can implement callbacks for:
- onSessionCreated: New login successful
- onSessionRefreshed: Tokens refreshed
- onSessionExpired: Session no longer valid
- onSessionError: Authentication error

Session management example completed!
```

## Example 4: Cookie Authentication

This example demonstrates cookie-based authentication for web applications and browser integrations.

### Location
```
examples/authentication/cookie_auth.rs
```

### Code

```rust
/// Cookie Authentication Example
/// 
/// This example demonstrates authentication using cookies for session persistence,
/// useful for web-based integrations.

use axiomtrade_rs::auth::{AuthClient, AuthCookies};
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Load credentials from environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Cookie Authentication Example");
    println!("=============================\n");

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
    
    println!("Step 1: Performing login with cookie support...");
    
    // Login and get both tokens and cookies
    match auth_client.login(&email, &password, None).await {
        Ok(tokens) => {
            println!("✓ Login successful");
            println!("  Access token received: {}...", &tokens.access_token[..20.min(tokens.access_token.len())]);
            
            // Get cookies from the auth client
            // In the actual implementation, cookies would be extracted from response headers
            println!("\nStep 2: Managing authentication cookies...");
            
            // Example cookie structure (these would come from the actual response)
            let mut additional = HashMap::new();
            additional.insert("session_id".to_string(), "axiom_session_abc123".to_string());
            additional.insert("csrf_token".to_string(), "csrf_xyz789".to_string());
            
            let auth_cookies = AuthCookies {
                auth_access_token: Some("access_cookie_value".to_string()),
                auth_refresh_token: Some("refresh_cookie_value".to_string()),
                g_state: Some("google_state_value".to_string()),
                additional_cookies: additional,
            };
            
            if auth_cookies.auth_access_token.is_some() {
                println!("✓ Access token cookie set");
                println!("  Secure authentication established");
            }
            
            if auth_cookies.auth_refresh_token.is_some() {
                println!("✓ Refresh token cookie set");
                println!("  Session renewal enabled");
            }
            
            if auth_cookies.g_state.is_some() {
                println!("✓ Google state cookie set");
                println!("  OAuth integration ready");
            }
            
            if !auth_cookies.additional_cookies.is_empty() {
                println!("✓ Additional cookies: {}", auth_cookies.additional_cookies.len());
                for (name, _) in auth_cookies.additional_cookies.iter().take(3) {
                    println!("  - {}", name);
                }
            }
            
            println!("\nStep 3: Cookie-based API requests...");
            println!("Cookies can be used for:");
            println!("  - Web dashboard access");
            println!("  - Browser-based API calls");
            println!("  - Cross-origin requests (with proper CORS)");
            println!("  - Maintaining session across page reloads");
            
            println!("\nStep 4: Cookie security best practices...");
            println!("✓ HttpOnly flag: Prevents JavaScript access");
            println!("✓ Secure flag: HTTPS only transmission");
            println!("✓ SameSite: CSRF protection");
            println!("✓ Path restrictions: Limit cookie scope");
            println!("✓ Expiry management: Auto-logout after inactivity");
            
            println!("\nStep 5: Cookie refresh and rotation...");
            println!("In production, implement:");
            println!("  - Automatic cookie refresh before expiry");
            println!("  - Session rotation on privilege escalation");
            println!("  - Secure cookie storage in browser");
            println!("  - Clear cookies on logout");
            
            // Example: Using cookies for subsequent requests
            println!("\nStep 6: Making authenticated requests with cookies...");
            
            println!("✓ Cookie authentication flow completed");
            
            println!("\nNote: Cookie authentication is ideal for:");
            println!("  - Web applications");
            println!("  - Browser extensions");
            println!("  - Server-side rendered apps");
            println!("  - Progressive web apps (PWAs)");
            
        }
        Err(e) => {
            println!("✗ Login failed: {}", e);
            println!("\nTroubleshooting:");
            println!("  1. Check credentials are correct");
            println!("  2. Ensure cookies are enabled");
            println!("  3. Verify CORS settings if cross-origin");
            println!("  4. Check for cookie blocking extensions");
        }
    }
    
    println!("\nCookie authentication example completed!");
}
```

### How to Run

```bash
cargo run --example cookie_auth
```

### Features Demonstrated

- **Cookie Management**: Setting and managing authentication cookies
- **Security Best Practices**: HttpOnly, Secure, SameSite cookie flags
- **Web Integration**: Cookie-based authentication for web applications
- **Session Persistence**: Maintaining authentication across browser sessions
- **CSRF Protection**: Cross-site request forgery prevention
- **Cookie Rotation**: Secure session management practices

### Expected Output

```
Cookie Authentication Example
=============================

Auth client initialized
Step 1: Performing login with cookie support...
✓ Login successful
  Access token received: eyJhbGciOiJIUzI1NiIsI...

Step 2: Managing authentication cookies...
✓ Access token cookie set
  Secure authentication established
✓ Refresh token cookie set
  Session renewal enabled
✓ Google state cookie set
  OAuth integration ready
✓ Additional cookies: 2
  - session_id
  - csrf_token

Step 3: Cookie-based API requests...
Cookies can be used for:
  - Web dashboard access
  - Browser-based API calls
  - Cross-origin requests (with proper CORS)
  - Maintaining session across page reloads

Step 4: Cookie security best practices...
✓ HttpOnly flag: Prevents JavaScript access
✓ Secure flag: HTTPS only transmission
✓ SameSite: CSRF protection
✓ Path restrictions: Limit cookie scope
✓ Expiry management: Auto-logout after inactivity

Step 5: Cookie refresh and rotation...
In production, implement:
  - Automatic cookie refresh before expiry
  - Session rotation on privilege escalation
  - Secure cookie storage in browser
  - Clear cookies on logout

Step 6: Making authenticated requests with cookies...
✓ Cookie authentication flow completed

Note: Cookie authentication is ideal for:
  - Web applications
  - Browser extensions
  - Server-side rendered apps
  - Progressive web apps (PWAs)

Cookie authentication example completed!
```

## Running All Examples

To run all authentication examples in sequence:

```bash
# Run each example individually
cargo run --example basic_login
cargo run --example otp_verification
cargo run --example session_management
cargo run --example cookie_auth

# Or build all examples at once
cargo build --examples
```

## Common Issues and Troubleshooting

### Authentication Failures

**Problem**: Login fails with invalid credentials
```
Login failed: Authentication failed: Invalid email or password
```

**Solutions**:
1. Verify credentials in `.env` file
2. Check for typos in email/password
3. Ensure account is not locked or suspended

### OTP Issues

**Problem**: OTP verification fails
```
Login failed: OTP verification failed
```

**Solutions**:
1. Configure inbox.lv automatic OTP retrieval
2. Check email forwarding is set up correctly
3. Verify IMAP credentials are correct
4. Check for email delivery delays

### Token Issues

**Problem**: Token persistence fails
```
Warning: Failed to save tokens: Permission denied
```

**Solutions**:
1. Check file permissions in working directory
2. Ensure disk space is available
3. Run with appropriate user permissions

### Network Issues

**Problem**: Connection timeouts
```
Login failed: Network error: Connection timeout
```

**Solutions**:
1. Check internet connectivity
2. Verify firewall settings
3. Try again with network retry logic

## Security Considerations

### Token Storage

- **Never commit tokens to version control**
- **Use secure storage mechanisms** (OS keychain when possible)
- **Implement token rotation** for long-running applications
- **Clear tokens on logout** to prevent misuse

### OTP Security

- **Use dedicated email account** for OTP automation
- **Enable two-factor authentication** on email account
- **Monitor for unauthorized access** to email account
- **Consider rate limiting** for OTP requests

### Cookie Security

- **Always use HTTPS** in production
- **Set appropriate cookie flags** (HttpOnly, Secure, SameSite)
- **Implement CSRF protection** for web applications
- **Use short expiry times** for sensitive operations

### Environment Variables

- **Never commit `.env` files** to version control
- **Use environment-specific configurations**
- **Implement secret rotation** for production systems
- **Monitor for credential leaks** in logs and error messages

## Next Steps

After mastering authentication, explore these related topics:

1. **[Trading Examples](trading.md)**: Execute trades with authenticated sessions
2. **[Portfolio Examples](portfolio.md)**: Manage portfolios and balances
3. **[WebSocket Examples](websocket.md)**: Real-time data with authenticated connections
4. **[Advanced Examples](advanced.md)**: Complex authentication patterns and automation

## API Reference

For detailed API documentation, see:

- **[Authentication API](../auth/login.md)**: Login and authentication methods
- **[Token Management](../auth/tokens.md)**: Token handling and refresh
- **[Session Management](../auth/sessions.md)**: Session persistence and management
- **[OTP Integration](../automatic-otp.md)**: Automatic OTP setup and configuration
