use rand::Rng;

/// Latest real user agents from various browsers and platforms
/// Updated periodically to match current browser versions
const USER_AGENTS: &[&str] = &[
    // Chrome on Windows (latest versions)
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36",
    
    // Chrome on macOS (latest versions)
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    
    // Firefox on Windows (latest versions)
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:131.0) Gecko/20100101 Firefox/131.0",
    
    // Firefox on macOS (latest versions)
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:132.0) Gecko/20100101 Firefox/132.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:133.0) Gecko/20100101 Firefox/133.0",
    
    // Safari on macOS (latest versions)
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.1.1 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0.1 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.15",
    
    // Edge on Windows (latest versions)
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Edg/130.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36 Edg/129.0.0.0",
    
    // Chrome on Linux (latest versions)
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36",
    
    // Firefox on Linux (latest versions)
    "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
    "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
    
    // Chrome on Android (latest versions)
    "Mozilla/5.0 (Linux; Android 14; SM-G998B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
    "Mozilla/5.0 (Linux; Android 14; SM-S918B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Mobile Safari/537.36",
    
    // Safari on iOS (latest versions)
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 17_6_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Mobile/15E148 Safari/604.1",
    
    // Opera on Windows (latest versions)
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 OPR/117.0.0.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 OPR/116.0.0.0",
    
    // Vivaldi on Windows (latest versions)
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Vivaldi/6.9",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Vivaldi/6.8",
    
    // Brave on Windows (latest versions)
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Brave/131",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36 Brave/130",
];

/// Get a random realistic user agent string
/// 
/// This function returns a random user agent from the latest versions of
/// popular browsers across different platforms. The user agents are updated
/// to reflect current browser versions as of December 2024.
/// 
/// # Returns
/// 
/// &'static str - A random user agent string
pub fn get_random_user_agent() -> &'static str {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..USER_AGENTS.len());
    USER_AGENTS[index]
}

/// Get a random desktop-only user agent (excludes mobile)
/// 
/// # Returns
/// 
/// &'static str - A random desktop user agent string
pub fn get_random_desktop_user_agent() -> &'static str {
    let desktop_user_agents: Vec<&str> = USER_AGENTS.iter()
        .filter(|ua| !ua.contains("Mobile") && !ua.contains("Android") && !ua.contains("iPhone") && !ua.contains("iPad"))
        .copied()
        .collect();
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..desktop_user_agents.len());
    desktop_user_agents[index]
}

/// Get a user agent for a specific browser type
/// 
/// # Arguments
/// 
/// * `browser_type` - &str - Browser type: "chrome", "firefox", "safari", "edge", "opera", "vivaldi", "brave"
/// 
/// # Returns
/// 
/// Option<&'static str> - A random user agent for the specified browser, or None if not found
pub fn get_user_agent_for_browser(browser_type: &str) -> Option<&'static str> {
    let browser_agents: Vec<&str> = USER_AGENTS.iter()
        .filter(|ua| {
            match browser_type.to_lowercase().as_str() {
                "chrome" => ua.contains("Chrome/") && !ua.contains("Edg/") && !ua.contains("OPR/") && !ua.contains("Vivaldi") && !ua.contains("Brave"),
                "firefox" => ua.contains("Firefox/"),
                "safari" => ua.contains("Safari/") && !ua.contains("Chrome/"),
                "edge" => ua.contains("Edg/"),
                "opera" => ua.contains("OPR/"),
                "vivaldi" => ua.contains("Vivaldi"),
                "brave" => ua.contains("Brave"),
                _ => false,
            }
        })
        .copied()
        .collect();
    
    if browser_agents.is_empty() {
        return None;
    }
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..browser_agents.len());
    Some(browser_agents[index])
}

