use gettext::Catalog;

use crate::Renderable;

pub struct NavTop<'a> {
    pub(crate) catalog: &'a Catalog,
}

impl<'a> NavTop<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Self { catalog }
    }
}

impl<'a> Renderable for NavTop<'a> {
    fn render(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::home::nav_top_html(writer, self.catalog)
    }
}