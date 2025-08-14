use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use p256::{SecretKey, PublicKey, elliptic_curve::sec1::ToEncodedPoint, ecdsa::SigningKey};
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use num_traits::identities::Zero;
use crate::errors::{AxiomError, Result};

const CURVE_ORDER_HEX: &str = "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632550";
const PBKDF2_ITERATIONS: u32 = 600_000;

/// P256 key generation result matching Axiom Trade's implementation
#[derive(Debug, Clone)]
pub struct P256KeyPair {
    pub private_key: String,
    pub public_key: String,
    pub client_secret: String,
}

/// Generate P256 key pair from password using Axiom Trade's method
/// 
/// # Arguments
/// 
/// * `password` - &str - The password to derive the key from
/// * `salt` - Option<&[u8]> - Optional salt bytes (if None, generates random)
/// 
/// # Returns
/// 
/// Result<P256KeyPair> - Generated key pair with private key, public key, and client secret
/// 
/// # Errors
/// 
/// Returns error if key generation fails or provided salt produces invalid key
pub fn generate_p256_keypair_from_password(
    password: &str, 
    salt: Option<&[u8]>
) -> Result<P256KeyPair> {
    let curve_order = num_bigint::BigUint::parse_bytes(CURVE_ORDER_HEX.as_bytes(), 16)
        .ok_or_else(|| AxiomError::Crypto {
            message: "Failed to parse P256 curve order".to_string(),
        })?;
    
    loop {
        let salt_bytes = match salt {
            Some(s) => s.to_vec(),
            None => {
                let mut salt = vec![0u8; 32];
                rand::thread_rng().fill_bytes(&mut salt);
                salt
            }
        };
        
        let mut derived_key = [0u8; 32];
        pbkdf2::<Hmac<Sha256>>(
            password.as_bytes(),
            &salt_bytes,
            PBKDF2_ITERATIONS,
            &mut derived_key
        );
        
        let key_bigint = num_bigint::BigUint::from_bytes_be(&derived_key);
        if key_bigint < curve_order && !key_bigint.is_zero() {
            let secret_key = SecretKey::from_bytes(&derived_key.into())
                .map_err(|e| AxiomError::Crypto {
                    message: format!("Failed to create P256 secret key: {}", e),
                })?;
            
            let public_key = secret_key.public_key();
            let public_key_bytes = public_key.to_encoded_point(true);
            
            return Ok(P256KeyPair {
                private_key: hex::encode(derived_key),
                public_key: hex::encode(public_key_bytes.as_bytes()),
                client_secret: general_purpose::STANDARD.encode(&salt_bytes),
            });
        }
        
        if salt.is_some() {
            return Err(AxiomError::Crypto {
                message: "Failed to generate valid API key with provided salt".to_string(),
            });
        }
    }
}

/// Decrypt client secret to recover salt bytes
/// 
/// # Arguments
/// 
/// * `client_secret` - &str - Base64 encoded client secret
/// 
/// # Returns
/// 
/// Result<Vec<u8>> - Decoded salt bytes
pub fn decrypt_client_secret(client_secret: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD.decode(client_secret)
        .map_err(|e| AxiomError::Crypto {
            message: format!("Failed to decode client secret: {}", e),
        })
}

/// Recreate P256 key pair from password and client secret
/// 
/// # Arguments
/// 
/// * `password` - &str - The original password
/// * `client_secret` - &str - Base64 encoded client secret (salt)
/// 
/// # Returns
/// 
/// Result<P256KeyPair> - Recreated key pair
pub fn recreate_keypair_from_client_secret(
    password: &str,
    client_secret: &str
) -> Result<P256KeyPair> {
    let salt_bytes = decrypt_client_secret(client_secret)?;
    generate_p256_keypair_from_password(password, Some(&salt_bytes))
}

/// Create a P256 signing key from hex private key
/// 
/// # Arguments
/// 
/// * `private_key_hex` - &str - Hex encoded private key
/// 
/// # Returns
/// 
/// Result<SigningKey> - P256 signing key for cryptographic operations
pub fn create_signing_key(private_key_hex: &str) -> Result<SigningKey> {
    let private_key_bytes = hex::decode(private_key_hex)
        .map_err(|e| AxiomError::Crypto {
            message: format!("Failed to decode private key hex: {}", e),
        })?;
    
    if private_key_bytes.len() != 32 {
        return Err(AxiomError::Crypto {
            message: "Private key must be exactly 32 bytes".to_string(),
        });
    }
    
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&private_key_bytes);
    
    let secret_key = SecretKey::from_bytes(&key_array.into())
        .map_err(|e| AxiomError::Crypto {
            message: format!("Failed to create secret key: {}", e),
        })?;
    
    Ok(SigningKey::from(secret_key))
}

/// Sign message with P256 private key
/// 
/// # Arguments
/// 
/// * `message` - &[u8] - Message bytes to sign
/// * `private_key_hex` - &str - Hex encoded private key
/// 
/// # Returns
/// 
/// Result<Vec<u8>> - DER encoded signature bytes
pub fn sign_message(message: &[u8], private_key_hex: &str) -> Result<Vec<u8>> {
    let signing_key = create_signing_key(private_key_hex)?;
    
    use p256::ecdsa::{Signature, signature::Signer};
    let signature: Signature = signing_key.sign(message);
    
    Ok(signature.to_der().as_bytes().to_vec())
}

