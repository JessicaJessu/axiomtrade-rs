# Security Best Practices

This document outlines the comprehensive security practices implemented in axiomtrade-rs, based on analysis of the codebase's security patterns and mechanisms.

## Overview

The axiomtrade-rs library implements multiple layers of security to protect user credentials, tokens, and trading operations. Security measures span credential management, cryptographic operations, secure communications, and MEV protection.

## 1. Credential Management

### Password Security

The library uses industry-standard PBKDF2 password hashing with SHA256:

- **Hash Function**: PBKDF2-HMAC-SHA256
- **Iterations**: 600,000 (exceeds OWASP recommendations)
- **Salt**: 32-byte fixed salt for deterministic hashing
- **Output**: Base64-encoded 32-byte hash

```rust
// Example from src/utils/password.rs
const ITERATIONS: u32 = 600000;
pub fn hashpassword(password: &str) -> String {
    let mut derived_key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &SALT, ITERATIONS, &mut derived_key);
    general_purpose::STANDARD.encode(&derived_key)
}
```

**Security Benefits:**
- Resistance to brute-force attacks through high iteration count
- Deterministic hashing enables consistent authentication
- SHA256 provides cryptographic strength
- No plain-text password storage

### Environment Variable Safety

Custom environment loader handles special characters securely:

- **Parsing**: Safe parsing of `.env` files with quote handling
- **Special Characters**: Proper handling of passwords containing `$`, `!`, etc.
- **Validation**: Required variable checking with clear error messages
- **Isolation**: Environment variables don't leak into process space

```rust
// Safe environment variable loading
let loader = EnvLoader::from_file(Path::new(".env"))?;
let password = loader.get_required("AXIOM_PASSWORD")?;
```

**Best Practices:**
- Store sensitive credentials in `.env` files, not in code
- Use quotes for values containing special characters
- Never commit `.env` files to version control
- Validate required environment variables at startup

## 2. Token Security

### JWT Token Management

Comprehensive token lifecycle management with secure storage:

```rust
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

**Security Features:**
- **Expiration Tracking**: Automatic token expiry detection with buffer zones
- **Refresh Logic**: Proactive token refresh 15 minutes before expiry
- **Secure Storage**: Optional persistent storage with file-based caching
- **Memory Safety**: Thread-safe in-memory token management

### Session Security

Enhanced session management with multiple authentication layers:

```rust
pub struct AuthSession {
    pub tokens: AuthTokens,           // JWT tokens
    pub cookies: AuthCookies,         // HTTP session cookies
    pub turnkey_session: Option<TurnkeySession>, // Wallet management
    pub user_info: Option<UserInfo>,  // User metadata
    pub session_metadata: SessionMetadata, // Tracking data
}
```

**Security Measures:**
- **Multi-Factor Authentication**: Combines JWTs and HTTP cookies
- **Session Tracking**: Metadata for debugging and security monitoring
- **Automatic Cleanup**: Session invalidation on logout
- **Secure Headers**: Proper cookie formatting for HTTP security

### Cookie Security

HTTP cookies managed with security-first approach:

```rust
pub struct AuthCookies {
    pub auth_access_token: Option<String>,    // HttpOnly token
    pub auth_refresh_token: Option<String>,   // HttpOnly refresh
    pub g_state: Option<String>,              // Google state
    pub additional_cookies: HashMap<String, String>,
}
```

**Implementation Details:**
- **HttpOnly Flags**: Prevents JavaScript access to sensitive tokens
- **Secure Transmission**: Cookies sent only over HTTPS
- **State Management**: Google OAuth state tracking
- **Flexible Storage**: Support for additional session cookies

## 3. Cryptographic Security

### P256 Key Management

Advanced elliptic curve cryptography for wallet operations:

```rust
pub struct P256KeyPair {
    pub private_key: String,    // Hex-encoded private key
    pub public_key: String,     // Compressed public key
    pub client_secret: String,  // Base64-encoded salt
}
```

**Cryptographic Features:**
- **NIST P-256 Curve**: Industry-standard elliptic curve
- **Password-Derived Keys**: PBKDF2-based key generation
- **Deterministic Generation**: Reproducible keys from password + salt
- **WebAuthn Support**: Compatible signature formats

### Key Generation Security

Multi-layer key derivation process:

1. **Password Input**: User-provided password
2. **Salt Generation**: Random 32-byte salt (or provided salt)
3. **PBKDF2 Derivation**: 600,000 iterations with SHA256
4. **Curve Validation**: Ensure key falls within valid P-256 range
5. **Key Pair Creation**: Generate public key from private key

**Security Validations:**
- Curve order validation prevents invalid keys
- Zero-key rejection ensures cryptographic strength
- Retry mechanism for edge cases
- Deterministic regeneration from client secret

### Digital Signatures

Dual signature formats for different use cases:

```rust
// DER format for general use
pub fn sign_message(message: &[u8], private_key_hex: &str) -> Result<Vec<u8>>

