use gettext::Catalog;
use std::io::{Write, Result};

use crate::Renderable;

pub struct Feed<'a> {
    pub(crate) catalog: &'a Catalog,
}

impl<'a> Feed<'a> {
    pub fn new(catalog: &'a Catalog) -> Self {
        Feed { catalog }
    }
}

impl<'a> Renderable for Feed<'a> {
    fn render(&self, writer: &mut dyn Write) -> Result<()> {
        write!(writer, "{}", self.catalog.gettext("This is the feed template"))?;
        writer.flush()?; // Ensure the writer is flushed after writing
        Ok(())
    }
}