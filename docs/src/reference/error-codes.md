# Error Codes Reference

This document provides a comprehensive reference for all error types, HTTP status codes, and error handling patterns in the Axiom Trade Rust SDK. Use this reference for debugging and implementing robust error handling in your applications.

## Error Type Hierarchy

### Core Error Types

#### `AxiomError` (Primary Error Type)
The main error type that encompasses all possible errors in the SDK.

```rust
pub enum AxiomError {
    Auth(AuthError),                    // Authentication-related errors
    Network(reqwest::Error),            // HTTP network errors
    Serialization(serde_json::Error),   // JSON serialization/deserialization
    Io(std::io::Error),                 // File system I/O errors
    Api { message: String },            // General API errors
    InvalidResponse,                    // Malformed API responses
    RateLimit,                          // Rate limiting exceeded
    ServiceUnavailable,                 // Service temporarily unavailable
    Timeout,                            // Request timeout
    Config(String),                     // Configuration errors
    WebSocket(String),                  // WebSocket connection errors
    Hyperliquid(String),                // Hyperliquid API errors
    Infrastructure(String),             // Infrastructure health check failures
    Social(String),                     // Social API errors
    Notifications(String),              // Notifications system errors
    Crypto { message: String },         // Cryptographic operation errors
    Authentication { message: String }, // Authentication state errors
    Unknown(String),                    // Catch-all for unexpected errors
}
```

#### `AuthError` (Authentication Errors)
Specific to authentication operations including login, OTP, and token management.

```rust
pub enum AuthError {
    NetworkError(reqwest::Error),       // Network failures during auth
    InvalidCredentials,                 // Wrong email/password
    OtpRequired,                        // OTP verification needed
    InvalidOtp,                         // Incorrect OTP code
    TokenExpired,                       // Access token has expired
    TokenNotFound,                      // Token missing from storage
    SerializationError(serde_json::Error), // Token serialization issues
    IoError(std::io::Error),            // Token file I/O errors
    EmailError(String),                 // Email OTP fetching errors
    ApiError { message: String },       // API-specific auth errors
    Unauthorized,                       // HTTP 401 - invalid token
    NotAuthenticated,                   // No valid authentication present
}
```

#### Module-Specific Error Types

**Trading Errors (`TradingError`)**
```rust
pub enum TradingError {
    AuthError(AuthError),               // Authentication failures
    NetworkError(reqwest::Error),       // Network issues
    InvalidTokenMint(String),           // Invalid token address
    InsufficientBalance(String),        // Not enough funds
    SlippageExceeded(String),           // Price slippage too high
    TransactionFailed(String),          // Transaction execution failed
    ApiError(String),                   // General API errors
    ParsingError(String),               // Response parsing errors
}
```

**Market Data Errors (`MarketDataError`)**
```rust
pub enum MarketDataError {
    AuthError(AuthError),               // Authentication failures
    NetworkError(reqwest::Error),       // Network issues
    InvalidTokenMint(String),           // Invalid token address
    TokenNotFound(String),              // Token doesn't exist
    ApiError(String),                   // General API errors
    ParsingError(String),               // Response parsing errors
}
```

**Portfolio Errors (`PortfolioError`)**
```rust
pub enum PortfolioError {
    AuthError(AuthError),               // Authentication failures
    NetworkError(reqwest::Error),       // Network issues
    InvalidWalletAddress(String),       // Invalid Solana address
    ApiError(String),                   // General API errors
    ParsingError(String),               // Response parsing errors
}
```

## HTTP Status Codes

### Success Codes (2xx)
- **200 OK**: Request successful, response contains data
- **201 Created**: Resource created successfully (orders, subscriptions)
- **204 No Content**: Request successful, no response body

### Client Error Codes (4xx)

#### **400 Bad Request**
Invalid request parameters or malformed data.

**Common Causes:**
- Invalid token mint addresses
- Malformed wallet addresses
- Missing required parameters
- Invalid parameter types or ranges

**Example Response:**
```json
{
  "error": "Invalid token mint address",
  "code": 400,
  "details": "Token mint must be a valid Solana address"
}
```

**Handling Pattern:**
```rust
StatusCode::BAD_REQUEST => {
    let error_text = response.text().await?;
    Err(TradingError::ApiError(format!("Bad request: {}", error_text)))
}
```

