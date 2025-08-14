# Environment Setup

This guide covers all aspects of configuring your environment for axiomtrade-rs, including required credentials, automated setup tools, and security best practices.

## Overview

axiomtrade-rs requires specific environment variables to function properly. You can configure these using:

- **Automated setup utility** - Interactive tool for guided configuration
- **Manual .env file creation** - Direct file editing
- **Environment variable validation** - Built-in verification tools

## Quick Start

For immediate setup, run the interactive configuration tool:

```bash
cargo run --example environment_setup
```

This tool will guide you through the entire configuration process, including credential setup, OTP automation, and validation testing.

## Required Environment Variables

### Core Authentication Variables

These variables are required for basic functionality:

```env
# Axiom Trade account credentials
AXIOM_EMAIL=your-axiom-email@domain.com
AXIOM_PASSWORD=your-axiom-password
```

### Optional OTP Automation Variables

For automatic OTP retrieval (eliminates manual code entry):

```env
# inbox.lv IMAP credentials for automatic OTP
INBOX_LV_EMAIL=your-username@inbox.lv
INBOX_LV_PASSWORD=your-special-imap-password
```

**Note**: The `INBOX_LV_PASSWORD` is a special IMAP password, not your regular web login password.

### Additional Configuration Options

```env
# Optional: API endpoints (uses defaults if not specified)
AXIOM_API_BASE_URL=https://api.axiom.trade
AXIOM_WS_URL=wss://api.axiom.trade/ws

# Optional: Logging level
RUST_LOG=debug
```

## Setup Methods

### Method 1: Automated Setup Tool

The interactive setup tool provides guided configuration with proper validation:

```bash
cargo run --example environment_setup
```

**Features:**
- Interactive credential collection
- Automatic special character escaping
- Environment validation
- Configuration testing
- Backup creation for existing files

**Sample interaction:**
```
axiomtrade-rs Environment Setup Helper
This tool will help you configure your environment for the library

❌ No .env file found
Let's create one with the required configuration

=== Axiom Trade Credentials ===
Axiom Trade email: user@example.com
Axiom Trade password: [secure input]

=== Optional: Automatic OTP Setup ===
To enable automatic OTP, you need an inbox.lv account
Leave blank to skip automatic OTP (you'll enter codes manually)
Set up automatic OTP? (y/n): y

See examples/setup/auto_otp_setup.md for inbox.lv setup instructions
inbox.lv email (username@inbox.lv): myuser@inbox.lv
inbox.lv IMAP password: [secure input]

✓ Configuration file created successfully
```

### Method 2: Legacy Setup Utility

The older setup utility from `oldstuff/setup_env.rs` provides similar functionality:

```bash
cd oldstuff
cargo run --bin setup_env
```

**Features:**
- Advanced password escaping
- Secure file permissions (Unix systems)
- Comprehensive validation
- Detailed troubleshooting guidance

### Method 3: Manual Configuration

Create a `.env` file in your project root directory:

```bash
# Create the file
touch .env

# Set secure permissions (Unix/Linux/macOS)
chmod 600 .env
```

Add the required variables:

```env
# axiomtrade-rs Environment Configuration

# Axiom Trade Credentials
AXIOM_EMAIL=your-email@domain.com
AXIOM_PASSWORD=your-password

# Optional: Automatic OTP via inbox.lv
INBOX_LV_EMAIL=your-username@inbox.lv
INBOX_LV_PASSWORD=your-imap-password

# Optional: Additional Configuration
# AXIOM_API_BASE_URL=https://api.axiom.trade
# AXIOM_WS_URL=wss://api.axiom.trade/ws
# RUST_LOG=debug
```

## Using the Setup Environment Utility

The environment setup utility provides comprehensive configuration management:

### Initial Setup

```bash
cargo run --example environment_setup
```

When no `.env` file exists, the tool will:
1. Guide you through credential entry
2. Offer optional OTP automation setup
3. Create a properly formatted `.env` file
4. Provide next steps for testing

### Updating Existing Configuration

If you already have a `.env` file:

```bash
cargo run --example environment_setup
```

The tool offers options to:
- **Test current configuration** - Validate all settings
- **Update configuration** - Modify specific sections
- **View configuration guide** - Display help information
- **Reset configuration** - Start fresh

### Configuration Testing

