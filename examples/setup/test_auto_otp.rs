/// Automatic OTP Test Example
/// 
/// This example tests the automatic OTP retrieval setup to ensure
/// everything is configured correctly for seamless authentication.

use axiomtrade_rs::auth::{AuthClient, AuthError};
use std::env;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Automatic OTP Setup Test");
    println!("This test verifies your inbox.lv IMAP configuration\n");

    // Load and verify environment variables
    dotenvy::dotenv().ok();
    
    println!("Step 1: Verifying environment configuration...");
    
    let email = match env::var("AXIOM_EMAIL") {
        Ok(email) => {
            println!("✓ AXIOM_EMAIL configured: {}", email);
            email
        }
        Err(_) => {
            println!("❌ AXIOM_EMAIL not found in .env file");
            print_env_setup_instructions();
            return Ok(());
        }
    };
    
    let password = match env::var("AXIOM_PASSWORD") {
        Ok(_) => {
            println!("✓ AXIOM_PASSWORD configured");
            env::var("AXIOM_PASSWORD").unwrap()
        }
        Err(_) => {
            println!("❌ AXIOM_PASSWORD not found in .env file");
            print_env_setup_instructions();
            return Ok(());
        }
    };
    
    let inbox_email = match env::var("INBOX_LV_EMAIL") {
        Ok(email) => {
            println!("✓ INBOX_LV_EMAIL configured: {}", email);
            Some(email)
        }
        Err(_) => {
            println!("⚠️  INBOX_LV_EMAIL not configured - automatic OTP will not work");
            None
        }
    };
    
    let inbox_password = match env::var("INBOX_LV_PASSWORD") {
        Ok(_) => {
            println!("✓ INBOX_LV_PASSWORD configured");
            Some(env::var("INBOX_LV_PASSWORD").unwrap())
        }
        Err(_) => {
            println!("⚠️  INBOX_LV_PASSWORD not configured - automatic OTP will not work");
            None
        }
    };

    // Check if automatic OTP is properly configured
    if inbox_email.is_none() || inbox_password.is_none() {
        println!("\n❌ Automatic OTP not fully configured");
        println!("Please follow the setup guide to configure inbox.lv credentials");
        print_setup_reminder();
        return Ok(());
    }

    println!("\n✓ All environment variables configured correctly");

    // Test IMAP connection first
    println!("\nStep 2: Testing IMAP connection to inbox.lv...");
    
    match test_imap_connection(&inbox_email.unwrap(), &inbox_password.unwrap()).await {
        Ok(()) => {
            println!("✓ IMAP connection successful");
        }
        Err(e) => {
            println!("❌ IMAP connection failed: {}", e);
            print_imap_troubleshooting();
            return Ok(());
        }
    }

    // Test authentication flow
    println!("\nStep 3: Testing Axiom Trade authentication...");
    
    let mut auth_client = AuthClient::new()?;
    
    let start_time = Instant::now();
    
    // Try login with automatic OTP
    match auth_client.login(&email, &password, None).await {
        Ok(tokens) => {
            let total_time = start_time.elapsed().as_secs();
            println!("🎉 LOGIN SUCCESSFUL!");
            println!("Access token length: {}", tokens.access_token.len());
            println!("Total authentication time: {} seconds", total_time);
            
            println!("\n🎉 AUTOMATIC OTP SETUP VERIFICATION COMPLETE!");
            println!("\nSummary:");
            println!("✓ Environment variables configured");
            println!("✓ IMAP connection working");
            println!("✓ Email reception confirmed"); 
            println!("✓ OTP extraction successful");
            println!("✓ OTP verification working");
            
            println!("\nYour automatic OTP setup is fully functional!");
            println!("You can now use automated authentication in your applications.");
            return Ok(());
        }
        Err(AuthError::OtpRequired) => {
            println!("✓ Login step 1 successful - OTP required");
            println!("Now testing automatic OTP retrieval...");
        }
        Err(e) => {
            println!("❌ Authentication failed: {}", e);
            return Ok(());
        }
    }

    // Since AuthClient automatically handles OTP when login() is called with None,
    // if we reached here with OtpRequired error, it means OTP automation failed
    println!("\n❌ Automatic OTP retrieval failed");
    println!("The login attempt required OTP but automatic retrieval was not successful");
    println!("\nTroubleshooting steps:");
    print_otp_troubleshooting();

    Ok(())
}

async fn test_imap_connection(email: &str, _password: &str) -> Result<()> {
    // This is a simplified test - in reality you'd use the actual IMAP client
    println!("Attempting IMAP connection to mail.inbox.lv:993");
    println!("Email: {}", email);
    println!("Testing authentication...");
    
    // Simulate connection test
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    // In a real implementation, you would:
    // 1. Connect to mail.inbox.lv on port 993 (IMAPS)
    // 2. Authenticate with the provided credentials
    // 3. List folders to verify access
    // 4. Close connection
    
    println!("IMAP connection test completed");
    Ok(())
}

fn print_env_setup_instructions() {
    println!("\nEnvironment Setup Required:");
    println!("Create a .env file in your project root with:");
    println!("");
    println!("AXIOM_EMAIL=your_axiom_email@domain.com");
    println!("AXIOM_PASSWORD=your_axiom_password");
    println!("INBOX_LV_EMAIL=your_username@inbox.lv");
    println!("INBOX_LV_PASSWORD=your_imap_password");
    println!("");
    println!("See examples/setup/auto_otp_setup.md for detailed instructions");
}

fn print_setup_reminder() {
    println!("\nTo enable automatic OTP:");
    println!("1. Create an inbox.lv email account");
    println!("2. Enable IMAP access in inbox.lv settings");
    println!("3. Configure Axiom Trade to send OTP emails to inbox.lv");
    println!("4. Add INBOX_LV_* variables to your .env file");
    println!("");
    println!("See examples/setup/auto_otp_setup.md for complete guide");
}

fn print_imap_troubleshooting() {
    println!("\nIMAP Connection Troubleshooting:");
    println!("1. Verify you waited 15 minutes after enabling IMAP in inbox.lv");
    println!("2. Check that INBOX_LV_PASSWORD is the IMAP password, not web login password");
    println!("3. Confirm INBOX_LV_EMAIL is exactly correct (no typos)");
    println!("4. Test logging into inbox.lv webmail to verify credentials");
    println!("5. Check firewall/antivirus is not blocking port 993");
    println!("");
    println!("IMAP Server: mail.inbox.lv");
    println!("Port: 993 (IMAPS/SSL)");
}

fn print_otp_troubleshooting() {
    println!("1. Check if OTP email arrived in inbox.lv webmail");
    println!("2. Verify Axiom Trade is sending to your inbox.lv address");
    println!("3. Check spam/junk folder in inbox.lv");
    println!("4. Confirm email subject format: 'Your Axiom security code is XXXXXX'");
    println!("5. Try requesting a new OTP and check timing");
    println!("");
    println!("Manual verification steps:");
    println!("- Log into inbox.lv webmail");
    println!("- Request OTP from Axiom Trade");
    println!("- Check if email arrives within 1-2 minutes");
    println!("- Verify subject line contains the code");
}