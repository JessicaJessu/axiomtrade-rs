# Debugging Guide

This guide provides comprehensive debugging techniques for axiomtrade-rs, covering logging configuration, network inspection, WebSocket debugging, and common error pattern identification.

## Enable Debug Logging

### Basic Logging Setup

The axiomtrade-rs library uses the `tracing` crate for structured logging. To enable debug logging in your application:

```rust
use tracing::{info, debug, error, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axiomtrade_rs=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() {
    init_logging();
    // Your application code here
}
```

### Log Levels

Configure different log levels for different components:

```rust
// Maximum verbosity - shows all internal operations
tracing_subscriber::EnvFilter::new("axiomtrade_rs=trace,debug")

// Standard debugging - shows important operations and errors
tracing_subscriber::EnvFilter::new("axiomtrade_rs=debug,info")

// Production logging - errors and warnings only
tracing_subscriber::EnvFilter::new("axiomtrade_rs=warn,error")
```

### Structured Logging

Use structured logging to capture context:

```rust
use tracing::{info, error, Span, span, Level};

let span = span!(Level::INFO, "trading_operation", 
    operation = "buy", 
    token = "SOL", 
    amount = 1.0
);
let _enter = span.enter();

info!("Starting trade execution");
// Trading logic here
```

## Using RUST_LOG Environment Variable

### Basic Configuration

Set the `RUST_LOG` environment variable to control logging verbosity:

```bash
# Windows Command Prompt
set RUST_LOG=axiomtrade_rs=debug
cargo run --example trading_demo

# Windows PowerShell
$env:RUST_LOG="axiomtrade_rs=debug"
cargo run --example trading_demo

# Linux/macOS
export RUST_LOG=axiomtrade_rs=debug
cargo run --example trading_demo
```

### Advanced RUST_LOG Patterns

```bash
# Enable debug logging for all modules
RUST_LOG=debug

# Enable specific module logging
RUST_LOG=axiomtrade_rs::auth=debug,axiomtrade_rs::websocket=trace

# Enable reqwest HTTP client debugging
RUST_LOG=axiomtrade_rs=debug,reqwest=debug,hyper=debug

# Filter by specific operations
RUST_LOG=axiomtrade_rs::trading=debug,axiomtrade_rs::portfolio=info

# Exclude noisy modules
RUST_LOG=debug,h2=off,rustls=off
```

### Environment-Specific Configurations

Create different logging configurations for different environments:

```bash
# Development
RUST_LOG=axiomtrade_rs=debug,reqwest=debug

# Testing
RUST_LOG=axiomtrade_rs=trace,test

# Production
RUST_LOG=axiomtrade_rs=warn,error
```

## Inspecting Network Requests

### HTTP Request Debugging

Enable detailed HTTP request/response logging:

```rust
use reqwest::Client;
use tracing::{debug, info};

// In your client configuration
let client = Client::builder()
    .connection_verbose(true)  // Enable connection debugging
    .build()?;

// Log requests manually
debug!(
    method = %request.method(),
    url = %request.url(),
    headers = ?request.headers(),
    "Sending HTTP request"
);

let response = client.execute(request).await?;

debug!(
    status = %response.status(),
    headers = ?response.headers(),
    "Received HTTP response"
);
```

### Request/Response Interception

Create a middleware to log all API calls:

```rust
use reqwest::{Request, Response};
use tracing::{debug, error};

pub async fn log_request_response(
    request: Request,
    client: &Client,
) -> Result<Response, reqwest::Error> {
    let url = request.url().clone();
    let method = request.method().clone();
    
    debug!(
        method = %method,
        url = %url,
        "Sending request to Axiom API"
    );
    
    let start_time = std::time::Instant::now();
    let response = client.execute(request).await;
    let duration = start_time.elapsed();
    
    match &response {
        Ok(resp) => {
            debug!(
                status = %resp.status(),
                duration_ms = duration.as_millis(),
                "Received response from Axiom API"
            );
        }
        Err(e) => {
            error!(
                error = %e,
                duration_ms = duration.as_millis(),
                "Request failed"
            );
        }
    }
    
    response
}
```

