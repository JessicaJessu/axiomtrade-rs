# Configuration Reference

This document provides a comprehensive reference for all configuration options available in the axiomtrade-rs library.

## Environment Variables

### Authentication Configuration

#### Required Variables
- **`AXIOM_EMAIL`** - Your Axiom Trade account email address
- **`AXIOM_PASSWORD`** - Your Axiom Trade account password (will be hashed automatically)

#### Optional OTP Automation
For automated OTP fetching via IMAP:
- **`INBOX_LV_EMAIL`** - Your inbox.lv email address (e.g., `username@inbox.lv`)
- **`INBOX_LV_PASSWORD`** - Your inbox.lv IMAP password (not your web login password)

**Note**: The OTP automation requires setting up email forwarding from your Axiom Trade account to your inbox.lv address. See the [Automatic OTP Guide](../automatic-otp.md) for detailed setup instructions.

### Turnkey Integration
Optional variables for Turnkey wallet integration:
- **`TURNKEY_ORGANIZATION_ID`** - Your Turnkey organization ID
- **`TURNKEY_USER_ID`** - Your Turnkey user ID  
- **`TURNKEY_CLIENT_SECRET`** - Your Turnkey client secret

### Token Storage
- **`AXIOM_TOKEN_FILE`** - Path to store authentication tokens (default: `.axiom_tokens.json`)

### Logging Configuration
- **`RUST_LOG`** - Logging level configuration (e.g., `debug`, `info`, `warn`, `error`)

## API Endpoints

The library uses multiple API endpoints for redundancy and load balancing:

### Primary Endpoints
```
https://api2.axiom.trade
https://api3.axiom.trade
https://api6.axiom.trade
https://api7.axiom.trade
https://api8.axiom.trade
https://api9.axiom.trade
https://api10.axiom.trade
```

### Endpoint Selection
- The client automatically selects endpoints randomly for load distribution
- Failed endpoints are automatically excluded from subsequent requests
- No manual endpoint configuration is required

### WebSocket Endpoints

#### Market Data WebSocket Regions
- **US West**: `socket8.axiom.trade`, `cluster-usw2.axiom.trade`
- **US Central**: `cluster3.axiom.trade`, `cluster-usc2.axiom.trade`
- **US East**: `cluster5.axiom.trade`, `cluster-use2.axiom.trade`
- **EU West**: `cluster6.axiom.trade`, `cluster-euw2.axiom.trade`
- **EU Central**: `cluster2.axiom.trade`, `cluster-euc2.axiom.trade`
- **EU East**: `cluster8.axiom.trade`
- **Asia**: `cluster4.axiom.trade`
- **Australia**: `cluster7.axiom.trade`
- **Global**: `cluster9.axiom.trade`

#### Token Price WebSocket
- **Primary**: `socket8.axiom.trade`

## Timeout Settings

### HTTP Client Timeouts
- **Request Timeout**: 30 seconds (authentication requests)
- **Infrastructure Health Check**: 5 seconds
- **Turnkey API Requests**: 5 seconds

### WebSocket Timeouts
- **Connection Timeout**: 30 seconds
- **Token Refresh Interval**: 600 seconds (10 minutes)
- **Reconnection Delay**: 1 second

### OTP Fetching Timeouts
- **Default OTP Wait**: 120 seconds
- **Email Check Interval**: 5 seconds

### Custom Timeout Configuration
```rust
use axiomtrade_rs::client::enhanced_client::EnhancedClient;
use std::time::Duration;

// Create client with custom timeout
let mut client = EnhancedClient::builder()
    .with_timeout(Duration::from_secs(60))
    .build()?;
```

## Retry Configuration

### Default Retry Settings
- **Maximum Retries**: 3 attempts
- **Initial Delay**: 100 milliseconds
- **Maximum Delay**: 30 seconds
- **Backoff Strategy**: Exponential with jitter

### Retry Conditions
The following errors trigger automatic retries:
- Network timeouts
- Connection errors
- HTTP 5xx status codes
- Rate limit errors (429)
- Temporary authentication failures

### Custom Retry Configuration
```rust
use axiomtrade_rs::utils::retry::RetryConfig;
use std::time::Duration;

let retry_config = RetryConfig::builder()
    .with_max_attempts(5)
    .with_initial_delay(Duration::from_millis(200))
    .with_max_delay(Duration::from_secs(60))
    .build();
```

## Rate Limiting Configuration

### Global Rate Limits
- **Default**: 300 requests per 60 seconds
- **Sliding Window**: 60-second window
- **Per-Endpoint**: Individual rate limiters can be configured

### Rate Limiting Implementation
The library uses a token bucket algorithm with the following characteristics:
- **Token Refill Rate**: Configurable tokens per second
- **Bucket Size**: Maximum burst capacity
- **Automatic Backoff**: Waits when rate limit is exceeded

### Custom Rate Limiting
```rust
use axiomtrade_rs::utils::rate_limiter::RateLimiter;
use std::time::Duration;

// Create custom rate limiter: 100 requests per minute
let rate_limiter = RateLimiter::new(100, Duration::from_secs(60));

// Or use bucket rate limiter
use axiomtrade_rs::utils::rate_limiter::BucketRateLimiter;
let bucket_limiter = BucketRateLimiter::new(100.0, 10.0); // 100 tokens, 10/sec refill
```

