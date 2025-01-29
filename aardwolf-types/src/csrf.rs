use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use csrf::{
    CsrfError, CsrfProtection, CsrfToken, HmacCsrfProtection, UnencryptedCsrfCookie,
    UnencryptedCsrfToken,
};
use rand::{rngs::OsRng, TryRngCore};
use sha2::Sha256;

#[derive(Clone)]
pub struct CsrfTokenManager {
    protection: HmacCsrfProtection, // No need for <Sha256> here
}

impl CsrfTokenManager {
    // Constructor for creating a manager with a random key
    pub fn new() -> Result<Self> {
        let mut key = [0u8; 32];
        OsRng.try_fill_bytes(&mut key)?;
        Self::new_from_key(&key)
    }

    // Constructor for creating a manager with a custom key
    pub fn new_from_key(key: &[u8; 32]) -> Result<Self> {
        // Create the HmacCsrfProtection instance with no arguments
        let hmac = HmacCsrfProtection::new();
        Ok(CsrfTokenManager { protection: hmac })
    }

    // Generates a new CSRF token
    pub fn generate_token(&self) -> Result<CsrfToken> {
        let mut nonce = [0u8; 64]; 
        OsRng.try_fill_bytes(&mut nonce)?;
        self.protection
            .generate_token(&nonce)
            .map_err(|e| anyhow!("Failed to generate token: {:?}", e))
    }

    // Validates a CSRF token by comparing it with the provided cookie value
    pub fn validate_token(&self, csrf_token: &CsrfToken, csrf_cookie: &str) -> Result<(), CsrfError> {
        // Decode base64 strings into bytes, return error if invalid
        let decode = |s: &str| {
            general_purpose::STANDARD
                .decode(s)
                .map_err(|e| CsrfError::EncryptionFailure(e.to_string()))
        };
    
        // Decode the token and cookie values
        let unencrypted_token = UnencryptedCsrfToken::decode(&csrf_token.b64_string()).map_err(|e| CsrfError::EncryptionFailure(e.to_string()))?;
        let unencrypted_cookie = UnencryptedCsrfCookie::decode(csrf_cookie).map_err(|e| CsrfError::EncryptionFailure(e.to_string()))?;
    
        // Access the underlying bytes without decoding them
        let token_bytes = unencrypted_token.value();
        let cookie_bytes = unencrypted_cookie.value();
    
        // Verify the token and cookie pair
        self.protection
            .verify_token_pair(&unencrypted_token, &unencrypted_cookie)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate() {
        let manager = CsrfTokenManager::new().expect("Failed to create manager");
        let token = manager.generate_token().expect("Failed to generate token");
        let cookie_value = token.cookie_value();
        assert!(manager.validate_token(&token, &cookie_value).is_ok());
    }

    #[test]
    fn test_invalid_key() {
        let short_key = [0u8; 16]; // Key too short
        assert!(CsrfTokenManager::new_from_key(&short_key).is_err());
    }
}
