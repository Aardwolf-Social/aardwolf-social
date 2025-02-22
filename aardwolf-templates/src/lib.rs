use rust_i18n::i18n;
use std::borrow::Cow;

i18n!("../aardwolf-localization/locales", fallback = "en-us");

#[derive(Debug, Clone)]
pub struct Translations {
    locale: Cow<'static, str>,
}

impl Translations {
    pub fn new(locale: impl Into<Cow<'static, str>>) -> Self {
        Self { locale: locale.into() }
    }

    pub fn get(&self, key: &str) -> String {
        t!(key, locale = self.locale.as_ref())
    }

    pub fn get_or_fallback(&self, key: &str, default: &str) -> String {
        let translation = self.get(key);
        if translation == key {
            default.to_string()
        } else {
            translation
        }
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }
}

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
