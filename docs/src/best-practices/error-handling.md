# Error Handling Best Practices

This guide covers comprehensive error handling strategies for axiomtrade-rs, including error types, Result handling patterns, retry logic, graceful degradation, and debugging approaches.

## Error Type Hierarchy

### Core Error Types

The axiomtrade-rs library uses a well-structured error hierarchy with the `AxiomError` enum as the central error type:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AxiomError {
    #[error("Authentication error: {0}")]
    Auth(#[from] crate::auth::error::AuthError),
    
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("API error: {message}")]
    Api { message: String },
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Service unavailable")]
    ServiceUnavailable,
    
    #[error("Timeout error")]
    Timeout,
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
}

pub type Result<T> = std::result::Result<T, AxiomError>;
```

### Authentication Errors

Authentication errors are handled through a dedicated `AuthError` enum:

```rust
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Network request failed: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("OTP required but not provided")]
    OtpRequired,
    
    #[error("Invalid OTP code")]
    InvalidOtp,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Token not found")]
    TokenNotFound,
    
    #[error("Email fetcher error: {0}")]
    EmailError(String),
    
    #[error("API error: {message}")]
    ApiError { message: String },
}
```

### Client-Specific Errors

Enhanced client operations use `EnhancedClientError` for more specific error handling:

```rust
#[derive(Error, Debug)]
pub enum EnhancedClientError {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
    
    #[error("Request failed: {0}")]
    RequestFailed(String),
}
```

## Result Handling Patterns

### Basic Error Propagation

Use the `?` operator for clean error propagation:

```rust
pub async fn get_portfolio_balance(&self, wallet: &str) -> Result<PortfolioData> {
    let auth_token = self.auth_client.get_valid_token().await?;
    let response = self.make_request("GET", &format!("/portfolio/{}", wallet), None).await?;
    let portfolio: PortfolioData = serde_json::from_value(response)?;
    Ok(portfolio)
}
```

### Error Mapping and Context

Add context to errors using `map_err`:

```rust
pub async fn login(&mut self, email: &str, password: &str) -> Result<AuthTokens> {
    let response = self.client
        .post(&format!("{}/auth/login", self.base_url))
        .json(&login_request)
        .send()
        .await
        .map_err(|e| AxiomError::Network(e))?;
    
    let auth_data: AuthResponse = response
        .json()
        .await
        .map_err(|e| AxiomError::Serialization(serde_json::Error::from(e)))?;
    
    Ok(auth_data.into())
}
```

### Handling Multiple Error Types

Pattern match on specific error types for different handling strategies:

```rust
pub async fn robust_api_call(&self, endpoint: &str) -> Result<Value> {
    match self.make_request(endpoint).await {
        Ok(response) => Ok(response),
        Err(AxiomError::Auth(AuthError::TokenExpired)) => {
            self.refresh_token().await?;
            self.make_request(endpoint).await
        },
        Err(AxiomError::RateLimit) => {
            tokio::time::sleep(Duration::from_secs(60)).await;
            self.make_request(endpoint).await
        },
        Err(e) => Err(e),
    }
}
```

## Retry Logic Implementation

### RetryConfig Structure

The library provides a comprehensive retry configuration system:

```rust
#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub exponential_base: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            exponential_base: 2.0,
            jitter: true,
        }
    }
}
```

### Exponential Backoff with Jitter

The retry system implements exponential backoff with optional jitter to prevent thundering herd problems:

```rust
impl RetryConfig {
    fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.initial_delay.as_millis() as f64;
        let exponential_delay = base_delay * self.exponential_base.powi(attempt as i32);
        
        let mut delay_ms = exponential_delay.min(self.max_delay.as_millis() as f64);
        
        if self.jitter {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let jitter_factor = rng.gen_range(0.5..1.5);
            delay_ms *= jitter_factor;
        }
        
        Duration::from_millis(delay_ms as u64)
    }
}
```

### Retryable Error Detection

Implement the `RetryableError` trait to determine which errors should trigger retries:

```rust
pub trait RetryableError {
    fn is_retryable(&self) -> bool;
}

impl RetryableError for reqwest::Error {
    fn is_retryable(&self) -> bool {
        if self.is_timeout() || self.is_connect() {
            return true;
        }
        
        if let Some(status) = self.status() {
            matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504)
        } else {
            true
        }
    }
}