### Endpoint-Specific Rate Limits
```rust
use axiomtrade_rs::utils::rate_limiter::EndpointRateLimiter;

let endpoint_limiter = EndpointRateLimiter::new();

// Add specific limits for trading endpoints
endpoint_limiter.add_endpoint_limit(
    "/trade/buy".to_string(),
    10,
    Duration::from_secs(60)
).await;
```

## WebSocket Parameters

### Connection Parameters
- **Protocol**: WSS (WebSocket Secure)
- **Headers**: Authentication cookies, origin validation
- **Reconnection**: Automatic on token expiry
- **Heartbeat**: Automatic ping/pong handling

### Subscription Types
- **Market Data**: New token pairs (`new_pairs` room)
- **Price Alerts**: Token-specific price updates
- **Portfolio**: Wallet transaction monitoring
- **Trading**: Order status updates

### WebSocket Configuration
```rust
use axiomtrade_rs::websocket::client::{WebSocketClient, Region};

// Create client with specific region
let client = WebSocketClient::with_region(handler, Region::USWest)?;

// Configure auto-reconnect
client.set_auto_reconnect(true);
```

### Message Format
WebSocket messages follow this general structure:
```json
{
  "action": "join|leave",
  "room": "room_name",
  "data": { /* optional payload */ }
}
```

## Password Hashing Configuration

### PBKDF2 Parameters
- **Algorithm**: SHA256
- **Iterations**: 600,000
- **Salt Length**: 32 bytes (random)
- **Output Length**: 32 bytes
- **Encoding**: Base64

### Security Settings
```rust
use axiomtrade_rs::utils::password::hashpassword;

// Hash a password (uses secure defaults)
let hashed = hashpassword("your_password");
```

## TLS Configuration

### Certificate Validation
- **Certificate Verification**: Enabled by default
- **Protocol**: TLS 1.2 minimum
- **Cipher Suites**: Modern secure ciphers only

### Custom TLS Configuration
```rust
use reqwest::ClientBuilder;

let client = ClientBuilder::new()
    .min_tls_version(reqwest::tls::Version::TLS_1_2)
    .build()?;
```

## Error Handling Configuration

### Error Categories
- **Authentication Errors**: Login, token management
- **Network Errors**: Timeouts, connection failures
- **API Errors**: Invalid requests, rate limits
- **Parsing Errors**: JSON deserialization issues

### Error Retry Logic
```rust
use axiomtrade_rs::errors::AxiomError;

match error {
    AxiomError::NetworkError(_) => {
        // Automatically retried
    },
    AxiomError::AuthenticationError(_) => {
        // Token refresh attempted
    },
    AxiomError::RateLimitError(_) => {
        // Automatic backoff applied
    },
    _ => {
        // Manual handling required
    }
}
```

## Performance Tuning

### Connection Pooling
- **Keep-Alive**: Enabled by default
- **Pool Size**: Automatically managed
- **Connection Reuse**: Aggressive reuse for efficiency

### Memory Management
- **Token Caching**: In-memory with file persistence
- **Response Buffering**: Streamed for large responses
- **JSON Parsing**: Zero-copy where possible

### Optimization Settings
```rust
use axiomtrade_rs::client::enhanced_client::EnhancedClient;

let client = EnhancedClient::builder()
    .with_max_requests_per_minute(500)  // Higher rate limit
    .with_connection_pool_size(20)      // More concurrent connections
    .build()?;
```

## Security Configuration

### API Security
- **Request Signing**: Automatic where required
- **Token Rotation**: Automatic refresh
- **Secure Storage**: OS keychain integration (planned)

### Input Validation
- **Parameter Sanitization**: Automatic
- **SQL Injection Prevention**: Not applicable (REST API)
- **XSS Prevention**: JSON-only communication

### Audit Logging
Configure audit logging for security compliance:
```rust
use tracing::{info, warn, error};

// Enable security event logging
info!("Authentication successful for user: {}", email);
warn!("Rate limit exceeded for endpoint: {}", endpoint);
error!("Authentication failed: {}", error);
```

## Example Configuration File

Create a `.env` file in your project root:

```env
# Required authentication
AXIOM_EMAIL=your-email@example.com
AXIOM_PASSWORD=your-secure-password

# Optional OTP automation
INBOX_LV_EMAIL=username@inbox.lv
INBOX_LV_PASSWORD=your-imap-password

# Optional Turnkey integration
TURNKEY_ORGANIZATION_ID=your-org-id
TURNKEY_USER_ID=your-user-id
TURNKEY_CLIENT_SECRET=your-client-secret

# Optional token storage path
AXIOM_TOKEN_FILE=./tokens/axiom_tokens.json

# Logging configuration
RUST_LOG=axiomtrade_rs=info,reqwest=warn
```

## Advanced Configuration

### Custom HTTP Headers
```rust
use reqwest::header::{HeaderMap, HeaderValue};

let mut headers = HeaderMap::new();
headers.insert("X-Custom-Header", HeaderValue::from_static("value"));

let client = ClientBuilder::new()
    .default_headers(headers)
    .build()?;
```

### Proxy Configuration
```rust
let client = ClientBuilder::new()
    .proxy(reqwest::Proxy::http("http://proxy:8080")?)
    .build()?;
```

### Custom User Agents
```rust
use axiomtrade_rs::utils::user_agents::get_random_desktop_user_agent;

let user_agent = get_random_desktop_user_agent();
let client = AuthClient::new_with_user_agent(&user_agent)?;
```

This configuration reference covers all the major configuration options available in the axiomtrade-rs library. For specific implementation examples, refer to the examples in the [Examples](../examples/) section.