/// Sign message with P256 private key for WebAuthn/Turnkey (raw r,s format)
/// 
/// # Arguments
/// 
/// * `message` - &[u8] - Message bytes to sign
/// * `private_key_hex` - &str - Hex encoded private key
/// 
/// # Returns
/// 
/// Result<Vec<u8>> - Raw signature bytes (64 bytes: 32-byte r + 32-byte s)
pub fn sign_message_webauthn(message: &[u8], private_key_hex: &str) -> Result<Vec<u8>> {
    let signing_key = create_signing_key(private_key_hex)?;
    
    use p256::ecdsa::{Signature, signature::Signer};
    let signature: Signature = signing_key.sign(message);
    
    // Convert to raw r,s bytes (WebAuthn format)
    let signature_bytes = signature.to_bytes();
    Ok(signature_bytes.to_vec())
}

/// Verify P256 signature
/// 
/// # Arguments
/// 
/// * `message` - &[u8] - Original message bytes
/// * `signature_der` - &[u8] - DER encoded signature
/// * `public_key_hex` - &str - Hex encoded compressed public key
/// 
/// # Returns
/// 
/// Result<bool> - True if signature is valid
pub fn verify_signature(
    message: &[u8],
    signature_der: &[u8],
    public_key_hex: &str
) -> Result<bool> {
    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(|e| AxiomError::Crypto {
            message: format!("Failed to decode public key hex: {}", e),
        })?;
    
    let public_key = PublicKey::from_sec1_bytes(&public_key_bytes)
        .map_err(|e| AxiomError::Crypto {
            message: format!("Failed to parse public key: {}", e),
        })?;
    
    use p256::ecdsa::{Signature, signature::Verifier, VerifyingKey};
    
    let signature = Signature::from_der(signature_der)
        .map_err(|e| AxiomError::Crypto {
            message: format!("Failed to parse signature: {}", e),
        })?;
    
    let verifying_key = VerifyingKey::from(public_key);
    
    Ok(verifying_key.verify(message, &signature).is_ok())
}

/// Generate random P256 key pair (not password-based)
/// 
/// # Returns
/// 
/// Result<P256KeyPair> - Randomly generated key pair
pub fn generate_random_p256_keypair() -> Result<P256KeyPair> {
    let mut rng = rand::thread_rng();
    let secret_key = SecretKey::random(&mut rng);
    let public_key = secret_key.public_key();
    
    let public_key_bytes = public_key.to_encoded_point(true);
    let private_key_hex = hex::encode(secret_key.to_bytes());
    let public_key_hex = hex::encode(public_key_bytes.as_bytes());
    
    let mut client_secret_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut client_secret_bytes);
    let client_secret = general_purpose::STANDARD.encode(&client_secret_bytes);
    
    Ok(P256KeyPair {
        private_key: private_key_hex,
        public_key: public_key_hex,
        client_secret,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_p256_keypair_generation() {
        let password = "test_password_123";
        let keypair = generate_p256_keypair_from_password(password, None).unwrap();
        
        assert_eq!(keypair.private_key.len(), 64); // 32 bytes = 64 hex chars
        assert!(keypair.public_key.len() >= 66); // Compressed public key
        assert!(!keypair.client_secret.is_empty());
    }
    
    #[test]
    fn test_deterministic_generation() {
        let password = "test_password_123";
        let salt = b"test_salt_32_bytes_exactly_here!";
        
        let keypair1 = generate_p256_keypair_from_password(password, Some(salt)).unwrap();
        let keypair2 = generate_p256_keypair_from_password(password, Some(salt)).unwrap();
        
        assert_eq!(keypair1.private_key, keypair2.private_key);
        assert_eq!(keypair1.public_key, keypair2.public_key);
        assert_eq!(keypair1.client_secret, keypair2.client_secret);
    }
    
    #[test]
    fn test_client_secret_roundtrip() {
        let password = "test_password_123";
        let keypair = generate_p256_keypair_from_password(password, None).unwrap();
        
        let recreated = recreate_keypair_from_client_secret(password, &keypair.client_secret).unwrap();
        
        assert_eq!(keypair.private_key, recreated.private_key);
        assert_eq!(keypair.public_key, recreated.public_key);
        assert_eq!(keypair.client_secret, recreated.client_secret);
    }
    
    #[test]
    fn test_signature_generation_and_verification() {
        let password = "test_password_123";
        let keypair = generate_p256_keypair_from_password(password, None).unwrap();
        
        let message = b"test message to sign";
        let signature = sign_message(message, &keypair.private_key).unwrap();
        
        let is_valid = verify_signature(message, &signature, &keypair.public_key).unwrap();
        assert!(is_valid);
        
        let wrong_message = b"different message";
        let is_invalid = verify_signature(wrong_message, &signature, &keypair.public_key).unwrap();
        assert!(!is_invalid);
    }
    
    #[test]
    fn test_random_keypair_generation() {
        let keypair1 = generate_random_p256_keypair().unwrap();
        let keypair2 = generate_random_p256_keypair().unwrap();
        
        assert_ne!(keypair1.private_key, keypair2.private_key);
        assert_ne!(keypair1.public_key, keypair2.public_key);
        assert_ne!(keypair1.client_secret, keypair2.client_secret);
    }
    
    #[test]
    fn test_webauthn_signature_format() {
        let password = "test_password_123";
        let keypair = generate_p256_keypair_from_password(password, None).unwrap();
        
        let message = b"test message for webauthn";
        
        // Test WebAuthn format (should be 64 bytes)
        let webauthn_sig = sign_message_webauthn(message, &keypair.private_key).unwrap();
        assert_eq!(webauthn_sig.len(), 64, "WebAuthn signature should be exactly 64 bytes");
        
        // Test DER format (should be longer)
        let der_sig = sign_message(message, &keypair.private_key).unwrap();
        assert!(der_sig.len() > 64, "DER signature should be longer than 64 bytes");
        assert_ne!(webauthn_sig, der_sig, "WebAuthn and DER signatures should be different");
    }
}