impl RetryableError for EnhancedClientError {
    fn is_retryable(&self) -> bool {
        match self {
            EnhancedClientError::NetworkError(e) => e.is_timeout() || e.is_connect(),
            EnhancedClientError::RateLimitExceeded => true,
            EnhancedClientError::RequestFailed(msg) => {
                msg.contains("timeout") || msg.contains("connection")
            }
            _ => false,
        }
    }
}
```

### Using Retry Functions

Use the retry utilities for resilient operations:

```rust
use crate::utils::retry::{retry_with_config, RetryConfig};

pub async fn resilient_api_call(&self) -> Result<Value> {
    let retry_config = RetryConfig::default()
        .with_max_delay(Duration::from_secs(10))
        .with_jitter(true);

    retry_with_config(retry_config, || async {
        self.make_api_request("/some-endpoint").await
    }).await
}
```

## Graceful Degradation Strategies

### Service Availability Fallbacks

Implement fallback mechanisms when primary services are unavailable:

```rust
pub async fn get_token_price_with_fallback(&self, token: &str) -> Result<f64> {
    // Try primary price source
    match self.get_primary_price(token).await {
        Ok(price) => Ok(price),
        Err(AxiomError::ServiceUnavailable) => {
            // Fall back to secondary source
            self.get_fallback_price(token).await
        },
        Err(e) => Err(e),
    }
}

async fn get_fallback_price(&self, token: &str) -> Result<f64> {
    // Implement fallback price fetching logic
    // Could use different API, cached data, or estimated values
    Ok(0.0) // Placeholder
}
```

### Partial Success Handling

Handle scenarios where some operations succeed and others fail:

```rust
pub async fn bulk_portfolio_update(&self, wallets: &[String]) -> PartialResult<Vec<PortfolioData>> {
    let mut successes = Vec::new();
    let mut failures = Vec::new();
    
    for wallet in wallets {
        match self.get_portfolio_balance(wallet).await {
            Ok(portfolio) => successes.push(portfolio),
            Err(e) => failures.push((wallet.clone(), e)),
        }
    }
    
    PartialResult {
        successes,
        failures,
        total_attempted: wallets.len(),
    }
}

pub struct PartialResult<T> {
    pub successes: Vec<T>,
    pub failures: Vec<(String, AxiomError)>,
    pub total_attempted: usize,
}

impl<T> PartialResult<T> {
    pub fn success_rate(&self) -> f64 {
        self.successes.len() as f64 / self.total_attempted as f64
    }
    
    pub fn has_any_success(&self) -> bool {
        !self.successes.is_empty()
    }
}
```

### Circuit Breaker Pattern

Implement circuit breaker for failing services:

```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum CircuitState {
    Closed,    // Normal operation
    Open,      // Failing fast
    HalfOpen,  // Testing if service recovered
}

pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_count: Arc<Mutex<u32>>,
    last_failure_time: Arc<Mutex<Option<Instant>>>,
    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(Mutex::new(0)),
            last_failure_time: Arc::new(Mutex::new(None)),
            failure_threshold,
            recovery_timeout,
        }
    }
    
    pub async fn call<F, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
    {
        {
            let state = self.state.lock().unwrap();
            match *state {
                CircuitState::Open => {
                    if self.should_attempt_reset() {
                        drop(state);
                        *self.state.lock().unwrap() = CircuitState::HalfOpen;
                    } else {
                        return Err(AxiomError::ServiceUnavailable);
                    }
                }
                _ => {}
            }
        }
        
        match operation().await {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(e)
            }
        }
    }
    
    fn should_attempt_reset(&self) -> bool {
        if let Some(last_failure) = *self.last_failure_time.lock().unwrap() {
            Instant::now().duration_since(last_failure) >= self.recovery_timeout
        } else {
            false
        }
    }
    
    fn on_success(&self) {
        *self.state.lock().unwrap() = CircuitState::Closed;
        *self.failure_count.lock().unwrap() = 0;
    }
    
    fn on_failure(&self) {
        let mut failure_count = self.failure_count.lock().unwrap();
        *failure_count += 1;
        *self.last_failure_time.lock().unwrap() = Some(Instant::now());
        
        if *failure_count >= self.failure_threshold {
            *self.state.lock().unwrap() = CircuitState::Open;
        }
    }
}
```

## Logging and Debugging

### Structured Logging

Use structured logging for better error tracking:

```rust
use tracing::{error, warn, info, debug, span, Level};

