# Turnkey Integration

The Turnkey integration provides enterprise-grade hardware wallet management with P256 cryptographic operations for secure trading on Axiom Trade. This integration handles key management, authentication, and secure signing through Turnkey's infrastructure.

## Overview

Turnkey is a secure wallet infrastructure that provides:
- Hardware-level key security
- P256 elliptic curve cryptography
- API key management
- Session-based authentication
- Enterprise-grade access controls

## Turnkey Setup

### Prerequisites

Before using the Turnkey integration, ensure you have:

1. **Turnkey Organization Account**: A registered organization on Turnkey
2. **User Credentials**: Valid user account within the organization
3. **Environment Variables**: Properly configured authentication details

### Initial Configuration

The Turnkey client requires minimal setup:

```rust
use axiomtrade_rs::api::turnkey::TurnkeyClient;

// Create a new Turnkey client
let mut turnkey_client = TurnkeyClient::new();

// Set credentials for authenticated operations
turnkey_client.set_credentials(
    organization_id,
    user_id,
    password  // Raw password for P256 key derivation
);
```

### Session Management

Turnkey sessions are automatically managed through the authentication flow:

```rust
// Load existing session from file
let session_content = std::fs::read_to_string(".axiom_turnkey_session.json")?;
let session: AuthSession = serde_json::from_str(&session_content)?;

if let Some(turnkey_session) = session.turnkey_session {
    // Use existing session
    println!("Organization: {}", turnkey_session.organization_id);
    println!("User: {}", turnkey_session.username);
}
```

## Hardware Wallet Authentication

### Authentication Flow

The Turnkey authentication process involves several steps:

1. **Identity Verification**: Confirm user identity with the organization
2. **API Key Retrieval**: Fetch available API keys for operations
3. **Session Creation**: Establish secure session for trading operations

#### Step 1: Identity Verification

```rust
// Verify user identity
let whoami = turnkey_client.whoami(
    &organization_id,
    &client_secret
).await?;

println!("Authenticated as: {}", whoami.username);
println!("Organization: {}", whoami.organization_name);
```

#### Step 2: API Key Management

```rust
// Retrieve API keys for the user
let api_keys = turnkey_client.get_api_keys(
    &user_id,
    &organization_id,
    &client_secret
).await?;

println!("Available API keys: {}", api_keys.api_keys.len());

for key in &api_keys.api_keys {
    println!("Key: {} (Type: {})", key.api_key_name, key.credential.credential_type);
}
```

#### Step 3: Session Creation

```rust
// Create read/write session for trading operations
let success = turnkey_client.create_read_write_session(
    organization_id,
    user_id,
    target_public_key,
    "trading-session"
).await?;

if success {
    println!("Trading session created successfully");
}
```

### Session Validation

The client provides methods to validate and monitor session health:

```rust
// Check service health
let is_healthy = turnkey_client.health_check().await?;

// Parse and validate session
let parsed_session = turnkey_client.parse_session(
    &whoami,
    &api_keys,
    &client_secret
);

// Get session summary
let summary = turnkey_client.session_summary(&parsed_session);
println!("Session status: {}", summary);
```

## P256 Cryptography

### Key Generation

The integration uses P256 elliptic curve cryptography for secure operations:

```rust
use axiomtrade_rs::utils::p256_crypto;

// Generate P256 keypair from password
let keypair = p256_crypto::generate_p256_keypair_from_password(
    password,
    None  // Random salt, or provide specific salt
)?;

println!("Public key: {}", keypair.public_key);
println!("Private key length: {}", keypair.private_key.len());
```

### Deterministic Key Recreation

Keys can be recreated deterministically using the client secret:

```rust
// Recreate keypair from stored client secret
let keypair = p256_crypto::recreate_keypair_from_client_secret(
    password,
    client_secret
)?;

// Keys will be identical to original generation
```

### Key Security Features

- **PBKDF2 with SHA256**: 600,000 iterations for key derivation
- **P256 Curve**: NIST P-256 elliptic curve (secp256r1)
- **Compressed Public Keys**: Space-efficient key representation
- **Secure Salt Generation**: Cryptographically secure random salts

## Secure Signing

### Message Signing

The client handles request signing automatically:

```rust
// Sign arbitrary message
let message = b"transaction_data";
let signature = p256_crypto::sign_message(message, &private_key)?;

println!("Signature length: {} bytes", signature.len());
println!("Format: DER encoded");
```

### WebAuthn Compatible Signing

For WebAuthn and browser-compatible operations:

```rust
// Generate WebAuthn-compatible signature (raw r,s format)
let webauthn_signature = p256_crypto::sign_message_webauthn(
    message,
    &private_key
)?;

// Signature is exactly 64 bytes (32-byte r + 32-byte s)
assert_eq!(webauthn_signature.len(), 64);
```

### Signature Verification

```rust
// Verify signature authenticity
let is_valid = p256_crypto::verify_signature(
    message,
    &signature,
    &public_key
)?;

if is_valid {
    println!("Signature verified successfully");
}
```

### Request Authentication

Turnkey requests are automatically signed with the X-Stamp header:

```rust
// Internal signing process (handled automatically)
let signature = self.sign_request(payload_json.as_bytes(), client_secret)?;

// Headers include authentication stamp
.header("X-Stamp", signature)
.header("x-client-version", "@turnkey/sdk-server@1.7.3")
```

## API Methods

### Core Operations

#### Identity and Authentication

```rust
// Get current user identity
let whoami: TurnkeyWhoAmI = turnkey_client.whoami(
    organization_id,
    client_secret
).await?;

// Fields available:
// - organization_id: String
// - organization_name: String  
// - user_id: String
// - username: String
```

