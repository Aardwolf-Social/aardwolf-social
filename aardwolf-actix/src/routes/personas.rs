use aardwolf_models::{base_actor::persona::Persona, user::AuthenticatedUser};
use aardwolf_templates::FirstLogin;
use aardwolf_types::{
    error::AardwolfFail,
    forms::personas::{
        PersonaCreationFail, PersonaCreationForm, PersonaCreationFormState,
        ValidatePersonaCreationFail, ValidatePersonaCreationForm,
    },
    operations::{
        check_create_persona_permission::{
            CheckCreatePersonaPermission, CheckCreatePersonaPermissionFail,
        },
        check_delete_persona_permission::CheckDeletePersonaPermission,
        create_persona::CreatePersona,
        delete_persona::{DeletePersona, DeletePersonaFail},
        fetch_persona::FetchPersona,
    },
    traits::{DbAction, DbActionError, Validate},
};
use actix_session::Session;
use actix_web::{
    web::{Data, Form, Path},
    HttpResponse, ResponseError,
};
use serde_derive::Serialize;
use std::fmt;
use thiserror::Error;

use crate::{
    action::redirect,
    error::redirect_error,
    traits::{RenderableExt, WithRucte},
    types::user::SignedInUser,
    AppConfig,
};

pub(crate) async fn new((_user, i18n): (SignedInUser, Data<I18n>)) -> HttpResponse {
    let res = FirstLogin::new(
        &i18n.catalog,
        "csrf",
        &PersonaCreationFormState::default(),
        None,
        false,
    )
    .ok();

    drop(i18n);

    res
}

async fn create_inner(
    state: &AppConfig,
    form: PersonaCreationForm,
    user: AuthenticatedUser,
    session: Session,
) -> Result<HttpResponse, PersonaCreateError> {
    let form = ValidatePersonaCreationForm(form).validate()?;
    let creator = CheckCreatePersonaPermission(user)
        .run(state.pool.clone())
        .await?;
    let (_, persona) = CreatePersona(creator, form, state.generator.clone())
        .run(state.pool.clone())
        .await?;
    set_persona_cookie(session, persona)?;
    Ok(redirect("/"))
}

pub(crate) async fn create(
    (session, state, user, form, i18n): (
        Session,
        Data<AppConfig>,
        SignedInUser,
        Form<PersonaCreationForm>,
        Data<I18n>,
    ),
) -> Result<HttpResponse, PersonaCreateResponseError> {
    let form = form.into_inner();
    let form_state = form.as_state();

    create_inner(state.as_ref(), form, user.0, session)
        .await
        .map_err(move |error| PersonaCreateResponseError {
            i18n: I18n {
                // I18n can't be cloned but its fields can be
                catalog: i18n.catalog.clone(),
                lang: i18n.lang,
            },
            csrf_token: "csrf token".to_owned(),
            form_state,
            error,
        })
}

pub(crate) async fn delete(
    (state, user, id): (Data<AppConfig>, SignedInUser, Path<i32>),
) -> Result<HttpResponse, PersonaDeleteError> {
    let persona = FetchPersona(id.into_inner())
        .run(state.pool.clone())
        .await?;
    let deleter = CheckDeletePersonaPermission(user.0, persona)
        .run(state.pool.clone())
        .await?;
    DeletePersona(deleter).run(state.pool.clone()).await?;
    Ok(redirect("/"))
}

#[derive(Error)]
#[error("Error")]
pub struct PersonaCreateResponseError {
    i18n: I18n,
    csrf_token: String,
    form_state: PersonaCreationFormState,
    error: PersonaCreateError,
}

impl fmt::Debug for PersonaCreateResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...")
    }
}

impl ResponseError for PersonaCreateResponseError {
    fn error_response(&self) -> HttpResponse {
        let (mut res, validation, system) = match self.error {
            PersonaCreateError::Form(ref e) => (HttpResponse::BadRequest(), Some(e), false),
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        res.ructe(FirstLogin::new(
            &self.i18n.catalog,
            &self.csrf_token,
            &self.form_state,
            validation,
            system,
        ))
    }
}

#[derive(Clone, Debug, Error)]
pub enum PersonaCreateError {
    #[error("Error talking to db actor")]
    Canceled,
    #[error("Error talking db")]
    Database,
    #[error("User does not have permission to create a persona")]
    Permission,
    #[error("Could not set cookie")]
    Cookie,
    #[error("Submitted form is invalid")]
    Form(#[source] ValidatePersonaCreationFail),
    #[error("Could not generate keys")]
    Keygen,
}

impl From<ValidatePersonaCreationFail> for PersonaCreateError {
    fn from(e: ValidatePersonaCreationFail) -> Self {
        PersonaCreateError::Form(e)
    }
}

impl From<PersonaCreationFail> for PersonaCreateError {
    fn from(e: PersonaCreationFail) -> Self {
        match e {
            PersonaCreationFail::Validation(e) => PersonaCreateError::Form(e),
            PersonaCreationFail::Permission => PersonaCreateError::Permission,
            PersonaCreationFail::Database => PersonaCreateError::Database,
            PersonaCreationFail::Keygen => PersonaCreateError::Keygen,
        }
    }
}

impl From<CheckCreatePersonaPermissionFail> for PersonaCreateError {
    fn from(e: CheckCreatePersonaPermissionFail) -> Self {
        match e {
            CheckCreatePersonaPermissionFail::Database => PersonaCreateError::Database,
            CheckCreatePersonaPermissionFail::Permission => PersonaCreateError::Permission,
        }
    }
}

impl From<DbActionError<CheckCreatePersonaPermissionFail>> for PersonaCreateError {
    fn from(e: DbActionError<CheckCreatePersonaPermissionFail>) -> Self {
        match e {
            DbActionError::Pool(_) => PersonaCreateError::Database,
            DbActionError::Canceled => PersonaCreateError::Canceled,
            DbActionError::Error(e) => e.into(),
        }
    }
}

impl From<DbActionError<PersonaCreationFail>> for PersonaCreateError {
    fn from(e: DbActionError<PersonaCreationFail>) -> Self {
        match e {
            DbActionError::Pool(_) => PersonaCreateError::Database,
            DbActionError::Canceled => PersonaCreateError::Canceled,
            DbActionError::Error(e) => e.into(),
        }
    }
}

fn set_persona_cookie(session: Session, persona: Persona) -> Result<(), PersonaCreateError> {
    session
        .insert("persona_id", persona.id())
        .map_err(|_| PersonaCreateError::Cookie)
}

#[derive(Clone, Debug, Error, Serialize)]
pub enum PersonaDeleteError {
    #[error("Error talking to db actor")]
    Canceled,
    #[error("Error talking db")]
    Database,
    #[error("Error deleting persona: {}", _0)]
    Delete(#[source] DeletePersonaFail),
}

impl<E> From<DbActionError<E>> for PersonaDeleteError
where
    E: Into<DeletePersonaFail> + AardwolfFail,
{
    fn from(e: DbActionError<E>) -> Self {
        match e {
            DbActionError::Pool(_) => PersonaDeleteError::Database,
            DbActionError::Canceled => PersonaDeleteError::Canceled,
            DbActionError::Error(e) => PersonaDeleteError::Delete(e.into()),
        }
    }
}

impl ResponseError for PersonaDeleteError {
    fn error_response(&self) -> HttpResponse {
        redirect_error("/personas", None)
    }
}
