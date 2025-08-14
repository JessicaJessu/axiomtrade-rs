use crate::auth::{AuthClient, AuthError};
use crate::websocket::handler::MessageHandler;
use crate::websocket::messages::{WebSocketMessage, SubscriptionType};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashSet;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde_json::json;
use rand::Rng;
use tokio::time::{interval, Duration};

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("Authentication error: {0}")]
    AuthError(#[from] AuthError),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Not connected")]
    NotConnected,
    
    #[error("Send error: {0}")]
    SendError(String),
    
    #[error("Receive error: {0}")]
    ReceiveError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] http::Error),
}

#[derive(Clone, Debug)]
pub enum Region {
    USWest,
    USCentral,
    USEast,
    EUWest,
    EUCentral,
    EUEast,
    Asia,
    Australia,
    Global,
}

impl Region {
    /// Gets WebSocket URLs for the region
    fn get_urls(&self) -> Vec<&'static str> {
        match self {
            Region::USWest => vec!["socket8.axiom.trade", "cluster-usw2.axiom.trade"],
            Region::USCentral => vec!["cluster3.axiom.trade", "cluster-usc2.axiom.trade"],
            Region::USEast => vec!["cluster5.axiom.trade", "cluster-use2.axiom.trade"],
            Region::EUWest => vec!["cluster6.axiom.trade", "cluster-euw2.axiom.trade"],
            Region::EUCentral => vec!["cluster2.axiom.trade", "cluster-euc2.axiom.trade"],
            Region::EUEast => vec!["cluster8.axiom.trade"],
            Region::Asia => vec!["cluster4.axiom.trade"],
            Region::Australia => vec!["cluster7.axiom.trade"],
            Region::Global => vec!["cluster9.axiom.trade"],
        }
    }
    
    /// Randomly selects a URL from the region's available URLs
    fn get_random_url(&self) -> &'static str {
        let urls = self.get_urls();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..urls.len());
        urls[index]
    }
}

pub struct WebSocketClient {
    auth_client: Arc<RwLock<AuthClient>>,
    region: Region,
    handler: Arc<dyn MessageHandler>,
    subscriptions: Arc<RwLock<HashSet<SubscriptionType>>>,
    sender: Option<futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message
    >>,
    is_connected: Arc<RwLock<bool>>,
    reconnect_on_expire: bool,
    is_token_price: bool,
}

impl WebSocketClient {
    /// Creates a new WebSocket client with default region (Global)
    /// 
    /// # Arguments
    /// 
    /// * `handler` - Arc<dyn MessageHandler> - Message handler
    /// 
    /// # Returns
    /// 
    /// Result<WebSocketClient, WebSocketError> - A new WebSocket client
    pub fn new(handler: Arc<dyn MessageHandler>) -> Result<Self, WebSocketError> {
        Ok(Self {
            auth_client: Arc::new(RwLock::new(AuthClient::new()?)),
            region: Region::Global,
            handler,
            subscriptions: Arc::new(RwLock::new(HashSet::new())),
            sender: None,
            is_connected: Arc::new(RwLock::new(false)),
            reconnect_on_expire: true,
            is_token_price: false,
        })
    }
    
    /// Creates a WebSocket client with specific region
    /// 
    /// # Arguments
    /// 
    /// * `handler` - Arc<dyn MessageHandler> - Message handler
    /// * `region` - Region - The region to connect to
    /// 
    /// # Returns
    /// 
    /// Result<WebSocketClient, WebSocketError> - A new WebSocket client
    pub fn with_region(
        handler: Arc<dyn MessageHandler>,
        region: Region,
    ) -> Result<Self, WebSocketError> {
        Ok(Self {
            auth_client: Arc::new(RwLock::new(AuthClient::new()?)),
            region,
            handler,
            subscriptions: Arc::new(RwLock::new(HashSet::new())),
            sender: None,
            is_connected: Arc::new(RwLock::new(false)),
            reconnect_on_expire: true,
            is_token_price: false,
        })
    }
    
