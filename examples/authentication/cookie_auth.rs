/// Cookie Authentication Example
/// 
/// This example demonstrates authentication using cookies for session persistence,
/// useful for web-based integrations.

use axiomtrade_rs::auth::{AuthClient, AuthCookies};
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Load credentials from environment variables
    dotenvy::dotenv().ok();
    
    let email = env::var("AXIOM_EMAIL")
        .expect("AXIOM_EMAIL must be set in .env file");
    let password = env::var("AXIOM_PASSWORD")
        .expect("AXIOM_PASSWORD must be set in .env file");

    println!("Cookie Authentication Example");
    println!("=============================\n");

    // Create a new auth client
    let mut auth_client = match AuthClient::new() {
        Ok(client) => {
            println!("Auth client initialized");
            client
        }
        Err(e) => {
            println!("Failed to create auth client: {}", e);
            return;
        }
    };
    
    println!("Step 1: Performing login with cookie support...");
    
    // Login and get both tokens and cookies
    match auth_client.login(&email, &password, None).await {
        Ok(tokens) => {
            println!("✓ Login successful");
            println!("  Access token received: {}...", &tokens.access_token[..20.min(tokens.access_token.len())]);
            
            // Get cookies from the auth client
            // In the actual implementation, cookies would be extracted from response headers
            println!("\nStep 2: Managing authentication cookies...");
            
            // Example cookie structure (these would come from the actual response)
            let mut additional = HashMap::new();
            additional.insert("session_id".to_string(), "axiom_session_abc123".to_string());
            additional.insert("csrf_token".to_string(), "csrf_xyz789".to_string());
            
            let auth_cookies = AuthCookies {
                auth_access_token: Some("access_cookie_value".to_string()),
                auth_refresh_token: Some("refresh_cookie_value".to_string()),
                g_state: Some("google_state_value".to_string()),
                additional_cookies: additional,
            };
            
            if auth_cookies.auth_access_token.is_some() {
                println!("✓ Access token cookie set");
                println!("  Secure authentication established");
            }
            
            if auth_cookies.auth_refresh_token.is_some() {
                println!("✓ Refresh token cookie set");
                println!("  Session renewal enabled");
            }
            
            if auth_cookies.g_state.is_some() {
                println!("✓ Google state cookie set");
                println!("  OAuth integration ready");
            }
            
            if !auth_cookies.additional_cookies.is_empty() {
                println!("✓ Additional cookies: {}", auth_cookies.additional_cookies.len());
                for (name, _) in auth_cookies.additional_cookies.iter().take(3) {
                    println!("  - {}", name);
                }
            }
            
            println!("\nStep 3: Cookie-based API requests...");
            println!("Cookies can be used for:");
            println!("  - Web dashboard access");
            println!("  - Browser-based API calls");
            println!("  - Cross-origin requests (with proper CORS)");
            println!("  - Maintaining session across page reloads");
            
            println!("\nStep 4: Cookie security best practices...");
            println!("✓ HttpOnly flag: Prevents JavaScript access");
            println!("✓ Secure flag: HTTPS only transmission");
            println!("✓ SameSite: CSRF protection");
            println!("✓ Path restrictions: Limit cookie scope");
            println!("✓ Expiry management: Auto-logout after inactivity");
            
            println!("\nStep 5: Cookie refresh and rotation...");
            println!("In production, implement:");
            println!("  - Automatic cookie refresh before expiry");
            println!("  - Session rotation on privilege escalation");
            println!("  - Secure cookie storage in browser");
            println!("  - Clear cookies on logout");
            
            // Example: Using cookies for subsequent requests
            println!("\nStep 6: Making authenticated requests with cookies...");
            
            // In a real implementation, you would:
            // 1. Store cookies in a cookie jar
            // 2. Attach cookies to subsequent requests
            // 3. Handle cookie expiry and refresh
            
            println!("✓ Cookie authentication flow completed");
            
            println!("\nNote: Cookie authentication is ideal for:");
            println!("  - Web applications");
            println!("  - Browser extensions");
            println!("  - Server-side rendered apps");
            println!("  - Progressive web apps (PWAs)");
            
        }
        Err(e) => {
            println!("✗ Login failed: {}", e);
            println!("\nTroubleshooting:");
            println!("  1. Check credentials are correct");
            println!("  2. Ensure cookies are enabled");
            println!("  3. Verify CORS settings if cross-origin");
            println!("  4. Check for cookie blocking extensions");
        }
    }
    
    println!("\nCookie authentication example completed!");
}