#### **401 Unauthorized**
Authentication token missing, expired, or invalid.

**Common Causes:**
- Access token expired
- Invalid access token
- Missing Authorization header
- Revoked token

**Example Response:**
```json
{
  "error": "Token expired",
  "code": 401,
  "message": "Access token has expired. Please refresh or re-authenticate."
}
```

**Handling Pattern:**
```rust
StatusCode::UNAUTHORIZED => {
    Err(TradingError::AuthError(AuthError::Unauthorized))
}
```

#### **403 Forbidden**
Valid authentication but insufficient permissions.

**Common Causes:**
- Account suspended
- Feature not enabled for account
- Trading limits exceeded
- Geographic restrictions

#### **404 Not Found**
Requested resource doesn't exist.

**Common Causes:**
- Invalid token mint address
- Non-existent wallet address
- Deleted or unavailable endpoint

**Example Handling:**
```rust
StatusCode::NOT_FOUND => {
    Err(MarketDataError::TokenNotFound(token_symbol.to_string()))
}
```

#### **429 Too Many Requests**
Rate limit exceeded.

**Response Headers:**
- `Retry-After`: Seconds to wait before retrying
- `X-RateLimit-Limit`: Maximum requests per window
- `X-RateLimit-Remaining`: Remaining requests in current window
- `X-RateLimit-Reset`: Timestamp when limit resets

**Handling Pattern:**
```rust
// Retryable status codes include 429
matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504)
```

### Server Error Codes (5xx)

#### **500 Internal Server Error**
Unexpected server-side error.

**Common Causes:**
- Database connectivity issues
- Internal service failures
- Unhandled exceptions

#### **502 Bad Gateway**
Upstream service unavailable.

**Common Causes:**
- Solana RPC node failures
- Third-party API timeouts
- Load balancer issues

#### **503 Service Unavailable**
Service temporarily overloaded or under maintenance.

**Common Causes:**
- Scheduled maintenance
- High traffic overload
- Infrastructure scaling

#### **504 Gateway Timeout**
Request timeout to upstream services.

**Common Causes:**
- Solana network congestion
- Slow blockchain confirmations
- Database query timeouts

## API-Specific Error Codes

### Authentication API Errors

| Code | Type | Description | Recovery |
|------|------|-------------|----------|
| `AUTH_001` | `InvalidCredentials` | Wrong email or password | Re-enter credentials |
| `AUTH_002` | `OtpRequired` | OTP verification needed | Provide OTP code |
| `AUTH_003` | `InvalidOtp` | Incorrect OTP code | Re-enter correct OTP |
| `AUTH_004` | `TokenExpired` | Access token expired | Refresh token or re-login |
| `AUTH_005` | `TokenNotFound` | No stored tokens | Perform fresh login |
| `AUTH_006` | `EmailError` | OTP email fetch failed | Check email configuration |

### Trading API Errors

| Code | Type | Description | Recovery |
|------|------|-------------|----------|
| `TRADE_001` | `InvalidTokenMint` | Invalid token address | Verify token mint address |
| `TRADE_002` | `InsufficientBalance` | Not enough funds | Add funds or reduce amount |
| `TRADE_003` | `SlippageExceeded` | Price moved too much | Increase slippage tolerance |
| `TRADE_004` | `TransactionFailed` | Blockchain transaction failed | Check network status, retry |
| `TRADE_005` | `InvalidAmount` | Amount outside valid range | Check min/max trading limits |

### Market Data API Errors

| Code | Type | Description | Recovery |
|------|------|-------------|----------|
| `MARKET_001` | `TokenNotFound` | Token doesn't exist | Verify token address/symbol |
| `MARKET_002` | `InvalidTokenMint` | Malformed token address | Use valid Solana address |
| `MARKET_003` | `DataUnavailable` | Price data not available | Try different token or wait |

### Portfolio API Errors

| Code | Type | Description | Recovery |
|------|------|-------------|----------|
| `PORTFOLIO_001` | `InvalidWalletAddress` | Invalid Solana address | Use valid wallet address |
| `PORTFOLIO_002` | `WalletNotFound` | Wallet has no activity | Verify address or check different wallet |

## Error Message Formats

