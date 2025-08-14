# OTP Verification

The OTP (One-Time Password) verification system in axiomtrade-rs provides both manual and automatic OTP handling for secure authentication with Axiom Trade. This system supports automatic OTP retrieval from email via IMAP, eliminating the need for manual intervention in automated trading systems.

## Overview

The OTP verification flow consists of two main approaches:

1. **Manual OTP Entry** - User manually enters the OTP code when prompted
2. **Automatic OTP Fetching** - System automatically retrieves OTP from email via IMAP

Both approaches integrate seamlessly with the authentication system and handle retries, timeouts, and error recovery.

## Manual OTP Entry

Manual OTP entry is the simplest approach and requires no additional setup beyond basic authentication credentials.

### Basic Usage

```rust
use axiomtrade_rs::auth::AuthClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()?;
    
    // The None parameter triggers manual OTP entry
    let tokens = auth_client.login(
        "your-email@domain.com",
        "your-password",
        None  // Will prompt for manual OTP entry
    ).await?;
    
    println!("Authentication successful!");
    Ok(())
}
```

### Manual OTP Helper Function

You can also provide the OTP code directly if you have it from another source:

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
    
    // Get OTP from user input
    let otp_code = get_otp_from_user()?;
    
    let tokens = auth_client.login(
        "your-email@domain.com",
        "your-password",
        Some(otp_code)
    ).await?;
    
    println!("Authentication successful!");
    Ok(())
}
```

## Automatic OTP Fetching

Automatic OTP fetching eliminates manual intervention by retrieving OTP codes directly from your email via IMAP. This feature is essential for automated trading systems and production applications.

### Prerequisites

Before enabling automatic OTP fetching, you need:

1. An inbox.lv email account with IMAP enabled
2. Axiom Trade configured to send OTP emails to your inbox.lv address
3. Environment variables configured with IMAP credentials

### Environment Configuration

Add these variables to your `.env` file:

```env
# Axiom Trade Credentials
AXIOM_EMAIL=your_axiom_email@domain.com
AXIOM_PASSWORD=your_axiom_password

# inbox.lv IMAP Configuration
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_imap_password
```

### Automatic OTP Usage

When environment variables are configured, the system automatically fetches OTP codes:

```rust
use axiomtrade_rs::auth::AuthClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let mut auth_client = AuthClient::new()?;
    
    // Automatic OTP fetching when None is passed
    let tokens = auth_client.login(
        &std::env::var("AXIOM_EMAIL")?,
        &std::env::var("AXIOM_PASSWORD")?,
        None  // System will automatically fetch OTP from email
    ).await?;
    
    println!("Authentication successful with automatic OTP!");
    Ok(())
}
```

### Advanced OTP Fetching

For more control over the OTP fetching process:

```rust
use axiomtrade_rs::email::otp_fetcher::{OtpFetcher, from_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create OTP fetcher from environment variables
    let otp_fetcher = from_env()?
        .ok_or("OTP fetcher not configured")?;
    
    // Wait for OTP with custom timeout and check interval
    if let Some(otp) = otp_fetcher.wait_for_otp(120, 5)? {
        println!("Retrieved OTP: {}", otp);
        
        // Use the OTP for authentication
        let mut auth_client = AuthClient::new()?;
        let tokens = auth_client.login(
            "your-email@domain.com",
            "your-password",
            Some(otp)
        ).await?;
        
        println!("Authentication successful!");
    } else {
        println!("OTP not received within timeout");
    }
    
    Ok(())
}
```

## OTP Validation Flow

The OTP validation process follows this sequence:

1. **Initial Login Request** - Send credentials to `/login-password-v2`
2. **OTP JWT Token** - Receive temporary JWT token for OTP verification
3. **OTP Retrieval** - Get OTP code (manual entry or automatic fetch)
4. **OTP Verification** - Send OTP code to `/login-otp` with JWT token
5. **Token Receipt** - Receive access and refresh tokens on success

### Flow Diagram

```
[Credentials] → [login-password-v2] → [OTP JWT Token]
                                            ↓
[OTP Code] ← [Manual/Auto Fetch] ← [OTP Required]
     ↓
