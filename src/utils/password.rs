use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

const SALT: [u8; 32] = [
    217, 3, 161, 123, 53, 200, 206, 36, 143, 2, 220, 252, 240, 109, 204, 23,
    217, 174, 79, 158, 18, 76, 149, 117, 73, 40, 207, 77, 34, 194, 196, 163
];

const ITERATIONS: u32 = 600000;

/// Hashes a password using PBKDF2 with SHA256
/// 
/// # Arguments
/// 
/// * `password` - &str - The plain text password to hash
/// 
/// # Returns
/// 
/// String - Base64 encoded hash of the password
pub fn hashpassword(password: &str) -> String {
    let mut derived_key = [0u8; 32];
    
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &SALT,
        ITERATIONS,
        &mut derived_key
    );
    
    general_purpose::STANDARD.encode(&derived_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hashpassword() {
        let password = "test_password";
        let hashed = hashpassword(password);
        
        assert!(!hashed.is_empty());
        assert_eq!(hashed.len(), 44);
        
        let hashed2 = hashpassword(password);
        assert_eq!(hashed, hashed2);
    }
    
    #[test]
    fn test_different_passwords() {
        let hash1 = hashpassword("password1");
        let hash2 = hashpassword("password2");
        
        assert_ne!(hash1, hash2);
    }
}