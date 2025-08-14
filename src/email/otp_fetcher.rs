use chrono::Utc;
use imap::Session;
use native_tls::{TlsConnector, TlsStream};
use regex::Regex;
use std::error::Error;
use std::net::TcpStream;

const IMAP_DOMAIN: &str = "mail.inbox.lv";
const IMAP_PORT: u16 = 993;

pub struct OtpFetcher {
    email: String,
    password: String,
}

impl OtpFetcher {
    /// Creates a new OTP fetcher instance
    /// 
    /// # Arguments
    /// 
    /// * `email` - String - The inbox.lv email address
    /// * `password` - String - The password for the email account
    /// 
    /// # Returns
    /// 
    /// OtpFetcher - A new instance of the OTP fetcher
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    /// Establishes an IMAP session with inbox.lv
    /// 
    /// # Returns
    /// 
    /// Result<Session<TlsStream<TcpStream>>, Box<dyn Error>> - The IMAP session or an error
    fn connect(&self) -> Result<Session<TlsStream<TcpStream>>, Box<dyn Error>> {
        let tls = TlsConnector::builder().build()?;
        let client = imap::connect((IMAP_DOMAIN, IMAP_PORT), IMAP_DOMAIN, &tls)?;
        let session = client.login(&self.email, &self.password)
            .map_err(|e| format!("Login failed: {:?}", e))?;
        Ok(session)
    }

    /// Fetches the OTP from the latest UNREAD Axiom email
    /// 
    /// # Returns
    /// 
    /// Result<Option<String>, Box<dyn Error>> - The OTP code if found, None if no OTP email exists
    pub fn fetchotp(&self) -> Result<Option<String>, Box<dyn Error>> {
        let mut session = self.connect()?;
        
        session.select("INBOX")?;
        
        let search_query = "UNSEEN SUBJECT \"Your Axiom security code\"";
        let message_ids = session.search(search_query)?;
        
        if message_ids.is_empty() {
            return Ok(None);
        }
        
        let latest_id = message_ids.iter().max().ok_or("No messages found")?;
        let fetch_query = format!("{}", latest_id);
        let messages = session.fetch(&fetch_query, "BODY[HEADER.FIELDS (SUBJECT)]")?;
        
        if let Some(message) = messages.iter().next() {
            if let Some(header) = message.header() {
                let header_str = std::str::from_utf8(header)?;
                if let Some(otp) = self.extract_otp_from_subject(header_str) {
                    session.store(&fetch_query, "+FLAGS (\\Seen)")?;
                    session.logout()?;
                    return Ok(Some(otp));
                }
            }
        }
        
        let messages = session.fetch(&fetch_query, "RFC822")?;
        if let Some(message) = messages.iter().next() {
            if let Some(body) = message.body() {
                let body_str = std::str::from_utf8(body)?;
                let otp = self.extract_otp_from_email(body_str)?;
                
                session.store(&fetch_query, "+FLAGS (\\Seen)")?;
                session.logout()?;
                return Ok(otp);
            }
        }
        
        session.logout()?;
        Ok(None)
    }

    /// Fetches OTP from UNREAD emails received within a time window
    /// 
    /// # Arguments
    /// 
    /// * `minutes_ago` - u32 - Number of minutes to look back for OTP emails
    /// 
    /// # Returns
    /// 
    /// Result<Option<String>, Box<dyn Error>> - The OTP code if found within the time window
    pub fn fetchotp_recent(&self, minutes_ago: u32) -> Result<Option<String>, Box<dyn Error>> {
        let mut session = self.connect()?;
        
        session.select("INBOX")?;
        
        let since_date = Utc::now() - chrono::Duration::minutes(minutes_ago as i64);
        let date_str = since_date.format("%d-%b-%Y").to_string();
        let search_query = format!("UNSEEN SUBJECT \"Your Axiom security code\" SINCE {}", date_str);
        
        let message_ids = session.search(search_query)?;
        
        if message_ids.is_empty() {
            return Ok(None);
        }
        
        let latest_id = message_ids.iter().max().ok_or("No messages found")?;
        let fetch_query = format!("{}", latest_id);
        
        let messages = session.fetch(&fetch_query, "BODY[HEADER.FIELDS (SUBJECT)]")?;
        if let Some(message) = messages.iter().next() {
            if let Some(header) = message.header() {
                let header_str = std::str::from_utf8(header)?;
                if let Some(otp) = self.extract_otp_from_subject(header_str) {
                    session.store(&fetch_query, "+FLAGS (\\Seen)")?;
                    session.logout()?;
                    return Ok(Some(otp));
                }
            }
        }
        
        let messages = session.fetch(&fetch_query, "RFC822")?;
        if let Some(message) = messages.iter().next() {
            if let Some(body) = message.body() {
                let body_str = std::str::from_utf8(body)?;
                let otp = self.extract_otp_from_email(body_str)?;
                
                session.store(&fetch_query, "+FLAGS (\\Seen)")?;
                session.logout()?;
                return Ok(otp);
            }
        }
        
        session.logout()?;
        Ok(None)
    }

