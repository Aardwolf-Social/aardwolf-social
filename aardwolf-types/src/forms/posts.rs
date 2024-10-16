use aardwolf_models::sql_types::{Mime, PostVisibility};
use mime::TEXT_HTML;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use crate::{error::AardwolfFail, traits::Validate};

#[derive(Clone, Debug, Deserialize)]
pub struct PostCreationForm {
    csrf_token: String,
    visibility: PostVisibility,
    name: Option<String>,
    source: String,
}

impl PostCreationForm {
    pub fn into_state(self) -> PostCreationFormState {
        PostCreationFormState {
            visibility: self.visibility,
            name: self.name,
            source: self.source,
            username: String::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PostCreationFormState {
    pub visibility: PostVisibility,
    pub name: Option<String>,
    pub source: String,
    pub username: String,
}

#[derive(Clone, Debug, Error, Serialize)]
pub enum ValidatePostCreationError {
    #[error("Invalid visibility")]
    InvalidVisibility,
    #[error("Source must not be empty")]
    EmptySource,
    #[error("Name must not be empty")]
    EmptyName,
}

impl AardwolfFail for ValidatePostCreationError {}

pub struct ValidatePostCreationForm(PostCreationForm);

impl Validate for ValidatePostCreationForm {
    type Item = ValidatedPostCreationForm;
    type Error = ValidatePostCreationError;

    fn validate(&self) -> Result<Self::Item, Self::Error> {
        if self.0.source.is_empty() {
            return Err(ValidatePostCreationError::EmptySource);
        }

        let name = self.0.name.as_deref().map(|n| n.trim().to_string()).filter(|n| !n.is_empty());

        Ok(ValidatedPostCreationForm {
            media_type: TEXT_HTML,
            visibility: self.0.visibility,
            content: self.0.source.clone(),
            source: self.0.source.clone(),
            name,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ValidatedPostCreationForm {
    pub media_type: Mime,
    pub visibility: PostVisibility,
    pub content: String,
    pub source: String,
    pub name: Option<String>,
}
