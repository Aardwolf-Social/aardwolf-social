use aardwolf_types::forms::posts::{PostCreationFormState, ValidatePostCreationForm};
use gettext::Catalog;

use crate::{
    asides::Shortcuts,
    home::{feed::Feed, nav_top::NavTop},
    posts::NewPost,
    Renderable,
};

pub struct Home<'a> {
    pub catalog: &'a Catalog,
    pub shortcuts: Shortcuts<'a>,
    pub nav_top: NavTop<'a>,
    pub feed: Feed<'a>,
    pub new_post: NewPost<'a>,
}

impl<'a> Home<'a> {
    pub fn new(
        catalog: &'a Catalog,
        profile_link: &'a str,
        username: &'a str,
        csrf_token: &'a str,
        form_state: &'a PostCreationFormState,
        validation_error: Option<&'a ValidatePostCreationForm>,
    ) -> Self {
        Self {
            catalog,
            shortcuts: Shortcuts::new(catalog, profile_link, username),
            nav_top: NavTop::new(catalog),
            feed: Feed::new(catalog),
            new_post: NewPost::new(catalog, csrf_token, form_state, validation_error),
        }
    }
}

impl<'a> Renderable for Home<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::home::home_html(write, self)
    }
}

