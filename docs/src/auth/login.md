# Login and Sessions

This guide covers the complete authentication and session management system in axiomtrade-rs, including basic login, OTP verification, cookie-based authentication, and comprehensive error handling.

## Overview

The authentication system in axiomtrade-rs provides a robust, secure login mechanism with automatic OTP fetching, cookie-based session persistence, and comprehensive token management. The system is designed to handle the two-step authentication process required by Axiom Trade's API.

## Basic Login Flow

The standard login process involves two steps:

1. **Password Verification**: Submit email and hashed password to get an OTP JWT token
2. **OTP Verification**: Submit the OTP code to complete authentication and receive access tokens

### Simple Login Example

```rust
use axiomtrade_rs::auth::AuthClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")?;
    let password = env::var("AXIOM_PASSWORD")?;

    // Create auth client
    let mut auth_client = AuthClient::new()?;
    
    // Perform login (automatic OTP if configured)
    let tokens = auth_client.login(&email, &password, None).await?;
    
    println!("Login successful!");
    println!("Access token: {}...", &tokens.access_token[..20]);
    
    Ok(())
}
```

### Manual OTP Entry

If automatic OTP fetching is not configured, you can provide the OTP manually:

```rust
use std::io::{self, Write};

fn get_otp_from_user() -> Result<String, io::Error> {
    print!("Enter OTP code: ");
    io::stdout().flush()?;
    
    let mut otp = String::new();
    io::stdin().read_line(&mut otp)?;
    
    Ok(otp.trim().to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()?;
    
    // Get OTP from user if auto-fetch not configured
    let otp_code = if env::var("INBOX_LV_EMAIL").is_ok() {
        None  // Auto-fetch enabled
    } else {
        Some(get_otp_from_user()?)
    };
    
    let tokens = auth_client.login(&email, &password, otp_code).await?;
    
    Ok(())
}
```

## Login with OTP

The system supports automatic OTP fetching from inbox.lv email accounts when properly configured.

### Automatic OTP Configuration

Set up environment variables for automatic OTP fetching:

```env
# Axiom Trade credentials
AXIOM_EMAIL=your.email@example.com
AXIOM_PASSWORD=your_password

# Inbox.lv credentials for OTP auto-fetching
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_imap_password
```

### OTP Verification Example

```rust
use axiomtrade_rs::auth::AuthClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()?;
    
    // Check if auto-OTP is configured
    let auto_otp_configured = env::var("INBOX_LV_EMAIL").is_ok() 
        && env::var("INBOX_LV_PASSWORD").is_ok();
    
    if auto_otp_configured {
        println!("Automatic OTP retrieval is configured");
        println!("The system will automatically fetch OTP from inbox.lv");
    } else {
        println!("Manual OTP entry will be required");
    }
    
    // Login with automatic OTP handling
    let tokens = auth_client.login(&email, &password, None).await?;
    
    println!("Login successful with OTP verification!");
    
    Ok(())
}
```

### Full Login Result

Use `login_full()` to get complete authentication information including Turnkey credentials:

```rust
let login_result = auth_client.login_full(&email, &password, None).await?;

// Access tokens
let tokens = login_result.tokens;

// Turnkey credentials for wallet operations
if let Some(turnkey) = login_result.turnkey_credentials {
    println!("Turnkey Organization ID: {}", turnkey.organization_id);
    println!("Turnkey User ID: {}", turnkey.user_id);
    println!("Client Secret: {}...", &turnkey.client_secret[..8]);
}

// User information
if let Some(user) = login_result.user_info {
    println!("User ID: {:?}", user.id);
    println!("Email: {:?}", user.email);
}
```

## Cookie-Based Authentication

The authentication system automatically manages HTTP cookies for session persistence, which is particularly useful for web-based integrations.

### Cookie Management

```rust
use axiomtrade_rs::auth::{AuthClient, AuthCookies};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()?;
    
    // Login and get tokens
    let tokens = auth_client.login(&email, &password, None).await?;
    
    // Cookies are automatically managed by the HTTP client
    // The auth client maintains cookie store with:
    // - auth-access-token (HttpOnly)
    // - auth-refresh-token (HttpOnly) 
    // - g_state (Google OAuth state)
    // - Additional session cookies
    
    println!("Authentication cookies are automatically managed");
    
    Ok(())
}
```

### Cookie Security Features

The authentication system implements several security best practices for cookies:

- **HttpOnly Flag**: Prevents JavaScript access to authentication cookies
- **Secure Flag**: Ensures cookies are only transmitted over HTTPS
- **SameSite Protection**: Provides CSRF protection
- **Path Restrictions**: Limits cookie scope to appropriate paths
- **Automatic Expiry**: Manages cookie lifecycle and cleanup

### Making Authenticated Requests