    /// Extracts the OTP code from the email subject line
    /// 
    /// # Arguments
    /// 
    /// * `subject` - &str - The email subject header
    /// 
    /// # Returns
    /// 
    /// Option<String> - The extracted OTP code or None if not found
    fn extract_otp_from_subject(&self, subject: &str) -> Option<String> {
        let re = Regex::new(r"Your Axiom security code is (\d{6})").ok()?;
        if let Some(captures) = re.captures(subject) {
            if let Some(otp) = captures.get(1) {
                return Some(otp.as_str().to_string());
            }
        }
        None
    }
    
    /// Extracts the OTP code from the email body
    /// 
    /// # Arguments
    /// 
    /// * `email_body` - &str - The raw email body
    /// 
    /// # Returns
    /// 
    /// Result<Option<String>, Box<dyn Error>> - The extracted OTP code or None if not found
    fn extract_otp_from_email(&self, email_body: &str) -> Result<Option<String>, Box<dyn Error>> {
        let patterns = vec![
            r"Your Axiom security code is[:\s]+(\d{6})",
            r"Your security code is[:\s]+(\d{6})",
            r"security code[:\s]+(\d{6})",
            r"<span[^>]*>(\d{6})</span>",
            r"<b>(\d{6})</b>",
            r"<strong>(\d{6})</strong>",
        ];
        
        for pattern in patterns {
            let re = Regex::new(pattern)?;
            if let Some(captures) = re.captures(email_body) {
                if let Some(otp) = captures.get(1) {
                    return Ok(Some(otp.as_str().to_string()));
                }
            }
        }
        
        let digit_re = Regex::new(r"\b(\d{6})\b")?;
        for capture in digit_re.captures_iter(email_body) {
            if let Some(otp) = capture.get(1) {
                let otp_str = otp.as_str();
                if email_body.contains("security code") || email_body.contains("Your Axiom") {
                    return Ok(Some(otp_str.to_string()));
                }
            }
        }
        
        Ok(None)
    }

    /// Waits for a new OTP email to arrive
    /// 
    /// # Arguments
    /// 
    /// * `timeout_seconds` - u64 - Maximum time to wait for OTP email
    /// * `check_interval_seconds` - u64 - How often to check for new emails
    /// 
    /// # Returns
    /// 
    /// Result<Option<String>, Box<dyn Error>> - The OTP code when received or None if timeout
    pub fn wait_for_otp(&self, timeout_seconds: u64, check_interval_seconds: u64) -> Result<Option<String>, Box<dyn Error>> {
        let start_time = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_secs(timeout_seconds);
        let check_interval = std::time::Duration::from_secs(check_interval_seconds);
        
        println!("Checking for new OTP emails (will only check UNREAD messages)...");
        let mut check_count = 0;
        
        while start_time.elapsed() < timeout_duration {
            check_count += 1;
            
            if let Some(otp) = self.fetchotp_recent(3)? {
                println!("✓ Found new OTP!");
                return Ok(Some(otp));
            }
            
            let remaining = timeout_duration - start_time.elapsed();
            println!("  Check #{}: No new OTP yet, {} seconds remaining...", 
                     check_count, remaining.as_secs());
            
            std::thread::sleep(check_interval);
        }
        
        println!("✗ Timeout: No new OTP received within {} seconds", timeout_seconds);
        Ok(None)
    }
}

/// Creates an OTP fetcher from environment variables
/// 
/// # Returns
/// 
/// Result<Option<OtpFetcher>, Box<dyn Error>> - The OTP fetcher if env vars are set, None otherwise
pub fn from_env() -> Result<Option<OtpFetcher>, Box<dyn Error>> {
    let email = std::env::var("INBOX_LV_EMAIL").ok();
    let password = std::env::var("INBOX_LV_PASSWORD").ok();
    
    match (email, password) {
        (Some(e), Some(p)) => Ok(Some(OtpFetcher::new(e, p))),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_otp_extraction() {
        let fetcher = OtpFetcher::new("test@inbox.lv".to_string(), "password".to_string());
        
        let email_body = r#"
        Your Axiom security code is: 280296
        
        This code will expire in 10 minutes.
        "#;
        
        let result = fetcher.extract_otp_from_email(email_body).unwrap();
        assert_eq!(result, Some("280296".to_string()));
    }
    
    #[test]
    fn test_otp_extraction_html() {
        let fetcher = OtpFetcher::new("test@inbox.lv".to_string(), "password".to_string());
        
        let email_body = r#"
        <div style="background-color: #f5f5f5; padding: 15px;">
          <span style="font-size: 24px; font-weight: bold;">280296</span>
        </div>
        <p>Your Axiom security code</p>
        "#;
        
        let result = fetcher.extract_otp_from_email(email_body).unwrap();
        assert_eq!(result, Some("280296".to_string()));
    }
    
    #[test]
    fn test_no_otp_in_email() {
        let fetcher = OtpFetcher::new("test@inbox.lv".to_string(), "password".to_string());
        
        let email_body = "This email contains no OTP code.";
        
        let result = fetcher.extract_otp_from_email(email_body).unwrap();
        assert_eq!(result, None);
    }
}