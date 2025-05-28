use aardwolf_types::forms::auth::{
    SignInEmailValidationFail, SignInFormState, SignInPasswordValidationFail,
    ValidateSignInFormFail,
};
use rust_i18n::t;

use crate::elements::alert::{Alert, AlertKind};
use crate::elements::input::{InputEmail, InputPassword};
use crate::sign_up::SignUp;
use crate::Renderable;

pub struct SignIn<'a> {
    pub csrf: &'a str,
    pub alert: Option<Alert>,
    pub email: InputEmail<'a>,
    pub password: InputPassword<'a>,
}

impl<'a> SignIn<'a> {
    pub fn new(
        csrf: &'a str,
        state: &'a SignInFormState,
        validation_error: Option<&'a ValidateSignInFormFail>,
        server_error: bool,
    ) -> Self {
        let alert = if server_error {
            Some(Alert {
                kind: AlertKind::Error,
                message: t!("error.logging_in").into_owned(),
            })
        } else {
            None
        };

        let email = InputEmail {
            name: "email",
            label: t!("email").into_owned(),
            placeholder: Option::from(t!("email_placeholder").into_owned()),
            value: &state.email,
            error: validation_error.and_then(|e| {
                e.email.as_ref().map(|e| match *e {
                    SignInEmailValidationFail::Empty => t!("Email cannot be empty").into_owned(),
                })
            }),
        };

        let password = InputPassword {
            name: "password",
            label: t!("password").into_owned(),
            placeholder: Option::from(t!("password_placeholder").into_owned()),
            value: &state.password,
            error: validation_error.and_then(|e| {
                e.password.as_ref().map(|e| match *e {
                    SignInPasswordValidationFail::Empty => {
                        t!("Password cannot be empty").into_owned()
                    }
                })
            }),
        };

        Self {
            csrf,
            alert,
            email,
            password,
        }
    }
}

impl<'a> Renderable for SignIn<'a> {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // implementation for rendering the HTML template for SignUp
        Ok(()) // return a successful Result
    }
}
