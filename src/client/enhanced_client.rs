use crate::auth::{AuthClient, AuthError};
use crate::utils::rate_limiter::{EndpointRateLimiter, RateLimiter};
use crate::utils::retry::{retry_with_config, RetryConfig, RetryableError};
use reqwest::{Method, Response, StatusCode};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;

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

pub struct EnhancedClient {
    auth_client: Arc<RwLock<AuthClient>>,
    rate_limiter: EndpointRateLimiter,
    global_rate_limiter: RateLimiter,
    retry_config: RetryConfig,
}

impl EnhancedClient {
    /// Creates a new enhanced client with rate limiting and retry
    /// 
    /// # Returns
    /// 
    /// Result<EnhancedClient, EnhancedClientError> - A new enhanced client
    pub fn new() -> Result<Self, EnhancedClientError> {
        Ok(Self {
            auth_client: Arc::new(RwLock::new(AuthClient::new()?)),
            rate_limiter: EndpointRateLimiter::new(),
            global_rate_limiter: RateLimiter::new(300, Duration::from_secs(60)),
            retry_config: RetryConfig::default()
                .with_max_delay(Duration::from_secs(10))
                .with_jitter(true),
        })
    }
    
    /// Creates an enhanced client with custom configuration
    /// 
    /// # Arguments
    /// 
    /// * `max_requests_per_minute` - usize - Global rate limit
    /// * `retry_config` - RetryConfig - Retry configuration
    /// 
    /// # Returns
    /// 
    /// Result<EnhancedClient, EnhancedClientError> - A new enhanced client
    pub fn with_config(
        max_requests_per_minute: usize,
        retry_config: RetryConfig,
    ) -> Result<Self, EnhancedClientError> {
        Ok(Self {
            auth_client: Arc::new(RwLock::new(AuthClient::new()?)),
            rate_limiter: EndpointRateLimiter::new(),
            global_rate_limiter: RateLimiter::new(max_requests_per_minute, Duration::from_secs(60)),
            retry_config,
        })
    }
    
    /// Adds a rate limit for a specific endpoint
    /// 
    /// # Arguments
    /// 
    /// * `endpoint` - String - The endpoint path
    /// * `max_requests` - usize - Maximum requests allowed
    /// * `window` - Duration - Time window for the limit
    pub async fn add_endpoint_limit(&self, endpoint: String, max_requests: usize, window: Duration) {
        self.rate_limiter.add_endpoint_limit(endpoint, max_requests, window).await;
    }
    
    /// Makes an authenticated request with rate limiting and retry
    /// 
    /// # Arguments
    /// 
    /// * `method` - Method - HTTP method
    /// * `url` - &str - Request URL
    /// * `body` - Option<Value> - Request body
    /// 
    /// # Returns
    /// 
    /// Result<Response, EnhancedClientError> - The response
    pub async fn make_request(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<Response, EnhancedClientError> {
        let endpoint = self.extract_endpoint(url);
        
        self.global_rate_limiter.wait_if_needed().await;
        self.rate_limiter.wait_for_endpoint(&endpoint).await;
        
        let auth_client = Arc::clone(&self.auth_client);
        let url = url.to_string();
        
        retry_with_config(self.retry_config.clone(), || {
            let method = method.clone();
            let url = url.clone();
            let body = body.clone();
            let auth_client = Arc::clone(&auth_client);
            async move {
                auth_client.write().await
                    .make_authenticated_request(method, &url, body)
                    .await
                    .map_err(|e| match e {
                        AuthError::NetworkError(ne) => EnhancedClientError::NetworkError(ne),
                        other => EnhancedClientError::AuthError(other),
                    })
            }
        })
        .await
    }
    
    /// Makes a request and parses JSON response
    /// 
    /// # Arguments
    /// 
    /// * `method` - Method - HTTP method
    /// * `url` - &str - Request URL
    /// * `body` - Option<Value> - Request body
    /// 
    /// # Returns
    /// 
    /// Result<T, EnhancedClientError> - The parsed response
    pub async fn make_json_request<T>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<T, EnhancedClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.make_request(method, url, body).await?;
        
        match response.status() {
            StatusCode::OK => {
                response
                    .json::<T>()
                    .await
                    .map_err(EnhancedClientError::NetworkError)
            }
            StatusCode::TOO_MANY_REQUESTS => {
                Err(EnhancedClientError::RateLimitExceeded)
            }
            status => {
                let error_text = response.text().await.unwrap_or_else(|_| status.to_string());
                Err(EnhancedClientError::RequestFailed(format!(
                    "Request failed with status {}: {}",
                    status, error_text
                )))
            }
        }
    }
    
    /// Extracts endpoint from URL for rate limiting
    /// 
    /// # Arguments
    /// 
    /// * `url` - &str - The full URL
    /// 
    /// # Returns
    /// 
    /// String - The endpoint path
    fn extract_endpoint(&self, url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.path().to_string()
        } else {
            url.to_string()
        }
    }
    
    /// Sets custom retry configuration
    /// 
    /// # Arguments
    /// 
    /// * `config` - RetryConfig - New retry configuration
    pub fn set_retry_config(&mut self, config: RetryConfig) {
        self.retry_config = config;
    }
    
    /// Gets the current rate limit status
    /// 
    /// # Returns
    /// 
    /// usize - Number of requests made in current window
    pub async fn get_rate_limit_status(&self) -> usize {
        self.global_rate_limiter.get_request_count().await
    }
    
    /// Resets rate limiters
    pub async fn reset_rate_limits(&self) {
        self.global_rate_limiter.reset().await;
    }
}