    /// Connects to the WebSocket server
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if connected successfully
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        self.connect_with_token_price(false).await
    }
    
    /// Connects to the token price WebSocket server
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if connected successfully
    pub async fn connect_token_price(&mut self) -> Result<(), WebSocketError> {
        self.connect_with_token_price(true).await
    }
    
    /// Internal connection method
    async fn connect_with_token_price(&mut self, is_token_price: bool) -> Result<(), WebSocketError> {
        self.is_token_price = is_token_price;
        
        let auth_tokens = self.auth_client.write().await.ensure_valid_authentication().await
            .map_err(WebSocketError::AuthError)?;
        
        // Get URL for connection
        let host = if is_token_price {
            "socket8.axiom.trade"
        } else {
            self.region.get_random_url()
        };
        
        let url = format!("wss://{}/", host);
        
        // Build the HTTP request with custom headers
        let request = http::Request::builder()
            .method("GET")
            .uri(&url)
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", tungstenite::handshake::client::generate_key())
            // Add custom headers for authentication
            .header("Cookie", format!("auth-access-token={}; auth-refresh-token={}", 
                auth_tokens.access_token, auth_tokens.refresh_token))
            .header("Origin", "https://axiom.trade")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .header("Cache-Control", "no-cache")
            .header("Pragma", "no-cache")
            .header("Accept-Language", "en-US,en;q=0.9")
            .body(())?;
        
        // Connect to WebSocket
        match connect_async(request).await {
            Ok((ws_stream, _response)) => {
                *self.is_connected.write().await = true;
                
                let (write, mut read) = ws_stream.split();
                self.sender = Some(write);
                
                let handler = Arc::clone(&self.handler);
                let is_connected = Arc::clone(&self.is_connected);
                
                // Spawn read task
                tokio::spawn(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                // Try to parse as WebSocketMessage first
                                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                                    match &ws_msg {
                                        WebSocketMessage::Connected { session_id } => {
                                            handler.on_connected(session_id.clone()).await;
                                        }
                                        WebSocketMessage::Disconnected { reason } => {
                                            *is_connected.write().await = false;
                                            handler.on_disconnected(reason.clone()).await;
                                        }
                                        _ => {}
                                    }
                                    handler.handle_message(ws_msg).await;
                                } else if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                                    // Handle raw JSON messages from the server
                                    if let Some(room) = data.get("room").and_then(|r| r.as_str()) {
                                        if room == "new_pairs" {
                                            if let Some(content) = data.get("content") {
                                                // Convert to MarketUpdate
                                                let update = crate::websocket::messages::MarketUpdate {
                                                    token_mint: content.get("token_address")
                                                        .and_then(|a| a.as_str())
                                                        .unwrap_or("")
                                                        .to_string(),
                                                    symbol: content.get("token_ticker")
                                                        .and_then(|s| s.as_str())
                                                        .unwrap_or("")
                                                        .to_string(),
                                                    price_usd: content.get("initial_liquidity_sol")
                                                        .and_then(|p| p.as_f64())
                                                        .unwrap_or(0.0),
                                                    price_sol: 0.0,
                                                    price_change_24h: 0.0,
                                                    volume_24h: 0.0,
                                                    market_cap: content.get("supply")
                                                        .and_then(|s| s.as_f64())
                                                        .unwrap_or(0.0),
                                                    timestamp: chrono::Utc::now().timestamp(),
                                                };
                                                handler.handle_message(WebSocketMessage::MarketUpdate(update)).await;
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(Message::Close(_)) => {
                                *is_connected.write().await = false;
                                handler.on_disconnected("Connection closed".to_string()).await;
                                break;
                            }
                            Err(e) => {
                                *is_connected.write().await = false;
                                handler.on_error(format!("WebSocket error: {}", e)).await;
                                break;
                            }
                            _ => {}
                        }
                    }
                });
                
                if self.reconnect_on_expire {
                    self.spawn_token_refresh_task();
                }
                
                self.handler.on_connected(url).await;
                Ok(())
            }
            Err(e) => {
                Err(WebSocketError::WebSocketError(e))
            }
        }
    }
    
    /// Disconnects from the WebSocket server
    pub async fn disconnect(&mut self) {
        *self.is_connected.write().await = false;
        
        if let Some(mut sender) = self.sender.take() {
            let _ = sender.send(Message::Close(None)).await;
            let _ = sender.close().await;
        }
        
        self.subscriptions.write().await.clear();
        self.handler.on_disconnected("Manual disconnect".to_string()).await;
    }
    
    /// Subscribes to new token pairs
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if subscribed successfully
    pub async fn subscribe_new_tokens(&mut self) -> Result<(), WebSocketError> {
        if !*self.is_connected.read().await {
            return Err(WebSocketError::NotConnected);
        }
        
        let msg = json!({
            "action": "join",
            "room": "new_pairs"
        });
        
        self.send_message(msg).await?;
        self.subscriptions.write().await.insert(SubscriptionType::MarketData);
        
        Ok(())
    }
    
    /// Subscribes to token price updates
    /// 
    /// # Arguments
    /// 
    /// * `token_address` - &str - The token address to monitor
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if subscribed successfully
    pub async fn subscribe_token_price(&mut self, token_address: &str) -> Result<(), WebSocketError> {
        if !*self.is_connected.read().await {
            return Err(WebSocketError::NotConnected);
        }
        
        let msg = json!({
            "action": "join",
            "room": token_address
        });
        
        self.send_message(msg).await?;
        self.subscriptions.write().await.insert(SubscriptionType::PriceAlerts);
        
        Ok(())
    }
    
    /// Subscribes to wallet transaction updates
    /// 
    /// # Arguments
    /// 
    /// * `wallet_address` - &str - The wallet address to monitor
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if subscribed successfully
    pub async fn subscribe_wallet_transactions(&mut self, wallet_address: &str) -> Result<(), WebSocketError> {
        if !*self.is_connected.read().await {
            return Err(WebSocketError::NotConnected);
        }
        
        let msg = json!({
            "action": "join",
            "room": format!("v:{}", wallet_address)
        });
        
        self.send_message(msg).await?;
        self.subscriptions.write().await.insert(SubscriptionType::Portfolio);
        
        Ok(())
    }
    
    /// Sends a message to the WebSocket
    /// 
    /// # Arguments
    /// 
    /// * `message` - serde_json::Value - The message to send
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if sent successfully
    async fn send_message(&mut self, message: serde_json::Value) -> Result<(), WebSocketError> {
        if let Some(sender) = &mut self.sender {
            let json = serde_json::to_string(&message)
                .map_err(|e| WebSocketError::SerializationError(e.to_string()))?;
            
            sender.send(Message::Text(json)).await
                .map_err(|e| WebSocketError::SendError(e.to_string()))?;
            
            Ok(())
        } else {
            Err(WebSocketError::NotConnected)
        }
    }
    
    /// Checks if the client is connected
    /// 
    /// # Returns
    /// 
    /// bool - True if connected
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.read().await
    }
    
    /// Gets current subscriptions
    /// 
    /// # Returns
    /// 
    /// HashSet<SubscriptionType> - Set of active subscriptions
    pub async fn get_subscriptions(&self) -> HashSet<SubscriptionType> {
        self.subscriptions.read().await.clone()
    }
    
    /// Set whether to automatically reconnect when tokens expire
    /// 
    /// # Arguments
    /// 
    /// * `enabled` - bool - Whether to enable auto-reconnect
    pub fn set_auto_reconnect(&mut self, enabled: bool) {
        self.reconnect_on_expire = enabled;
    }
    
    /// Spawn a task to periodically refresh tokens
    fn spawn_token_refresh_task(&self) {
        let auth_client = Arc::clone(&self.auth_client);
        let is_connected = Arc::clone(&self.is_connected);
        let handler = Arc::clone(&self.handler);
        
        tokio::spawn(async move {
            let mut refresh_interval = interval(Duration::from_secs(600));
            
            loop {
                refresh_interval.tick().await;
                
                if !*is_connected.read().await {
                    break;
                }
                
                match auth_client.write().await.ensure_valid_authentication().await {
                    Ok(_) => {
                        handler.on_connected("Token refreshed".to_string()).await;
                    }
                    Err(e) => {
                        handler.on_error(format!("Token refresh failed: {}", e)).await;
                        *is_connected.write().await = false;
                        handler.on_disconnected("Token expired".to_string()).await;
                        break;
                    }
                }
            }
        });
    }
    
    /// Reconnect with fresh tokens
    /// 
    /// # Returns
    /// 
    /// Result<(), WebSocketError> - Ok if reconnected successfully
    pub async fn reconnect(&mut self) -> Result<(), WebSocketError> {
        self.disconnect().await;
        
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        self.auth_client.write().await.ensure_valid_authentication().await
            .map_err(WebSocketError::AuthError)?;
        
        if self.is_token_price {
            self.connect_token_price().await?
        } else {
            self.connect().await?
        }
        
        let subs = self.subscriptions.read().await.clone();
        for sub in subs {
            match sub {
                SubscriptionType::MarketData => {
                    self.subscribe_new_tokens().await?;
                }
                SubscriptionType::PriceAlerts => {
                }
                SubscriptionType::Portfolio => {
                }
                _ => {}
            }
        }
        
        Ok(())
    }
}