The `AuthClient` provides methods for making authenticated API requests with automatic cookie handling:

```rust
use reqwest::Method;
use serde_json::json;

// Make authenticated request with automatic cookie handling
let response = auth_client.make_authenticated_request(
    Method::GET,
    "https://api.axiom.trade/portfolio/balance",
    None
).await?;

// Make authenticated POST request with JSON body
let body = json!({
    "token_address": "So11111111111111111111111111111111111111112",
    "amount": "1000000"
});

let response = auth_client.make_authenticated_request(
    Method::POST,
    "https://api.axiom.trade/trade/buy",
    Some(body)
).await?;
```

## Session Management

The system provides comprehensive session management with automatic token refresh and persistence.

### Token Persistence

```rust
use axiomtrade_rs::auth::{TokenManager, AuthClient};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create token manager with persistent storage
    let tokens_file = PathBuf::from(".axiom_tokens.json");
    let token_manager = TokenManager::new(Some(tokens_file));
    
    // Check for existing session
    if let Some(existing_tokens) = token_manager.get_tokens().await {
        println!("Found existing session");
        
        if !existing_tokens.is_expired() {
            println!("Session is still valid");
            // Use existing tokens
        } else {
            println!("Session expired, need to refresh");
            // Refresh or re-login
        }
    } else {
        println!("No existing session, performing new login");
        
        let mut auth_client = AuthClient::new()?;
        let tokens = auth_client.login(&email, &password, None).await?;
        
        // Save tokens for future use
        token_manager.set_tokens(tokens).await?;
    }
    
    Ok(())
}
```

### Automatic Token Refresh

The authentication client automatically handles token refresh when making API requests:

```rust
// Tokens are automatically refreshed when needed
let mut auth_client = AuthClient::new()?;

// This will automatically refresh tokens if they're expired
let valid_tokens = auth_client.ensure_valid_authentication().await?;

println!("Guaranteed valid tokens: {}...", &valid_tokens.access_token[..20]);
```

### Multiple Session Management

```rust
use axiomtrade_rs::auth::SessionManager;

// Manage multiple sessions for different accounts
let session_manager = SessionManager::new(
    Some(PathBuf::from(".axiom_sessions.json")), 
    true  // Enable encryption
);

// Example session files for different purposes
let session_files = vec![
    ".axiom_session_trading.json",
    ".axiom_session_portfolio.json",
    ".axiom_session_notifications.json",
];

for session_file in session_files {
    let path = PathBuf::from(session_file);
    if path.exists() {
        println!("Found session: {}", session_file);
    }
}
```

## Error Handling

The authentication system provides comprehensive error handling for various failure scenarios.

### Authentication Error Types

```rust
use axiomtrade_rs::auth::error::AuthError;

match auth_client.login(&email, &password, None).await {
    Ok(tokens) => {
        println!("Login successful");
    }
    Err(AuthError::InvalidCredentials) => {
        println!("Invalid email or password");
    }
    Err(AuthError::InvalidOtp) => {
        println!("Invalid OTP code");
    }
    Err(AuthError::OtpRequired) => {
        println!("OTP required but not provided");
    }
    Err(AuthError::TokenExpired) => {
        println!("Authentication token has expired");
    }
    Err(AuthError::TokenNotFound) => {
        println!("No authentication token found");
    }
    Err(AuthError::NetworkError(e)) => {
        println!("Network error: {}", e);
    }
    Err(AuthError::EmailError(msg)) => {
        println!("Email fetcher error: {}", msg);
    }
    Err(e) => {
        println!("Other authentication error: {}", e);
    }
}
```

### Comprehensive Error Handling Example

```rust
use axiomtrade_rs::auth::{AuthClient, AuthError};

async fn robust_login(email: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()
        .map_err(|e| format!("Failed to create auth client: {}", e))?;
    
    let max_retries = 3;
    let mut attempts = 0;
    
    loop {
        attempts += 1;
        
        match auth_client.login(email, password, None).await {
            Ok(tokens) => {
                println!("Login successful on attempt {}", attempts);
                return Ok(());
            }
            Err(AuthError::NetworkError(e)) if attempts < max_retries => {
                println!("Network error on attempt {}, retrying: {}", attempts, e);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                continue;
            }
            Err(AuthError::InvalidCredentials) => {
                return Err("Invalid credentials - check email and password".into());
            }
            Err(AuthError::OtpRequired) => {
                return Err("OTP required but auto-fetching not configured".into());
            }
            Err(AuthError::EmailError(msg)) => {
                return Err(format!("Email OTP fetching failed: {}", msg).into());
            }
            Err(e) => {
                return Err(format!("Login failed: {}", e).into());
            }
        }
    }
}
```

### Recovery Strategies

