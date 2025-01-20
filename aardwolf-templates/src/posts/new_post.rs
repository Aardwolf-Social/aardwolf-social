use aardwolf_types::forms::posts::{PostCreationFormState, ValidatePostCreationError};
use gettext::Catalog;

use crate::elements::{
    alert::{Alert, AlertKind},
    input_select::InputSelect,
    input_text::InputText,
    input_textarea::InputTextarea,
};

pub struct NewPost<'a> {
    pub csrf_token: &'a str,
    pub alert: Option<Alert>,
    pub catalog: &'a Catalog,
    pub username: Option<&'a str>,
    pub source: InputTextarea<'a>,
    pub visibility: InputSelect<'a>,
    pub name: InputText<'a>,
}

impl<'a> NewPost<'a> {
    pub fn new(
        catalog: &'a Catalog,
        csrf_token: &'a str,
        form_state: &'a PostCreationFormState,
        validation_error: Option<&'a ValidatePostCreationError>,
    ) -> Self {
        let username = form_state.username.as_ref();
        let alert = match validation_error {
            Some(error) => Some(Alert {
                kind: AlertKind::Error,
                message: error.to_string(),
            }),
            None => None,
        };

        NewPost {
            csrf_token,
            alert,
            catalog,
            username: Some(form_state.username.as_str()),
source: InputTextarea::new(
    "source",
    Some(catalog.gettext("Post source")),
    match form_state.source.as_ref() {
        Some(s) => s.as_str(),
        None => "",
    },,
            ),
            visibility: InputSelect::new(
                "visibility",
                Some(catalog.gettext("Post visibility")),
                form_state.visibility.into(),
                match validation_error {
                    Some(ValidatePostCreationError::InvalidVisibility) => {
                        Some("Invalid visibility")
                    }
                    _ => None,
                },
            ),
            name: InputText::new(
                "name",
                Some(catalog.gettext("Post name")),
                form_state.name.as_deref(),
                match validation_error {
                    Some(ValidatePostCreationError::EmptyName) => {
                        Some("Name must not be empty")
                    }
                    _ => None,
                },
            ),
        }
    }
}

