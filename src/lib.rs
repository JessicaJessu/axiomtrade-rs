pub mod auth;
pub mod email;
pub mod utils;
pub mod api;
pub mod models;
pub mod websocket;
pub mod client;
pub mod errors;

// Re-export main types for convenience
pub use client::EnhancedClient;
pub use auth::AuthClient;
pub use auth::TokenManager;
pub use websocket::WebSocketClient;
pub use errors::{AxiomError, Result};