# Automatic OTP

The Axiom Trade Rust client provides automatic OTP (One-Time Password) retrieval functionality that eliminates the need to manually check emails for verification codes. This feature uses IMAP protocol to fetch OTP codes directly from your inbox.lv email account.

## Overview

When enabled, the automatic OTP system:
- Monitors your inbox.lv email account for new Axiom security codes
- Extracts 6-digit OTP codes from email subjects and bodies
- Provides methods for immediate retrieval or waiting for new codes
- Automatically marks processed emails as read to avoid duplicates

## Prerequisites

- Axiom Trade account with OTP authentication enabled
- inbox.lv email account with IMAP access
- Environment variables configured for email credentials

## Configuration Setup

### 1. inbox.lv Account Setup

Create a dedicated inbox.lv email account for OTP purposes:

1. **Register Account**
   - Navigate to [https://www.inbox.lv/](https://www.inbox.lv/)
   - Click "Register" and complete the form
   - Choose a unique username (becomes `username@inbox.lv`)
   - Verify your account through the confirmation email

2. **Enable IMAP Access**
   - Log into inbox.lv web interface
   - Go to Settings → "Outlook, email programs"
   - Click "Enable" button for IMAP access
   - Direct link: [https://email.inbox.lv/prefs?group=enable_pop3](https://email.inbox.lv/prefs?group=enable_pop3)
   - Wait 15 minutes for activation to complete

3. **Retrieve IMAP Credentials**
   - After the 15-minute wait, refresh your settings page
   - Locate the "IMAP/SMTP Password" section
   - Copy the special IMAP password (different from web login password)
   - Save these credentials securely

### 2. IMAP Configuration

The client uses these IMAP settings for inbox.lv:
- **Server**: `mail.inbox.lv`
- **Port**: `993` (SSL/TLS)
- **Security**: TLS encryption
- **Authentication**: Username/password

### 3. Email Forwarding Setup

Configure Axiom Trade to send OTP emails to your inbox.lv account:

1. **Access Axiom Settings**
   - Log into your Axiom Trade account
   - Navigate to Account Settings or Security Settings
   - Find "Email Preferences" or "Notification Settings"

2. **Configure Forwarding**
   - Add your inbox.lv email as the notification address
   - Enable "Security Code" or "OTP" notifications
   - Save the configuration

3. **Test Email Delivery**
   - Trigger an OTP request from Axiom Trade
   - Verify the email arrives at your inbox.lv account
   - Confirm subject format: "Your Axiom security code is XXXXXX"

### 4. Environment Variables

Set the required environment variables in your `.env` file:

```env
# inbox.lv IMAP Configuration
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_special_imap_password

# Axiom Trade Credentials (if using automatic login)
AXIOM_EMAIL=your_axiom_email@domain.com
AXIOM_PASSWORD=your_axiom_password
```

**Important Notes:**
- Use the IMAP password, not your web login password
- Ensure no spaces around the `=` signs
- Never commit `.env` files to version control

## Usage Examples

### Basic OTP Retrieval

```rust
use axiomtrade_rs::email::otp_fetcher::{OtpFetcher, from_env};

// Create fetcher from environment variables
let fetcher = from_env()?.expect("OTP environment variables not configured");

// Fetch the latest unread OTP
if let Some(otp) = fetcher.fetchotp()? {
    println!("Retrieved OTP: {}", otp);
} else {
    println!("No unread OTP emails found");
}
```

### Waiting for New OTP

```rust
// Wait up to 60 seconds for a new OTP email, checking every 5 seconds
let timeout_seconds = 60;
let check_interval = 5;

if let Some(otp) = fetcher.wait_for_otp(timeout_seconds, check_interval)? {
    println!("New OTP received: {}", otp);
} else {
    println!("Timeout: No OTP received within {} seconds", timeout_seconds);
}
```

### Time-Based OTP Retrieval

```rust
// Fetch OTP from emails received in the last 3 minutes
let minutes_ago = 3;

if let Some(otp) = fetcher.fetchotp_recent(minutes_ago)? {
    println!("Recent OTP found: {}", otp);
} else {
    println!("No OTP emails from the last {} minutes", minutes_ago);
}
```

### Integration with Client Authentication

```rust
use axiomtrade_rs::AxiomClient;

let mut client = AxiomClient::new().await?;

// Attempt login
let login_result = client.login(&email, &password).await?;

if login_result.requires_otp {
    // Use automatic OTP retrieval
    if let Some(fetcher) = from_env()? {
        if let Some(otp) = fetcher.wait_for_otp(60, 5)? {
            client.verify_otp(&otp).await?;
            println!("Authentication successful with automatic OTP");
        } else {
            return Err("Failed to retrieve OTP automatically".into());
        }
    } else {
        // Fall back to manual OTP entry
        print!("Enter OTP: ");
        // ... manual input logic
    }
}
```

## OTP Extraction Methods

The system uses multiple strategies to extract OTP codes from emails:

### Subject Line Extraction
- Primary pattern: `"Your Axiom security code is (\d{6})"`
- Extracts 6-digit codes from email subjects

### Body Content Extraction
The system tries multiple patterns in order:
1. `"Your Axiom security code is[:\s]+(\d{6})"`
2. `"Your security code is[:\s]+(\d{6})"`
3. `"security code[:\s]+(\d{6})"`
4. HTML tags: `<span>`, `<b>`, `<strong>` containing 6 digits
5. Fallback: Any 6-digit number in context containing "security code" or "Your Axiom"

### Example Email Formats

**Subject Line Format:**
```
Subject: Your Axiom security code is 123456
```

**Plain Text Body:**
```
Your Axiom security code is: 123456

This code will expire in 10 minutes.
```

**HTML Body:**
```html
<div style="background-color: #f5f5f5; padding: 15px;">
  <span style="font-size: 24px; font-weight: bold;">123456</span>
</div>
<p>Your Axiom security code</p>
```

## Troubleshooting OTP Issues

### Connection Problems

**Issue**: "IMAP connection failed"
- **Cause**: IMAP not enabled or incorrect credentials
- **Solutions**:
  - Verify 15-minute IMAP activation wait period completed
  - Check IMAP password vs web login password
  - Confirm email address spelling in environment variables
  - Test connection manually using an IMAP client

**Issue**: "Authentication failed to inbox.lv"
- **Cause**: Wrong IMAP credentials
- **Solutions**:
  - Verify IMAP password from inbox.lv settings
  - Check for typos in email address
  - Try logging into inbox.lv webmail to verify credentials
  - Regenerate IMAP password if necessary

### Email Delivery Problems

**Issue**: "No OTP emails found"
- **Cause**: Email forwarding not configured or emails not arriving
- **Solutions**:
  - Verify Axiom Trade sends OTP emails to inbox.lv address
  - Check spam/junk folder in inbox.lv
  - Manually trigger OTP and verify email arrives
  - Confirm email subject format matches expected pattern

**Issue**: "OTP extraction failed"
- **Cause**: Email format changed or parsing issue
- **Solutions**:
  - Check recent OTP email for exact subject format
  - Verify subject contains "Your Axiom security code is"
  - Review email body format if subject extraction fails
  - Report format changes to maintain compatibility

### Debug and Testing

Enable debug logging for detailed information:

```env
RUST_LOG=debug
```

Test individual components:

```rust
// Test IMAP connection
let fetcher = OtpFetcher::new(
    "your_email@inbox.lv".to_string(),
    "your_imap_password".to_string()
);

// Test email parsing
let email_body = "Your Axiom security code is: 123456";
let result = fetcher.extract_otp_from_email(email_body)?;
```

Manual verification steps:
1. Send test email to inbox.lv account
2. Verify email appears in webmail interface
3. Request OTP manually from Axiom Trade
4. Check exact subject line format in received email

## Security Considerations

### Email Account Security
- Use dedicated inbox.lv account only for OTP purposes
- Avoid using this email for other services
- Consider the email account semi-public
- Use strong, unique password for inbox.lv account

### Credential Management
- Store IMAP credentials securely in environment variables
- Never commit `.env` files to version control
- Use proper file permissions on configuration files
- Consider using OS keychain for production deployments

### Access Control
- The inbox.lv account only needs to receive emails
- Enable two-factor authentication on Axiom Trade account
- Regularly review account access and settings
- Monitor for unauthorized access attempts

### Network Security
- All IMAP connections use TLS encryption
- Verify SSL certificate validation
- Use secure networks for production systems
- Consider VPN for additional protection

## Alternative Configurations

### Other Email Providers

The OTP fetcher can be adapted for other IMAP-enabled providers:

**Gmail** (requires app-specific passwords):
```rust
let fetcher = OtpFetcher::new(
    "user@gmail.com".to_string(),
    "app_specific_password".to_string()
);
// Note: Different IMAP server settings required
```

**Custom Email Servers**:
- Modify `IMAP_DOMAIN` and `IMAP_PORT` constants
- Adjust TLS settings as needed
- Update authentication method if required

### Manual Fallback

Always provide manual OTP entry as fallback:

```rust
fn get_otp_manual() -> Result<String> {
    print!("Enter OTP code: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

// Use in authentication flow
let otp = if let Some(fetcher) = from_env()? {
    fetcher.fetchotp()?.unwrap_or_else(|| get_otp_manual())
} else {
    get_otp_manual()?
};
```

## Performance and Limitations

### Performance Characteristics
- IMAP connection establishment: ~1-2 seconds
- Email search and retrieval: <1 second
- OTP extraction: <100 milliseconds
- Total process time: 2-4 seconds typically

### Rate Limiting
- IMAP servers may limit connection frequency
- Implement backoff strategies for production use
- Consider connection pooling for high-frequency operations

### Email Limitations
- Only processes UNREAD emails to avoid duplicates
- Requires emails to match specific subject patterns
- Dependent on email delivery timing and reliability
- Limited to 6-digit numeric OTP codes

### Best Practices
- Use timeouts to prevent hanging operations
- Implement retry logic with exponential backoff
- Log operations for debugging and monitoring
- Handle network interruptions gracefully
- Cache credentials securely to avoid repeated lookups
