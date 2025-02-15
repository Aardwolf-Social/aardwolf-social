use gettext::Catalog;

/// A dropdown menu for displaying notifications.
#[derive(Debug)]
pub struct NotificationDropdown<'a> {
    /// The gettext catalog for translating strings.
    catalog: &'a Catalog,
}
