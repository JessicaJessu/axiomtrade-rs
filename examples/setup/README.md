# Setup Examples

This directory contains comprehensive setup guides and tools to help you configure axiomtrade-rs for your environment.

## Files in this Directory

### 1. auto_otp_setup.md
**Complete step-by-step guide for automatic OTP configuration**

This guide walks you through:
- Creating an inbox.lv email account
- Enabling IMAP access
- Configuring Axiom Trade email forwarding
- Setting up environment variables
- Testing the complete setup

**When to use**: If you want to eliminate manual OTP entry and enable fully automated authentication.

### 2. test_auto_otp.rs
**Automated test to verify your OTP setup**

```bash
cargo run --example test_auto_otp
```

This example:
- Verifies all environment variables are configured
- Tests IMAP connection to inbox.lv
- Performs end-to-end OTP retrieval and verification
- Provides detailed troubleshooting information

**When to use**: After following the auto_otp_setup.md guide to verify everything works.

### 3. environment_setup.rs
**Interactive tool for environment configuration**

```bash
cargo run --example environment_setup
```

This tool helps you:
- Create or update your .env file
- Test current configuration
- Troubleshoot configuration issues
- Reset configuration if needed

**When to use**: For initial setup or when you need to update credentials.

## Quick Start Guide

### Option 1: Manual OTP (Simpler)
If you're okay with manually entering OTP codes:

1. Run the environment setup tool:
   ```bash
   cargo run --example environment_setup
   ```

2. Provide your Axiom Trade credentials when prompted

3. Skip the automatic OTP setup (answer 'n')

4. Test with basic authentication:
   ```bash
   cargo run --example basic_login
   ```

### Option 2: Automatic OTP (More Advanced)
For fully automated authentication:

1. Follow the complete guide in `auto_otp_setup.md`

2. Run the environment setup tool:
   ```bash
   cargo run --example environment_setup
   ```

3. Configure both Axiom Trade and inbox.lv credentials

4. Test the complete setup:
   ```bash
   cargo run --example test_auto_otp
   ```

## Environment Variables Reference

### Required Variables
```env
AXIOM_EMAIL=your_axiom_trade_email@domain.com
AXIOM_PASSWORD=your_axiom_trade_password
```

### Optional Variables (for automatic OTP)
```env
INBOX_LV_EMAIL=your_username@inbox.lv
INBOX_LV_PASSWORD=your_special_imap_password
```

## Troubleshooting

### Common Issues

#### "Environment variables not found"
- **Solution**: Run `cargo run --example environment_setup` to create your .env file
- **Cause**: Missing or incorrectly named .env file

#### "IMAP connection failed"
- **Solution**: Follow the inbox.lv setup in `auto_otp_setup.md` exactly
- **Common causes**:
  - IMAP not enabled (wait 15 minutes after enabling)
  - Using web password instead of IMAP password
  - Typos in email address

#### "OTP not found in emails"
- **Solution**: Check Axiom Trade email forwarding configuration
- **Common causes**:
  - OTP emails not forwarded to inbox.lv
  - Emails in spam folder
  - Email subject format changed

#### "Authentication failed"
- **Solution**: Verify Axiom Trade credentials are correct
- **Test**: Try logging into Axiom Trade website manually

### Debug Steps

1. **Test environment variables**:
   ```bash
   cargo run --example environment_setup
   ```

2. **Test IMAP connection**:
   ```bash
   cargo run --example test_auto_otp
   ```

3. **Test basic authentication**:
   ```bash
   cargo run --example basic_login
   ```

4. **Test OTP verification**:
   ```bash
   cargo run --example otp_verification
   ```

## Security Best Practices

### Environment File Security
- Never commit `.env` files to version control
- Use `.gitignore` to exclude `.env` files
- Keep credentials private and secure

### Password Management
- Use strong, unique passwords for all accounts
- Consider using a password manager
- Regularly rotate passwords

### Email Security
- Use dedicated inbox.lv account only for OTP
- Don't use OTP email for other purposes
- Monitor account for suspicious activity

### Network Security
- Use secure networks when testing
- Be aware of network monitoring in corporate environments
- Consider VPN if using public networks

## Production Considerations

### Automated Systems
- Implement proper error handling for network failures
- Add retry logic for OTP retrieval
- Monitor authentication success rates
- Set up alerts for authentication failures

### Scalability
- Consider rate limiting to avoid overwhelming services
- Implement connection pooling for high-volume applications
- Cache authentication tokens appropriately

### Monitoring
- Log authentication attempts (without sensitive data)
- Monitor OTP retrieval performance
- Track inbox.lv service availability

## Support and Updates

### Getting Help
- Check error messages carefully - they often contain specific solutions
- Review the troubleshooting sections in each guide
- Test individual components to isolate issues

### Staying Updated
- Monitor Axiom Trade for authentication changes
- Check inbox.lv for service updates
- Keep the axiomtrade-rs library updated

### Contributing
- Report issues with setup guides
- Suggest improvements to the setup process
- Share solutions for new authentication scenarios

## Next Steps

After completing setup:

1. **Explore authentication examples**: `examples/authentication/`
2. **Try portfolio management**: `examples/portfolio/`
3. **Test market data**: `examples/market_data/`
4. **Experiment with trading**: `examples/trading/`
5. **Set up WebSocket connections**: `examples/websocket/`
6. **Configure Turnkey wallets**: `examples/turnkey/`

Each category contains multiple examples with increasing complexity to help you build sophisticated trading applications.