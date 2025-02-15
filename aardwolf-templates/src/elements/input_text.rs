use super::input::Input;

#[derive(Debug)]
pub struct InputText<'a> {
    pub name: &'a str,
    pub label: String,
    pub placeholder: Option<String>,
    pub icon: Option<&'a str>,
    pub value: &'a str,
    pub error: Option<String>,
}

impl<'a> Default for InputText<'a> {
    fn default() -> Self {
        Self {
            name: "",
            label: String::new(),
            placeholder: None,
            icon: None,
            value: "",
            error: None,
        }
    }
}

impl<'a> InputText<'a> {
    pub fn new(
        name: &'a str,
        label: Option<&'a str>,
        value: Option<&'a str>,
        error: Option<&'a str>,
    ) -> Self {
        Self {
            name,
            label: label.map_or_else(String::new, Into::into),
            value: value.unwrap_or(""),
            error: error.map(Into::into),
            ..Default::default()
        }
    }
}

impl<'a> From<&'a InputText<'a>> for Input<'a> {
    fn from(input_text: &'a InputText<'a>) -> Self {
        Self {
            kind: "text",
            name: input_text.name,
            label: Some(input_text.label.clone()),
            placeholder: input_text.placeholder.clone(),
            icon: input_text.icon,
            value: input_text.value,
            error: input_text.error.clone(),
        }
    }
}
