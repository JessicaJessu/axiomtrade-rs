use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::sleep;

#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<RwLock<VecDeque<Instant>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    /// Creates a new rate limiter
    /// 
    /// # Arguments
    /// 
    /// * `max_requests` - usize - Maximum number of requests allowed
    /// * `window` - Duration - Time window for the rate limit
    /// 
    /// # Returns
    /// 
    /// RateLimiter - A new rate limiter instance
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            requests: Arc::new(RwLock::new(VecDeque::new())),
            max_requests,
            window,
        }
    }
    
    /// Acquires permission to make a request
    /// 
    /// # Returns
    /// 
    /// Duration - Time to wait before making the request
    pub async fn acquire(&self) -> Duration {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        while !requests.is_empty() {
            if let Some(front) = requests.front() {
                if now.duration_since(*front) > self.window {
                    requests.pop_front();
                } else {
                    break;
                }
            }
        }
        
        if requests.len() >= self.max_requests {
            if let Some(oldest) = requests.front() {
                let wait_time = self.window - now.duration_since(*oldest);
                return wait_time;
            }
        }
        
        requests.push_back(now);
        Duration::ZERO
    }
    
    /// Waits if necessary and then acquires permission
    pub async fn wait_if_needed(&self) {
        let wait_time = self.acquire().await;
        if wait_time > Duration::ZERO {
            sleep(wait_time).await;
        }
    }
    
    /// Gets the current request count
    /// 
    /// # Returns
    /// 
    /// usize - Number of requests in the current window
    pub async fn get_request_count(&self) -> usize {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        while !requests.is_empty() {
            if let Some(front) = requests.front() {
                if now.duration_since(*front) > self.window {
                    requests.pop_front();
                } else {
                    break;
                }
            }
        }
        
        requests.len()
    }
    
    /// Resets the rate limiter
    pub async fn reset(&self) {
        self.requests.write().await.clear();
    }
}

pub struct BucketRateLimiter {
    tokens: Arc<RwLock<f64>>,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Arc<RwLock<Instant>>,
}

impl BucketRateLimiter {
    /// Creates a new token bucket rate limiter
    /// 
    /// # Arguments
    /// 
    /// * `max_tokens` - f64 - Maximum number of tokens in the bucket
    /// * `refill_rate` - f64 - Tokens refilled per second
    /// 
    /// # Returns
    /// 
    /// BucketRateLimiter - A new bucket rate limiter
    pub fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            tokens: Arc::new(RwLock::new(max_tokens)),
            max_tokens,
            refill_rate,
            last_refill: Arc::new(RwLock::new(Instant::now())),
        }
    }
    
    /// Tries to consume tokens
    /// 
    /// # Arguments
    /// 
    /// * `tokens_needed` - f64 - Number of tokens to consume
    /// 
    /// # Returns
    /// 
    /// Option<Duration> - None if successful, Some(wait_time) if need to wait
    pub async fn try_consume(&self, tokens_needed: f64) -> Option<Duration> {
        let mut tokens = self.tokens.write().await;
        let mut last_refill = self.last_refill.write().await;
        let now = Instant::now();
        
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let tokens_to_add = (elapsed * self.refill_rate).min(self.max_tokens - *tokens);
        *tokens = (*tokens + tokens_to_add).min(self.max_tokens);
        *last_refill = now;
        
        if *tokens >= tokens_needed {
            *tokens -= tokens_needed;
            None
        } else {
            let tokens_deficit = tokens_needed - *tokens;
            let wait_time = Duration::from_secs_f64(tokens_deficit / self.refill_rate);
            Some(wait_time)
        }
    }
    
    /// Consumes tokens, waiting if necessary
    /// 
    /// # Arguments
    /// 
    /// * `tokens_needed` - f64 - Number of tokens to consume
    pub async fn consume(&self, tokens_needed: f64) {
        while let Some(wait_time) = self.try_consume(tokens_needed).await {
            sleep(wait_time).await;
        }
    }
}

#[derive(Clone)]
pub struct EndpointRateLimiter {
    limiters: Arc<RwLock<std::collections::HashMap<String, RateLimiter>>>,
    default_limiter: RateLimiter,
}

impl EndpointRateLimiter {
    /// Creates a new endpoint-specific rate limiter
    /// 
    /// # Returns
    /// 
    /// EndpointRateLimiter - A new endpoint rate limiter
    pub fn new() -> Self {
        Self {
            limiters: Arc::new(RwLock::new(std::collections::HashMap::new())),
            default_limiter: RateLimiter::new(100, Duration::from_secs(60)),
        }
    }
    
    /// Adds a rate limiter for a specific endpoint
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - String - The endpoint path
    /// * `max_requests` - usize - Maximum requests for this endpoint
    /// * `window` - Duration - Time window for the limit
    pub async fn add_endpoint_limit(&self, endpoint: String, max_requests: usize, window: Duration) {
        let limiter = RateLimiter::new(max_requests, window);
        self.limiters.write().await.insert(endpoint, limiter);
    }
    
    /// Waits if necessary before allowing a request to an endpoint
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - &str - The endpoint path
    pub async fn wait_for_endpoint(&self, endpoint: &str) {
        let limiters = self.limiters.read().await;
        
        if let Some(limiter) = limiters.get(endpoint) {
            limiter.wait_if_needed().await;
        } else {
            drop(limiters);
            self.default_limiter.wait_if_needed().await;
        }
    }
}