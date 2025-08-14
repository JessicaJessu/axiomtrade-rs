# Frequently Asked Questions

This section covers the most common questions and issues encountered when using axiomtrade-rs.

## Authentication & OTP

### How to handle OTP timeouts?

**Q: My OTP codes are expiring before I can use them. What should I do?**

OTP codes from Axiom Trade typically expire within 5 minutes. Here are solutions:

1. **Enable automatic OTP fetching** (recommended):
   ```bash
   # Set up inbox.lv credentials
   export INBOX_LV_EMAIL="your_username@inbox.lv"
   export INBOX_LV_PASSWORD="your_special_imap_password"
   ```

2. **Reduce manual entry time**:
   - Have your email client open and ready
   - Use email forwarding to a fast email provider
   - Consider using a mobile app for quicker access

3. **Request new OTP if expired**:
   ```rust
   // The client will automatically request a new OTP if the previous one expired
   let result = client.login_with_credentials("username", "password").await;
   ```

4. **Configure timeout settings**:
   ```rust
   let client = EnhancedAxiomClient::new()
       .with_otp_timeout(Duration::from_secs(300)) // 5 minutes
       .build();
   ```

### Why are my tokens expiring?

**Q: I keep getting authentication errors even though I just logged in.**

Token expiration can occur for several reasons:

1. **Access tokens expire quickly** (typically 1 hour):
   ```rust
   // Enable automatic token refresh
   let client = EnhancedAxiomClient::new()
       .with_auto_refresh(true)
       .build();
   ```

2. **Server-side session invalidation**:
   - Multiple logins from different locations
   - Security policy changes
   - Server maintenance

3. **Clock synchronization issues**:
   ```bash
   # Ensure system time is synchronized
   ntpdate -s pool.ntp.org  # Linux/macOS
   w32tm /resync            # Windows
   ```

4. **Check token status**:
   ```rust
   if client.is_token_expired().await {
       client.refresh_token().await?;
   }
   ```

## Rate Limiting & Performance

### How to increase rate limits?

**Q: I'm getting rate limited. How can I handle more requests?**

Rate limiting is enforced by Axiom Trade's servers. Here's how to optimize:

1. **Use built-in rate limiting**:
   ```rust
   let client = EnhancedAxiomClient::new()
       .with_rate_limit(10, Duration::from_secs(1)) // 10 requests per second
       .build();
   ```

2. **Implement exponential backoff**:
   ```rust
   let retry_config = RetryConfig::new()
       .with_max_attempts(3)
       .with_exponential_backoff(Duration::from_millis(100));
   ```

3. **Batch operations when possible**:
   ```rust
   // Instead of individual balance calls
   let balances = client.get_batch_balances(&wallet_addresses).await?;
   ```

4. **Use WebSocket for real-time data**:
   ```rust
   // Reduces HTTP request load
   let ws_client = client.create_websocket_connection().await?;
   ws_client.subscribe_to_price_updates().await?;
   ```

5. **Contact Axiom Trade for increased limits**:
   - Premium accounts may have higher rate limits
   - Business partnerships can provide dedicated endpoints

## WebSocket Issues

### WebSocket reconnection strategies

**Q: My WebSocket connections keep dropping. How do I handle reconnections?**

WebSocket connections can be unstable due to network issues or server maintenance:

1. **Enable automatic reconnection**:
   ```rust
   let ws_client = client
       .websocket()
       .with_auto_reconnect(true)
       .with_ping_interval(Duration::from_secs(30))
       .build()
       .await?;
   ```

2. **Implement custom reconnection logic**:
   ```rust
   async fn handle_websocket_connection() -> Result<(), AxiomError> {
       let mut retry_count = 0;
       const MAX_RETRIES: u32 = 5;
       
       loop {
           match client.connect_websocket().await {
               Ok(ws) => {
                   retry_count = 0; // Reset on successful connection
                   handle_messages(ws).await?;
               }
               Err(e) if retry_count < MAX_RETRIES => {
                   retry_count += 1;
                   let delay = Duration::from_secs(2_u64.pow(retry_count));
                   tokio::time::sleep(delay).await;
               }
               Err(e) => return Err(e),
           }
       }
   }
   ```

3. **Monitor connection health**:
   ```rust
   ws_client.on_disconnect(|reason| {
       log::warn!("WebSocket disconnected: {}", reason);
       // Trigger reconnection
   });
   ```

4. **Handle message queuing during disconnection**:
   ```rust
   let (tx, rx) = tokio::sync::mpsc::channel(1000);
   // Queue messages when disconnected
   // Replay when reconnected
   ```

## Platform-Specific Issues

### Windows-specific problems

**Q: I'm having issues on Windows. What should I check?**

1. **TLS/SSL certificate issues**:
   ```toml
   # In Cargo.toml, ensure native-tls feature
   [dependencies]
   reqwest = { version = "0.11", features = ["native-tls"] }
   ```

2. **Firewall and antivirus**:
   - Add axiomtrade-rs to firewall exceptions
   - Whitelist in antivirus software
   - Check Windows Defender settings

3. **Path and environment variables**:
   ```cmd
   # Use double quotes for paths with spaces
   set AXIOM_CONFIG_PATH="C:\Users\Username\My Documents\axiom"
   ```