### Capture Full Request/Response Bodies

```rust
use tracing::debug;

// For debugging authentication issues
let body = response.text().await?;
debug!(
    response_body = %body,
    "Full response body received"
);

// For request body debugging
let request_body = serde_json::to_string(&payload)?;
debug!(
    request_body = %request_body,
    "Sending request body"
);
```

### Network-Level Debugging

For deeper network inspection, use external tools:

```bash
# Using curl to replicate requests
curl -X POST "https://api.axiom.trade/auth/login" \
     -H "Content-Type: application/json" \
     -H "User-Agent: axiomtrade-rs/1.0.0" \
     -d '{"email":"user@example.com","password_hash":"..."}' \
     -v

# Using tcpdump to capture network traffic (Linux/macOS)
sudo tcpdump -i any -A -s 0 host api.axiom.trade

# Using Wireshark for GUI network analysis
# Filter: host api.axiom.trade
```

## Debugging WebSocket Connections

### WebSocket Connection Logging

Enable comprehensive WebSocket debugging:

```rust
use tracing::{debug, info, warn, error};
use tokio_tungstenite::{connect_async, tungstenite::Message};

async fn debug_websocket_connection(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!(url = %url, "Attempting WebSocket connection");
    
    let (ws_stream, response) = connect_async(url).await
        .map_err(|e| {
            error!(error = %e, "WebSocket connection failed");
            e
        })?;
    
    info!(
        status = response.status().as_u16(),
        "WebSocket handshake completed"
    );
    
    let (mut write, mut read) = ws_stream.split();
    
    // Log all incoming messages
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                debug!(message = %text, "Received WebSocket text message");
            }
            Ok(Message::Binary(data)) => {
                debug!(
                    size = data.len(),
                    "Received WebSocket binary message"
                );
            }
            Ok(Message::Close(frame)) => {
                info!(frame = ?frame, "WebSocket connection closed");
                break;
            }
            Err(e) => {
                error!(error = %e, "WebSocket message error");
                break;
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### WebSocket Message Tracing

Track message flow and timing:

```rust
use std::time::Instant;
use tracing::{debug, span, Level};

pub struct WebSocketMessageTracer {
    sent_messages: HashMap<String, Instant>,
}

impl WebSocketMessageTracer {
    pub fn track_sent_message(&mut self, message_id: &str) {
        let span = span!(Level::DEBUG, "ws_message_sent", message_id = message_id);
        let _enter = span.enter();
        
        self.sent_messages.insert(message_id.to_string(), Instant::now());
        debug!(message_id = message_id, "WebSocket message sent");
    }
    
    pub fn track_received_response(&mut self, message_id: &str) {
        if let Some(sent_time) = self.sent_messages.remove(message_id) {
            let duration = sent_time.elapsed();
            debug!(
                message_id = message_id,
                response_time_ms = duration.as_millis(),
                "WebSocket response received"
            );
        }
    }
}
```

### Connection State Monitoring

Monitor WebSocket connection health:

```rust
use std::time::{Duration, Instant};
use tokio::time::{interval, timeout};

pub struct ConnectionMonitor {
    last_message: Instant,
    ping_interval: Duration,
}

impl ConnectionMonitor {
    pub fn new() -> Self {
        Self {
            last_message: Instant::now(),
            ping_interval: Duration::from_secs(30),
        }
    }
    
    pub async fn monitor_connection(&mut self, ws_sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
        let mut ping_timer = interval(self.ping_interval);
        
        loop {
            ping_timer.tick().await;
            
            if self.last_message.elapsed() > Duration::from_secs(60) {
                warn!("No messages received for 60 seconds, sending ping");
                
                if let Err(e) = ws_sender.send(Message::Ping(vec![])).await {
                    error!(error = %e, "Failed to send ping");
                    break;
                }
            }
        }
    }
    
    pub fn update_last_message_time(&mut self) {
        self.last_message = Instant::now();
    }
}
```

### WebSocket Debugging Tools

External tools for WebSocket debugging:

```bash
# Using websocat to test WebSocket connections
websocat wss://ws.axiom.trade/v1/stream -v

