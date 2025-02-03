use rust_i18n::t; // Import localization crate
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

#[derive(Clone, Debug, Default)]
pub struct Translations {
    locale: &'static str,
}

impl Translations {
    pub fn new(locale: &'static str) -> Self {
        Self { locale }
    }

    pub fn get(&self, key: &str) -> String {
        t!(key, locale = self.locale)
    }
}

