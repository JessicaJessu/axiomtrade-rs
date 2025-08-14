use crate::email::otp_fetcher::{from_env as otp_from_env, OtpFetcher};
use crate::utils::password::hashpassword;
use reqwest::{Client, header, Method, Response};
use serde_json::Value;
use std::time::Duration;
use super::error::AuthError;
use super::types::*;
use super::token_manager::TokenManager;

const API_ENDPOINTS: &[&str] = &[
    "https://api2.axiom.trade",
    "https://api3.axiom.trade",
    "https://api6.axiom.trade",
    "https://api7.axiom.trade",
    "https://api8.axiom.trade",
    "https://api9.axiom.trade",
    "https://api10.axiom.trade",
];

pub struct AuthClient {
    client: Client,
    otp_fetcher: Option<OtpFetcher>,
    token_manager: TokenManager,
    last_used_endpoint: Option<String>,
}

impl AuthClient {
    /// Creates a new authentication client with random user agent
    /// 
    /// # Returns
    /// 
    /// AuthClient - A new instance of the authentication client
    pub fn new() -> Result<Self, AuthError> {
        let user_agent = crate::utils::user_agents::get_random_desktop_user_agent();
        Self::new_with_user_agent(user_agent)
    }
    
    /// Creates a new authentication client with specific user agent
    /// 
    /// # Arguments
    /// 
    /// * `user_agent` - &str - User agent string to use for requests
    /// 
    /// # Returns
    /// 
    /// AuthClient - A new instance of the authentication client
    pub fn new_with_user_agent(user_agent: &str) -> Result<Self, AuthError> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", user_agent.parse().unwrap());
        headers.insert("Accept", "application/json, text/plain, */*".parse().unwrap());
        headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
        headers.insert("Origin", "https://axiom.trade".parse().unwrap());
        headers.insert("Referer", "https://axiom.trade/".parse().unwrap());
        
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()?;
        
        let otp_fetcher = otp_from_env()
            .map_err(|e| AuthError::EmailError(e.to_string()))?;
        
        let token_manager = TokenManager::new(Some(std::path::PathBuf::from(".axiom_tokens.json")));
        
