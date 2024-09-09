use gettext::Catalog;

use crate::elements::{alert::Alert, input_select::InputSelect, input_text::InputText, input_textarea::InputTextarea};

#[derive(Debug)]
pub struct ReplyPost<'a> {
    pub catalog: &'a Catalog,
    pub csrf_token: &'a str,
    pub alert: Option<Alert>,
    pub author: &'a str,
    pub source: InputTextarea<'a>,
    pub visibility: InputSelect<'a>,
    pub title: InputText<'a>,
}
