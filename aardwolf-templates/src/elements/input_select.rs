use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use gettext::Catalog;
use rust_i18n::t;

#[derive(Clone, Debug, PartialEq)]
pub struct InputSelect<'a> {
    pub name: &'a str,
    pub label: String,
    pub selected_value: String,
    pub options: Vec<SelectOption<'a>>,
    pub error: Option<String>,
    pub(crate) selected: String,
}

impl<'a> Default for InputSelect<'a> {
    fn default() -> Self {
        InputSelect {
            name: "",
            label: "".to_string(),
            selected_value: "".to_string(),
            options: vec![],
            error: None,
            selected: "".to_string(), // Add this line to provide a value for the selected field
        }
    }
}

impl<'a> InputSelect<'a> {
    pub fn new(name: &'a str, label: Option<&str>, value: &str, error: Option<&str>) -> Self {
        Self {
            name,
            label: label.map(|l| l.to_string()).unwrap_or_default(),
            selected_value: value.to_string(),
            options: Vec::new(),
            error: error.map(|e| e.to_string()),
            selected: value.to_string(),
        }
    }

    pub fn with_visibility_options(catalog: &'a Catalog) -> Self {
        Self {
            name: "visibility",
            label: t!("Post visibility"),
            selected_value: PostVisibility::Public.to_string(),
            options: visibility_options(catalog),
            error: None,
            selected: PostVisibility::Public.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption<'a> {
    pub value: &'a str,
    pub display: String,
}

fn follow_policy_options(catalog: &Catalog) -> Vec<SelectOption<'_>> {
    vec![
        SelectOption {
            value: FollowPolicy::AutoAccept.into(),
            display: t!("Automatically accept new followers"),
        },
        SelectOption {
            value: FollowPolicy::AutoReject.into(),
            display: t!("Automatically reject new followers"),
        },
        SelectOption {
            value: FollowPolicy::ManualReview.into(),
            display: t!("Manually review new followers"),
        },
    ]
}

fn visibility_options(catalog: &Catalog) -> Vec<SelectOption<'_>> {
    vec![
        SelectOption {
            value: PostVisibility::Public.into(),
            display: t!("Visible to everyone"),
        },
        SelectOption {
            value: PostVisibility::FollowersOnly.into(),
            display: t!("Visible to followers"),
        },
        SelectOption {
            value: PostVisibility::FriendsOnly.into(),
            display: t!("Visible to mutuals"),
        },
        SelectOption {
            value: PostVisibility::ListedPeopleOnly.into(),
            display: t!("Only visible to mentioned users"),
        },
    ]
}

impl InputSelect<'_> {
    pub fn with_follow_policy_options(catalog: &Catalog) -> Vec<SelectOption<'_>> {
        follow_policy_options(catalog)
    }
}

enum ValidateFollowPolicyFail {
    Listed,
    // Other variants...
}
