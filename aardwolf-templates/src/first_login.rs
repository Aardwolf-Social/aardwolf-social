use aardwolf_types::forms::personas::{
    PersonaCreationFormState, ValidateDisplayNameFail, ValidateFollowPolicyFail,
    ValidateIsSearchableFail, ValidatePersonaCreationFail, ValidateShortnameFail,
};

use crate::{
    elements::{alert, input, input_select},
    sign_up::SignUp,
    Renderable,
};

use rust_i18n::t;

/// Represents a first login form.
pub struct FirstLogin<'a> {
    /// CSRF token.
    csrf_token: &'a str,

    /// Alert message.
    alert: Option<alert::Alert>,

    /// Display name input field.
    display_name_input: input::Input<'a>,

    /// Shortname input field.
    shortname_input: input::Input<'a>,

    /// Follow policy input field.
    follow_policy_input: input_select::InputSelect<'a>,

    /// Default visibility input field.
    default_visibility_input: input_select::InputSelect<'a>,

    /// Is searchable input field.
    is_searchable_input: input::InputCheckbox<'a>,
}

impl<'a> FirstLogin<'a> {
    /// Creates a new `FirstLogin` instance.
    pub fn new(
        csrf_token: &'a str,
        state: &'a PersonaCreationFormState,
        validation_error: Option<&'a ValidatePersonaCreationFail>,
    ) -> Self {
        let alert = validation_error.map(|_| alert::Alert {
            kind: alert::AlertKind::Error,
            message: t!("error.creating_persona").into_owned(),
        });

        let display_name_input =
            input::Input::new("display_name", state.display_name.clone(), vec![]);
        let shortname_input = input::Input::new("shortname", state.shortname.clone(), vec![]);
        let follow_policy_input = input_select::InputSelect::new(
            "follow_policy",
            state.follow_policy.clone(),
            vec![
                (
                    FollowPolicy::AutoAccept.to_string(),
                    t!("follow_policy.public"),
                ),
                (
                    FollowPolicy::AutoReject.to_string(),
                    t!("follow_policy.private"),
                ),
            ],
        );
        let default_visibility_input = input_select::InputSelect::new(
            "default_visibility",
            state.default_visibility.clone(),
            vec![
                ("public", t!("default_visibility.public")),
                ("private", t!("default_visibility.private")),
            ],
        );
        let is_searchable_input =
            input::InputCheckbox::new("is_searchable", state.is_searchable, t!("is_searchable"));

        Self {
            csrf_token,
            alert,
            display_name_input,
            shortname_input,
            follow_policy_input,
            default_visibility_input,
            is_searchable_input,
        }
    }
}

impl<'a> Renderable for FirstLogin<'a> {
    /// Renders the HTML template for the first login form.
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // implementation for rendering the HTML template for FirstLogin
        Ok(())
    }
}
