use thiserror::Error;

/// Central error type for the Axiom Trade client
#[derive(Error, Debug)]
pub enum AxiomError {
    #[error("Authentication error: {0}")]
    Auth(#[from] crate::auth::error::AuthError),
    
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("API error: {message}")]
    Api { message: String },
    
    #[error("Invalid response format")]
    InvalidResponse,
    
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
    
    #[error("Hyperliquid API error: {0}")]
    Hyperliquid(String),
    
    #[error("Infrastructure health check failed: {0}")]
    Infrastructure(String),
    
    #[error("Social API error: {0}")]
    Social(String),
    
    #[error("Notifications error: {0}")]
    Notifications(String),
    
    #[error("Cryptographic error: {message}")]
    Crypto { message: String },
    
    #[error("Authentication required: {message}")]
    Authentication { message: String },
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AxiomError>;

impl From<&str> for AxiomError {
    fn from(s: &str) -> Self {
        AxiomError::Unknown(s.to_string())
    }
}

impl From<String> for AxiomError {
    fn from(s: String) -> Self {
        AxiomError::Unknown(s)
    }
}