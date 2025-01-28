use anyhow::{anyhow, Result};
use base64::Engine;
use csrf::{CsrfProtection, HmacCsrfProtection, CsrfToken, CsrfError, UnencryptedCsrfCookie, UnencryptedCsrfToken};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use rand::{rngs::OsRng, TryRngCore};
use std::fmt;

#[derive(Clone)]
struct ProtectedHmac(HmacCsrfProtection);

impl fmt::Debug for ProtectedHmac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProtectedHmac").finish()
    }
}

#[derive(Debug, Clone)]
pub struct CsrfTokenManager {
    protection: ProtectedHmac,
}

impl CsrfTokenManager {
    pub fn new() -> Result<Self> {
        let mut key = [0u8; 32];
        OsRng.try_fill_bytes(&mut key)?;
        Self::new_from_key(&key)
    }

    pub fn new_from_key(key: &[u8; 32]) -> Result<Self> {
        Hmac::<Sha256>::new_from_slice(key)
            .map(|_| Self {
                protection: ProtectedHmac(HmacCsrfProtection::new()),
            })
            .map_err(|e| anyhow!("Failed to create HMAC: {}", e))
    }

    pub fn generate_token(&self) -> Result<CsrfToken> {
        let mut nonce = [0u8; 64];
        OsRng.try_fill_bytes(&mut nonce)?;
        self.protection.0.generate_token(&nonce)
            .map_err(|e| anyhow!("Failed to generate token: {:?}", e))
    }

    pub fn validate_token(&self, token: &CsrfToken, cookie_value: &str) -> Result<(), CsrfError> {
        let decode_base64 = |s: &str| {
            base64::prelude::BASE64_STANDARD
                .decode(s)
                .map_err(|e| anyhow!("Base64 decode error: {:?}", e))
        };

        let token_bytes = decode_base64(&token.b64_string())?;
        let cookie_bytes = decode_base64(cookie_value)?;

        let unencrypted_token = UnencryptedCsrfToken::from_bytes(&token_bytes)?;
        let unencrypted_cookie = UnencryptedCsrfCookie::from_bytes(&cookie_bytes)?;

        self.protection.0.verify_token_pair(&unencrypted_token, &unencrypted_cookie)
            .map_err(|e| anyhow!("Token validation failed: {:?}", e))
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