[login-otp] → [Access/Refresh Tokens]
```

### Implementation Details

The authentication client handles the complete flow automatically:

```rust
// Internal flow (handled automatically by AuthClient)
async fn login_flow_example() -> Result<(), AuthError> {
    let mut client = AuthClient::new()?;
    
    // Step 1: Initial login - gets OTP JWT token
    let otp_jwt_token = client.login_step1(email, hashed_password).await?;
    
    // Step 2: Fetch OTP (automatic or manual)
    let otp_code = client.fetch_otp().await?;
    
    // Step 3: Verify OTP and get tokens
    let tokens = client.login_step2(&otp_jwt_token, &otp_code, email, hashed_password).await?;
    
    // Tokens are automatically saved by TokenManager
    Ok(())
}
```

## Retry Logic

The OTP system includes comprehensive retry logic for robust operation:

### Email Fetching Retries

```rust
// Automatic retry with exponential backoff
pub fn wait_for_otp(&self, timeout_seconds: u64, check_interval_seconds: u64) -> Result<Option<String>, Box<dyn Error>> {
    let start_time = std::time::Instant::now();
    let timeout_duration = std::time::Duration::from_secs(timeout_seconds);
    let check_interval = std::time::Duration::from_secs(check_interval_seconds);
    
    while start_time.elapsed() < timeout_duration {
        // Check for new OTP emails
        if let Some(otp) = self.fetchotp_recent(3)? {
            return Ok(Some(otp));
        }
        
        // Wait before next check
        std::thread::sleep(check_interval);
    }
    
    Ok(None) // Timeout reached
}
```

### Authentication Retries

The authentication client automatically retries failed requests:

```rust
// Built-in retry logic for authentication requests
impl AuthClient {
    async fn login_with_retry(&mut self, email: &str, password: &str, max_retries: u32) -> Result<AuthTokens, AuthError> {
        let mut retries = 0;
        
        loop {
            match self.login(email, password, None).await {
                Ok(tokens) => return Ok(tokens),
                Err(AuthError::NetworkError(_)) if retries < max_retries => {
                    retries += 1;
                    println!("Login attempt {} failed, retrying...", retries);
                    tokio::time::sleep(tokio::time::Duration::from_secs(retries as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

### Multiple Endpoint Fallback

The system automatically tries different API endpoints on failure:

```rust
// Automatic endpoint rotation on failure
let endpoints = [
    "https://api2.axiom.trade",
    "https://api3.axiom.trade", 
    "https://api6.axiom.trade",
    // ... more endpoints
];

// Client automatically selects different endpoint on retry
```

## Common Issues

### IMAP Connection Issues

**Problem**: Connection to inbox.lv IMAP server fails

**Symptoms**:
- "IMAP connection failed" errors
- "Authentication failed" messages
- Timeout errors connecting to mail.inbox.lv

**Solutions**:

1. **Verify IMAP is enabled**: Wait 15 minutes after enabling IMAP in inbox.lv settings
2. **Check credentials**: Use IMAP password, not web login password  
3. **Test connection**: Try logging into inbox.lv webmail to verify credentials
4. **Firewall check**: Ensure port 993 (IMAPS) is not blocked

```rust
// Test IMAP connection manually
use imap::ClientBuilder;

async fn test_imap_connection() -> Result<(), Box<dyn std::error::Error>> {
    let tls = native_tls::TlsConnector::builder().build()?;
    let client = imap::connect(("mail.inbox.lv", 993), "mail.inbox.lv", &tls)?;
    
    let _session = client.login("your_email@inbox.lv", "your_imap_password")
        .map_err(|e| format!("IMAP login failed: {:?}", e))?;
    
    println!("IMAP connection successful!");
    Ok(())
}
```

### OTP Email Not Found

**Problem**: System cannot find OTP emails in inbox

**Symptoms**:
- "No OTP emails found" errors
- "OTP not received within timeout" messages
- Empty email search results

**Solutions**:

1. **Check email forwarding**: Verify Axiom Trade sends OTP to inbox.lv address
2. **Check spam folder**: OTP emails might be filtered as spam
3. **Verify email format**: Ensure subject contains "Your Axiom security code is XXXXXX"
4. **Test email delivery**: Send test email to inbox.lv to verify delivery

```rust
// Manual email search for debugging
use axiomtrade_rs::email::otp_fetcher::OtpFetcher;

async fn debug_email_search() -> Result<(), Box<dyn std::error::Error>> {
    let fetcher = OtpFetcher::new(
        "your_email@inbox.lv".to_string(),
        "your_imap_password".to_string()
    );
    
    // Check for any recent emails (not just OTP)
    println!("Searching for recent emails...");
    
    // This would require additional methods in the OtpFetcher implementation
    // to help debug email reception issues
    
    Ok(())
}
```

### OTP Extraction Failures

**Problem**: OTP found in email but extraction fails

**Symptoms**:
- "OTP extraction failed" errors
- Retrieved empty or invalid OTP codes
- Regex pattern matching failures

**Solutions**:

1. **Check email format**: Verify actual email subject and body format
2. **Update patterns**: Add new regex patterns if email format changed
3. **Manual verification**: Check raw email content for OTP code location

```rust
// Debug OTP extraction
fn debug_otp_extraction(email_content: &str) {
    let patterns = vec![
        r"Your Axiom security code is[:\s]+(\d{6})",
        r"Your security code is[:\s]+(\d{6})",
        r"security code[:\s]+(\d{6})",
        r"<span[^>]*>(\d{6})</span>",
        r"<b>(\d{6})</b>",
        r"<strong>(\d{6})</strong>",
    ];
    
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(email_content) {
                if let Some(otp) = captures.get(1) {
                    println!("Found OTP '{}' using pattern: {}", otp.as_str(), pattern);
                    return;
                }
            }
        }
    }
    
    println!("No OTP found in email content");
    println!("Email content preview: {}", &email_content[..std::cmp::min(200, email_content.len())]);
}
```

### Authentication Timeout Issues

**Problem**: OTP verification takes too long or times out

**Symptoms**:
- "Authentication timeout" errors
- Slow OTP retrieval (>2 minutes)
- Connection timeouts during verification

**Solutions**:

1. **Increase timeout**: Extend OTP wait time for slow email delivery
2. **Reduce check interval**: Check for emails more frequently
3. **Network check**: Verify stable internet connection
4. **Server selection**: Try different API endpoints

```rust
// Custom timeout configuration
async fn login_with_custom_timeout() -> Result<(), Box<dyn std::error::Error>> {
    let mut auth_client = AuthClient::new()?;
    
    // Configure longer timeout for OTP fetching
    if let Some(otp_fetcher) = auth_client.get_otp_fetcher() {
        // Wait up to 5 minutes, check every 10 seconds
        if let Some(otp) = otp_fetcher.wait_for_otp(300, 10)? {
            let tokens = auth_client.login(
                "email@domain.com",
                "password", 
                Some(otp)
            ).await?;
            println!("Authentication successful with extended timeout!");
        }
    }
    
    Ok(())
}
```

### Token Management Issues

**Problem**: Token storage or refresh failures

**Symptoms**:
- "Token not found" errors
- "Token expired" messages
- Authentication succeeds but tokens not saved

**Solutions**:

1. **Check file permissions**: Ensure write access to token file location
2. **Verify token format**: Check saved token file structure
3. **Manual token refresh**: Implement custom token refresh logic

```rust
// Manual token management
use axiomtrade_rs::auth::{TokenManager, AuthTokens};

async fn manage_tokens_manually() -> Result<(), Box<dyn std::error::Error>> {
    let token_manager = TokenManager::new(Some(std::path::PathBuf::from(".axiom_tokens.json")));
    
    // Check for existing tokens
    if let Ok(Some(tokens)) = token_manager.get_tokens().await {
        if tokens.is_expired() {
            println!("Tokens expired, refreshing...");
            
            let mut auth_client = AuthClient::new()?;
            let new_access_token = auth_client.refresh_token(&tokens.refresh_token).await?;
            
            let updated_tokens = AuthTokens {
                access_token: new_access_token,
                refresh_token: tokens.refresh_token,
                expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            };
            
            token_manager.set_tokens(updated_tokens).await?;
            println!("Tokens refreshed successfully!");
        } else {
            println!("Tokens are still valid");
        }
    } else {
        println!("No existing tokens found, need to login");
    }
    
    Ok(())
}
```

## Best Practices

### Security Considerations

1. **Dedicated Email**: Use inbox.lv account only for OTP purposes
2. **Environment Variables**: Store credentials in `.env` file, never in code
3. **File Permissions**: Secure token storage files with appropriate permissions
4. **Credential Rotation**: Regularly update IMAP passwords

### Performance Optimization

1. **Connection Pooling**: Reuse IMAP connections when possible
2. **Caching**: Cache successful configurations to reduce setup time
3. **Async Operations**: Use async/await for all network operations
4. **Timeout Tuning**: Optimize timeout values based on email delivery speed

### Error Handling

1. **Graceful Degradation**: Fall back to manual OTP when automatic fails
2. **Detailed Logging**: Log OTP retrieval steps for debugging
3. **User Feedback**: Provide clear status messages during OTP process
4. **Retry Strategies**: Implement exponential backoff for failed requests

### Production Deployment

1. **Health Checks**: Monitor OTP system health and email delivery
2. **Alerting**: Set up alerts for OTP failures or slow delivery
3. **Backup Methods**: Have manual OTP fallback for system maintenance
4. **Documentation**: Maintain setup documentation for team members

## Testing Your Setup

Use the provided test example to verify your OTP configuration:

```bash
cargo run --example test_auto_otp
```

This test will:
1. Verify environment variable configuration
2. Test IMAP connection to inbox.lv
3. Attempt full authentication flow with automatic OTP
4. Provide detailed diagnostic information on failures

The test output will guide you through any configuration issues and provide specific troubleshooting steps for your setup.
