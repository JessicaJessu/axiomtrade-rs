# Session Management

This document covers the comprehensive session management system in axiomtrade-rs, which handles authentication state, persistence, and multi-session support.

## Overview

The `SessionManager` provides a centralized system for managing authentication sessions that include:

- JWT tokens (access and refresh)
- HTTP cookies for browser-like authentication
- Turnkey wallet integration sessions
- Session metadata and tracking
- Persistent storage capabilities

## Session Lifecycle

### 1. Session Creation

Sessions are created after successful authentication through multiple pathways:

```rust
use axiomtrade_rs::auth::{SessionManager, AuthTokens, UserInfo};
use std::path::PathBuf;

// Create session manager with persistent storage
let storage_path = Some(PathBuf::from(".axiom_session.json"));
let session_manager = SessionManager::new(storage_path, true);

// Method 1: Basic session creation
let tokens = AuthTokens {
    access_token: "jwt_access_token".to_string(),
    refresh_token: "jwt_refresh_token".to_string(),
    expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
};

session_manager.create_session(tokens, user_info, cookies).await?;

// Method 2: Session from complete login result
let login_result = auth_client.login_with_otp(&credentials, &otp_code).await?;
session_manager.create_session_from_login_result(login_result, cookies).await?;
```

### 2. Session Validation

The session manager provides multiple validation levels:

```rust
// Check if session exists and is valid
if session_manager.is_session_valid().await {
    println!("Session is active and valid");
}

// Check if session needs token refresh
if session_manager.needs_refresh().await {
    println!("Session needs token refresh");
    // Trigger token refresh process
}

// Get detailed session summary
let summary = session_manager.get_session_summary().await;
println!("Session status: {}", summary);
```

### 3. Session Updates

Sessions are automatically updated during API operations:

```rust
// Update tokens after refresh
let new_tokens = auth_client.refresh_tokens().await?;
session_manager.update_tokens(new_tokens).await?;

// Update cookies from API responses
let new_cookies = AuthCookies::parse_from_headers(&response.headers());
session_manager.update_cookies(new_cookies).await?;

// Track API calls
session_manager.mark_api_call(Some("api.axiom.trade")).await;
```

### 4. Session Termination

Sessions can be cleared manually or automatically:

```rust
// Manual session cleanup
session_manager.clear_session().await;

// Automatic cleanup on token expiration
// Sessions become invalid when tokens expire beyond refresh window
```

## Session Persistence

### Automatic Persistence

When `auto_save` is enabled, sessions are automatically saved after modifications:

```rust
// Enable auto-save during manager creation
let session_manager = SessionManager::new(
    Some(PathBuf::from(".axiom_session.json")),
    true  // auto_save enabled
);

// All session updates are automatically persisted
session_manager.update_tokens(new_tokens).await?;  // Auto-saved
session_manager.update_cookies(new_cookies).await?;  // Auto-saved
```

### Manual Persistence

For fine-grained control over when sessions are saved:

```rust
// Disable auto-save for manual control
let session_manager = SessionManager::new(storage_path, false);

// Make multiple changes
session_manager.update_tokens(new_tokens).await?;
session_manager.update_cookies(new_cookies).await?;

// Save manually when ready
session_manager.save_session().await?;
```

### Session Loading

Sessions are automatically loaded from storage on manager creation:

```rust
// Sessions are loaded automatically if storage file exists
let session_manager = SessionManager::new(
    Some(PathBuf::from(".axiom_session.json")),
    true
);

// Manual loading (if needed)
session_manager.load_session().await?;
```

## Multi-Session Support

### Session Isolation

Each `SessionManager` instance manages one authentication session:

```rust
// Multiple isolated sessions
let trading_session = SessionManager::new(
    Some(PathBuf::from(".axiom_trading_session.json")),
    true
);

let monitoring_session = SessionManager::new(
    Some(PathBuf::from(".axiom_monitoring_session.json")),
    true
);

// Each session maintains independent state
trading_session.create_session(trading_tokens, None, None).await?;
monitoring_session.create_session(monitoring_tokens, None, None).await?;
```

### Session Switching

For applications needing multiple concurrent sessions:

```rust
use std::collections::HashMap;

struct MultiSessionManager {
    sessions: HashMap<String, SessionManager>,
    active_session: Option<String>,
}

impl MultiSessionManager {
    pub async fn switch_session(&mut self, session_id: &str) -> Result<(), AuthError> {
        if self.sessions.contains_key(session_id) {
            self.active_session = Some(session_id.to_string());
            Ok(())
        } else {
            Err(AuthError::NotAuthenticated)
        }
    }
    
    pub async fn get_active_session(&self) -> Option<&SessionManager> {
        self.active_session
            .as_ref()
            .and_then(|id| self.sessions.get(id))
    }
}
```

## Session Validation

### Token Validation

The session manager implements intelligent token validation:

```rust
impl AuthTokens {
    // Checks if token is expired with 5-minute buffer
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => {
                let buffer = chrono::Duration::minutes(5);
                chrono::Utc::now() >= (expires_at - buffer)
            }
            None => false,
        }
    }
    
    // Checks if token needs refresh with 15-minute buffer
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
```

### Cookie Validation

HTTP cookies are validated for completeness:

