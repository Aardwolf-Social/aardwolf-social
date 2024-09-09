use super::Input;

pub struct InputText<'a> {
    name: &'a str,
    label: String,
    placeholder: Option<String>,
    icon: Option<&'a str>,
    value: &'a str,
    error: Option<String>,
}

impl<'a> Default for InputText<'a> {
    fn default() -> Self {
        Self {
            name: "",
            label: "".into(),
            placeholder: None,
            icon: None,
            value: "",
            error: None,
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