pub async fn authenticated_request(&self, endpoint: &str) -> Result<Value> {
    let span = span!(Level::INFO, "authenticated_request", endpoint = endpoint);
    let _enter = span.enter();
    
    debug!("Starting authenticated request");
    
    match self.make_request(endpoint).await {
        Ok(response) => {
            info!("Request successful");
            Ok(response)
        }
        Err(e) => {
            error!(error = %e, "Request failed");
            
            // Log additional context based on error type
            match &e {
                AxiomError::Auth(auth_err) => {
                    warn!(auth_error = %auth_err, "Authentication error occurred");
                }
                AxiomError::RateLimit => {
                    warn!("Rate limit exceeded, consider implementing backoff");
                }
                AxiomError::Network(net_err) => {
                    warn!(network_error = %net_err, "Network connectivity issue");
                }
                _ => {}
            }
            
            Err(e)
        }
    }
}
```

### Error Context and Tracing

Add context to errors for better debugging:

```rust
use anyhow::{Context, Result as AnyhowResult};

pub async fn complex_operation(&self, user_id: u64) -> AnyhowResult<ProcessedData> {
    let user_data = self.fetch_user_data(user_id).await
        .with_context(|| format!("Failed to fetch user data for user {}", user_id))?;
    
    let portfolio = self.get_portfolio(&user_data.wallet_address).await
        .with_context(|| format!("Failed to get portfolio for wallet {}", user_data.wallet_address))?;
    
    let processed = self.process_portfolio_data(&portfolio).await
        .context("Failed to process portfolio data")?;
    
    Ok(processed)
}
```

### Debug Logging for Development

Implement debug logging that can be enabled in development:

```rust
#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        eprintln!("[DEBUG] {}: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"), format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($($arg:tt)*) => {};
}

pub async fn debug_enabled_request(&self, endpoint: &str) -> Result<Value> {
    debug_log!("Making request to endpoint: {}", endpoint);
    
    let start_time = std::time::Instant::now();
    let result = self.make_request(endpoint).await;
    let duration = start_time.elapsed();
    
    match &result {
        Ok(_) => debug_log!("Request to {} completed successfully in {:?}", endpoint, duration),
        Err(e) => debug_log!("Request to {} failed after {:?}: {}", endpoint, duration, e),
    }
    
    result
}
```

## Error Handling Best Practices

### 1. Fail Fast Principle

Validate inputs early and return errors immediately:

```rust
pub async fn create_trade_order(&self, amount: f64, token_address: &str) -> Result<TradeOrder> {
    // Validate inputs early
    if amount <= 0.0 {
        return Err(AxiomError::Config("Amount must be positive".to_string()));
    }
    
    if token_address.len() != 44 {
        return Err(AxiomError::Config("Invalid token address format".to_string()));
    }
    
    // Proceed with operation
    self.execute_trade(amount, token_address).await
}
```

### 2. Specific Error Types

Use specific error types rather than generic strings:

```rust
// Good: Specific error types
#[derive(Error, Debug)]
pub enum TradeError {
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: f64, available: f64 },
    
    #[error("Invalid token address: {address}")]
    InvalidTokenAddress { address: String },
    
    #[error("Slippage tolerance exceeded: expected {expected}%, actual {actual}%")]
    SlippageExceeded { expected: f64, actual: f64 },
}

