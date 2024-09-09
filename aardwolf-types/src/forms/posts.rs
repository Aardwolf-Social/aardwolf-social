use aardwolf_models::sql_types::{Mime, PostVisibility};
use mime::TEXT_HTML;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use std::fmt;

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

#[derive(Clone, Debug)]
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

pub struct ValidatePostCreationForm(pub PostCreationForm);

impl Validate for ValidatePostCreationForm {
    type Item = ValidatedPostCreationForm;
    type Error = ValidatePostCreationError;

    fn validate(self) -> Result<Self::Item, Self::Error> {
        let source = self.0.source.trim().to_string();
        let content = source.clone();
        let visibility = self.0.visibility;
        let media_type = TEXT_HTML.into();

        let name = match self.0.name {
            Some(name) if name.trim().is_empty() => return Err(ValidatePostCreationError::EmptyName),
            Some(name) => Some(name.trim().to_string()),
            None => None,
        };

        if source.is_empty() {
            return Err(ValidatePostCreationError::EmptySource);
        }

        Ok(ValidatedPostCreationForm {
            media_type,
            visibility,
            content,
            source,
            name,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ValidatedPostCreationForm {
    pub(crate) media_type: Mime,
    pub(crate) visibility: PostVisibility,
    pub(crate) content: String,
    pub(crate) source: String,
    pub(crate) name: Option<String>,
}

impl fmt::Display for ValidatePostCreationForm {
    fn fmt(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.validate() {
            Err(ValidatePostCreationError::EmptyName) => write!(f, "Name must not be empty"),
            Err(ValidatePostCreationError::EmptySource) => write!(f, "Source must not be empty"),
            Err(ValidatePostCreationError::InvalidVisibility) => {
                write!(f, "Invalid visibility")
            }
            _ => write!(f, "Validation successful"),
            Ok(_) => todo!(),
        }
    }
}
