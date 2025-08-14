use super::error::AuthError;
use super::types::{AuthSession, AuthTokens, AuthCookies, TurnkeySession, SessionMetadata, UserInfo, LoginResult, TurnkeyCredentials};
use crate::api::turnkey::TurnkeyClient;
use crate::models::turnkey::{TurnkeyWhoAmI, GetApiKeysResponse};
use serde_json;
use std::fs;
use std::path::PathBuf;
use tokio::sync::RwLock;
use std::sync::Arc;
use chrono::Utc;

/// Enhanced session manager that handles complete authentication state
/// Includes JWT tokens, cookies, Turnkey sessions, and metadata
pub struct SessionManager {
    session: Arc<RwLock<Option<AuthSession>>>,
    storage_path: Option<PathBuf>,
    turnkey_client: Option<TurnkeyClient>,
    auto_save: bool,
}

impl SessionManager {
    /// Create a new session manager
    ///
    /// Args:
    ///     storage_path: Option<PathBuf> - Optional path to store session persistently
    ///     auto_save: bool - Whether to automatically save session changes
    ///
    /// Returns:
    ///     SessionManager: New instance of the session manager
    pub fn new(storage_path: Option<PathBuf>, auto_save: bool) -> Self {
        let session = if let Some(ref path) = storage_path {
            Self::load_session_from_file(path).ok()
        } else {
            None
        };
        
        Self {
            session: Arc::new(RwLock::new(session)),
            storage_path,
            turnkey_client: Some(TurnkeyClient::new()),
            auto_save,
        }
    }
    
