use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enhanced authentication session with all auth-related data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    /// JWT tokens
    pub tokens: AuthTokens,
    /// HTTP cookies for session persistence
    pub cookies: AuthCookies,
    /// Turnkey wallet management session
    pub turnkey_session: Option<TurnkeySession>,
    /// User information
    pub user_info: Option<UserInfo>,
    /// Session metadata
    pub session_metadata: SessionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// HTTP cookies used for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCookies {
    /// Main authentication cookie (HttpOnly)
    pub auth_access_token: Option<String>,
    /// Refresh token cookie (HttpOnly)
    pub auth_refresh_token: Option<String>,
    /// Google state cookie
    pub g_state: Option<String>,
    /// Additional session cookies
    pub additional_cookies: HashMap<String, String>,
}

/// Turnkey session information for secure wallet operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnkeySession {
    /// Turnkey organization ID
    pub organization_id: String,
    /// Turnkey user ID
    pub user_id: String,
    /// Username on Turnkey
    pub username: String,
    /// Client secret for P256 key derivation
    pub client_secret: String,
    /// Active API keys
    pub api_keys: Vec<TurnkeyApiKey>,
    /// Session creation time
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Session expiration time
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Turnkey API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnkeyApiKey {
    pub api_key_id: String,
    pub api_key_name: String,
    pub public_key: String,
    pub key_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Session metadata for tracking and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    /// When this session was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last time tokens were refreshed
    pub last_refreshed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Last successful API call
    pub last_api_call_at: Option<chrono::DateTime<chrono::Utc>>,
    /// API server currently being used
    pub current_api_server: Option<String>,
    /// User agent used for requests
    pub user_agent: String,
    /// IP address (if available)
    pub ip_address: Option<String>,
    /// Browser/client fingerprint
    pub client_fingerprint: Option<String>,
}

impl AuthSession {
    /// Create a new authentication session with random user agent
    pub fn new(tokens: AuthTokens, user_info: Option<UserInfo>) -> Self {
        Self {
            tokens,
            cookies: AuthCookies::default(),
            turnkey_session: None,
            user_info,
            session_metadata: SessionMetadata::new(),
        }
    }
    
    /// Create a new authentication session with specific user agent
    pub fn new_with_user_agent(tokens: AuthTokens, user_info: Option<UserInfo>, user_agent: String) -> Self {
        Self {
            tokens,
            cookies: AuthCookies::default(),
            turnkey_session: None,
            user_info,
            session_metadata: SessionMetadata::new_with_user_agent(user_agent),
        }
    }
    
    /// Get the user agent for this session
    pub fn get_user_agent(&self) -> &str {
        &self.session_metadata.user_agent
    }
    
    /// Check if the entire session is valid
    pub fn is_valid(&self) -> bool {
        !self.tokens.is_expired() && self.has_valid_cookies()
    }
    
    /// Check if session needs refresh
    pub fn needs_refresh(&self) -> bool {
        self.tokens.needs_refresh() || self.turnkey_needs_refresh()
    }
    
    /// Check if cookies are present and valid
    pub fn has_valid_cookies(&self) -> bool {
        self.cookies.auth_access_token.is_some() && self.cookies.auth_refresh_token.is_some()
    }
    
    /// Check if Turnkey session needs refresh
    pub fn turnkey_needs_refresh(&self) -> bool {
        if let Some(turnkey) = &self.turnkey_session {
            if let Some(expires_at) = turnkey.expires_at {
                let buffer = chrono::Duration::hours(1);
                chrono::Utc::now() >= (expires_at - buffer)
            } else {
                false
            }
        } else {
            false
        }
    }
    
    /// Update last API call timestamp
    pub fn mark_api_call(&mut self) {
        self.session_metadata.last_api_call_at = Some(chrono::Utc::now());
    }
    
    /// Update tokens and refresh timestamp
    pub fn update_tokens(&mut self, new_tokens: AuthTokens) {
        self.tokens = new_tokens;
        self.session_metadata.last_refreshed_at = Some(chrono::Utc::now());
    }
    
    /// Update Turnkey session
    pub fn update_turnkey_session(&mut self, turnkey_session: TurnkeySession) {
        self.turnkey_session = Some(turnkey_session);
    }
    
    /// Get cookies as a formatted string for HTTP headers
    pub fn get_cookie_header(&self) -> String {
        let mut cookies = Vec::new();
        
        if let Some(g_state) = &self.cookies.g_state {
            cookies.push(format!("g_state={}", g_state));
        }
        
        if let Some(refresh_token) = &self.cookies.auth_refresh_token {
            cookies.push(format!("auth-refresh-token={}", refresh_token));
        }
        
        if let Some(access_token) = &self.cookies.auth_access_token {
            cookies.push(format!("auth-access-token={}", access_token));
        }
        
        for (name, value) in &self.cookies.additional_cookies {
            cookies.push(format!("{}={}", name, value));
        }
        
        cookies.join("; ")
    }
}

