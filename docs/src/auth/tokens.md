# Token Management

The Axiom Trade Rust client provides a comprehensive token management system that handles authentication tokens, automatic refresh, persistent storage, and validation. This system ensures secure and reliable API access while maintaining session integrity.

## Overview

The token management system consists of two main components:

- **AuthTokens**: Core JWT token structure containing access and refresh tokens
- **TokenManager**: Thread-safe manager for storing, validating, and persisting tokens

## Token Types

### Access Tokens

Access tokens are JWT tokens used to authenticate API requests. They have a limited lifespan and are included in the `Authorization` header of HTTP requests.

```rust
// Access token is automatically included in API requests
let balance = client.get_portfolio().await?;
```

### Refresh Tokens

Refresh tokens are long-lived tokens used to obtain new access tokens when they expire. They provide a secure way to maintain authentication without requiring the user to log in repeatedly.

```rust
// Refresh happens automatically when needed
let new_tokens = auth_client.refresh_tokens().await?;
```

## Token Structure

The `AuthTokens` struct contains all necessary token information:

```rust
pub struct AuthTokens {
    pub access_token: String,      // JWT access token
    pub refresh_token: String,     // JWT refresh token  
    pub expires_at: Option<DateTime<Utc>>, // Token expiration time
}
```

## TokenManager

The `TokenManager` provides thread-safe token operations with optional persistent storage.

### Creating a TokenManager

```rust
use axiomtrade_rs::auth::TokenManager;
use std::path::PathBuf;

// In-memory token storage
let token_manager = TokenManager::new(None);

// Persistent token storage
let storage_path = PathBuf::from("tokens.json");
let token_manager = TokenManager::new(Some(storage_path));

// Create from environment variables
let token_manager = TokenManager::from_env()?;
```

### Environment Variable Setup

The TokenManager can automatically load tokens from environment variables:

```bash
export AXIOM_ACCESS_TOKEN="your_access_token_here"
export AXIOM_REFRESH_TOKEN="your_refresh_token_here"
```

```rust
// Automatically loads tokens from environment
if let Some(manager) = TokenManager::from_env()? {
    println!("Tokens loaded from environment");
}
```

## Token Operations

### Setting Tokens

Store new authentication tokens in the manager:

```rust
use axiomtrade_rs::auth::types::AuthTokens;
use chrono::{Utc, Duration};

let tokens = AuthTokens {
    access_token: "eyJhbGciOiJIUzI1NiIs...".to_string(),
    refresh_token: "refresh_token_value".to_string(),
    expires_at: Some(Utc::now() + Duration::hours(1)),
};

token_manager.set_tokens(tokens).await?;
```

### Retrieving Tokens

Get stored tokens for API requests:

```rust
// Get complete token structure
if let Some(tokens) = token_manager.get_tokens().await {
    println!("Access token: {}", tokens.access_token);
}

// Get individual tokens
let access_token = token_manager.get_access_token().await?;
let refresh_token = token_manager.get_refresh_token().await?;
```

### Token Validation

Check token status before making API calls:

```rust
// Check if tokens are expired (with 5-minute buffer)
if token_manager.is_expired().await {
    println!("Tokens have expired");
}

// Check if tokens need refresh (with 15-minute buffer)
if token_manager.needs_refresh().await {
    println!("Tokens should be refreshed soon");
}
```

### Clearing Tokens

Remove stored tokens and delete persistent storage:

```rust
// Clear tokens from memory and delete storage file
token_manager.clear().await?;
```

## Token Storage and Persistence

### File-Based Storage

When a storage path is provided, tokens are automatically saved to disk as JSON:

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "refresh_token_value",
  "expires_at": "2024-01-15T10:30:00Z"
}
```

### Storage Benefits

- **Persistence**: Tokens survive application restarts
- **Security**: Only stored locally on the filesystem
- **Convenience**: Automatic loading on TokenManager creation

### Storage Location

Choose an appropriate storage location based on your application:

```rust
// User-specific storage
let storage_path = dirs::config_dir()
    .unwrap()
    .join("axiomtrade")
    .join("tokens.json");

// Application-specific storage  
let storage_path = PathBuf::from("./config/tokens.json");