4. **Line ending issues**:
   ```bash
   git config core.autocrlf true
   ```

### macOS-specific problems

**Q: Having trouble on macOS. Any known issues?**

1. **Keychain access for credentials**:
   ```rust
   // Use keychain-rs for secure credential storage
   use keychain::Keychain;
   let keychain = Keychain::new();
   ```

2. **Certificate validation**:
   ```bash
   # Update certificates
   brew install ca-certificates
   ```

3. **Permission issues**:
   ```bash
   # Ensure proper permissions
   sudo chown -R $(whoami) ~/.axiom
   ```

### Linux-specific problems

**Q: Issues running on Linux distributions. What to check?**

1. **Missing dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install pkg-config libssl-dev
   
   # CentOS/RHEL
   sudo yum install openssl-devel
   
   # Arch Linux
   sudo pacman -S openssl pkg-config
   ```

2. **glibc version compatibility**:
   ```bash
   # Check glibc version
   ldd --version
   # May need to compile with older glibc target
   ```

3. **SELinux or AppArmor restrictions**:
   ```bash
   # Check SELinux status
   sestatus
   # Configure policies if needed
   ```

## Environment & Configuration

### Environment setup issues

**Q: My environment variables aren't being loaded correctly.**

1. **Verify .env file location**:
   ```bash
   # Should be in project root or specify path
   AXIOM_CONFIG_PATH=/path/to/config/.env
   ```

2. **Check .env file format**:
   ```bash
   # Correct format (no spaces around =)
   AXIOM_API_KEY=your_key_here
   INBOX_LV_EMAIL=user@inbox.lv
   
   # Incorrect format
   AXIOM_API_KEY = your_key_here  # Spaces cause issues
   ```

3. **Environment variable precedence**:
   ```rust
   // Order of precedence:
   // 1. System environment variables
   // 2. .env file in current directory
   // 3. .env file in project root
   // 4. Default values
   ```

4. **Permission issues**:
   ```bash
   chmod 600 .env  # Secure permissions
   ```

## Trading & Portfolio

### Portfolio balance discrepancies

**Q: The balance I see doesn't match what's in my wallet.**

1. **Check for pending transactions**:
   ```rust
   let portfolio = client.get_portfolio_with_pending().await?;
   ```

2. **Network synchronization delays**:
   - Solana transactions can take 30-60 seconds to confirm
   - Check transaction status on Solana explorer

3. **Multiple token account addresses**:
   ```rust
   // Get all token accounts for a wallet
   let token_accounts = client.get_token_accounts(&wallet_address).await?;
   ```

4. **Cache invalidation**:
   ```rust
   // Force refresh from blockchain
   let fresh_balance = client.get_balance_fresh(&wallet_address).await?;
   ```

### Trading execution failures

**Q: My trades are failing to execute. What could be wrong?**

1. **Insufficient balance or slippage**:
   ```rust
   let trade_params = TradeParams {
       slippage_tolerance: 0.05, // 5% slippage tolerance
       max_gas_fee: Some(0.01),  // Maximum gas fee in SOL
       ..Default::default()
   };
   ```

2. **Network congestion**:
   - Increase gas fees during high congestion
   - Use priority fee estimation

3. **Market conditions**:
   - High volatility periods
   - Low liquidity for specific tokens

4. **Wallet connectivity**:
   ```rust
   // Verify wallet connection
   let wallet_status = client.check_wallet_connection(&wallet_address).await?;
   ```

## Debugging & Diagnostics

### Enable debug logging

**Q: How do I get more detailed error information?**

1. **Set logging level**:
   ```bash
   export RUST_LOG=axiomtrade_rs=debug,reqwest=debug
   ```

2. **Custom logging configuration**:
   ```rust
   use tracing_subscriber;
   
   tracing_subscriber::fmt()
       .with_max_level(tracing::Level::DEBUG)
       .init();
   ```

3. **Network request logging**:
   ```rust
   let client = EnhancedAxiomClient::new()
       .with_debug_requests(true)
       .build();
   ```

### Performance profiling

**Q: My application is running slowly. How do I identify bottlenecks?**

1. **Enable performance metrics**:
   ```rust
   let client = EnhancedAxiomClient::new()
       .with_metrics(true)
       .build();
   
   // Get performance statistics
   let stats = client.get_performance_stats().await;
   ```

2. **Use async profiling tools**:
   ```bash
   cargo install tokio-console
   # Run with console subscriber
   ```

3. **Monitor resource usage**:
   ```rust
   // Check memory usage
   let memory_usage = client.get_memory_usage();
   
   // Check connection pool status
   let pool_stats = client.get_connection_pool_stats();
   ```

## Getting Help

### Community support

- **GitHub Issues**: Report bugs and feature requests
- **Discord**: Join the Axiom Trade community for real-time help
- **Documentation**: Check the latest docs for API changes

### Professional support

For business-critical applications:
- Contact Axiom Trade directly for premium support
- Consider professional services for custom integrations
- Enterprise SLA options available for high-volume traders

### Contributing

Found a bug or have a feature request?
1. Check existing GitHub issues
2. Create a detailed issue with reproduction steps
3. Consider submitting a pull request with fixes
