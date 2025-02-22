use gettext::Catalog;

/// A struct representing the content of a notification.
pub struct NotificationContent<'a> {
    /// The translation catalog to use for translating the notification content.
    pub catalog: &'a Catalog,
}