        Ok(Self {
            client,
            otp_fetcher,
            token_manager,
            last_used_endpoint: None,
        })
    }
    
    /// Login with email and password
    /// 
    /// # Arguments
    /// 
    /// * `email` - &str - User's email address
    /// * `password` - &str - User's plain text password (will be hashed)
    /// * `otp_code` - Option<String> - Optional OTP code, will auto-fetch if not provided
    /// 
    /// # Returns
    /// 
    /// Result<LoginResult, AuthError> - Complete login result with tokens and Turnkey credentials
    pub async fn login_full(
        &mut self,
        email: &str,
        password: &str,
        otp_code: Option<String>,
    ) -> Result<LoginResult, AuthError> {
        let b64_password = hashpassword(password);
        
        let otp_jwt_token = self.login_step1(email, &b64_password).await?;
        
        let otp = match otp_code {
            Some(code) => code,
            None => self.fetch_otp().await?,
        };
        
        let result = self.login_step2_full(&otp_jwt_token, &otp, email, &b64_password).await?;
        
        Ok(result)
    }

    /// Login with email and password (legacy method for backward compatibility)
    /// 
    /// # Arguments
    /// 
    /// * `email` - &str - User's email address
    /// * `password` - &str - User's plain text password (will be hashed)
    /// * `otp_code` - Option<String> - Optional OTP code, will auto-fetch if not provided
    /// 
    /// # Returns
    /// 
    /// Result<AuthTokens, AuthError> - Authentication tokens on success
    pub async fn login(
        &mut self,
        email: &str,
        password: &str,
        otp_code: Option<String>,
    ) -> Result<AuthTokens, AuthError> {
        let result = self.login_full(email, password, otp_code).await?;
        Ok(result.tokens)
    }
    
    /// Login with pre-hashed password (full result)
    /// 
    /// # Arguments
    /// 
    /// * `email` - &str - User's email address
    /// * `b64_password` - &str - Base64 encoded hashed password
    /// * `otp_code` - Option<String> - Optional OTP code
    /// 
    /// # Returns
    /// 
    /// Result<LoginResult, AuthError> - Complete login result with tokens and Turnkey credentials
    pub async fn login_with_hash_full(
        &mut self,
        email: &str,
        b64_password: &str,
        otp_code: Option<String>,
    ) -> Result<LoginResult, AuthError> {
        let otp_jwt_token = self.login_step1(email, b64_password).await?;
        
        let otp = match otp_code {
            Some(code) => code,
            None => self.fetch_otp().await?,
        };
        
        let result = self.login_step2_full(&otp_jwt_token, &otp, email, b64_password).await?;
        
        Ok(result)
    }

    /// Login with pre-hashed password (legacy method for backward compatibility)
    /// 
    /// # Arguments
    /// 
    /// * `email` - &str - User's email address
    /// * `b64_password` - &str - Base64 encoded hashed password
    /// * `otp_code` - Option<String> - Optional OTP code
    /// 
    /// # Returns
    /// 
    /// Result<AuthTokens, AuthError> - Authentication tokens on success
    pub async fn login_with_hash(
        &mut self,
        email: &str,
        b64_password: &str,
        otp_code: Option<String>,
    ) -> Result<AuthTokens, AuthError> {
        let result = self.login_with_hash_full(email, b64_password, otp_code).await?;
        Ok(result.tokens)
    }
    
    /// Get the current API endpoint (selects a random one if needed)
    pub fn get_current_endpoint(&mut self) -> &'static str {
        self.get_random_endpoint()
    }
    
    /// Get a random API endpoint, excluding the last used one if possible
    fn get_random_endpoint(&mut self) -> &'static str {
        use rand::Rng;
        
        let available_endpoints: Vec<&str> = if let Some(ref last) = self.last_used_endpoint {
            API_ENDPOINTS.iter()
                .filter(|&&e| e != last.as_str())
                .copied()
                .collect()
        } else {
            API_ENDPOINTS.to_vec()
        };
        
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..available_endpoints.len());
        let endpoint = available_endpoints[index];
        
        self.last_used_endpoint = Some(endpoint.to_string());
        endpoint
    }
    
    /// First step of login process
    async fn login_step1(&mut self, email: &str, b64_password: &str) -> Result<String, AuthError> {
        let endpoint = self.get_random_endpoint();
        let url = format!("{}/login-password-v2", endpoint);
        
        let request = LoginRequest {
            email: email.to_string(),
            b64_password: b64_password.to_string(),
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            eprintln!("Login step 1 failed: {}", text);
            return Err(AuthError::InvalidCredentials);
        }
        
        let result: LoginStep1Response = response.json().await?;
        Ok(result.otp_jwt_token)
    }
    
    /// Second step of login process (full result with Turnkey credentials)
    async fn login_step2_full(
        &mut self,
        otp_jwt_token: &str,
        otp_code: &str,
        email: &str,
        b64_password: &str,
    ) -> Result<LoginResult, AuthError> {
        let endpoint = self.get_random_endpoint();
        let url = format!("{}/login-otp", endpoint);
        
        let request = OtpRequest {
            code: otp_code.to_string(),
            email: email.to_string(),
            b64_password: b64_password.to_string(),
        };
        
        let response = self.client
            .post(&url)
            .header("Cookie", format!("auth-otp-login-token={}", otp_jwt_token))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(AuthError::InvalidOtp);
        }
        
        let cookies = response.cookies();
        let mut access_token = None;
        let mut refresh_token = None;
        
        for cookie in cookies {
            match cookie.name() {
                "auth-access-token" => access_token = Some(cookie.value().to_string()),
                "auth-refresh-token" => refresh_token = Some(cookie.value().to_string()),
                _ => {}
            }
        }
        
        let response_data: LoginResponse = response.json().await?;
        
        let access = access_token
            .or(response_data.access_token)
            .ok_or(AuthError::TokenNotFound)?;
        let refresh = refresh_token
            .or(response_data.refresh_token)
            .ok_or(AuthError::TokenNotFound)?;
        
        let tokens = AuthTokens {
            access_token: access,
            refresh_token: refresh,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        };
        
        self.token_manager.set_tokens(tokens.clone()).await?;
        
        // Extract Turnkey credentials
        let turnkey_credentials = if let (Some(org_id), Some(user_id), Some(client_secret)) = 
            (&response_data.org_id, &response_data.user_id, &response_data.client_secret) {
            println!("🏛️  Captured Turnkey credentials:");
            println!("   • Organization ID: {}", org_id);
            println!("   • User ID: {}", user_id);
            println!("   • Client Secret: {}...", &client_secret[..std::cmp::min(8, client_secret.len())]);
            
            Some(TurnkeyCredentials {
                organization_id: org_id.clone(),
                user_id: user_id.clone(),
                client_secret: client_secret.clone(),
            })
        } else {
            println!("⚠️  No Turnkey credentials found in login response");
            None
        };
        
        Ok(LoginResult {
            tokens,
            turnkey_credentials,
            user_info: response_data.user,
        })
    }

    /// Second step of login process (legacy method for backward compatibility)
    async fn login_step2(
        &mut self,
        otp_jwt_token: &str,
        otp_code: &str,
        email: &str,
        b64_password: &str,
    ) -> Result<AuthTokens, AuthError> {
        let result = self.login_step2_full(otp_jwt_token, otp_code, email, b64_password).await?;
        Ok(result.tokens)
    }
    
    /// Fetch OTP automatically using configured email fetcher
    async fn fetch_otp(&self) -> Result<String, AuthError> {
        match &self.otp_fetcher {
            Some(fetcher) => {
                println!("Waiting for OTP email...");
                
                fetcher.wait_for_otp(120, 5)
                    .map_err(|e| AuthError::EmailError(e.to_string()))?
                    .ok_or(AuthError::EmailError("OTP not received within timeout".to_string()))
            }
            None => {
                Err(AuthError::OtpRequired)
            }
        }
    }
    
    /// Refresh access token using refresh token
    /// 
    /// # Arguments
    /// 
    /// * `refresh_token` - &str - The refresh token
    /// 
    /// # Returns
    /// 
    /// Result<String, AuthError> - New access token
    pub async fn refresh_token(&mut self, refresh_token: &str) -> Result<String, AuthError> {
        let endpoint = self.get_random_endpoint();
        let url = format!("{}/refresh-access-token", endpoint);
        
        let response = self.client
            .post(url)
            .header("Cookie", format!("auth-refresh-token={}", refresh_token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(AuthError::TokenExpired);
        }
        
        let cookies = response.cookies();
        for cookie in cookies {
            if cookie.name() == "auth-access-token" {
                return Ok(cookie.value().to_string());
            }
        }
        
        Err(AuthError::TokenNotFound)
    }
    
    /// Gets the current authentication tokens
    /// 
    /// # Returns
    /// 
    /// Option<AuthTokens> - The current tokens if available
    pub async fn get_tokens(&self) -> Option<AuthTokens> {
        self.token_manager.get_tokens().await
    }
    
    /// Refresh tokens if needed
    /// 
    /// # Returns
    /// 
    /// Result<AuthTokens, AuthError> - Updated tokens
    pub async fn refresh_tokens(&mut self) -> Result<AuthTokens, AuthError> {
        let tokens = self.token_manager.get_tokens().await
            .ok_or(AuthError::TokenNotFound)?;
        
        let new_access_token = self.refresh_token(&tokens.refresh_token).await?;
        
        let new_tokens = AuthTokens {
            access_token: new_access_token,
            refresh_token: tokens.refresh_token,
            expires_at: Some(chrono::Utc::now() + chrono::Duration::minutes(15)),
        };
        
        self.token_manager.set_tokens(new_tokens.clone()).await?;
        
        Ok(new_tokens)
    }
    
    /// Ensure we have valid authentication tokens
    /// Automatically refreshes or re-authenticates as needed
    /// 
    /// # Returns
    /// 
    /// Result<AuthTokens, AuthError> - Valid authentication tokens
    pub async fn ensure_valid_authentication(&mut self) -> Result<AuthTokens, AuthError> {
        let tokens = match self.token_manager.get_tokens().await {
            Some(t) => t,
            None => return Err(AuthError::NotAuthenticated),
        };
        
        if !tokens.is_expired() {
            return Ok(tokens);
        }
        
        match self.refresh_tokens().await {
            Ok(new_tokens) => Ok(new_tokens),
            Err(_) => {
                Err(AuthError::TokenExpired)
            }
        }
    }
    
    /// Makes an authenticated request to the API
    /// 
    /// # Arguments
    /// 
    /// * `method` - Method - HTTP method
    /// * `url` - &str - The URL to request
    /// * `body` - Option<Value> - Optional JSON body
    /// 
    /// # Returns
    /// 
    /// Result<Response, AuthError> - The response from the API
    pub async fn make_authenticated_request(
        &mut self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<Response, AuthError> {
        let tokens = self.token_manager.get_tokens().await
            .ok_or(AuthError::TokenNotFound)?;
        
        let mut request = self.client.request(method.clone(), url)
            .header("Cookie", format!("auth-access-token={}", tokens.access_token));
        
        if let Some(ref body_val) = body {
            request = request.json(body_val);
        }
        
        let response = request.send().await?;
        
        if response.status() == 401 {
            let new_access_token = self.refresh_token(&tokens.refresh_token).await?;
            
            let new_tokens = AuthTokens {
                access_token: new_access_token.clone(),
                refresh_token: tokens.refresh_token.clone(),
                expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            };
            self.token_manager.set_tokens(new_tokens).await?;
            
            let mut retry_request = self.client.request(method, url)
                .header("Cookie", format!("auth-access-token={}", new_access_token));
            
            if let Some(body_val) = body {
                retry_request = retry_request.json(&body_val);
            }
            
            return retry_request.send().await.map_err(|e| e.into());
        }
        
        Ok(response)
    }
}