use aardwolf_types::forms::posts::{PostCreationFormState, ValidatePostCreationFail};
use gettext::Catalog;

use crate::{Alert, AlertKind, InputSelect, InputText, InputTextarea};

pub struct NewPost<'a> {
    csrf_token: &'a str,
    alert: Option<Alert>,
    catalog: &'a Catalog,
    username: &'a str,
    post_source: InputTextarea<'a>,
    post_visibility: InputSelect<'a>,
    post_name: InputText<'a>,
}

impl<'a> NewPost<'a> {
    pub fn new(
        catalog: &'a Catalog,
        csrf_token: &'a str,
        form_state: &'a PostCreationFormState,
        validation_error: Option<&'a ValidatePostCreationFail>,
    ) -> Self {
        Self {
            csrf_token,
            alert: validation_error.map(|e| Alert {
                kind: AlertKind::Error,
                message: e.to_string(),
            }),
            catalog,
            username: form_state.username.as_ref(),
            post_source: InputTextarea::new(
                "source",
                Some(catalog.gettext("Post source")),
                form_state.source.as_ref(),
                validation_error
                    .and_then(|e| e.source.as_ref())
                    .map(|e| e.to_string()),
            ),
            post_visibility: InputSelect::new(
                "visibility",
                Some(catalog.gettext("Post visibility")),
                form_state.visibility,
                validation_error
                    .and_then(|e| e.visibility.as_ref())
                    .map(|e| e.to_string()),
            ),
            post_name: InputText::new(
                "name",
                Some(catalog.gettext("Name")),
                form_state.name.as_ref(),
                validation_error
                    .and_then(|e| e.name.as_ref())
                    .map(|e| e.to_string()),
            ),
        }
    }
}

