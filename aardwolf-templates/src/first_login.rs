use aardwolf_types::forms::personas::{
    PersonaCreationFormState, ValidateDisplayNameFail, ValidateFollowPolicyFail,
    ValidateIsSearchableFail, ValidatePersonaCreationFail, ValidateShortnameFail,
};

use crate::elements::{
    alert::{Alert, AlertKind},
    input, input_select,
};

use rust_i18n::t;

pub struct FirstLogin<'a> {
    pub catalog: &'a Catalog,
    pub csrf_token: &'a str,
    pub alert: Option<Alert>,
    pub display_name_input: input::Input<'a>,
    pub shortname_input: input::Input<'a>,
    pub follow_policy_input: input_select::InputSelect<'a>,
    pub default_visibility_input: input_select::InputSelect<'a>,
    pub is_searchable_input: input::InputCheckbox<'a>,
}

impl<'a> FirstLogin<'a> {
    pub fn new(
        catalog: &'a mut Catalog,
        csrf_token: &'a str,
        state: &'a PersonaCreationFormState,
        validation_error: Option<&'a ValidatePersonaCreationFail>,
    ) -> Self {
        let alert = match validation_error {
            Some(_error) => Some(Alert {
                kind: AlertKind::Error,
                message: t!("There was an error creating your persona"),
            }),
            None => None,
        };

        FirstLogin {
            catalog,
            csrf_token,
            alert,
            display_name_input: input::Input {
                name: "display_name",
                label: t!("Display Name"),
                placeholder: t!("Display name"),
                value: &state.display_name,
                error: validation_error
                    .and_then(|e| e.display_name.as_ref())
                    .map(|e| match e {
                        ValidateDisplayNameFail::Empty => {
                            t!("Display name must not be empty")
                        }
                    }),
                icon: Some("fas fa-user"),
                kind: "text",
            },
            shortname_input: input::Input {
                name: "shortname",
                label: t!("Username"),
                placeholder: t!("Username"),
                value: &state.shortname,
                error: validation_error
                    .and_then(|e| e.shortname.as_ref())
                    .map(|e| match e {
                        ValidateShortnameFail::Empty => {
                            t!("Username must not be empty")
                        }
                        ValidateShortnameFail::SpecialCharacters => {
                            t!("Username must not contain special characters")
                        }
                        ValidateShortnameFail::TooLong => {
                            t!("Username is too long")
                        }
                    }),
                icon: Some("fas fa-user"),
                kind: "text",
            },
            default_visibility_input: input_select::InputSelect {
                name: "default_visibility",
                label: t!("Post Visibility"),
                selected: state.default_visibility.to_string(),
                options: input_select::InputSelect::with_visibility_options(catalog).options,
                error: validation_error
                    .and_then(|e| e.is_searchable.as_ref())
                    .map(|e| match e {
                        ValidateIsSearchableFail::SomeError => {
                            t!("Some error message")
                        }
                        ValidateIsSearchableFail::Invalid => todo!(),
                        // Add arms for other possible values of ValidateIsSearchableFail
                    }),
                selected_value: state.default_visibility.to_string(),
            },
            is_searchable_input: input::InputCheckbox {
                name: "is_searchable",
                label: t!("Allow people to search for this profile"),
                checked: state.is_searchable,
                error: validation_error
                    .and_then(|e| e.is_searchable.as_ref())
                    .map(|e| match e {
                        ValidateIsSearchableFail::Invalid => {
                            t!("Invalid value for `is_searchable`")
                        }
                        ValidateIsSearchableFail::SomeError => {
                            t!("Some error message")
                        }
                    }),
                icon: Some("fas fa-user"),
            },
            follow_policy_input: input_select::InputSelect {
                name: "follow_policy",
                label: t!("Follow Policy"),
                selected: state.follow_policy.to_string(),
                options: input_select::InputSelect::with_follow_policy_options(catalog),
                error: validation_error
                    .and_then(|e| e.follow_policy.as_ref())
                    .map(|e| match e {
                        ValidateFollowPolicyFail::Invalid => {
                            t!("Invalid follow policy")
                        }
                    }),
                selected_value: state.follow_policy.to_string(),
            },
        }
    }
}

impl<'a> crate::Renderable for FirstLogin<'a> {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::first_login_html(writer, self)
    }
}
