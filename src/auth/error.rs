use thiserror::Error;

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
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Email fetcher error: {0}")]
    EmailError(String),
    
    #[error("API error: {message}")]
    ApiError { message: String },
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Not authenticated")]
    NotAuthenticated,
}