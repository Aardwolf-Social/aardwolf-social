use gettext::Catalog;

pub struct Shortcuts<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) profile_link: &'a str,
    pub(crate) username: &'a str,
}

impl<'a> Shortcuts<'a> {
    pub fn new(catalog: &'a Catalog, profile_link: &'a str, username: &'a str) -> Self {
        Self {
            catalog,
            profile_link,
            username,
        }
    }
}