### Standard Error Response
```json
{
  "error": "Human-readable error message",
  "code": "ERROR_CODE",
  "details": "Additional context or debugging information",
  "timestamp": "2024-01-15T10:30:00Z",
  "request_id": "req_123456789"
}
```

### Validation Error Response
```json
{
  "error": "Validation failed",
  "code": "VALIDATION_ERROR",
  "field_errors": {
    "token_mint": ["Invalid Solana address format"],
    "amount": ["Must be greater than 0.001"]
  }
}
```

### Rate Limit Error Response
```json
{
  "error": "Rate limit exceeded",
  "code": "RATE_LIMIT_EXCEEDED",
  "retry_after": 60,
  "limit": 100,
  "remaining": 0,
  "reset_time": "2024-01-15T10:31:00Z"
}
```

## Common Error Scenarios and Solutions

### Authentication Issues

**Scenario: Login fails with invalid credentials**
```rust
match auth_client.login(&email, &password, None).await {
    Err(AxiomError::Auth(AuthError::InvalidCredentials)) => {
        println!("Invalid email or password. Please check your credentials.");
        // Guide user to re-enter credentials or password reset
    }
    Err(e) => println!("Login failed: {}", e),
    Ok(tokens) => println!("Login successful!"),
}
```

**Scenario: OTP required but auto-fetch fails**
```rust
match auth_client.login(&email, &password, None).await {
    Err(AxiomError::Auth(AuthError::EmailError(msg))) => {
        println!("OTP auto-fetch failed: {}", msg);
        println!("Please configure inbox.lv integration or enter OTP manually");
        // Fall back to manual OTP entry
    }
    Ok(tokens) => println!("Login successful with auto-OTP!"),
    Err(e) => println!("Login failed: {}", e),
}
```

### Trading Issues

**Scenario: Insufficient balance for trade**
```rust
match trading_client.buy_token(token_mint, amount, slippage).await {
    Err(AxiomError::Trading(TradingError::InsufficientBalance(msg))) => {
        println!("Insufficient balance: {}", msg);
        // Show current balance and suggest funding wallet
        let balance = portfolio_client.get_balance(&wallet_address).await?;
        println!("Current SOL balance: {:.6}", balance.sol_balance);
        println!("Add more SOL to your wallet to complete this trade");
    }
    Ok(response) => println!("Trade successful: {}", response.transaction_signature),
    Err(e) => println!("Trade failed: {}", e),
}
```

**Scenario: Slippage exceeded during volatile market**
```rust
match trading_client.buy_token(token_mint, amount, Some(1.0)).await {
    Err(AxiomError::Trading(TradingError::SlippageExceeded(msg))) => {
        println!("Slippage exceeded: {}", msg);
        println!("Market is volatile. Try:");
        println!("1. Increase slippage tolerance to 2-5%");
        println!("2. Reduce trade amount");
        println!("3. Wait for market to stabilize");
        
        // Retry with higher slippage
        let retry_result = trading_client.buy_token(token_mint, amount, Some(3.0)).await;
        match retry_result {
            Ok(response) => println!("Retry successful with higher slippage"),
            Err(e) => println!("Retry also failed: {}", e),
        }
    }
    Ok(response) => println!("Trade successful"),
    Err(e) => println!("Trade failed: {}", e),
}
```

### Network and Rate Limiting Issues

**Scenario: Rate limit exceeded**
```rust
match market_client.get_trending_tokens().await {
    Err(AxiomError::RateLimit) => {
        println!("Rate limit exceeded. Waiting before retry...");
        tokio::time::sleep(Duration::from_secs(60)).await;
        
        // Retry the request
        match market_client.get_trending_tokens().await {
            Ok(tokens) => println!("Retry successful"),
            Err(e) => println!("Retry failed: {}", e),
        }
    }
    Ok(tokens) => println!("Found {} trending tokens", tokens.len()),
    Err(e) => println!("Request failed: {}", e),
}
```

### WebSocket Connection Issues

