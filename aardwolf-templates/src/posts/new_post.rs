use aardwolf_types::forms::posts::{PostCreationFormState, ValidatePostCreationForm};
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
        validation_error: Option<&'a ValidatePostCreationForm>,
    ) -> Self {
        let username = form_state.username;
        let alert = validation_error.map(|e| Alert {
            kind: AlertKind::Error,
            message: e.to_string(),
        });

        NewPost {
            csrf_token,
            alert,
            catalog,
            username: Some(username.as_ref()),
            source: InputTextarea::new(
                "source",
                Some(catalog.gettext("Post source")),
                Some(form_state.source.as_str()),
                validation_error
                    .and_then(|e| e.0.source().as_deref()) // Use getter method
                    .map(ToString::to_string)
                    .as_deref(),
            ),
            visibility: InputSelect::new(
                "visibility",
                Some(catalog.gettext("Post visibility")),
                form_state.visibility.into(),
                validation_error
                    .and_then(|e| e.0.source().as_deref()) // Use getter method
                    .map(|e| e.to_string())
                    .as_deref(),
            ),
            name: InputText::new(
                "name",
                Some(catalog.gettext("Post name")),
                form_state.name.as_deref(),
                validation_error
                    .and_then(|e| e.0.name().as_deref()) // Use getter method
                    .map(|e| e.to_string())
                    .as_deref(),
            ),
        }
    }
}
