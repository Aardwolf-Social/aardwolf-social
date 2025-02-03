#[macro_use]
extern crate rust_i18n;

pub struct Translations {
    locale: std::borrow::Cow<'static, str>,
}

impl Translations {
    pub fn new<S: Into<std::borrow::Cow<'static, str>>>(locale: S) -> Self {
        Self { locale: locale.into() }
    }

    pub fn get(&self, key: &str) -> String {
        t!(key, locale = self.locale.as_ref())
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub mod asides;
pub mod containers;
pub mod elements;
pub mod error;
pub mod home;
pub mod posts;
pub mod first_login;
pub mod sign_in;
pub mod sign_up;

pub trait Renderable {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()>;
}
