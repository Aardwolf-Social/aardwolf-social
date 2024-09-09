#[derive(Debug)]
pub struct InputTextarea<'a> {
    pub name: &'a str,
    pub label: Option<String>,
    pub icon: Option<&'a str>,
    pub placeholder: Option<String>,
    pub value: &'a str,
    pub error: Option<String>,
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

impl<'a> InputTextarea<'a> {
    pub fn new(
        name: &'a str,
        label: Option<&'a str>,
        value: Option<&'a str>,
        error: Option<&'a str>,
    ) -> Self {
        Self {
            name,
            label: label.map(ToString::to_string),
            value: value.unwrap_or_default(),
            error: error.map(ToString::to_string),
            ..Default::default()
        }
    }
}