# Using wscat (Node.js)
npm install -g wscat
wscat -c wss://ws.axiom.trade/v1/stream

# Browser WebSocket debugging
# Open Chrome DevTools → Network → WS tab
# Shows all WebSocket frames and timing
```

## Common Error Patterns

### Authentication Errors

```rust
// Pattern: Invalid credentials
match error {
    AxiomError::AuthenticationFailed { message } => {
        error!(
            error_type = "authentication_failed",
            message = %message,
            "Check email/password combination"
        );
        // Check: email format, password hashing, OTP requirements
    }
    AxiomError::TokenExpired => {
        warn!("Access token expired, attempting refresh");
        // Implement automatic token refresh
    }
    AxiomError::InvalidOtp => {
        error!("OTP verification failed");
        // Check: OTP format, timing, auto-fetcher configuration
    }
}
```

### Rate Limiting Issues

```rust
// Pattern: Rate limit exceeded
match error {
    AxiomError::RateLimitExceeded { retry_after } => {
        warn!(
            retry_after_seconds = retry_after,
            "Rate limit exceeded, backing off"
        );
        tokio::time::sleep(Duration::from_secs(retry_after as u64)).await;
        // Implement exponential backoff
    }
}
```

### Network Connectivity Problems

```rust
// Pattern: Network timeouts and connection issues
match error {
    AxiomError::NetworkError { source } => {
        error!(error = %source, "Network connectivity issue");
        // Check: internet connection, DNS resolution, firewall
    }
    AxiomError::TimeoutError => {
        warn!("Request timeout, retrying with longer timeout");
        // Increase timeout values or implement retry logic
    }
}
```

### WebSocket Connection Issues

```rust
// Pattern: WebSocket disconnections
match ws_error {
    AxiomError::WebSocketConnectionLost => {
        warn!("WebSocket connection lost, attempting reconnection");
        // Implement automatic reconnection with exponential backoff
    }
    AxiomError::WebSocketAuthenticationFailed => {
        error!("WebSocket authentication failed");
        // Check: token validity, subscription format
    }
}
```

### API Response Parsing Errors

```rust
// Pattern: Malformed API responses
match error {
    AxiomError::ParseError { response, source } => {
        error!(
            response_body = %response,
            parse_error = %source,
            "Failed to parse API response"
        );
        // Check: API version compatibility, response format changes
    }
}
```

### Trading Operation Errors

```rust
// Pattern: Trading execution failures
match error {
    AxiomError::InsufficientBalance { required, available } => {
        error!(
            required_amount = %required,
            available_amount = %available,
            "Insufficient balance for trade"
        );
    }
    AxiomError::InvalidTokenAddress { address } => {
        error!(
            token_address = %address,
            "Invalid or unsupported token address"
        );
    }
    AxiomError::SlippageExceeded { expected, actual } => {
        warn!(
            expected_slippage = %expected,
            actual_slippage = %actual,
            "Trade slippage exceeded tolerance"
        );
    }
}
```

### Debugging Strategies

1. **Start with Basic Logging**: Enable `RUST_LOG=debug` to see general operation flow
2. **Isolate Components**: Test authentication, WebSocket, and trading separately
3. **Check Network Layer**: Use external tools to verify API endpoints
4. **Monitor Resource Usage**: Check memory and CPU usage during operations
5. **Test with Minimal Examples**: Create simple reproduction cases
6. **Compare with Python Implementation**: Use the `oldstuff/axiompy` reference

### Performance Debugging

```rust
use std::time::Instant;
use tracing::{debug, warn};

// Measure operation performance
let start = Instant::now();
let result = some_expensive_operation().await;
let duration = start.elapsed();

if duration > Duration::from_millis(1000) {
    warn!(
        operation = "expensive_operation",
        duration_ms = duration.as_millis(),
        "Slow operation detected"
    );
} else {
    debug!(
        operation = "expensive_operation",
        duration_ms = duration.as_millis(),
        "Operation completed"
    );
}
```

This debugging guide provides comprehensive tools and techniques for troubleshooting issues in axiomtrade-rs. Use these patterns to identify and resolve problems efficiently during development and production use.
