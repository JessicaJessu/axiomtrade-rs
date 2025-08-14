# Rate Limiting

## Overview

Rate limiting is a critical component for managing API requests and ensuring stable operation within Axiom Trade's API limits. The `axiomtrade-rs` library provides three distinct rate limiting implementations to handle different scenarios and requirements.

## Rate Limiting Implementations

### 1. Window-Based Rate Limiter

The `RateLimiter` uses a sliding window approach to track requests over a specified time period.

```rust
use axiomtrade_rs::utils::RateLimiter;
use std::time::Duration;

// Allow 100 requests per minute
let limiter = RateLimiter::new(100, Duration::from_secs(60));

// Wait if necessary before making a request
limiter.wait_if_needed().await;
```

**Key Features:**
- Sliding window implementation using `VecDeque`
- Thread-safe with `Arc<RwLock>`
- Automatic cleanup of expired requests
- Non-blocking permission acquisition

**Configuration Parameters:**
- `max_requests`: Maximum number of requests allowed in the window
- `window`: Time duration for the rate limit window

### 2. Token Bucket Rate Limiter

The `BucketRateLimiter` implements a token bucket algorithm for more flexible rate limiting with burst capabilities.

```rust
use axiomtrade_rs::utils::BucketRateLimiter;

// Allow 10 tokens with refill rate of 1 token per second
let bucket = BucketRateLimiter::new(10.0, 1.0);

// Consume 2 tokens
bucket.consume(2.0).await;
```

**Key Features:**
- Token bucket algorithm with configurable refill rate
- Supports burst requests up to bucket capacity
- Fractional token consumption
- Automatic token refill based on elapsed time

**Configuration Parameters:**
- `max_tokens`: Maximum bucket capacity
- `refill_rate`: Tokens added per second

### 3. Endpoint-Specific Rate Limiter

The `EndpointRateLimiter` provides per-endpoint rate limiting with fallback to default limits.

```rust
use axiomtrade_rs::utils::EndpointRateLimiter;
use std::time::Duration;

let endpoint_limiter = EndpointRateLimiter::new();

// Add specific limit for trading endpoint
endpoint_limiter.add_endpoint_limit(
    "/api/v1/trade".to_string(),
    50,
    Duration::from_secs(60)
).await;

// Wait for endpoint-specific limit
endpoint_limiter.wait_for_endpoint("/api/v1/trade").await;
```

**Key Features:**
- Per-endpoint rate limiting configuration
- Default rate limiter fallback (100 requests/minute)
- Dynamic endpoint limit addition
- Automatic endpoint-specific limit enforcement

## Configurable Limits

### Recommended Rate Limits by Endpoint Category

| Endpoint Category | Recommended Limit | Window | Rationale |
|------------------|-------------------|---------|-----------|
| Authentication | 10 requests/minute | 60s | Prevent brute force attacks |
| Trading | 50 requests/minute | 60s | Balance speed with stability |
| Portfolio | 100 requests/minute | 60s | Allow frequent balance checks |
| Market Data | 200 requests/minute | 60s | High-frequency data needs |
| WebSocket | No limit | N/A | Real-time streaming |

### Environment-Based Configuration

```rust
// Production limits (conservative)
let auth_limiter = RateLimiter::new(10, Duration::from_secs(60));
let trading_limiter = RateLimiter::new(50, Duration::from_secs(60));

// Development limits (more permissive)
let dev_limiter = RateLimiter::new(1000, Duration::from_secs(60));
```

## Backoff Strategies

### Linear Backoff

The basic rate limiters implement linear backoff by calculating exact wait times.

```rust
let wait_time = limiter.acquire().await;
if wait_time > Duration::ZERO {
    tokio::time::sleep(wait_time).await;
}
```

### Exponential Backoff (Recommended for Retries)

For API errors, implement exponential backoff in conjunction with rate limiting:

```rust
use std::cmp::min;

async fn make_request_with_backoff(limiter: &RateLimiter) -> Result<Response, Error> {
    let mut attempts = 0;
    let max_attempts = 5;
    
    loop {
        limiter.wait_if_needed().await;
        
        match api_request().await {
            Ok(response) => return Ok(response),
            Err(error) if attempts < max_attempts => {
                let delay = Duration::from_millis(100 * 2_u64.pow(attempts));
                let jitter = Duration::from_millis(rand::random::<u64>() % 100);
                tokio::time::sleep(delay + jitter).await;
                attempts += 1;
            }
            Err(error) => return Err(error),
        }
    }
}
```

## Circuit Breakers

While not implemented in the current rate limiters, circuit breakers can be combined with rate limiting for enhanced reliability:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

struct CircuitBreaker {
    failure_count: AtomicU32,
    failure_threshold: u32,
    recovery_timeout: Duration,
    last_failure: std::sync::Mutex<Option<Instant>>,
}

