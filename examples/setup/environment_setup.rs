/// Environment Setup Helper
/// 
/// This example helps you configure your environment variables
/// and test your configuration for axiomtrade-rs.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("axiomtrade-rs Environment Setup Helper");
    println!("This tool will help you configure your environment for the library\n");

    // Check if .env file exists
    let env_file_path = ".env";
    let env_exists = Path::new(env_file_path).exists();

    if env_exists {
        println!("✓ .env file found");
        println!("Current configuration:");
        display_current_config();
        
        println!("\nWhat would you like to do?");
        println!("1. Test current configuration");
        println!("2. Update configuration");
        println!("3. View configuration guide");
        println!("4. Reset configuration");
        
        let choice = get_user_input("Enter your choice (1-4): ")?;
        
        match choice.trim() {
            "1" => test_configuration().await?,
            "2" => update_configuration()?,
            "3" => show_configuration_guide(),
            "4" => reset_configuration()?,
            _ => println!("Invalid choice"),
        }
    } else {
        println!("❌ No .env file found");
        println!("Let's create one with the required configuration");
        
        create_new_configuration()?;
    }

    Ok(())
}

fn display_current_config() {
    dotenvy::dotenv().ok();
    
    let configs = vec![
        ("AXIOM_EMAIL", "Axiom Trade login email"),
        ("AXIOM_PASSWORD", "Axiom Trade password"),
        ("INBOX_LV_EMAIL", "inbox.lv email for OTP"),
        ("INBOX_LV_PASSWORD", "inbox.lv IMAP password"),
    ];
    
    for (key, description) in configs {
        match env::var(key) {
            Ok(value) => {
                if key.contains("PASSWORD") {
                    println!("  {} ({}): [CONFIGURED]", key, description);
                } else {
                    println!("  {} ({}): {}", key, description, value);
                }
            }
            Err(_) => {
                println!("  {} ({}): [NOT SET]", key, description);
            }
        }
    }
}

async fn test_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nTesting configuration...");
    
    dotenvy::dotenv().ok();
    
    // Test required variables
    let required_vars = vec![
        ("AXIOM_EMAIL", "Axiom Trade email"),
        ("AXIOM_PASSWORD", "Axiom Trade password"),
    ];
    
    let mut missing_required = Vec::new();
    
    for (key, description) in &required_vars {
        match env::var(key) {
            Ok(_) => println!("✓ {} configured", description),
            Err(_) => {
                println!("❌ {} not configured", description);
                missing_required.push(key);
            }
        }
    }
    
    // Test optional variables for OTP
    let otp_vars = vec![
        ("INBOX_LV_EMAIL", "inbox.lv email"),
        ("INBOX_LV_PASSWORD", "inbox.lv IMAP password"),
    ];
    
    let mut otp_configured = true;
    
    for (key, description) in &otp_vars {
        match env::var(key) {
            Ok(_) => println!("✓ {} configured", description),
            Err(_) => {
                println!("⚠️  {} not configured (automatic OTP will not work)", description);
                otp_configured = false;
            }
        }
    }
    
    if !missing_required.is_empty() {
        println!("\n❌ Configuration incomplete");
        println!("Missing required variables: {:?}", missing_required);
        return Ok(());
    }
    
    println!("\n✓ Basic configuration complete");
    
    if !otp_configured {
        println!("⚠️  Automatic OTP not configured");
        println!("You will need to manually enter OTP codes during authentication");
        println!("See examples/setup/auto_otp_setup.md for setup instructions");
    } else {
        println!("✓ Automatic OTP configuration detected");
        
        // Test IMAP connection if configured
        println!("\nTesting inbox.lv IMAP connection...");
        test_imap_connection().await?;
    }
    
    // Test basic authentication
    println!("\nTesting Axiom Trade authentication...");
    test_axiom_authentication().await?;
    
    Ok(())
}

async fn test_imap_connection() -> Result<(), Box<dyn std::error::Error>> {
    let email = env::var("INBOX_LV_EMAIL")?;
    let password = env::var("INBOX_LV_PASSWORD")?;
    
    println!("IMAP Server: mail.inbox.lv:993");
    println!("Email: {}", email);
    
    // In a real implementation, test actual IMAP connection
    // For this example, we'll simulate it
    println!("✓ IMAP connection test passed (simulated)");
    println!("Note: Run 'cargo run --example test_auto_otp' for full IMAP testing");
    
    Ok(())
}

async fn test_axiom_authentication() -> Result<(), Box<dyn std::error::Error>> {
    println!("Note: Authentication test requires network connection");
    println!("Run 'cargo run --example basic_login' to test authentication");
    
    Ok(())
}

fn update_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nUpdate Configuration");
    println!("Choose what to update:");
    println!("1. Axiom Trade credentials");
    println!("2. inbox.lv OTP configuration");
    println!("3. All configuration");
    
    let choice = get_user_input("Enter your choice (1-3): ")?;
    
    match choice.trim() {
        "1" => update_axiom_credentials()?,
        "2" => update_otp_configuration()?,
        "3" => create_new_configuration()?,
        _ => println!("Invalid choice"),
    }
    
    Ok(())
}