    /// Load session from file synchronously
    fn load_session_from_file(path: &PathBuf) -> Result<AuthSession, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let session = serde_json::from_str(&content)?;
        Ok(session)
    }
    
    /// Create a new authentication session with random user agent
    ///
    /// Args:
    ///     tokens: AuthTokens - JWT tokens from login
    ///     user_info: Option<UserInfo> - User information from login
    ///     cookies: Option<AuthCookies> - HTTP cookies from login response
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn create_session(
        &self,
        tokens: AuthTokens,
        user_info: Option<UserInfo>,
        cookies: Option<AuthCookies>,
    ) -> Result<(), AuthError> {
        let mut new_session = AuthSession::new(tokens, user_info);
        
        // Update cookies if provided
        if let Some(cookies) = cookies {
            new_session.cookies = cookies;
        }
        
        // Set session
        {
            let mut guard = self.session.write().await;
            *guard = Some(new_session);
        }
        
        if self.auto_save {
            self.save_session().await?;
        }
        
        Ok(())
    }
    
    /// Create a new authentication session with specific user agent
    ///
    /// Args:
    ///     tokens: AuthTokens - JWT tokens from login
    ///     user_info: Option<UserInfo> - User information from login
    ///     cookies: Option<AuthCookies> - HTTP cookies from login response
    ///     user_agent: String - Specific user agent to use
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn create_session_with_user_agent(
        &self,
        tokens: AuthTokens,
        user_info: Option<UserInfo>,
        cookies: Option<AuthCookies>,
        user_agent: String,
    ) -> Result<(), AuthError> {
        let mut new_session = AuthSession::new_with_user_agent(tokens, user_info, user_agent);
        
        // Update cookies if provided
        if let Some(cookies) = cookies {
            new_session.cookies = cookies;
        }
        
        // Set session
        {
            let mut guard = self.session.write().await;
            *guard = Some(new_session);
        }
        
        if self.auto_save {
            self.save_session().await?;
        }
        
        Ok(())
    }
    
    /// Create a session from a complete login result (includes Turnkey setup)
    ///
    /// Args:
    ///     login_result: LoginResult - Complete login result with tokens and Turnkey credentials
    ///     cookies: Option<AuthCookies> - HTTP cookies from login response
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn create_session_from_login_result(
        &self,
        login_result: LoginResult,
        cookies: Option<AuthCookies>,
    ) -> Result<(), AuthError> {
        let mut new_session = AuthSession::new(login_result.tokens, login_result.user_info);
        
        // Update cookies if provided
        if let Some(cookies) = cookies {
            new_session.cookies = cookies;
        }
        
        // Set up Turnkey session if credentials are available
        if let Some(turnkey_creds) = login_result.turnkey_credentials {
            println!("🔐 Setting up Turnkey session with real credentials...");
            
            match self.setup_turnkey_session_with_credentials(&turnkey_creds).await {
                Ok(turnkey_session) => {
                    new_session.update_turnkey_session(turnkey_session);
                    println!("   ✅ Turnkey session created successfully!");
                }
                Err(e) => {
                    println!("   ⚠️  Turnkey session creation failed: {}", e);
                }
            }
        }
        
        // Set session
        {
            let mut guard = self.session.write().await;
            *guard = Some(new_session);
        }
        
        if self.auto_save {
            self.save_session().await?;
        }
        
        Ok(())
    }
    
    /// Set up Turnkey session with real credentials from login
    async fn setup_turnkey_session_with_credentials(
        &self,
        credentials: &TurnkeyCredentials,
    ) -> Result<TurnkeySession, AuthError> {
        let turnkey_client = self.turnkey_client.as_ref()
            .ok_or_else(|| AuthError::ApiError { message: "Turnkey client not available".to_string() })?;
        
        // Create TurnkeySession directly from credentials
        // In a real implementation, we'd use the client_secret to create a proper Turnkey client
        // and fetch the user's API keys, but for now we'll create a basic session
        let turnkey_session = TurnkeySession {
            organization_id: credentials.organization_id.clone(),
            user_id: credentials.user_id.clone(),
            username: format!("user_{}", &credentials.user_id[..8]), // Truncated user ID as username
            client_secret: credentials.client_secret.clone(),
            api_keys: vec![], // Would be populated with real API keys
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::days(30)),
        };
        
        Ok(turnkey_session)
    }
    
    /// Get current session
    ///
    /// Returns:
    ///     Option<AuthSession>: Current session if available
    pub async fn get_session(&self) -> Option<AuthSession> {
        let guard = self.session.read().await;
        guard.clone()
    }
    
    /// Check if session is valid
    ///
    /// Returns:
    ///     bool: True if session exists and is valid
    pub async fn is_session_valid(&self) -> bool {
        let guard = self.session.read().await;
        guard.as_ref().map_or(false, |session| session.is_valid())
    }
    
    /// Check if session needs refresh
    ///
    /// Returns:
    ///     bool: True if session exists but needs refresh
    pub async fn needs_refresh(&self) -> bool {
        let guard = self.session.read().await;
        guard.as_ref().map_or(false, |session| session.needs_refresh())
    }
    
    /// Update tokens in the current session
    ///
    /// Args:
    ///     new_tokens: AuthTokens - Updated JWT tokens
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn update_tokens(&self, new_tokens: AuthTokens) -> Result<(), AuthError> {
        {
            let mut guard = self.session.write().await;
            if let Some(session) = guard.as_mut() {
                session.update_tokens(new_tokens);
            } else {
                return Err(AuthError::NotAuthenticated);
            }
        }
        
        if self.auto_save {
            self.save_session().await?;
        }
        
        Ok(())
    }
    
    /// Update cookies in the current session
    ///
    /// Args:
    ///     new_cookies: AuthCookies - Updated HTTP cookies
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn update_cookies(&self, new_cookies: AuthCookies) -> Result<(), AuthError> {
        {
            let mut guard = self.session.write().await;
            if let Some(session) = guard.as_mut() {
                session.cookies.merge_with(&new_cookies);
            } else {
                return Err(AuthError::NotAuthenticated);
            }
        }
        
        if self.auto_save {
            self.save_session().await?;
        }
        
        Ok(())
    }
    
    /// Set up Turnkey session integration
    ///
    /// Args:
    ///     organization_id: &str - Turnkey organization ID
    ///     user_id: &str - Turnkey user ID
    ///     client_secret: &str - Client secret for P256 signing
    ///     user_password: &str - User password for key generation
    ///
    /// Returns:
    ///     Result<TurnkeySession, AuthError>: Created Turnkey session
    pub async fn setup_turnkey_session(
        &self,
        organization_id: &str,
        user_id: &str,
        client_secret: &str,
        user_password: &str,
    ) -> Result<TurnkeySession, AuthError> {
        let mut turnkey_client = self.turnkey_client.as_ref()
            .ok_or_else(|| AuthError::ApiError { message: "Turnkey client not available".to_string() })?.clone();
        
        // Set credentials for signing
        turnkey_client.set_credentials(organization_id, user_id, user_password);
        
        // Get user identity
        let whoami = turnkey_client.whoami(organization_id, client_secret).await
            .map_err(|e| AuthError::ApiError { message: format!("Turnkey whoami failed: {}", e) })?;
        
        // Get API keys
        let api_keys = turnkey_client.get_api_keys(user_id, organization_id, client_secret).await
            .map_err(|e| AuthError::ApiError { message: format!("Turnkey get_api_keys failed: {}", e) })?;
        
        // Parse session
        let turnkey_session = turnkey_client.parse_session(&whoami, &api_keys, client_secret);
        
        // Update session with Turnkey data
        {
            let mut guard = self.session.write().await;
            if let Some(session) = guard.as_mut() {
                session.update_turnkey_session(turnkey_session.clone());
            } else {
                return Err(AuthError::NotAuthenticated);
            }
        }
        
        if self.auto_save {
            self.save_session().await?;
        }
        
        Ok(turnkey_session)
    }
    
    /// Mark an API call in session metadata
    ///
    /// Args:
    ///     api_server: Option<&str> - API server used for the call
    pub async fn mark_api_call(&self, api_server: Option<&str>) {
        let mut guard = self.session.write().await;
        if let Some(session) = guard.as_mut() {
            session.mark_api_call();
            if let Some(server) = api_server {
                session.session_metadata.current_api_server = Some(server.to_string());
            }
        }
    }
    
    /// Get cookie header string for HTTP requests
    ///
    /// Returns:
    ///     Option<String>: Formatted cookie header if session exists
    pub async fn get_cookie_header(&self) -> Option<String> {
        let guard = self.session.read().await;
        guard.as_ref().map(|session| session.get_cookie_header())
    }
    
    /// Get access token for Authorization header
    ///
    /// Returns:
    ///     Option<String>: Access token if session exists
    pub async fn get_access_token(&self) -> Option<String> {
        let guard = self.session.read().await;
        guard.as_ref().map(|session| session.tokens.access_token.clone())
    }
    
    /// Get refresh token for token refresh
    ///
    /// Returns:
    ///     Option<String>: Refresh token if session exists
    pub async fn get_refresh_token(&self) -> Option<String> {
        let guard = self.session.read().await;
        guard.as_ref().map(|session| session.tokens.refresh_token.clone())
    }
    
    /// Get Turnkey session for wallet operations
    ///
    /// Returns:
    ///     Option<TurnkeySession>: Turnkey session if available
    pub async fn get_turnkey_session(&self) -> Option<TurnkeySession> {
        let guard = self.session.read().await;
        guard.as_ref().and_then(|session| session.turnkey_session.clone())
    }
    
    /// Save session to file
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn save_session(&self) -> Result<(), AuthError> {
        if let Some(storage_path) = &self.storage_path {
            let guard = self.session.read().await;
            if let Some(session) = guard.as_ref() {
                let json_data = serde_json::to_string_pretty(session)
                    .map_err(|e| AuthError::SerializationError(e))?;
                
                // Create parent directory if it doesn't exist
                if let Some(parent) = storage_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| AuthError::IoError(e))?;
                }
                
                fs::write(storage_path, json_data)
                    .map_err(|e| AuthError::IoError(e))?;
            }
        }
        Ok(())
    }
    
    /// Load session from file
    ///
    /// Returns:
    ///     Result<(), AuthError>: Success or error status
    pub async fn load_session(&self) -> Result<(), AuthError> {
        if let Some(storage_path) = &self.storage_path {
            let session = Self::load_session_from_file(storage_path)
                .map_err(|e| AuthError::IoError(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to load session: {}", e)
                )))?;
            
            let mut guard = self.session.write().await;
            *guard = Some(session);
        }
        Ok(())
    }
    
    /// Clear current session
    pub async fn clear_session(&self) {
        let mut guard = self.session.write().await;
        *guard = None;
        
        // Optionally delete file
        if self.auto_save && self.storage_path.is_some() {
            if let Some(storage_path) = &self.storage_path {
                let _ = fs::remove_file(storage_path);
            }
        }
    }
    
    /// Get session summary for debugging
    ///
    /// Returns:
    ///     String: Human-readable session summary
    pub async fn get_session_summary(&self) -> String {
        let guard = self.session.read().await;
        if let Some(session) = guard.as_ref() {
            let token_status = if session.tokens.is_expired() {
                "EXPIRED"
            } else if session.tokens.needs_refresh() {
                "NEEDS_REFRESH"
            } else {
                "VALID"
            };
            
            let cookies_status = if session.has_valid_cookies() {
                "PRESENT"
            } else {
                "MISSING"
            };
            
            let turnkey_status = if session.turnkey_session.is_some() {
                "ACTIVE"
            } else {
                "NOT_SET"
            };
            
            let session_age = session.session_metadata.session_age_minutes();
            let last_api_call = session.session_metadata
                .minutes_since_last_api_call()
                .map_or("NEVER".to_string(), |mins| format!("{}m ago", mins));
            
            format!(
                "Session: {} | Tokens: {} | Cookies: {} | Turnkey: {} | Age: {}m | Last API: {}",
                if session.is_valid() { "VALID" } else { "INVALID" },
                token_status,
                cookies_status,
                turnkey_status,
                session_age,
                last_api_call
            )
        } else {
            "No active session".to_string()
        }
    }
    
    /// Check if Turnkey integration is available
    ///
    /// Returns:
    ///     Result<bool, AuthError>: True if Turnkey is healthy and available
    pub async fn check_turnkey_health(&self) -> Result<bool, AuthError> {
        if let Some(turnkey_client) = &self.turnkey_client {
            turnkey_client.health_check().await
                .map_err(|e| AuthError::ApiError { message: format!("Turnkey health check failed: {}", e) })
        } else {
            Ok(false)
        }
    }
}