impl AuthTokens {
    /// Check if token is expired (with 5 minute buffer)
    /// 
    /// # Returns
    /// 
    /// bool - True if token is expired or about to expire
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => {
                let buffer = chrono::Duration::minutes(5);
                chrono::Utc::now() >= (expires_at - buffer)
            }
            None => false,
        }
    }
    
    /// Check if token needs refresh (with 15 minute buffer)
    /// 
    /// # Returns
    /// 
    /// bool - True if token should be refreshed soon
    pub fn needs_refresh(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => {
                let buffer = chrono::Duration::minutes(15);
                chrono::Utc::now() >= (expires_at - buffer)
            }
            None => false,
        }
    }
}

impl Default for AuthCookies {
    fn default() -> Self {
        Self {
            auth_access_token: None,
            auth_refresh_token: None,
            g_state: Some("{\"i_l\":0}".to_string()),
            additional_cookies: HashMap::new(),
        }
    }
}

impl AuthCookies {
    /// Parse cookies from HTTP Set-Cookie headers
    pub fn parse_from_headers(headers: &reqwest::header::HeaderMap) -> Self {
        let mut cookies = Self::default();
        
        for header_value in headers.get_all(reqwest::header::SET_COOKIE) {
            if let Ok(header_str) = header_value.to_str() {
                let cookie_parts: Vec<&str> = header_str.split(';').collect();
                if let Some(name_value) = cookie_parts.first() {
                    let parts: Vec<&str> = name_value.split('=').collect();
                    if parts.len() == 2 {
                        let name = parts[0].trim();
                        let value = parts[1].trim();
                        
                        match name {
                            "auth-access-token" => cookies.auth_access_token = Some(value.to_string()),
                            "auth-refresh-token" => cookies.auth_refresh_token = Some(value.to_string()),
                            "g_state" => cookies.g_state = Some(value.to_string()),
                            _ => {
                                cookies.additional_cookies.insert(name.to_string(), value.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        cookies
    }
    
    /// Update cookies from another AuthCookies instance
    pub fn merge_with(&mut self, other: &AuthCookies) {
        if other.auth_access_token.is_some() {
            self.auth_access_token = other.auth_access_token.clone();
        }
        if other.auth_refresh_token.is_some() {
            self.auth_refresh_token = other.auth_refresh_token.clone();
        }
        if other.g_state.is_some() {
            self.g_state = other.g_state.clone();
        }
        
        for (key, value) in &other.additional_cookies {
            self.additional_cookies.insert(key.clone(), value.clone());
        }
    }
}

impl SessionMetadata {
    /// Create new session metadata with random realistic user agent
    pub fn new() -> Self {
        Self {
            created_at: chrono::Utc::now(),
            last_refreshed_at: None,
            last_api_call_at: None,
            current_api_server: None,
            user_agent: crate::utils::user_agents::get_random_desktop_user_agent().to_string(),
            ip_address: None,
            client_fingerprint: None,
        }
    }
    
    /// Create new session metadata with specific user agent
    pub fn new_with_user_agent(user_agent: String) -> Self {
        Self {
            created_at: chrono::Utc::now(),
            last_refreshed_at: None,
            last_api_call_at: None,
            current_api_server: None,
            user_agent,
            ip_address: None,
            client_fingerprint: None,
        }
    }
    
    /// Get session age in minutes
    pub fn session_age_minutes(&self) -> i64 {
        let now = chrono::Utc::now();
        (now - self.created_at).num_minutes()
    }
    
    /// Get time since last API call in minutes
    pub fn minutes_since_last_api_call(&self) -> Option<i64> {
        self.last_api_call_at.map(|last_call| {
            let now = chrono::Utc::now();
            (now - last_call).num_minutes()
        })
    }
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub b64_password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginStep1Response {
    pub otp_jwt_token: String,
}

#[derive(Debug, Serialize)]
pub struct OtpRequest {
    pub code: String,
    pub email: String,
    #[serde(rename = "b64Password")]
    pub b64_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    pub user: Option<UserInfo>,
    /// Turnkey organization ID (from /login-otp response)
    #[serde(rename = "orgId")]
    pub org_id: Option<String>,
    /// Turnkey user ID (from /login-otp response)
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    /// Client secret for Turnkey operations (from /login-otp response)
    #[serde(rename = "clientSecret")]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

/// Complete login result including tokens and Turnkey credentials
#[derive(Debug, Clone)]
pub struct LoginResult {
    pub tokens: AuthTokens,
    pub turnkey_credentials: Option<TurnkeyCredentials>,
    pub user_info: Option<UserInfo>,
}

/// Turnkey credentials extracted from login response
#[derive(Debug, Clone)]
pub struct TurnkeyCredentials {
    pub organization_id: String,
    pub user_id: String,
    pub client_secret: String,
}