fn update_axiom_credentials() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nUpdating Axiom Trade Credentials");
    
    let email = get_user_input("Enter Axiom Trade email: ")?;
    let password = get_password_input("Enter Axiom Trade password: ")?;
    
    update_env_file(vec![
        ("AXIOM_EMAIL", email.trim()),
        ("AXIOM_PASSWORD", password.trim()),
    ])?;
    
    println!("✓ Axiom Trade credentials updated");
    Ok(())
}

fn update_otp_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nUpdating inbox.lv OTP Configuration");
    println!("Note: You must have already set up an inbox.lv account");
    println!("See examples/setup/auto_otp_setup.md for detailed instructions\n");
    
    let email = get_user_input("Enter inbox.lv email (username@inbox.lv): ")?;
    let password = get_password_input("Enter inbox.lv IMAP password (not web password): ")?;
    
    update_env_file(vec![
        ("INBOX_LV_EMAIL", email.trim()),
        ("INBOX_LV_PASSWORD", password.trim()),
    ])?;
    
    println!("✓ inbox.lv OTP configuration updated");
    println!("Run 'cargo run --example test_auto_otp' to test the setup");
    Ok(())
}

fn create_new_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nCreating New Configuration");
    println!("Please provide the following information:\n");
    
    // Axiom Trade credentials
    println!("=== Axiom Trade Credentials ===");
    let axiom_email = get_user_input("Axiom Trade email: ")?;
    let axiom_password = get_password_input("Axiom Trade password: ")?;
    
    // OTP configuration
    println!("\n=== Optional: Automatic OTP Setup ===");
    println!("To enable automatic OTP, you need an inbox.lv account");
    println!("Leave blank to skip automatic OTP (you'll enter codes manually)");
    
    let setup_otp = get_user_input("Set up automatic OTP? (y/n): ")?;
    
    let mut env_vars = vec![
        ("AXIOM_EMAIL", axiom_email.trim().to_string()),
        ("AXIOM_PASSWORD", axiom_password.trim().to_string()),
    ];
    
    if setup_otp.trim().to_lowercase() == "y" {
        println!("\nSee examples/setup/auto_otp_setup.md for inbox.lv setup instructions");
        let inbox_email = get_user_input("inbox.lv email (username@inbox.lv): ")?;
        let inbox_password = get_password_input("inbox.lv IMAP password: ")?;
        
        env_vars.push(("INBOX_LV_EMAIL", inbox_email.trim().to_string()));
        env_vars.push(("INBOX_LV_PASSWORD", inbox_password.trim().to_string()));
    }
    
    // Create .env file
    let env_content = env_vars.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("\n");
    
    fs::write(".env", env_content)?;
    
    println!("\n✓ Configuration file created successfully");
    println!("File: .env");
    
    if setup_otp.trim().to_lowercase() == "y" {
        println!("\nNext steps for OTP setup:");
        println!("1. Follow examples/setup/auto_otp_setup.md");
        println!("2. Run 'cargo run --example test_auto_otp' to test");
    }
    
    println!("\nTest your configuration:");
    println!("cargo run --example basic_login");
    
    Ok(())
}

fn reset_configuration() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nReset Configuration");
    println!("This will delete your current .env file");
    
    let confirm = get_user_input("Are you sure? (y/n): ")?;
    
    if confirm.trim().to_lowercase() == "y" {
        if Path::new(".env").exists() {
            fs::remove_file(".env")?;
            println!("✓ Configuration reset");
            println!("Run this tool again to create a new configuration");
        } else {
            println!("No .env file to remove");
        }
    } else {
        println!("Reset cancelled");
    }
    
    Ok(())
}

fn update_env_file(updates: Vec<(&str, &str)>) -> Result<(), Box<dyn std::error::Error>> {
    // Read existing .env file
    let existing_content = if Path::new(".env").exists() {
        fs::read_to_string(".env")?
    } else {
        String::new()
    };
    
    let mut env_vars: std::collections::HashMap<String, String> = existing_content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect();
    
    // Update with new values
    for (key, value) in updates {
        env_vars.insert(key.to_string(), value.to_string());
    }
    
    // Write back to file
    let new_content = env_vars.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("\n");
    
    fs::write(".env", new_content)?;
    
    Ok(())
}

fn show_configuration_guide() {
    println!("\n=== Configuration Guide ===");
    println!("\nRequired Variables:");
    println!("AXIOM_EMAIL - Your Axiom Trade login email");
    println!("AXIOM_PASSWORD - Your Axiom Trade password");
    println!("");
    println!("Optional Variables (for automatic OTP):");
    println!("INBOX_LV_EMAIL - Your inbox.lv email address");
    println!("INBOX_LV_PASSWORD - Your inbox.lv IMAP password (not web login password)");
    println!("");
    println!("Example .env file:");
    println!("AXIOM_EMAIL=user@example.com");
    println!("AXIOM_PASSWORD=your_password");
    println!("INBOX_LV_EMAIL=username@inbox.lv");
    println!("INBOX_LV_PASSWORD=imap_password");
    println!("");
    println!("Security Notes:");
    println!("- Never commit .env files to version control");
    println!("- Use strong, unique passwords");
    println!("- Keep credentials secure and private");
    println!("");
    println!("For automatic OTP setup, see:");
    println!("examples/setup/auto_otp_setup.md");
}

fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input)
}

fn get_password_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    // In a production tool, you might use a crate like `rpassword` for hidden input
    // For this example, we'll use regular input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input)
}