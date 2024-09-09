pub struct InputTextarea<'a> {
    name: &'a str,
    label: Option<String>,
    icon: Option<&'a str>,
    placeholder: Option<String>,
    value: &'a str,
    error: Option<String>,
}

impl<'a> Default for InputTextarea<'a> {
    fn default() -> Self {
        Self {
            name: "",
            label: None,
            icon: None,
            placeholder: None,
            value: "",
            error: None,
        }
    }
}
