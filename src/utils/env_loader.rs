use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Custom environment loader that properly handles special characters
pub struct EnvLoader {
    vars: HashMap<String, String>,
}

impl EnvLoader {
    /// Load environment variables from .env file
    /// 
    /// # Arguments
    /// 
    /// * `path` - &Path - Path to the .env file
    /// 
    /// # Returns
    /// 
    /// Result<Self, std::io::Error> - The loaded environment variables
    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(path)?;
        Ok(Self::from_string(&contents))
    }
    
    /// Load environment variables from a string
    /// 
    /// # Arguments
    /// 
    /// * `contents` - &str - The .env file contents
    /// 
    /// # Returns
    /// 
    /// EnvLoader - The loaded environment variables
    pub fn from_string(contents: &str) -> Self {
        let mut vars = HashMap::new();
        
        for line in contents.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim();
                
                let parsed_value = if (value.starts_with('"') && value.ends_with('"')) 
                    || (value.starts_with('\'') && value.ends_with('\'')) {
                    value[1..value.len() - 1].to_string()
                } else if value.starts_with('"') || value.starts_with('\'') {
                    value.to_string()
                } else {
                    value.to_string()
                };
                
                vars.insert(key, parsed_value);
            }
        }
        
        Self { vars }
    }
    
    /// Get a variable value
    /// 
    /// # Arguments
    /// 
    /// * `key` - &str - The variable name
    /// 
    /// # Returns
    /// 
    /// Option<String> - The variable value if it exists
    pub fn get(&self, key: &str) -> Option<String> {
        self.vars.get(key).cloned()
    }
    
    /// Get a variable value or return an error
    /// 
    /// # Arguments
    /// 
    /// * `key` - &str - The variable name
    /// 
    /// # Returns
    /// 
    /// Result<String, String> - The variable value or error message
    pub fn get_required(&self, key: &str) -> Result<String, String> {
        self.get(key)
            .ok_or_else(|| format!("{} not found in environment", key))
    }
    
    /// Apply loaded variables to the process environment
    pub fn apply_to_env(&self) {
        for (key, value) in &self.vars {
            unsafe {
                std::env::set_var(key, value);
            }
        }
    }
    
    /// Load .env file and apply to environment (convenience method)
    /// 
    /// # Returns
    /// 
    /// Result<(), std::io::Error> - Success or error
    pub fn load() -> Result<(), std::io::Error> {
        let loader = Self::from_file(Path::new(".env"))?;
        loader.apply_to_env();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple() {
        let contents = "KEY=value\nKEY2=value2";
        let loader = EnvLoader::from_string(contents);
        assert_eq!(loader.get("KEY"), Some("value".to_string()));
        assert_eq!(loader.get("KEY2"), Some("value2".to_string()));
    }
    
    #[test]
    fn test_parse_with_quotes() {
        let contents = r#"
            KEY1="value with spaces"
            KEY2='value with $special'
            KEY3=no_quotes$
        "#;
        let loader = EnvLoader::from_string(contents);
        assert_eq!(loader.get("KEY1"), Some("value with spaces".to_string()));
        assert_eq!(loader.get("KEY2"), Some("value with $special".to_string()));
        assert_eq!(loader.get("KEY3"), Some("no_quotes$".to_string()));
    }
    
    #[test]
    fn test_special_characters() {
        let contents = r#"
            PASSWORD=Kulwinderkaur2024$
            EMAIL=test@example.com
            TOKEN=abc$def#ghi!
        "#;
        let loader = EnvLoader::from_string(contents);
        assert_eq!(loader.get("PASSWORD"), Some("Kulwinderkaur2024$".to_string()));
        assert_eq!(loader.get("EMAIL"), Some("test@example.com".to_string()));
        assert_eq!(loader.get("TOKEN"), Some("abc$def#ghi!".to_string()));
    }
    
    #[test]
    fn test_comments_and_empty_lines() {
        let contents = r#"
            # This is a comment
            KEY1=value1
            
            # Another comment
            KEY2=value2
        "#;
        let loader = EnvLoader::from_string(contents);
        assert_eq!(loader.get("KEY1"), Some("value1".to_string()));
        assert_eq!(loader.get("KEY2"), Some("value2".to_string()));
        assert_eq!(loader.get("#"), None);
    }
}