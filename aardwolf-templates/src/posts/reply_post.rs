use gettext::Catalog;

use crate::elements::{Alert, InputSelect, InputText, InputTextarea};

pub struct ReplyPost<'a> {
    pub catalog: &'a Catalog,
    pub csrf_token: &'a str,
    pub alert: Option<Alert>,
    pub author_username: &'a str,
    pub post_source: InputTextarea<'a>,
    pub post_visibility: InputSelect<'a>,
    pub post_name: InputText<'a>,
}