**Scenario: WebSocket disconnection with reconnection**
```rust
impl MessageHandler for MyHandler {
    async fn on_disconnected(&self, reason: String) {
        println!("WebSocket disconnected: {}", reason);
        
        // Implement exponential backoff for reconnection
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);
        
        loop {
            tokio::time::sleep(backoff).await;
            
            match self.ws_client.reconnect().await {
                Ok(()) => {
                    println!("Reconnection successful");
                    break;
                }
                Err(e) => {
                    println!("Reconnection failed: {}", e);
                    backoff = std::cmp::min(backoff * 2, max_backoff);
                }
            }
        }
    }
}
```

## Error Handling Best Practices

### 1. Use Result Types Consistently
```rust
// Good: Explicit error handling
pub async fn get_portfolio(&self) -> Result<Portfolio, AxiomError> {
    match self.make_request().await {
        Ok(response) => Ok(response.json().await?),
        Err(e) => Err(AxiomError::Network(e)),
    }
}

// Avoid: Panicking on errors
pub async fn get_portfolio_bad(&self) -> Portfolio {
    self.make_request().await.unwrap().json().await.unwrap()
}
```

### 2. Implement Retry Logic for Transient Errors
```rust
use crate::utils::retry::{RetryConfig, retry_with_backoff};

let config = RetryConfig {
    max_attempts: 3,
    initial_delay: Duration::from_millis(500),
    max_delay: Duration::from_secs(10),
    backoff_multiplier: 2.0,
};

let result = retry_with_backoff(config, || async {
    market_client.get_trending_tokens().await
}).await;
```

### 3. Provide Context in Error Messages
```rust
// Good: Contextual error messages
match trading_client.buy_token(mint, amount, slippage).await {
    Err(e) => return Err(AxiomError::Api {
        message: format!("Failed to buy {} tokens of {}: {}", amount, mint, e)
    }),
    Ok(response) => response,
}
```

### 4. Handle Authentication Errors Gracefully
```rust
async fn handle_auth_error(error: AuthError) -> Result<(), AxiomError> {
    match error {
        AuthError::TokenExpired => {
            println!("Token expired, attempting refresh...");
            // Attempt token refresh
            let token_manager = TokenManager::new(None);
            token_manager.refresh_token().await?;
            Ok(())
        }
        AuthError::Unauthorized => {
            println!("Authentication invalid, please log in again");
            // Clear stored tokens and prompt for re-authentication
            let token_manager = TokenManager::new(None);
            token_manager.clear_tokens().await?;
            Err(AxiomError::Authentication {
                message: "Re-authentication required".to_string()
            })
        }
        _ => Err(AxiomError::Auth(error)),
    }
}
```

### 5. Validate Input Parameters Early
```rust
fn validate_token_mint(mint: &str) -> Result<(), AxiomError> {
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
```

### 6. Use Structured Logging for Error Tracking
```rust
use tracing::{error, warn, info};

match trading_client.execute_trade(&order).await {
    Ok(result) => {
        info!(
            transaction_signature = %result.signature,
            amount = %order.amount,
            token = %order.token_mint,
            "Trade executed successfully"
        );
    }
    Err(e) => {
        error!(
            error = %e,
            order_id = %order.id,
            token = %order.token_mint,
            "Trade execution failed"
        );
        
        // Log additional context for debugging
        warn!(
            user_balance = %current_balance,
            required_amount = %order.amount,
            "Insufficient balance detected"
        );
    }
}
```

## Debugging Error Scenarios

### Enable Debug Logging
```rust
// In your application initialization
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
```

### Common Debugging Commands
```bash
# Run with debug output
RUST_LOG=debug cargo run --example basic_login

# Run specific test with detailed errors
cargo test test_authentication -- --nocapture

# Check network connectivity
curl -v https://api6.axiom.trade/health
```

### Error Investigation Checklist

1. **Check Network Connectivity**
   - Verify internet connection
   - Test API endpoint accessibility
   - Check firewall settings

2. **Validate Authentication**
   - Verify credentials are correct
   - Check token expiration
   - Confirm OTP configuration

3. **Review Request Parameters**
   - Validate token addresses
   - Check amount ranges
   - Verify wallet addresses

4. **Monitor Rate Limits**
   - Check request frequency
   - Review response headers
   - Implement backoff strategies

5. **Examine Server Status**
   - Check Axiom Trade status page
   - Monitor Solana network health
   - Review infrastructure alerts

This reference should help developers quickly identify, understand, and resolve errors when working with the Axiom Trade Rust SDK.