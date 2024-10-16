use gettext::Catalog;

pub struct NavTop<'a> {
    pub(crate) catalog: &'a Catalog,
}

impl<'a> NavTop<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Self { catalog }
    }
}
