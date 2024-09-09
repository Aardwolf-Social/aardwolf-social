use gettext::Catalog;

use crate::elements::{InputSelect, InputText, InputTextarea};

#[derive(Debug)]
pub struct ReplyPost<'a> {
    pub catalog: &'a Catalog,
    pub csrf_token: &'a str,
    pub alert: Option<crate::Alert>,
    pub author: &'a str,
    pub source: InputTextarea<'a>,
    pub visibility: InputSelect<'a>,
    pub title: InputText<'a>,
}
