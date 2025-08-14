use crate::errors::{AxiomError, Result};
use crate::models::turnkey::*;
use crate::auth::types::{TurnkeySession, TurnkeyApiKey};
use crate::utils::p256_crypto::{recreate_keypair_from_client_secret, sign_message};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{json, Value};
use chrono::{DateTime, Utc};

/// Turnkey wallet management client
/// Provides secure key management and cryptographic operations
#[derive(Clone)]
pub struct TurnkeyClient {
    client: reqwest::Client,
    base_url: String,
    organization_id: Option<String>,
    user_id: Option<String>,
    user_password: Option<String>,
}

impl TurnkeyClient {
    /// Create a new Turnkey client
    ///
    /// Returns:
    ///     Self: New instance of TurnkeyClient
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.turnkey.com".to_string(),
            organization_id: None,
            user_id: None,
            user_password: None,
        }
    }
    
    /// Set organization and user IDs for authenticated requests
    ///
    /// Args:
    ///     organization_id: &str - Turnkey organization ID
    ///     user_id: &str - Turnkey user ID
    ///     user_password: &str - User password for P256 key generation
    pub fn set_credentials(&mut self, organization_id: &str, user_id: &str, user_password: &str) {
        self.organization_id = Some(organization_id.to_string());
        self.user_id = Some(user_id.to_string());
        self.user_password = Some(user_password.to_string());
    }
    
    /// Sign a request payload with P256 private key for Turnkey API
    ///
    /// Args:
    ///     payload: &[u8] - Request payload bytes to sign
    ///     client_secret: &str - Base64 encoded client secret from session
    ///
    /// Returns:
    ///     Result<String> - Base64URL encoded signature JSON for X-Stamp header
    fn sign_request(&self, payload: &[u8], client_secret: &str) -> Result<String> {
        let password = self.user_password.as_ref()
            .ok_or_else(|| AxiomError::Crypto {
                message: "User password not set for signing".to_string(),
            })?;
        
        let keypair = recreate_keypair_from_client_secret(password, client_secret)?;
        let signature_bytes = sign_message(payload, &keypair.private_key)?;
        
        // Create Turnkey signature object
        let signature_object = json!({
            "publicKey": keypair.public_key,
            "scheme": "SIGNATURE_SCHEME_TK_API_P256",
            "signature": hex::encode(&signature_bytes)
        });
        
        // Convert to JSON string
        let signature_json = serde_json::to_string(&signature_object)?;
        
        // Base64 encode (same as working example)
        let base64_encoded = general_purpose::STANDARD.encode(signature_json.as_bytes());
        
        Ok(base64_encoded)
    }
    
    /// Get user identity information
    ///
    /// Args:
    ///     organization_id: &str - Organization ID to query
    ///     client_secret: &str - Client secret for request signing
    ///
    /// Returns:
    ///     Result<TurnkeyWhoAmI>: User identity and organization information
    pub async fn whoami(&self, organization_id: &str, client_secret: &str) -> Result<TurnkeyWhoAmI> {
        let url = format!("{}/public/v1/query/whoami", self.base_url);
        let payload = WhoAmIRequest {
            organization_id: organization_id.to_string(),
        };
        
        let payload_json = serde_json::to_string(&payload)?;
        let signature = self.sign_request(payload_json.as_bytes(), client_secret)?;
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "text/plain;charset=UTF-8")
            .header("Origin", "https://axiom.trade")
            .header("Referer", "https://axiom.trade/")
            .header("x-client-version", "@turnkey/sdk-server@1.7.3")
            .header("X-Stamp", signature)
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            let whoami: TurnkeyWhoAmI = response.json().await?;
            Ok(whoami)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("Turnkey whoami failed {}: {}", status, error_text),
            })
        }
    }
    
    /// Get API keys for a user
    ///
    /// Args:
    ///     user_id: &str - User ID to get keys for
    ///     organization_id: &str - Organization ID
    ///     client_secret: &str - Client secret for request signing
    ///
    /// Returns:
    ///     Result<GetApiKeysResponse>: List of API keys for the user
    pub async fn get_api_keys(&self, user_id: &str, organization_id: &str, client_secret: &str) -> Result<GetApiKeysResponse> {
        let url = format!("{}/public/v1/query/get_api_keys", self.base_url);
        let payload = GetApiKeysRequest {
            user_id: user_id.to_string(),
            organization_id: organization_id.to_string(),
        };
        
        let payload_json = serde_json::to_string(&payload)?;
        let signature = self.sign_request(payload_json.as_bytes(), client_secret)?;
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "text/plain;charset=UTF-8")
            .header("Origin", "https://axiom.trade")
            .header("Referer", "https://axiom.trade/")
            .header("x-client-version", "@turnkey/sdk-server@1.7.3")
            .header("X-Stamp", signature)
            .json(&payload)
            .send()
            .await?;
            
        if response.status().is_success() {
            let api_keys: GetApiKeysResponse = response.json().await?;
            Ok(api_keys)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AxiomError::Api {
                message: format!("Turnkey get_api_keys failed {}: {}", status, error_text),
            })
        }
    }
    
    /// Create a read/write session for secure operations
    ///
    /// Args:
    ///     organization_id: &str - Organization ID
    ///     user_id: &str - User ID
    ///     target_public_key: &str - Target public key for the session
    ///     api_key_name: &str - Name for the new API key
    ///
    /// Returns:
    ///     Result<bool>: Success status of session creation
    pub async fn create_read_write_session(
        &self,
        organization_id: &str,
        user_id: &str,
        target_public_key: &str,
        api_key_name: &str,
    ) -> Result<bool> {
        let url = format!("{}/public/v1/submit/create_read_write_session", self.base_url);
        
        let request = CreateReadWriteSessionRequest {
            parameters: CreateSessionParameters {
                api_key_name: api_key_name.to_string(),
                target_public_key: target_public_key.to_string(),
                user_id: user_id.to_string(),
                expiration_seconds: "2592000".to_string(), // 30 days
            },
            organization_id: organization_id.to_string(),
            timestamp_ms: Utc::now().timestamp_millis().to_string(),
            activity_type: "ACTIVITY_TYPE_CREATE_READ_WRITE_SESSION_V2".to_string(),
        };
        
        let response = self.client
            .post(&url)
            .header("Content-Type", "text/plain;charset=UTF-8")
            .header("Origin", "https://axiom.trade")
            .header("Referer", "https://axiom.trade/")
            .header("x-client-version", "@turnkey/sdk-server@1.7.3")
            .json(&request)
            .send()
            .await?;
            
        Ok(response.status().is_success())
    }
    
    /// Parse Turnkey session from API keys response
    ///
    /// Args:
    ///     whoami: &TurnkeyWhoAmI - User identity information
    ///     api_keys_response: &GetApiKeysResponse - API keys from Turnkey
    ///     client_secret: &str - Client secret for P256 operations
    ///
    /// Returns:
    ///     TurnkeySession: Parsed session information for storage
    pub fn parse_session(
        &self,
        whoami: &TurnkeyWhoAmI,
        api_keys_response: &GetApiKeysResponse,
        client_secret: &str,
    ) -> TurnkeySession {
        let api_keys: Vec<TurnkeyApiKey> = api_keys_response
            .api_keys
            .iter()
            .map(|key| {
                let created_at = self.parse_turnkey_timestamp(&key.created_at);
                let expires_at = key.expiration_seconds.as_ref().and_then(|exp_str| {
                    if let Ok(exp_seconds) = exp_str.parse::<i64>() {
                        Some(created_at + chrono::Duration::seconds(exp_seconds))
                    } else {
                        None
                    }
                });
                
                TurnkeyApiKey {
                    api_key_id: key.api_key_id.clone(),
                    api_key_name: key.api_key_name.clone(),
                    public_key: key.credential.public_key.clone(),
                    key_type: key.credential.credential_type.clone(),
                    created_at,
                    expires_at,
                }
            })
            .collect();
            
        // Calculate earliest expiration time
        let expires_at = api_keys
            .iter()
            .filter_map(|key| key.expires_at)
            .min();
            
        TurnkeySession {
            organization_id: whoami.organization_id.clone(),
            user_id: whoami.user_id.clone(),
            username: whoami.username.clone(),
            client_secret: client_secret.to_string(),
            api_keys,
            created_at: Utc::now(),
            expires_at,
        }
    }
    
    /// Parse Turnkey timestamp format to DateTime
    fn parse_turnkey_timestamp(&self, timestamp: &TurnkeyTimestamp) -> DateTime<Utc> {
        if let Ok(seconds) = timestamp.seconds.parse::<i64>() {
            DateTime::from_timestamp(seconds, 0).unwrap_or_else(Utc::now)
        } else {
            Utc::now()
        }
    }
    
    /// Check if Turnkey service is healthy
    ///
    /// Returns:
    ///     Result<bool>: True if service is responding
    pub async fn health_check(&self) -> Result<bool> {
        // Simple health check by making a basic request
        let response = self.client
            .get(&format!("{}/public/v1/health", self.base_url))
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await;
            
        match response {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }
    
    /// Get session key for specific operations
    ///
    /// Args:
    ///     session: &TurnkeySession - Active Turnkey session
    ///     key_type: &str - Type of key needed
    ///
    /// Returns:
    ///     Option<&TurnkeyApiKey>: The appropriate API key if found
    pub fn get_session_key<'a>(
        &self,
        session: &'a TurnkeySession,
        key_type: &str,
    ) -> Option<&'a TurnkeyApiKey> {
        session
            .api_keys
            .iter()
            .find(|key| key.key_type == key_type)
    }
    
    /// Generate session summary for debugging
    ///
    /// Args:
    ///     session: &TurnkeySession - Session to summarize
    ///
    /// Returns:
    ///     String: Human-readable session summary
    pub fn session_summary(&self, session: &TurnkeySession) -> String {
        let active_keys = session.api_keys.len();
        let expired_keys = session
            .api_keys
            .iter()
            .filter(|key| {
                key.expires_at
                    .map_or(false, |exp| Utc::now() > exp)
            })
            .count();
            
        let session_age = (Utc::now() - session.created_at).num_minutes();
        let expires_in = session
            .expires_at
            .map(|exp| (exp - Utc::now()).num_minutes())
            .unwrap_or(-1);
            
        format!(
            "Turnkey Session - User: {}, Keys: {}/{} active, Age: {}m, Expires: {}m",
            session.username,
            active_keys - expired_keys,
            active_keys,
            session_age,
            expires_in
        )
    }
}