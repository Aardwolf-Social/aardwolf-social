use bcrypt::{hash, verify};
use diesel::{backend::Backend, deserialize, serialize, sql_types::Text};
#[cfg(any(test, feature = "test"))]
use log::warn;
use rand::{distr::Alphanumeric, rngs::OsRng, seq::IteratorRandom, Rng};
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};
use std::fmt;
use thiserror::Error;

/// A trait used to verify emails
///
/// Emails should only be able to be verified in certain situations, so this trait must not be in
/// scope unless it should be possible to verify an email
pub trait VerifyEmail {
    fn verify_email(&self, token: EmailVerificationToken) -> Result<(), VerificationError>;
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CreationError {
    #[error("Failed to create Random Number Generator")]
    Rng,
    #[error("Failed to hash generated token")]
    Hash,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum VerificationError {
    #[error("Failed to verify token")]
    Process,
    #[error("Token was invalid")]
    Token,
}

/// A function used to create email tokens.
///
/// Email tokens should only be able to be created in certain situations, so this function must not
/// be in scope unless it should be possible to verify an email
pub fn create_token() -> Result<(EmailToken, HashedEmailToken), CreationError> {
    const HASH_COST: u32 = if cfg!(any(test, feature = "test")) {
        4
    } else {
        bcrypt::DEFAULT_COST
    };

    let token: String = (0..32)
        .map(|_| {
            let random_char: char = rand::random::<char>();
            random_char
        })
        .collect::<String>();

    #[cfg(any(test, feature = "test"))]
    warn!("BUILT IN TEST MODE");

    let h = hash(&token, HASH_COST).map_err(|e| {
        log::error!("Hashing failed: {}", e);
        CreationError::Hash
    })?;

    Ok((EmailToken(token), HashedEmailToken(h)))
}

#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct HashedEmailToken(String);

impl fmt::Debug for HashedEmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for HashedEmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl<DB> serialize::ToSql<Text, DB> for HashedEmailToken
where
    DB: Backend,
    String: serialize::ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, DB>) -> serialize::Result {
        self.0.to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for HashedEmailToken
where
    DB: Backend,
    *const str: deserialize::FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        deserialize::FromSql::<Text, DB>::from_sql(bytes).map(HashedEmailToken)
    }
}

impl VerifyEmail for HashedEmailToken {
    fn verify_email(
        &self,
        email_verification_token: EmailVerificationToken,
    ) -> Result<(), VerificationError> {
        verify(email_verification_token.0, &self.0)
            .map_err(|_| VerificationError::Process)?
            .then(|| ())
            .ok_or(VerificationError::Token)
    }
}

pub struct EmailToken(String);

impl Serialize for EmailToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.0, serializer)
    }
}

impl fmt::Debug for EmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for EmailToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct EmailVerificationToken(String);

impl<'de> Deserialize<'de> for EmailVerificationToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(EmailVerificationToken(String::deserialize(deserializer)?))
    }
}

impl fmt::Debug for EmailVerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

impl fmt::Display for EmailVerificationToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "********")
    }
}

#[cfg(test)]
mod tests {
    use super::{create_token, EmailVerificationToken, VerifyEmail};
    use crate::test_helper::transmute_email_token;

    #[test]
    fn create_and_verify_token() {
        let (email_token, hashed_token) = create_token().unwrap();
        let verification_token = transmute_email_token(&email_token).unwrap();

        assert!(
            hashed_token.verify_email(verification_token).is_ok(),
            "Should have verified email with correct token"
        );
    }

    #[test]
    fn dont_verify_invalid_token() {
        let (_email_token, hashed_token) = create_token().unwrap();
        let fake_token = EmailVerificationToken("fake token".to_owned());

        assert!(
            hashed_token.verify_email(fake_token).is_err(),
            "Should not have verified invalid token"
        );
    }

    #[test]
    fn token_creation_works() {
        let (email_token, hashed_token) = create_token().unwrap();
        assert!(!email_token.0.is_empty(), "Email token should not be empty");
        assert!(
            !hashed_token.0.is_empty(),
            "Hashed token should not be empty"
        );
    }
}
