use gettext::Catalog;

use crate::elements::{Alert, InputSelect, InputText, InputTextarea};

pub struct NewPost<'a> {
    pub(crate) csrf_token: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) catalog: &'a Catalog,
    pub(crate) username: &'a str,
    pub(crate) post_source: InputTextarea<'a>,
    pub(crate) post_visibility: InputSelect<'a>,
    pub(crate) post_name: InputText<'a>,
}

