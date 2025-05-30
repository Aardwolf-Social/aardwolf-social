// Load I18n macro, for allow you to use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;
i18n!("locales", fallback = "en");
#[derive(Debug, Clone)]
pub struct Translations {
    locale: Cow<'static, str>,
}

impl Translations {
    pub fn new(locale: impl Into<Cow<'static, str>>) -> Self {
        Self {
            locale: locale.into(),
        }
    }

    pub fn get(&self, key: &str) -> Cow<'static, str> {
        rust_i18n::t!(key, locale = self.locale.as_ref())
            .unwrap_or_else(|e| {
                eprintln!("Failed to translate key '{}': {}", key, e);
                key
            })
            .into()
    }

    pub fn get_or_fallback(&self, key: &str, default: &str) -> Cow<'static, str> {
        let translation = self.get(key);
        if translation == key {
            Cow::from(default.to_string())
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
pub mod first_login;
pub mod home;
pub mod posts;
pub mod sign_in;
pub mod sign_up;

pub trait Renderable {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()>;
}