```rust
async fn handle_authentication_failure(
    auth_client: &mut AuthClient,
    error: AuthError
) -> Result<(), AuthError> {
    match error {
        AuthError::TokenExpired => {
            // Try to refresh tokens
            match auth_client.refresh_tokens().await {
                Ok(_) => {
                    println!("Tokens refreshed successfully");
                    Ok(())
                }
                Err(_) => {
                    println!("Token refresh failed, need to re-login");
                    Err(AuthError::NotAuthenticated)
                }
            }
        }
        AuthError::NetworkError(_) => {
            // Implement exponential backoff
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            Err(error)
        }
        AuthError::OtpRequired => {
            println!("Configure automatic OTP fetching:");
            println!("1. Create inbox.lv account");
            println!("2. Set INBOX_LV_EMAIL and INBOX_LV_PASSWORD");
            println!("3. Forward Axiom OTP emails to inbox.lv");
            Err(error)
        }
        _ => Err(error)
    }
}
```

## Complete Example: Production Login

Here's a complete example demonstrating production-ready authentication with all features:

```rust
use axiomtrade_rs::auth::{AuthClient, TokenManager, AuthError};
use axiomtrade_rs::client::EnhancedClient;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")?;
    let password = env::var("AXIOM_PASSWORD")?;
    
    // Set up persistent token storage
    let tokens_file = PathBuf::from(".axiom_session.json");
    let token_manager = TokenManager::new(Some(tokens_file));
    
    // Check for existing valid session
    if let Some(tokens) = token_manager.get_tokens().await {
        if !tokens.is_expired() {
            println!("Using existing valid session");
            
            // Test the session with an API call
            let client = EnhancedClient::new()?;
            match client.get_portfolio().await {
                Ok(_) => {
                    println!("Session validated successfully");
                    return Ok(());
                }
                Err(_) => {
                    println!("Session invalid, performing fresh login");
                }
            }
        }
    }
    
    // Perform fresh login
    println!("Performing authentication...");
    let mut auth_client = AuthClient::new()?;
    
    // Check OTP configuration
    let auto_otp = env::var("INBOX_LV_EMAIL").is_ok() 
        && env::var("INBOX_LV_PASSWORD").is_ok();
    
    if auto_otp {
        println!("Automatic OTP fetching enabled");
    } else {
        println!("Manual OTP entry may be required");
    }
    
    // Login with comprehensive error handling
    let login_result = match auth_client.login_full(&email, &password, None).await {
        Ok(result) => result,
        Err(AuthError::InvalidCredentials) => {
            return Err("Invalid credentials - check AXIOM_EMAIL and AXIOM_PASSWORD".into());
        }
        Err(AuthError::OtpRequired) if !auto_otp => {
            return Err("OTP required but auto-fetching not configured. Set up inbox.lv integration.".into());
        }
        Err(AuthError::EmailError(msg)) => {
            return Err(format!("OTP email fetching failed: {}", msg).into());
        }
        Err(e) => {
            return Err(format!("Authentication failed: {}", e).into());
        }
    };
    
    println!("Login successful!");
    
    // Save tokens for future sessions
    token_manager.set_tokens(login_result.tokens.clone()).await?;
    println!("Session saved for future use");
    
    // Display authentication details
    if let Some(turnkey) = &login_result.turnkey_credentials {
        println!("Turnkey wallet access enabled:");
        println!("  Organization: {}", turnkey.organization_id);
        println!("  User: {}", turnkey.user_id);
    }
    
    if let Some(user) = &login_result.user_info {
        if let Some(email) = &user.email {
            println!("Authenticated as: {}", email);
        }
    }
    
    // Test authenticated API access
    let client = EnhancedClient::new()?;
    match client.get_portfolio().await {
        Ok(portfolio) => {
            println!("Portfolio access confirmed");
            println!("Ready for trading operations");
        }
        Err(e) => {
            println!("Warning: Portfolio access failed: {}", e);
        }
    }
    
    Ok(())
}
```

## Best Practices

### Security
- Always use environment variables for credentials
- Enable automatic OTP fetching to reduce manual intervention
- Store tokens securely with appropriate file permissions
- Implement proper session cleanup on logout
- Use HTTPS for all API communications

### Performance
- Reuse existing valid sessions when possible
- Implement automatic token refresh to avoid re-authentication
- Cache authentication state appropriately
- Handle network failures with exponential backoff

### Reliability
- Implement comprehensive error handling for all scenarios
- Use multiple API endpoints for redundancy
- Implement proper retry logic for transient failures
- Monitor authentication success rates and failures

### Development
- Use separate credentials for development and production
- Log authentication events for debugging
- Test authentication flows thoroughly
- Document credential setup procedures for team members

This comprehensive authentication system provides a robust foundation for all Axiom Trade API operations, with automatic session management, secure credential handling, and production-ready error handling.