#### API Key Management

```rust
// Retrieve all API keys for user
let api_keys: GetApiKeysResponse = turnkey_client.get_api_keys(
    user_id,
    organization_id,
    client_secret
).await?;

// Access key information
for key in api_keys.api_keys {
    println!("Key ID: {}", key.api_key_id);
    println!("Name: {}", key.api_key_name);
    println!("Public Key: {}", key.credential.public_key);
    println!("Type: {}", key.credential.credential_type);
}
```

#### Session Management

```rust
// Create secure session for operations
let success: bool = turnkey_client.create_read_write_session(
    organization_id,
    user_id,
    target_public_key,
    api_key_name
).await?;

// Sessions expire after 30 days by default
```

### Utility Methods

#### Session Information

```rust
// Get specific API key by type
let session_key = turnkey_client.get_session_key(
    &session,
    "CREDENTIAL_TYPE_READ_WRITE_SESSION_KEY_P256"
);

// Generate human-readable session summary
let summary = turnkey_client.session_summary(&session);
println!("{}", summary);
// Output: "Turnkey Session - User: alice, Keys: 2/3 active, Age: 45m, Expires: 120m"
```

#### Health Monitoring

```rust
// Check Turnkey service availability
let is_healthy: bool = turnkey_client.health_check().await?;

if !is_healthy {
    println!("Turnkey service may be experiencing issues");
}
```

### Error Handling

The integration provides comprehensive error handling:

```rust
match turnkey_client.whoami(org_id, client_secret).await {
    Ok(whoami) => {
        println!("Authentication successful: {}", whoami.username);
    }
    Err(AxiomError::Api { message, .. }) if message.contains("PUBLIC_KEY_NOT_FOUND") => {
        println!("Public key not registered with Turnkey organization");
    }
    Err(AxiomError::Api { message, .. }) if message.contains("unauthorized") => {
        println!("Invalid authentication credentials");
    }
    Err(AxiomError::Network { .. }) => {
        println!("Network connectivity issue");
    }
    Err(e) => {
        println!("Unexpected error: {}", e);
    }
}
```

## Integration with Axiom Trade

### Session Storage

Turnkey sessions are integrated with the main authentication system:

```rust
// Sessions are stored in .axiom_turnkey_session.json
let auth_session = AuthSession {
    tokens: auth_tokens,
    cookies: auth_cookies,
    turnkey_session: Some(turnkey_session),
    user_info: user_info,
    session_metadata: metadata,
};
```

### Automatic Session Management

The client automatically handles:
- **Session expiration checking**
- **Key rotation**
- **Error recovery**
- **Health monitoring**

### Trading Integration

Turnkey sessions enable secure trading operations:

```rust
// Session provides cryptographic signing for trades
if let Some(turnkey) = &session.turnkey_session {
    // Use Turnkey session for secure trade signing
    let trade_signature = sign_trade_request(&trade_data, &turnkey.client_secret)?;
}
```

## Security Considerations

### Key Security

- **Hardware-backed**: Keys are protected by Turnkey's hardware infrastructure
- **Never exposed**: Private keys never leave the secure environment
- **P256 cryptography**: Industry-standard elliptic curve cryptography
- **Secure derivation**: PBKDF2 with high iteration count

### Session Security

- **Limited lifetime**: Sessions expire automatically (30 days default)
- **Activity tracking**: All operations are logged and monitored
- **Secure transmission**: All communications use TLS encryption
- **Authentication required**: Every request requires cryptographic signature

### Best Practices

1. **Store passwords securely**: Use OS keychain or secure storage
2. **Monitor session health**: Regular health checks and renewal
3. **Handle errors gracefully**: Implement proper error recovery
4. **Rotate keys regularly**: Follow organizational key rotation policies
5. **Validate responses**: Always verify API responses and signatures

## Troubleshooting

### Common Issues

#### Authentication Failures

```rust
// Handle common authentication issues
fn handle_auth_failure(error: AxiomError) -> Result<()> {
    match error {
        AxiomError::Api { message, .. } if message.contains("PUBLIC_KEY_NOT_FOUND") => {
            println!("Solution: Ensure public key is registered in Turnkey organization");
        }
        AxiomError::Api { message, .. } if message.contains("unauthorized") => {
            println!("Solution: Check password and client secret are correct");
        }
        AxiomError::Network { .. } => {
            println!("Solution: Check internet connection");
        }
        _ => {
            println!("Unknown error: {}", error);
        }
    }
    Ok(())
}
```

#### Session Issues

- **Expired sessions**: Re-authenticate to refresh session
- **Invalid keys**: Verify key registration in Turnkey organization
- **Network issues**: Check connectivity and retry with backoff

#### Key Generation Problems

- **Invalid salt**: Ensure client secret is properly base64 encoded
- **Wrong password**: Verify password matches original key generation
- **Curve validation**: Ensure derived key is valid for P256 curve

### Debug Information

Enable detailed logging for troubleshooting:

```rust
// Get comprehensive session information
let summary = turnkey_client.session_summary(&session);
println!("Debug: {}", summary);

// Check individual key status
for key in &session.api_keys {
    if let Some(expires_at) = key.expires_at {
        let remaining = expires_at - chrono::Utc::now();
        println!("Key {} expires in {} minutes", key.api_key_name, remaining.num_minutes());
    }
}
```

This integration provides enterprise-grade security for cryptocurrency trading operations while maintaining ease of use and robust error handling.
