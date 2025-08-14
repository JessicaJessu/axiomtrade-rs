use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Turnkey API key credential information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnkeyCredential {
    pub public_key: String,
    #[serde(rename = "type")]
    pub credential_type: String,
}

/// Turnkey API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnkeyApiKey {
    pub credential: TurnkeyCredential,
    pub api_key_id: String,
    pub api_key_name: String,
    pub created_at: TurnkeyTimestamp,
    pub updated_at: TurnkeyTimestamp,
    pub expiration_seconds: Option<String>,
}

/// Turnkey timestamp format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnkeyTimestamp {
    pub seconds: String,
    pub nanos: String,
}

/// Response from get_api_keys endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiKeysResponse {
    pub api_keys: Vec<TurnkeyApiKey>,
}

/// User identity information from whoami endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnkeyWhoAmI {
    pub organization_id: String,
    pub organization_name: String,
    pub user_id: String,
    pub username: String,
}

/// Request to get API keys
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApiKeysRequest {
    pub user_id: String,
    pub organization_id: String,
}

/// Request for whoami endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhoAmIRequest {
    pub organization_id: String,
}

/// Create read/write session parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSessionParameters {
    pub api_key_name: String,
    pub target_public_key: String,
    pub user_id: String,
    pub expiration_seconds: String,
}

/// Create read/write session request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReadWriteSessionRequest {
    pub parameters: CreateSessionParameters,
    pub organization_id: String,
    pub timestamp_ms: String,
    #[serde(rename = "type")]
    pub activity_type: String,
}

/// Credential types used by Turnkey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    #[serde(rename = "CREDENTIAL_TYPE_READ_WRITE_SESSION_KEY_P256")]
    ReadWriteSessionKeyP256,
    #[serde(rename = "CREDENTIAL_TYPE_API_KEY_P256")]
    ApiKeyP256,
}

/// Activity types for Turnkey operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    #[serde(rename = "ACTIVITY_TYPE_CREATE_READ_WRITE_SESSION_V2")]
    CreateReadWriteSessionV2,
}

/// Turnkey authentication stamp for request signing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnkeyStamp {
    pub public_key: String,
    pub scheme: String,
    pub signature: String,
}

/// Signature schemes used by Turnkey
pub const SIGNATURE_SCHEME_TK_API_P256: &str = "SIGNATURE_SCHEME_TK_API_P256";

/// Turnkey session information for Axiom integration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AxiomTurnkeySession {
    pub organization_id: String,
    pub user_id: String,
    pub username: String,
    pub session_keys: Vec<TurnkeyApiKey>,
    pub password_key: Option<TurnkeyApiKey>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

impl AxiomTurnkeySession {
    /// Get the active session key for operations
    pub fn get_active_session_key(&self) -> Option<&TurnkeyApiKey> {
        self.session_keys
            .iter()
            .find(|key| key.credential.credential_type == "CREDENTIAL_TYPE_READ_WRITE_SESSION_KEY_P256")
    }
    
    /// Get the password key for authentication
    pub fn get_password_key(&self) -> Option<&TurnkeyApiKey> {
        self.password_key.as_ref()
            .filter(|key| key.credential.credential_type == "CREDENTIAL_TYPE_API_KEY_P256")
    }
    
    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }
    
    /// Check if session needs renewal (expires within 1 hour)
    pub fn needs_renewal(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let one_hour = chrono::Duration::hours(1);
            Utc::now() + one_hour > expires_at
        } else {
            false
        }
    }
}

/// Error types for Turnkey operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnkeyError {
    pub error_code: String,
    pub error_message: String,
    pub details: Option<serde_json::Value>,
}

/// Common Turnkey error codes
pub mod error_codes {
    pub const INVALID_SIGNATURE: &str = "INVALID_SIGNATURE";
    pub const SESSION_EXPIRED: &str = "SESSION_EXPIRED";
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const RATE_LIMITED: &str = "RATE_LIMITED";
    pub const INVALID_REQUEST: &str = "INVALID_REQUEST";
}