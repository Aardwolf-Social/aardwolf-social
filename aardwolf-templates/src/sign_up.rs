use aardwolf_types::forms::auth::{
    SignUpEmailValidationFail, SignUpFormState, SignUpPasswordConfirmationValidationFail,
    SignUpPasswordValidationFail, ValidateSignUpFormFail,
};

use crate::elements::alert::{Alert, AlertKind};
use crate::elements::input::{InputEmail, InputPassword, InputPasswordConfirm};
use crate::Renderable;

pub struct SignUp<'a> {
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) email: InputEmail<'a>,
    pub(crate) password: InputPassword<'a>,
    pub(crate) password_confirmation: InputPasswordConfirm<'a>,
}
impl<'a> SignUp<'a> {
    pub fn new(
        csrf: &'a str,
        state: &'a SignUpFormState,
        validation_error: Option<&'a ValidateSignUpFormFail>,
        server_error: bool,
    ) -> Self {
        SignUp {
            csrf,
            alert: if server_error {
                Some(Alert {
                    kind: AlertKind::Error,
                    message: t!("There was an error creating your account").into_owned(),
                })
            } else {
                None
            },
            email: InputEmail {
                name: "email",
                label: t!("E-Mail Address").into_owned(),
                placeholder: Option::from(t!("E-Mail Address").into_owned()),
                value: &state.email,
                error: validation_error
                    .and_then(|e| e.email.as_ref())
                    .map(|e| match *e {
                        SignUpEmailValidationFail::Empty => {
                            t!("Email cannot be empty").into_owned()
                        }
                        SignUpEmailValidationFail::Malformed => {
                            t!("Invalid email address").into_owned()
                        }
                    }),
            },
            password: InputPassword {
                name: "password",
                label: t!("Password").into_owned(),
                placeholder: Option::from(t!("Password").into_owned()),
                value: "",
                error: validation_error.and_then(|e| {
                    e.password.as_ref().map(|e| match e {
                        SignUpPasswordValidationFail::Empty => {
                            t!("Password cannot be empty").into_owned()
                        }
                        SignUpPasswordValidationFail::TooShort => {
                            t!("Password is too short").into_owned()
                        }
                    })
                }),
            },
            password_confirmation: InputPasswordConfirm {
                name: "password",
                label: t!("Password").into_owned(),
                placeholder: Option::from(t!("Password").into_owned()),
                value: "",
                error: validation_error.and_then(|e| {
                    e.password.as_ref().map(|e| match e {
                        SignUpPasswordValidationFail::Empty => {
                            t!("Password cannot be empty").into_owned()
                        }
                        SignUpPasswordValidationFail::TooShort => {
                            t!("Password is too short").into_owned()
                        }
                    })
                }),
            },
        }
    }
}

impl<'a> Renderable for SignUp<'a> {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        Ok(()) // return a successful Result
    }
}