/// Parse browser info from user agent string
/// 
/// # Arguments
/// 
/// * `user_agent` - &str - User agent string to parse
/// 
/// # Returns
/// 
/// BrowserInfo - Parsed browser information
pub fn parse_browser_info(user_agent: &str) -> BrowserInfo {
    let mut info = BrowserInfo {
        name: "Unknown".to_string(),
        version: "Unknown".to_string(),
        platform: "Unknown".to_string(),
        is_mobile: false,
    };
    
    // Detect platform
    if user_agent.contains("Windows NT") {
        info.platform = "Windows".to_string();
    } else if user_agent.contains("Macintosh") || user_agent.contains("Mac OS X") {
        info.platform = "macOS".to_string();
    } else if user_agent.contains("Linux") {
        info.platform = "Linux".to_string();
    } else if user_agent.contains("Android") {
        info.platform = "Android".to_string();
        info.is_mobile = true;
    } else if user_agent.contains("iPhone") || user_agent.contains("iPad") {
        info.platform = "iOS".to_string();
        info.is_mobile = true;
    }
    
    // Detect browser
    if user_agent.contains("Edg/") {
        info.name = "Microsoft Edge".to_string();
        if let Some(version) = extract_version(user_agent, "Edg/") {
            info.version = version;
        }
    } else if user_agent.contains("Firefox/") {
        info.name = "Firefox".to_string();
        if let Some(version) = extract_version(user_agent, "Firefox/") {
            info.version = version;
        }
    } else if user_agent.contains("OPR/") {
        info.name = "Opera".to_string();
        if let Some(version) = extract_version(user_agent, "OPR/") {
            info.version = version;
        }
    } else if user_agent.contains("Vivaldi") {
        info.name = "Vivaldi".to_string();
        if let Some(version) = extract_version(user_agent, "Vivaldi/") {
            info.version = version;
        }
    } else if user_agent.contains("Brave") {
        info.name = "Brave".to_string();
        if let Some(version) = extract_version(user_agent, "Brave/") {
            info.version = version;
        }
    } else if user_agent.contains("Chrome/") {
        info.name = "Chrome".to_string();
        if let Some(version) = extract_version(user_agent, "Chrome/") {
            info.version = version;
        }
    } else if user_agent.contains("Safari/") && user_agent.contains("Version/") {
        info.name = "Safari".to_string();
        if let Some(version) = extract_version(user_agent, "Version/") {
            info.version = version;
        }
    }
    
    info
}

/// Extract version number from user agent string
fn extract_version(user_agent: &str, pattern: &str) -> Option<String> {
    if let Some(start) = user_agent.find(pattern) {
        let version_start = start + pattern.len();
        let version_part = &user_agent[version_start..];
        
        // Find the end of the version (space, semicolon, or parenthesis)
        let version_end = version_part
            .find(' ')
            .or_else(|| version_part.find(';'))
            .or_else(|| version_part.find(')'))
            .unwrap_or(version_part.len());
        
        Some(version_part[..version_end].to_string())
    } else {
        None
    }
}

/// Browser information parsed from user agent
#[derive(Debug, Clone)]
pub struct BrowserInfo {
    pub name: String,
    pub version: String,
    pub platform: String,
    pub is_mobile: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_random_user_agent() {
        let ua = get_random_user_agent();
        assert!(!ua.is_empty());
        assert!(USER_AGENTS.contains(&ua));
    }
    
    #[test]
    fn test_get_random_desktop_user_agent() {
        let ua = get_random_desktop_user_agent();
        assert!(!ua.is_empty());
        assert!(!ua.contains("Mobile"));
        assert!(!ua.contains("Android"));
        assert!(!ua.contains("iPhone"));
        assert!(!ua.contains("iPad"));
    }
    
    #[test]
    fn test_get_user_agent_for_browser() {
        // Test Chrome
        let chrome_ua = get_user_agent_for_browser("chrome").unwrap();
        assert!(chrome_ua.contains("Chrome/"));
        assert!(!chrome_ua.contains("Edg/"));
        
        // Test Firefox
        let firefox_ua = get_user_agent_for_browser("firefox").unwrap();
        assert!(firefox_ua.contains("Firefox/"));
        
        // Test Safari
        let safari_ua = get_user_agent_for_browser("safari").unwrap();
        assert!(safari_ua.contains("Safari/"));
        assert!(!safari_ua.contains("Chrome/"));
    }
    
    #[test]
    fn test_parse_browser_info() {
        let chrome_ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
        let info = parse_browser_info(chrome_ua);
        
        assert_eq!(info.name, "Chrome");
        assert_eq!(info.platform, "Windows");
        assert!(!info.is_mobile);
        assert!(info.version.starts_with("131"));
    }
    
    #[test]
    fn test_mobile_detection() {
        let mobile_ua = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Mobile/15E148 Safari/604.1";
        let info = parse_browser_info(mobile_ua);
        
        assert_eq!(info.platform, "iOS");
        assert!(info.is_mobile);
    }
}