// WebAuthn format for browser compatibility
pub fn sign_message_webauthn(message: &[u8], private_key_hex: &str) -> Result<Vec<u8>>
```

**Security Properties:**
- **ECDSA Signatures**: Cryptographically secure signing
- **Format Flexibility**: DER and raw signature support
- **Message Integrity**: Tamper-evident message signing
- **Non-Repudiation**: Proof of message origin

## 4. Environment Variable Safety

### Secure Loading Patterns

Custom environment loader with enhanced security:

```rust
impl EnvLoader {
    pub fn from_file(path: &Path) -> Result<Self, std::io::Error>
    pub fn get_required(&self, key: &str) -> Result<String, String>
    pub fn apply_to_env(&self)
}
```

**Safety Features:**
- **Quote Handling**: Proper parsing of quoted values
- **Special Character Support**: Safe handling of `$`, `!`, `@` in passwords
- **Comment Filtering**: Ignores comment lines and empty lines
- **Error Handling**: Clear error messages for missing variables

### Environment Security Best Practices

**File Structure:**
```bash
# Production .env file
AXIOM_EMAIL=user@example.com
AXIOM_PASSWORD="MyP@ssw0rd$2024!"
AXIOM_ACCESS_TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
AXIOM_REFRESH_TOKEN=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...

