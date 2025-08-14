# Automatic OTP Setup Guide

This guide provides step-by-step instructions for setting up automatic OTP (One-Time Password) retrieval using inbox.lv email service with the axiomtrade-rs library.

## Overview

The automatic OTP feature eliminates the need to manually check emails for verification codes by automatically fetching them from your inbox.lv email account via IMAP protocol.

## Prerequisites

- A working Axiom Trade account
- Access to create a new email account
- Basic understanding of environment variables

## Step 1: Create inbox.lv Email Account

### 1.1 Navigate to inbox.lv
1. Open your web browser
2. Go to [https://www.inbox.lv/](https://www.inbox.lv/)
3. Click on "Register" or "Create Account"

### 1.2 Register New Account
1. Fill in the registration form:
   - Choose a username (this will be your email prefix)
   - Set a strong password
   - Complete any required verification steps
2. Your new email will be: `your_username@inbox.lv`
3. Complete the email verification process

### 1.3 Verify Account Access
1. Log into the inbox.lv web interface
2. Send yourself a test email to confirm it works
3. Make note of your full email address and password

## Step 2: Enable IMAP Access

### 2.1 Access Email Settings
1. Log into your inbox.lv account via web browser
2. Navigate to Settings
3. Look for "Outlook, email programs" or "IMAP/POP3" settings

### 2.2 Enable IMAP Protocol
1. Find the IMAP configuration section
2. Click the "Enable" button for IMAP access
3. Alternative: Direct URL [https://email.inbox.lv/prefs?group=enable_pop3](https://email.inbox.lv/prefs?group=enable_pop3)

### 2.3 Wait for Activation
- IMAP activation takes approximately **15 minutes**
- Do not proceed to the next step until this waiting period is complete
- You can continue with other setup steps during this time

### 2.4 Retrieve IMAP Password
1. After the 15-minute waiting period, refresh your settings page
2. Look for "IMAP/SMTP Password" or similar section
3. Copy the special IMAP password (this is different from your web login password)
4. Store this password securely - you'll need it for the environment configuration

## Step 3: Configure Axiom Trade Email Forwarding

### 3.1 Access Axiom Trade Account Settings
1. Log into your Axiom Trade account
2. Navigate to Account Settings or Security Settings
3. Look for "Email Preferences" or "Notification Settings"

### 3.2 Set Email Forwarding
1. Find the option to change notification email or add secondary email
2. Add your new inbox.lv email address: `your_username@inbox.lv`
3. If available, specifically enable "Security Code" or "OTP" notifications
4. Save the settings

### 3.3 Test Email Forwarding
1. Trigger an OTP request from Axiom Trade (attempt login)
2. Check your inbox.lv account for the OTP email
3. Verify the email subject contains: "Your Axiom security code is XXXXXX"
4. Confirm the OTP code is clearly visible in the subject line

## Step 4: Environment Variable Configuration

### 4.1 Locate .env File
1. Navigate to your axiomtrade-rs project directory
2. Look for the `.env` file (create one if it doesn't exist)
3. Open the file in a text editor

### 4.2 Add Required Variables
Add these lines to your `.env` file:

```env
# Axiom Trade Credentials
AXIOM_EMAIL=your_axiom_trade_email@domain.com
AXIOM_PASSWORD=your_axiom_trade_password

# inbox.lv IMAP Configuration
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_special_imap_password
```

### 4.3 Verify Configuration
- Ensure there are no spaces around the `=` signs
- Use the IMAP password, not your web login password
- Double-check all email addresses are correct
- Save the file

## Step 5: Test the Setup

### 5.1 Create Test Script
Create a file named `test_auto_otp.rs` in your examples directory:

```rust
use axiomtrade_rs::{AxiomClient, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set");
    
    println!("Testing automatic OTP setup...");
    
    // Create client and attempt login
    let mut client = AxiomClient::new().await?;
    let login_result = client.login(&email, &password).await?;
    
    if !login_result.requires_otp {
        println!("OTP not required for this account");
        return Ok(());
    }
    
    println!("OTP required - testing automatic retrieval...");
    
    // Test automatic OTP fetching
    match client.get_otp_from_email().await {
        Ok(otp) => {
            println!("✅ Success! OTP retrieved automatically: {}", otp);
            
            // Verify the OTP works
            match client.verify_otp(&otp).await {
                Ok(_) => println!("✅ OTP verification successful!"),
                Err(e) => println!("❌ OTP verification failed: {}", e),
            }
        }
        Err(e) => {
            println!("❌ Automatic OTP retrieval failed: {}", e);
            println!("Check the troubleshooting section below");
        }
    }
    
    Ok(())
}
```

### 5.2 Run the Test
```bash
cargo run --example test_auto_otp
```

### 5.3 Expected Output
```
Testing automatic OTP setup...
OTP required - testing automatic retrieval...
✅ Success! OTP retrieved automatically: 123456
✅ OTP verification successful!
```

## Step 6: Troubleshooting

### Common Issues and Solutions

#### Issue: "IMAP connection failed"
**Cause**: IMAP not properly enabled or credentials incorrect
**Solution**:
1. Verify you waited the full 15 minutes after enabling IMAP
2. Check that you're using the IMAP password, not web login password
3. Confirm the email address is exactly correct in `.env`

#### Issue: "No OTP emails found"
**Cause**: Email forwarding not configured or emails not arriving
**Solution**:
1. Verify Axiom Trade is sending OTP emails to your inbox.lv address
2. Check spam/junk folder in inbox.lv
3. Manually trigger an OTP and check if it arrives
4. Confirm email subject format matches expected pattern

#### Issue: "Authentication failed to inbox.lv"
**Cause**: Wrong IMAP credentials
**Solution**:
1. Double-check the IMAP password from inbox.lv settings
2. Ensure email address has no typos
3. Try logging into inbox.lv webmail to verify credentials
4. Regenerate IMAP password if necessary

#### Issue: "OTP extraction failed"
**Cause**: Email subject format changed or parsing issue
**Solution**:
1. Check a recent OTP email to see the exact subject format
2. Verify the subject contains "Your Axiom security code is"
3. Report the issue if format has changed

### Debug Mode

Enable debug logging by setting environment variable:
```env
RUST_LOG=debug
```

This will provide detailed information about IMAP connection and email processing.

### Manual Verification Steps

1. **Test IMAP Connection**:
   ```rust
   // Use your IMAP credentials to test connection manually
   let client = imap::ClientBuilder::new("mail.inbox.lv", 993)
       .connect()
       .unwrap();
   ```

2. **Check Email Delivery**:
   - Send a test email to your inbox.lv address
   - Verify it appears in the inbox via webmail

3. **Verify OTP Email Format**:
   - Request an OTP manually from Axiom Trade
   - Check the exact subject line format in inbox.lv

## Security Considerations

### Email Security
- Use a dedicated inbox.lv account only for OTP purposes
- Don't use this email for any other services
- Consider the email account as semi-public

### Password Security
- Use a strong, unique password for inbox.lv
- Store IMAP credentials securely in environment variables
- Never commit `.env` files to version control

### Access Control
- The inbox.lv account only needs to receive emails
- Consider enabling two-factor authentication on Axiom Trade account
- Regularly review account access and settings

## Alternative Setup Options

### Using Other Email Providers
The automatic OTP system can be adapted for other IMAP-enabled email providers:
- Gmail (requires app-specific passwords)
- Outlook/Hotmail
- Yahoo Mail
- Custom email servers

### Manual OTP Entry
If automatic OTP setup is not desired, you can still use manual entry:
```rust
// Manual OTP entry example
use std::io::{self, Write};

fn get_otp_manual() -> Result<String> {
    print!("Enter OTP code: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    Ok(input.trim().to_string())
}
```

## Support and Maintenance

### Regular Maintenance
- Periodically verify the setup still works
- Check for any changes in Axiom Trade OTP email format
- Monitor inbox.lv for any service changes

### Getting Help
- Check the main project documentation
- Review error logs for specific issues
- Test individual components (IMAP, email parsing, etc.)

### Updates and Changes
- Monitor Axiom Trade announcements for authentication changes
- Keep the axiomtrade-rs library updated
- Watch for any inbox.lv service updates that might affect IMAP

## Conclusion

With automatic OTP setup complete, you can now:
- Authenticate with Axiom Trade without manual intervention
- Run automated trading scripts and monitoring tools
- Build production applications with seamless authentication

The setup provides a robust foundation for automated interactions with the Axiom Trade platform while maintaining security best practices.