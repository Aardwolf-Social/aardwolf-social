use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use csrf::{
    CsrfError, CsrfProtection, CsrfToken, HmacCsrfProtection,
};
use rand::{rngs::OsRng, TryRngCore};

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
    pub fn new_from_key(key: &[u8]) -> Result<Self> {
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

    pub fn validate_token(&self, csrf_token: &str, csrf_cookie: &str) -> Result<(), CsrfError> {
        // Decode the base64-encoded CSRF token and cookie
        let token_bytes = general_purpose::STANDARD
            .decode(csrf_token.as_bytes())
            .map_err(|e| CsrfError::EncryptionFailure(format!("Invalid base64 token: {}", e)))?;
    
        let cookie_bytes = general_purpose::STANDARD
            .decode(csrf_cookie.as_bytes())
            .map_err(|e| CsrfError::EncryptionFailure(format!("Invalid base64 cookie: {}", e)))?;
    
        // Parse the token and cookie
        let parsed_token = self
            .protection
            .parse_token(&token_bytes)
            .map_err(|e| CsrfError::EncryptionFailure(
                format!("Token parsing error: {}", e)
            ))?;

        let parsed_cookie = self
            .protection
            .parse_cookie(&cookie_bytes)
            .map_err(|e| CsrfError::EncryptionFailure(
                format!("Token parsing error: {}", e)
            ))?;
    
        // Verify the token and cookie pair
        self.protection.verify_token_pair(&parsed_token, &parsed_cookie)
    }
    
    
    
    
    
}