// Temporary storage
let storage_path = std::env::temp_dir().join("axiom_tokens.json");
```

## Automatic Token Refresh

The token management system provides intelligent refresh logic:

### Refresh Timing

- **Expiration Buffer**: Tokens are considered expired 5 minutes before actual expiration
- **Refresh Buffer**: Tokens should be refreshed 15 minutes before expiration
- **Automatic Refresh**: The client automatically refreshes tokens when needed

### Refresh Implementation

```rust
// Manual refresh check and execution
if token_manager.needs_refresh().await {
    // Refresh logic would be implemented in the auth client
    let new_tokens = auth_client.refresh_tokens().await?;
    token_manager.set_tokens(new_tokens).await?;
}
```

## Security Considerations

### Token Protection

- **Local Storage Only**: Tokens are never transmitted except for authentication
- **File Permissions**: Ensure token files have appropriate read/write permissions
- **Environment Variables**: Use secure environment variable management

### Best Practices

1. **Regular Rotation**: Implement token refresh before expiration
2. **Secure Storage**: Use appropriate file permissions for token storage
3. **Error Handling**: Always handle token-related errors gracefully
4. **Cleanup**: Clear tokens on logout or application termination

```rust
// Secure token file permissions (Unix-like systems)
#[cfg(unix)]
fn set_token_file_permissions(path: &Path) -> Result<(), std::io::Error> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o600); // Read/write for owner only
    fs::set_permissions(path, perms)?;
    Ok(())
}
```

## Error Handling

The token management system uses structured error handling:

```rust
use axiomtrade_rs::auth::error::AuthError;

match token_manager.get_access_token().await {
    Ok(token) => {
        // Use token for API request
    }
    Err(AuthError::TokenNotFound) => {
        // Handle missing tokens - may need to log in
    }
    Err(AuthError::TokenExpired) => {
        // Handle expired tokens - refresh or re-authenticate
    }
    Err(e) => {
        // Handle other errors
        eprintln!("Token error: {}", e);
    }
}
```

## Thread Safety

The TokenManager is designed for concurrent access:

```rust
use std::sync::Arc;

// Share TokenManager across threads
let manager = Arc::new(TokenManager::new(None));
let manager_clone = Arc::clone(&manager);

tokio::spawn(async move {
    // Safe concurrent access
    let tokens = manager_clone.get_tokens().await;
});
```

## Integration Example

Complete example showing token management integration:

```rust
use axiomtrade_rs::auth::{TokenManager, AuthClient};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create token manager with persistent storage
    let storage_path = PathBuf::from("tokens.json");
    let token_manager = TokenManager::new(Some(storage_path));
    
    // Check if we have valid tokens
    if token_manager.is_expired().await {
        // Need to authenticate
        let auth_client = AuthClient::new();
        let login_result = auth_client.login("user@example.com", "password").await?;
        token_manager.set_tokens(login_result.tokens).await?;
    } else if token_manager.needs_refresh().await {
        // Refresh tokens proactively
        let auth_client = AuthClient::new();
        let new_tokens = auth_client.refresh_tokens().await?;
        token_manager.set_tokens(new_tokens).await?;
    }
    
    // Use tokens for API requests
    let access_token = token_manager.get_access_token().await?;
    println!("Ready to make authenticated requests");
    
    Ok(())
}
```

## Advanced Usage

### Token Lifecycle Management

```rust
// Complete token lifecycle management
pub struct TokenLifecycleManager {
    token_manager: TokenManager,
    auth_client: AuthClient,
}

impl TokenLifecycleManager {
    pub async fn ensure_valid_tokens(&self) -> Result<(), AuthError> {
        if self.token_manager.is_expired().await {
            // Re-authenticate required
            return Err(AuthError::TokenExpired);
        }
        
        if self.token_manager.needs_refresh().await {
            // Proactive refresh
            let new_tokens = self.auth_client.refresh_tokens().await?;
            self.token_manager.set_tokens(new_tokens).await?;
        }
        
        Ok(())
    }
    
    pub async fn get_valid_access_token(&self) -> Result<String, AuthError> {
        self.ensure_valid_tokens().await?;
        self.token_manager.get_access_token().await
    }
}
```

## Migration and Backup

### Token Migration

```rust
// Migrate tokens between storage locations
async fn migrate_tokens(
    old_path: &Path, 
    new_path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    let old_manager = TokenManager::new(Some(old_path.to_path_buf()));
    
    if let Some(tokens) = old_manager.get_tokens().await {
        let new_manager = TokenManager::new(Some(new_path.to_path_buf()));
        new_manager.set_tokens(tokens).await?;
        old_manager.clear().await?;
    }
    
    Ok(())
}
```

### Token Backup

```rust
// Create token backup
async fn backup_tokens(
    manager: &TokenManager,
    backup_path: &Path
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(tokens) = manager.get_tokens().await {
        let backup_manager = TokenManager::new(Some(backup_path.to_path_buf()));
        backup_manager.set_tokens(tokens).await?;
    }
    Ok(())
}
```

The token management system provides a robust foundation for maintaining authenticated sessions with the Axiom Trade API while ensuring security, reliability, and ease of use.