The utility includes built-in testing:

```bash
# Test all configuration
cargo run --example environment_setup
# Choose option 1: Test current configuration
```

Testing includes:
- Environment variable validation
- IMAP connection testing (if configured)
- Basic authentication verification
- OTP automation validation

## Automatic OTP Setup

For seamless authentication without manual OTP entry, configure automatic OTP retrieval via inbox.lv:

### Prerequisites

1. **inbox.lv account** - Free email service with IMAP support
2. **IMAP access enabled** - Must be activated in inbox.lv settings
3. **Email forwarding configured** - Axiom Trade OTP emails to inbox.lv

### Setup Process

1. **Create inbox.lv account**:
   - Visit [https://www.inbox.lv/](https://www.inbox.lv/)
   - Register a new account
   - Note your full email: `username@inbox.lv`

2. **Enable IMAP access**:
   ```
   Settings → "Outlook, email programs" → Enable
   Direct URL: https://email.inbox.lv/prefs?group=enable_pop3
   Wait 15 minutes for activation
   ```

3. **Configure email forwarding**:
   - Log into Axiom Trade account
   - Update notification email to your inbox.lv address
   - Ensure OTP emails are forwarded

4. **Get IMAP password**:
   - Return to inbox.lv settings after 15 minutes
   - Copy the special IMAP/SMTP password
   - This is different from your web login password

5. **Configure environment variables**:
   ```env
   INBOX_LV_EMAIL=your-username@inbox.lv
   INBOX_LV_PASSWORD=your-special-imap-password
   ```

6. **Test the setup**:
   ```bash
   cargo run --example test_auto_otp
   ```

For detailed setup instructions, see: `examples/setup/auto_otp_setup.md`

## Manual .env Configuration

### File Creation

```bash
# Create .env file in project root
touch .env

# Set restrictive permissions (recommended)
chmod 600 .env  # Unix/Linux/macOS only
```

### Variable Format

Use proper formatting to handle special characters:

```env
# Simple values (no spaces or special characters)
AXIOM_EMAIL=user@example.com

# Values with special characters (use quotes)
AXIOM_PASSWORD="my-complex-password!@#$"

# Values with spaces (use quotes)
SOME_VALUE="value with spaces"

# Avoid these characters without quotes: $ ` " ' \ # space
```

### Special Character Handling

The library's `EnvLoader` properly handles special characters:

```rust
// Values are automatically parsed correctly
let password = env::var("AXIOM_PASSWORD")?; // Works with special chars
```

**Characters requiring quotes**:
- Spaces: `"password with spaces"`
- Dollar signs: `"password$with$dollars"`
- Quotes: `"password\"with\"quotes"`
- Backslashes: `"password\\with\\backslashes"`

## Security Best Practices

### Credential Management

1. **Use strong passwords**:
   - 12+ characters with mixed case, numbers, symbols
   - Unique passwords for each service
   - Consider using a password manager

2. **Protect credential files**:
   ```bash
   # Set restrictive permissions
   chmod 600 .env
   
   # Verify permissions
   ls -la .env  # Should show: -rw-------
   ```

3. **Never commit credentials**:
   ```bash
   # Ensure .env is in .gitignore
   echo ".env" >> .gitignore
   ```

### Network Security

1. **Use secure connections** - axiomtrade-rs uses HTTPS/WSS by default
2. **Verify certificates** - Built-in certificate validation
3. **Monitor authentication** - Log successful/failed attempts
4. **Rotate credentials regularly** - Update passwords periodically

### Production Environment

1. **Environment isolation**:
   ```bash
   # Different .env files for different environments
   .env.development
   .env.staging  
   .env.production
   ```

2. **Secret management**:
   - Consider using secret management services
   - Use environment variables in production
   - Avoid storing secrets in container images

3. **Access control**:
   - Limit access to credential files
   - Use service accounts where appropriate
   - Implement proper logging and monitoring

## Configuration Validation

### Built-in Validation

Test your configuration using the provided tools:

```bash
# Comprehensive testing
cargo run --example environment_setup
# Choose: Test current configuration

# OTP-specific testing
cargo run --example test_auto_otp

# Basic authentication testing
cargo run --example basic_login
```

### Manual Validation

Verify variables are loaded correctly:

```rust
use std::env;

fn validate_config() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    // Check required variables
    let email = env::var("AXIOM_EMAIL")?;
    let password = env::var("AXIOM_PASSWORD")?;
    
    println!("Email configured: {}", email);
    println!("Password configured: [{}]", "*".repeat(password.len()));
    
    // Check optional OTP variables
    if let (Ok(otp_email), Ok(_otp_pass)) = (
        env::var("INBOX_LV_EMAIL"),
        env::var("INBOX_LV_PASSWORD")
    ) {
        println!("OTP automation configured for: {}", otp_email);
    } else {
        println!("Manual OTP entry will be required");
    }
    
    Ok(())
}
```

### Environment Variable Priority

The library loads variables in this order (last wins):

1. Process environment variables
2. `.env` file in current directory
3. `.env` file in parent directories (recursive search)

Override specific variables for testing:

```bash
# Override for single command
AXIOM_EMAIL=test@example.com cargo run --example basic_login