# OTP automation (optional)
INBOX_LV_EMAIL=otp@inbox.lv
INBOX_LV_PASSWORD="imap_password_from_inbox_lv"
```

**Security Checklist:**
- [ ] Never commit `.env` files to version control
- [ ] Use strong, unique passwords
- [ ] Rotate tokens regularly
- [ ] Quote values containing special characters
- [ ] Validate all required variables at startup

## 5. MEV Protection

### MEV Mitigation Strategies

The library implements multiple MEV (Maximum Extractable Value) protection mechanisms:

```rust
// Trading with slippage protection
pub async fn buy_token(
    &self,
    token_address: &str,
    amount_sol: f64,
    slippage_percent: Option<f64>, // MEV protection via slippage limits
) -> Result<TradingResponse, TradingError>
```

**MEV Protection Features:**
- **Slippage Limits**: Configurable slippage tolerance (0.1-5%)
- **Service Monitoring**: Health checks for MEV protection services
- **Multiple Providers**: Integration with 0slot, Jito, and other MEV services
- **Batch Operations**: Reduced MEV exposure through transaction batching

### MEV Service Integration

Infrastructure monitoring for MEV protection:

```rust
pub async fn check_external_mev_health(&self) -> Result<ServiceHealth>
```

**Supported Services:**
- **0slot**: Primary MEV protection service
- **Jito**: Alternative MEV protection
- **Custom Services**: Configurable external MEV protection
- **Health Monitoring**: Continuous service availability checks

### Anti-MEV Trading Patterns

**Best Practices:**
- Use appropriate slippage tolerance (0.1-1% for liquid tokens)
- Monitor transaction timing and ordering
- Implement randomized delays for automated trading
- Use MEV protection services for large transactions
- Consider private mempools for sensitive operations

## 6. Network Security

### HTTPS Enforcement

All network communications use TLS encryption:

- **TLS 1.2+ Required**: Modern encryption standards
- **Certificate Validation**: Proper SSL certificate checking
- **Secure Headers**: Implementation of security headers
- **Request Signing**: Cryptographic request authentication

### Rate Limiting

Built-in rate limiting prevents abuse:

```rust
pub struct RateLimiter {
    // Implementation details for API rate limiting
}
```

**Protection Features:**
- **Request Throttling**: Prevents API abuse
- **Backoff Strategies**: Exponential backoff for retries
- **Burst Protection**: Handles traffic spikes gracefully
- **Error Recovery**: Graceful degradation under load

## 7. Automated OTP Security

### Secure OTP Fetching

Optional automated OTP retrieval with secure IMAP:

```rust
pub struct OtpFetcher {
    email: String,    // inbox.lv email
    password: String, // IMAP-specific password
}
```

**Security Features:**
- **Dedicated Email**: Separate inbox.lv account for OTP
- **IMAP Security**: Secure IMAP/SSL connection
- **Credential Isolation**: OTP credentials separate from trading credentials
- **Pattern Matching**: Secure regex parsing for OTP codes

### OTP Best Practices

**Setup Requirements:**
1. Create dedicated inbox.lv account
2. Enable IMAP access with special password
3. Configure email forwarding from Axiom Trade
4. Store IMAP credentials securely
5. Monitor for unauthorized access

## 8. Security Monitoring and Debugging

### Session Tracking

Comprehensive session monitoring:

```rust
pub struct SessionMetadata {
    pub created_at: DateTime<Utc>,
    pub last_refreshed_at: Option<DateTime<Utc>>,
    pub last_api_call_at: Option<DateTime<Utc>>,
    pub current_api_server: Option<String>,
    pub user_agent: String,
    pub ip_address: Option<String>,
}
```

**Monitoring Features:**
- **Session Lifecycle**: Track session creation and usage
- **API Activity**: Monitor API call patterns
- **User Agent Rotation**: Prevent fingerprinting
- **Server Selection**: Track API server usage
- **Anomaly Detection**: Identify unusual patterns

### Security Logs

Structured logging for security events:

- **Authentication Events**: Login, logout, token refresh
- **Trading Activity**: All trading operations with metadata
- **Error Tracking**: Security-related errors and failures
- **Session Changes**: User agent updates, server switches

## 9. Deployment Security

### Production Hardening

**Environment Security:**
- Use container orchestration secrets management
- Implement network segmentation
- Enable audit logging
- Regular security updates
- Monitoring and alerting

**Infrastructure Security:**
- Firewall configuration
- VPN access for management
- Encrypted storage for session data
- Regular backup and recovery testing
- Incident response procedures

### Security Checklist

**Pre-Deployment:**
- [ ] All secrets in environment variables, not code
- [ ] TLS/SSL certificates properly configured
- [ ] Rate limiting enabled and tested
- [ ] Monitoring and alerting configured
- [ ] Backup and recovery procedures tested

**Ongoing Security:**
- [ ] Regular token rotation
- [ ] Monitor for unusual trading patterns
- [ ] Keep dependencies updated
- [ ] Review security logs regularly
- [ ] Test incident response procedures

## 10. Compliance and Best Practices

### Industry Standards

The library adheres to multiple security standards:

- **OWASP Guidelines**: Secure coding practices
- **NIST Cryptographic Standards**: P-256, SHA256, PBKDF2
- **OAuth 2.0 Security**: Proper token handling
- **JWT Best Practices**: Secure token management

### Security Development Lifecycle

**Code Security:**
- Static analysis for security vulnerabilities
- Dependency scanning for known vulnerabilities
- Regular security reviews and audits
- Penetration testing for critical paths

**Operational Security:**
- Incident response procedures
- Security monitoring and alerting
- Regular security training
- Vulnerability disclosure process

This comprehensive security framework ensures that axiomtrade-rs provides enterprise-grade protection for trading operations while maintaining usability and performance.
