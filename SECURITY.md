# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | ✅ Yes              |

## Reporting Security Issues

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please send an email to vibheksoni@engineer.com with:

- A description of the vulnerability
- Steps to reproduce the issue
- Any potential impact
- Suggested fix (if available)

You should receive a response within 48 hours. If the issue is confirmed, we will:

1. Work on a fix
2. Prepare a security advisory
3. Release a patched version
4. Credit you for the discovery (if desired)

## Security Best Practices

When using axiomtrade-rs:

### Environment Variables
- Store credentials in `.env` files, never in code
- Use different credentials for development and production
- Rotate API keys regularly

### Token Management
- Tokens are automatically stored securely by TokenManager
- Set appropriate token expiration times
- Clear tokens on logout

### Network Security
- All API calls use HTTPS/TLS encryption
- WebSocket connections are secure by default
- Rate limiting prevents abuse

### Input Validation
- All user inputs are validated before API calls
- Wallet addresses are validated for correct format
- Amount fields check for reasonable ranges

## Dependency Security

We regularly audit dependencies for known vulnerabilities using:
- `cargo audit` for Rust dependencies
- GitHub Security Advisories
- Dependabot alerts

## Cryptographic Operations

- Uses industry-standard encryption (P256, PBKDF2)
- Secure random number generation
- Proper key derivation functions
- No custom cryptography implementations

## Contact

For any security-related questions: vibheksoni@engineer.com