// Bad: Generic string errors
fn bad_example() -> Result<()> {
    Err(AxiomError::Config("Something went wrong".to_string()))
}
```

### 3. Error Recovery Strategies

Implement appropriate recovery strategies for different error types:

```rust
pub async fn resilient_portfolio_fetch(&self, wallet: &str, max_retries: u32) -> Result<Portfolio> {
    for attempt in 0..max_retries {
        match self.get_portfolio(wallet).await {
            Ok(portfolio) => return Ok(portfolio),
            Err(AxiomError::RateLimit) => {
                // Wait longer for rate limits
                let delay = Duration::from_secs(60 * (attempt + 1) as u64);
                tokio::time::sleep(delay).await;
            }
            Err(AxiomError::Network(_)) => {
                // Shorter wait for network issues
                let delay = Duration::from_millis(1000 * (attempt + 1) as u64);
                tokio::time::sleep(delay).await;
            }
            Err(AxiomError::Auth(_)) => {
                // Try to refresh authentication
                self.refresh_auth().await?;
            }
            Err(e) => {
                // Non-recoverable errors
                return Err(e);
            }
        }
    }
    
    Err(AxiomError::Config("Max retries exceeded".to_string()))
}
```

### 4. Resource Cleanup

Ensure proper resource cleanup even when errors occur:

```rust
pub async fn websocket_with_cleanup(&self) -> Result<Vec<Message>> {
    let ws_connection = self.connect_websocket().await?;
    let mut messages = Vec::new();
    
    let result = async {
        loop {
            match ws_connection.next().await {
                Some(Ok(message)) => messages.push(message),
                Some(Err(e)) => return Err(AxiomError::WebSocket(e.to_string())),
                None => break,
            }
        }
        Ok(messages)
    }.await;
    
    // Ensure connection is properly closed regardless of success/failure
    if let Err(e) = ws_connection.close().await {
        warn!("Failed to close WebSocket connection: {}", e);
    }
    
    result
}
```

### 5. Testing Error Scenarios

Write comprehensive tests for error conditions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_retry_on_rate_limit() {
        let mut client = MockClient::new();
        
        // First call returns rate limit error
        client.expect_get_portfolio()
            .times(1)
            .returning(|_| Err(AxiomError::RateLimit));
        
        // Second call succeeds
        client.expect_get_portfolio()
            .times(1)
            .returning(|_| Ok(Portfolio::default()));
        
        let result = client.resilient_portfolio_fetch("test_wallet", 2).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_non_retryable_error() {
        let mut client = MockClient::new();
        
        client.expect_get_portfolio()
            .times(1)
            .returning(|_| Err(AxiomError::Config("Invalid wallet".to_string())));
        
        let result = client.resilient_portfolio_fetch("invalid_wallet", 3).await;
        assert!(result.is_err());
        
        // Should not retry non-retryable errors
        assert_eq!(client.call_count(), 1);
    }
}
```

## Monitoring and Alerting

### Error Metrics Collection

Implement error metrics for monitoring:

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ErrorMetrics {
    error_counts: Arc<HashMap<String, AtomicU64>>,
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
}

impl ErrorMetrics {
    pub fn new() -> Self {
        Self {
            error_counts: Arc::new(HashMap::new()),
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
        }
    }
    
    pub fn record_request_success(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn record_request_error(&self, error_type: &str) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        // Note: This is simplified - in practice, you'd need thread-safe HashMap updates
    }
    
    pub fn get_error_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        
        if total == 0 {
            0.0
        } else {
            1.0 - (successful as f64 / total as f64)
        }
    }
}
```

### Health Check Endpoints

Implement health checks that consider error rates:

```rust
#[derive(serde::Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub error_rate: f64,
    pub recent_errors: Vec<String>,
    pub uptime_seconds: u64,
}

impl AxiomClient {
    pub async fn health_check(&self) -> HealthStatus {
        let error_rate = self.metrics.get_error_rate();
        
        let status = if error_rate > 0.5 {
            "unhealthy"
        } else if error_rate > 0.1 {
            "degraded"
        } else {
            "healthy"
        }.to_string();
        
        HealthStatus {
            status,
            error_rate,
            recent_errors: self.get_recent_errors(),
            uptime_seconds: self.get_uptime().as_secs(),
        }
    }
}
```

## Conclusion

Effective error handling in axiomtrade-rs requires:

1. **Structured Error Types**: Use the provided error hierarchy with specific, actionable error types
2. **Robust Retry Logic**: Implement exponential backoff with jitter for retryable errors
3. **Graceful Degradation**: Provide fallbacks and partial success handling
4. **Comprehensive Logging**: Use structured logging with appropriate context
5. **Proactive Monitoring**: Collect metrics and implement health checks
6. **Thorough Testing**: Test error scenarios and recovery strategies

By following these patterns, you can build resilient applications that handle failures gracefully and provide excellent user experience even when things go wrong.