```rust
impl AuthSession {
    pub fn has_valid_cookies(&self) -> bool {
        self.cookies.auth_access_token.is_some() 
            && self.cookies.auth_refresh_token.is_some()
    }
}
```

### Turnkey Session Validation

Turnkey integration sessions have separate validation:

```rust
impl AuthSession {
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
}
```

## Logout and Cleanup

### Immediate Cleanup

Clear session data immediately:

```rust
// Clear session from memory and optionally delete storage file
session_manager.clear_session().await;

// Session is now invalid
assert!(!session_manager.is_session_valid().await);
```

### Secure Cleanup

For security-sensitive applications:

```rust
// 1. Revoke tokens on server (if supported)
if let Some(refresh_token) = session_manager.get_refresh_token().await {
    auth_client.revoke_token(&refresh_token).await?;
}

// 2. Clear local session
session_manager.clear_session().await;

// 3. Clear any cached credentials
// (Application-specific cleanup)
```

### Automatic Cleanup

Sessions automatically become invalid when tokens expire:

```rust
// Sessions with expired tokens return false
let is_valid = session_manager.is_session_valid().await;

// Cleanup expired sessions periodically
async fn cleanup_expired_sessions(managers: &[SessionManager]) {
    for manager in managers {
        if !manager.is_session_valid().await {
            manager.clear_session().await;
        }
    }
}
```

## Session Metadata

### Tracking Information

Sessions include comprehensive metadata:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_refreshed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_api_call_at: Option<chrono::DateTime<chrono::Utc>>,
    pub current_api_server: Option<String>,
    pub user_agent: String,
    pub ip_address: Option<String>,
    pub client_fingerprint: Option<String>,
}

// Usage examples
let session = session_manager.get_session().await.unwrap();
let age_minutes = session.session_metadata.session_age_minutes();
let last_api_call = session.session_metadata.minutes_since_last_api_call();
```

### Usage Analytics

Track session usage patterns:

```rust
// Mark API calls for analytics
session_manager.mark_api_call(Some("api.axiom.trade")).await;

// Get session summary with timing information
let summary = session_manager.get_session_summary().await;
// Output: "Session: VALID | Tokens: VALID | Cookies: PRESENT | Turnkey: ACTIVE | Age: 45m | Last API: 2m ago"
```

## Integration Examples

### HTTP Client Integration

Use sessions with HTTP requests:

```rust
use reqwest::Client;

async fn make_authenticated_request(
    session_manager: &SessionManager,
    client: &Client,
    url: &str,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let mut request = client.get(url);
    
    // Add authorization header
    if let Some(token) = session_manager.get_access_token().await {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    // Add cookie header
    if let Some(cookies) = session_manager.get_cookie_header().await {
        request = request.header("Cookie", cookies);
    }
    
    // Add user agent from session
    if let Some(session) = session_manager.get_session().await {
        request = request.header("User-Agent", session.get_user_agent());
    }
    
    let response = request.send().await?;
    
    // Track API call
    session_manager.mark_api_call(Some("api.axiom.trade")).await;
    
    Ok(response)
}
```

### WebSocket Integration

Use sessions for WebSocket authentication:

```rust
use tokio_tungstenite::{connect_async, tungstenite::Message};

async fn connect_websocket(
    session_manager: &SessionManager,
) -> Result<(), Box<dyn std::error::Error>> {
    if !session_manager.is_session_valid().await {
        return Err("Invalid session for WebSocket connection".into());
    }
    
    let access_token = session_manager.get_access_token().await
        .ok_or("No access token available")?;
    
    let ws_url = format!("wss://ws.axiom.trade?token={}", access_token);
    let (ws_stream, _) = connect_async(&ws_url).await?;
    
    // WebSocket connection established with session authentication
    Ok(())
}
```

### Error Handling

Handle session-related errors gracefully:

```rust
use axiomtrade_rs::auth::error::AuthError;

async fn handle_session_error(
    result: Result<(), AuthError>,
    session_manager: &SessionManager,
) -> Result<(), AuthError> {
    match result {
        Err(AuthError::NotAuthenticated) => {
            // Session invalid, clear and require re-authentication
            session_manager.clear_session().await;
            Err(AuthError::NotAuthenticated)
        }
        Err(AuthError::TokenExpired) => {
            // Try to refresh tokens
            if session_manager.needs_refresh().await {
                // Implement token refresh logic
                Ok(())
            } else {
                session_manager.clear_session().await;
                Err(AuthError::TokenExpired)
            }
        }
        other => other,
    }
}
```

## Best Practices

### Security

1. **Use HTTPS only** - Never transmit session data over unencrypted connections
2. **Secure storage** - Store session files with appropriate file permissions
3. **Token rotation** - Refresh tokens before expiration
4. **Cleanup on exit** - Clear sessions when application terminates

### Performance

1. **Auto-save carefully** - Consider performance impact of frequent saves
2. **Session reuse** - Reuse sessions across requests instead of recreating
3. **Batch updates** - Disable auto-save for bulk operations, save manually
4. **Memory management** - Clear unused sessions promptly

### Reliability

1. **Handle network failures** - Implement retry logic for session operations
2. **Validate before use** - Always check session validity before API calls
3. **Graceful degradation** - Handle missing or corrupted session files
4. **Monitor session health** - Track session age and usage patterns

This comprehensive session management system provides robust, secure, and efficient handling of authentication state for the axiomtrade-rs client library.
