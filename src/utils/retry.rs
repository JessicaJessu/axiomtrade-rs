use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

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

impl RetryConfig {
    /// Creates a new retry configuration
    /// 
    /// # Arguments
    /// 
    /// * `max_retries` - u32 - Maximum number of retry attempts
    /// 
    /// # Returns
    /// 
    /// RetryConfig - A new retry configuration
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            ..Default::default()
        }
    }
    
    /// Sets the initial delay
    /// 
    /// # Arguments
    /// 
    /// * `delay` - Duration - Initial delay between retries
    /// 
    /// # Returns
    /// 
    /// Self - The modified configuration
    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }
    
    /// Sets the maximum delay
    /// 
    /// # Arguments
    /// 
    /// * `delay` - Duration - Maximum delay between retries
    /// 
    /// # Returns
    /// 
    /// Self - The modified configuration
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }
    
    /// Sets the exponential backoff base
    /// 
    /// # Arguments
    /// 
    /// * `base` - f64 - Base for exponential backoff
    /// 
    /// # Returns
    /// 
    /// Self - The modified configuration
    pub fn with_exponential_base(mut self, base: f64) -> Self {
        self.exponential_base = base;
        self
    }
    
    /// Enables or disables jitter
    /// 
    /// # Arguments
    /// 
    /// * `jitter` - bool - Whether to add jitter to delays
    /// 
    /// # Returns
    /// 
    /// Self - The modified configuration
    pub fn with_jitter(mut self, jitter: bool) -> Self {
        self.jitter = jitter;
        self
    }
    
    /// Calculates the delay for a given attempt
    /// 
    /// # Arguments
    /// 
    /// * `attempt` - u32 - The attempt number (0-based)
    /// 
    /// # Returns
    /// 
    /// Duration - The calculated delay
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

pub async fn retry_with_config<F, Fut, T, E>(
    config: RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error = None;
    
    for attempt in 0..=config.max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt < config.max_retries {
                    let delay = config.calculate_delay(attempt);
                    println!("Retry attempt {} after {:?}: {}", attempt + 1, delay, error);
                    sleep(delay).await;
                }
                last_error = Some(error);
            }
        }
    }
    
    Err(last_error.unwrap())
}

pub async fn retry<F, Fut, T, E>(
    max_retries: u32,
    operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    retry_with_config(RetryConfig::new(max_retries), operation).await
}

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

pub async fn retry_with_backoff<F, Fut, T, E>(
    config: RetryConfig,
    mut operation: F,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display + RetryableError,
{
    let mut last_error = None;
    
    for attempt in 0..=config.max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if !error.is_retryable() {
                    return Err(error);
                }
                
                if attempt < config.max_retries {
                    let delay = config.calculate_delay(attempt);
                    println!("Retrying after {:?} (attempt {}): {}", delay, attempt + 1, error);
                    sleep(delay).await;
                }
                last_error = Some(error);
            }
        }
    }
    
    Err(last_error.unwrap())
}