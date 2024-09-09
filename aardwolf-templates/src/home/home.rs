use aardwolf_types::forms::posts::{PostCreationFormState, ValidatePostCreationFail};
use gettext::Catalog;

use crate::{
    asides::Shortcuts,
    elements::{InputSelect, InputText, InputTextarea},
    home::{feed::Feed, nav_top::NavTop},
    posts::NewPost,
    Renderable,
};

pub struct Home<'a> {
    catalog: &'a Catalog,
    shortcuts: Shortcuts<'a>,
    nav_top: NavTop<'a>,
    feed: Feed<'a>,
    new_post: NewPost<'a>,
}

impl<'a> Home<'a> {
    pub fn new(
        catalog: &'a Catalog,
        profile_link: &'a str,
        username: &'a str,
        csrf_token: &'a str,
        form_state: &'a PostCreationFormState,
        validation_error: Option<&'a ValidatePostCreationFail>,
        server_error: bool,
    ) -> Self {
        Self {
            catalog,
            shortcuts: Shortcuts {
                catalog,
                profile_link,
                username,
            },
            nav_top: NavTop { catalog },
            feed: Feed { catalog },
            new_post: NewPost {
                csrf_token,
                alert: None, // or provide a value for alert
                catalog,
                username,
                post_source: InputTextarea::default(),
                post_visibility: InputSelect::default(),
                post_name: InputText::default(),
            },
        }
    }
}

impl<'a> Renderable for Home<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::home::home_html(write, self)
    }
}