# Export for session
export RUST_LOG=debug
cargo run --example environment_setup
```

## Command Line Instructions

### Setup Commands

```bash
# Initial environment setup
cargo run --example environment_setup

# Legacy setup utility
cd oldstuff && cargo run --bin setup_env

# Test automatic OTP setup
cargo run --example test_auto_otp

# Test basic authentication
cargo run --example basic_login
```

### Validation Commands

```bash
# Validate environment loading
cargo test utils::env_loader::tests

# Test password handling
cargo run --example test_password

# Check configuration completeness
cargo run --example environment_setup
# Choose option 1: Test current configuration
```

### Development Commands

```bash
# Enable debug logging
export RUST_LOG=debug
cargo run --example your_example

# Test with different environment
AXIOM_EMAIL=different@email.com cargo run --example basic_login

# Run with production config
cargo run --release --example trading_bot
```

## Troubleshooting

### Common Issues

#### "Environment variables not found"
```bash
# Check if .env file exists
ls -la .env

# Verify file contents (be careful with passwords)
head .env

# Run setup tool
cargo run --example environment_setup
```

#### "IMAP connection failed"
```bash
# Test IMAP configuration
cargo run --example test_auto_otp

# Check inbox.lv setup
# 1. Verify IMAP is enabled (wait 15 minutes after enabling)
# 2. Confirm you're using IMAP password, not web password
# 3. Check email address spelling
```

#### "Authentication failed"
```bash
# Verify credentials by logging into Axiom Trade website
# Update credentials using setup tool
cargo run --example environment_setup
# Choose option 2: Update configuration
```

#### "Special characters in password"
```bash
# Use the automated setup tool for proper escaping
cargo run --example environment_setup

# Or manually quote the value in .env:
AXIOM_PASSWORD="password-with-special-chars!@#$"
```

### Debug Steps

1. **Check environment loading**:
   ```rust
   use std::env;
   dotenvy::dotenv().ok();
   for (key, value) in env::vars() {
       if key.starts_with("AXIOM_") || key.starts_with("INBOX_") {
           println!("{}: {}", key, if key.contains("PASSWORD") { "[HIDDEN]" } else { &value });
       }
   }
   ```

2. **Test individual components**:
   ```bash
   # Test just environment loading
   cargo test env_loader
   
   # Test just IMAP (if configured)
   cargo run --example test_auto_otp
   
   # Test just authentication
   cargo run --example basic_login
   ```

3. **Enable verbose logging**:
   ```bash
   export RUST_LOG=axiomtrade_rs=debug
   cargo run --example your_example
   ```

## Next Steps

After completing environment setup:

1. **Test authentication**:
   ```bash
   cargo run --example basic_login
   ```

2. **Explore examples**:
   ```bash
   ls examples/
   cargo run --example portfolio_monitoring
   ```

3. **Build your application**:
   ```rust
   use axiomtrade_rs::AxiomClient;
   
   #[tokio::main]
   async fn main() -> Result<(), Box<dyn std::error::Error>> {
       // Environment is automatically loaded
       let client = AxiomClient::new().await?;
       // Your application logic here
       Ok(())
   }
   ```

4. **Review additional documentation**:
   - [Authentication Guide](auth/login.md)
   - [Automatic OTP Setup](automatic-otp.md)  
   - [Trading Examples](examples/trading.md)
   - [Security Best Practices](best-practices/security.md)