impl CircuitBreaker {
    fn is_open(&self) -> bool {
        let count = self.failure_count.load(Ordering::Relaxed);
        if count >= self.failure_threshold {
            if let Ok(last_failure) = self.last_failure.lock() {
                if let Some(time) = *last_failure {
                    return time.elapsed() < self.recovery_timeout;
                }
            }
        }
        false
    }
    
    fn record_success(&self) {
        self.failure_count.store(0, Ordering::Relaxed);
    }
    
    fn record_failure(&self) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut last_failure) = self.last_failure.lock() {
            *last_failure = Some(Instant::now());
        }
    }
}
```

## Best Practices

### 1. Choose the Right Rate Limiter

- **Window-based (`RateLimiter`)**: Use for consistent request rates
- **Token bucket (`BucketRateLimiter`)**: Use when burst requests are acceptable
- **Endpoint-specific (`EndpointRateLimiter`)**: Use for APIs with different limits per endpoint

### 2. Monitor Rate Limit Usage

```rust
let current_requests = limiter.get_request_count().await;
let utilization = (current_requests as f64 / max_requests as f64) * 100.0;

if utilization > 80.0 {
    log::warn!("Rate limit utilization high: {:.1}%", utilization);
}
```

### 3. Implement Graceful Degradation

```rust
async fn make_request_with_fallback(limiter: &RateLimiter) -> Result<Response, Error> {
    let wait_time = limiter.acquire().await;
    
    if wait_time > Duration::from_secs(5) {
        // If wait time is too long, use cached data or simplified response
        return get_cached_response().await;
    }
    
    if wait_time > Duration::ZERO {
        tokio::time::sleep(wait_time).await;
    }
    
    api_request().await
}
```

### 4. Use Jitter for Distributed Systems

When multiple clients are involved, add jitter to prevent thundering herd:

```rust
use rand::Rng;

async fn wait_with_jitter(base_duration: Duration) {
    let jitter_ms = rand::thread_rng().gen_range(0..=100);
    let jitter = Duration::from_millis(jitter_ms);
    tokio::time::sleep(base_duration + jitter).await;
}
```

### 5. Configure Based on Environment

```rust
fn create_rate_limiter_for_env() -> RateLimiter {
    match std::env::var("ENVIRONMENT").as_deref() {
        Ok("production") => RateLimiter::new(50, Duration::from_secs(60)),
        Ok("staging") => RateLimiter::new(100, Duration::from_secs(60)),
        _ => RateLimiter::new(200, Duration::from_secs(60)), // development
    }
}
```

### 6. Reset Rate Limiters When Appropriate

```rust
// Reset rate limiter after authentication renewal
if token_renewed {
    limiter.reset().await;
}
```

### 7. Log Rate Limiting Events

```rust
let wait_time = limiter.acquire().await;
if wait_time > Duration::ZERO {
    log::info!("Rate limited: waiting {:?}", wait_time);
}
```

## Integration with Axiom Client

The Axiom client integrates rate limiting at multiple levels:

```rust
use axiomtrade_rs::client::EnhancedClient;

let client = EnhancedClient::builder()
    .with_rate_limiting(true)
    .with_trading_rate_limit(50, Duration::from_secs(60))
    .with_portfolio_rate_limit(100, Duration::from_secs(60))
    .build()?;
```

## Performance Considerations

### Memory Usage

- Window-based rate limiter: O(n) where n is max_requests
- Token bucket: O(1) constant memory
- Endpoint-specific: O(m) where m is number of endpoints

### CPU Overhead

- Window cleanup: O(k) where k is expired requests
- Token calculation: O(1) constant time
- Lock contention: Minimized with `RwLock`

### Recommendations

1. Use token bucket for high-frequency operations
2. Clean up endpoint limiters periodically in long-running applications
3. Monitor lock contention in highly concurrent scenarios
4. Consider using `Arc::clone()` instead of sharing references

## Testing Rate Limiters

```rust
#[tokio::test]
async fn test_rate_limiter_behavior() {
    let limiter = RateLimiter::new(2, Duration::from_secs(1));
    
    // First two requests should be immediate
    assert_eq!(limiter.acquire().await, Duration::ZERO);
    assert_eq!(limiter.acquire().await, Duration::ZERO);
    
    // Third request should require waiting
    let wait_time = limiter.acquire().await;
    assert!(wait_time > Duration::ZERO);
}
```

## Troubleshooting

### Common Issues

1. **Requests still blocked after wait**: Check system clock synchronization
2. **High memory usage**: Verify window cleanup is working properly  
3. **Inconsistent behavior**: Ensure thread-safe access patterns
4. **Performance degradation**: Monitor lock contention and consider alternatives

### Debug Information

```rust
// Check current state
let request_count = limiter.get_request_count().await;
println!("Current requests in window: {}", request_count);

// Reset if needed
if request_count > expected_max {
    limiter.reset().await;
}
```

Rate limiting is essential for building robust, production-ready trading applications. By implementing appropriate rate limiting strategies, you can ensure reliable operation within API constraints while maintaining optimal performance.