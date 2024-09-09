use gettext_macros::init_i18n;
use rocket_i18n::Translations;

init_i18n!("aardwolf", en, pl);

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

/// Returns an empty Translations object to disable translations due to issues with the gettext library.
pub fn disabled_translations() -> Translations {
    Translations::default()
}
