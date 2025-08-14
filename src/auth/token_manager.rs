use super::error::AuthError;
use super::types::AuthTokens;
use serde_json;
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct TokenManager {
    tokens: Arc<RwLock<Option<AuthTokens>>>,
    storage_path: Option<PathBuf>,
}

impl TokenManager {
    /// Creates a new token manager
    /// 
    /// # Arguments
    /// 
    /// * `storage_path` - Option<PathBuf> - Optional path to store tokens persistently
    /// 
    /// # Returns
    /// 
    /// TokenManager - A new instance of the token manager
    pub fn new(storage_path: Option<PathBuf>) -> Self {
        let tokens = if let Some(ref path) = storage_path {
            Self::load_from_file_sync(path).ok()
        } else {
            None
        };
        
        Self {
            tokens: Arc::new(RwLock::new(tokens)),
            storage_path,
        }
    }
    
    fn load_from_file_sync(path: &PathBuf) -> Result<AuthTokens, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let tokens = serde_json::from_str(&content)?;
        Ok(tokens)
    }
    
    /// Set new tokens
    /// 
    /// # Arguments
    /// 
    /// * `tokens` - AuthTokens - The authentication tokens to store
    pub async fn set_tokens(&self, tokens: AuthTokens) -> Result<(), AuthError> {
        {
            let mut guard = self.tokens.write().await;
            *guard = Some(tokens.clone());
        }
        
        if let Some(path) = &self.storage_path {
            self.save_to_file(path, &tokens)?;
        }
        
        Ok(())
    }
    
    /// Get current tokens
    /// 
    /// # Returns
    /// 
    /// Option<AuthTokens> - The stored tokens if available
    pub async fn get_tokens(&self) -> Option<AuthTokens> {
        let guard = self.tokens.read().await;
        guard.clone()
    }
    
    /// Get access token
    /// 
    /// # Returns
    /// 
    /// Result<String, AuthError> - The access token if available
    pub async fn get_access_token(&self) -> Result<String, AuthError> {
        let guard = self.tokens.read().await;
        guard
            .as_ref()
            .map(|t| t.access_token.clone())
            .ok_or(AuthError::TokenNotFound)
    }
    
    /// Get refresh token
    /// 
    /// # Returns
    /// 
    /// Result<String, AuthError> - The refresh token if available
    pub async fn get_refresh_token(&self) -> Result<String, AuthError> {
        let guard = self.tokens.read().await;
        guard
            .as_ref()
            .map(|t| t.refresh_token.clone())
            .ok_or(AuthError::TokenNotFound)
    }
    
    /// Check if tokens are expired
    /// 
    /// # Returns
    /// 
    /// bool - True if tokens are expired or not set
    pub async fn is_expired(&self) -> bool {
        let guard = self.tokens.read().await;
        match &*guard {
            Some(tokens) => tokens.is_expired(),
            None => true,
        }
    }
    
    /// Check if tokens need refresh
    /// 
    /// # Returns
    /// 
    /// bool - True if tokens need refresh soon or not set
    pub async fn needs_refresh(&self) -> bool {
        let guard = self.tokens.read().await;
        match &*guard {
            Some(tokens) => tokens.needs_refresh(),
            None => true,
        }
    }
    
    /// Clear stored tokens
    pub async fn clear(&self) -> Result<(), AuthError> {
        {
            let mut guard = self.tokens.write().await;
            *guard = None;
        }
        
        if let Some(path) = &self.storage_path {
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        
        Ok(())
    }
    
    /// Save tokens to file
    fn save_to_file(&self, path: &PathBuf, tokens: &AuthTokens) -> Result<(), AuthError> {
        let json = serde_json::to_string_pretty(tokens)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    /// Create a token manager from environment variables
    /// 
    /// # Returns
    /// 
    /// Result<Option<TokenManager>, AuthError> - Token manager if tokens are in env
    pub fn from_env() -> Result<Option<Self>, AuthError> {
        let access_token = std::env::var("AXIOM_ACCESS_TOKEN").ok();
        let refresh_token = std::env::var("AXIOM_REFRESH_TOKEN").ok();
        
        match (access_token, refresh_token) {
            (Some(access), Some(refresh)) => {
                let manager = Self::new(None);
                let tokens = AuthTokens {
                    access_token: access,
                    refresh_token: refresh,
                    expires_at: None,
                };
                
                let tokens_arc = manager.tokens.clone();
                tokio::spawn(async move {
                    let mut guard = tokens_arc.write().await;
                    *guard = Some(tokens);
                });
                
                Ok(Some(manager))
            }
            _ => Ok(None),
        }
    }
}