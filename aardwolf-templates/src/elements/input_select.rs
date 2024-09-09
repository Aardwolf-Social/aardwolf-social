use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use gettext::Catalog;
use gettext_macros::i18n;

#[derive(Clone, Debug, PartialEq)]
pub struct InputSelect<'a> {
    pub name: &'a str,
    pub label: String,
    pub selected_value: String,
    pub options: Vec<SelectOption<'a>>,
    pub error: Option<String>,
    pub(crate) selected: String,
}

impl<'a> InputSelect<'a> {
    pub fn new(
        name: &'a str,
        label: Option<&str>,
        value: &str,
        error: Option<&str>,
    ) -> Self {
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
            label: i18n!(catalog, "Post visibility"),
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
            display: i18n!(catalog, "Automatically accept new followers"),
        },
        SelectOption {
            value: FollowPolicy::AutoReject.into(),
            display: i18n!(catalog, "Automatically reject new followers"),
        },
        SelectOption {
            value: FollowPolicy::ManualReview.into(),
            display: i18n!(catalog, "Manually review new followers"),
        },
    ]
}

fn visibility_options(catalog: &Catalog) -> Vec<SelectOption<'_>> {
    vec![
        SelectOption {
            value: PostVisibility::Public.into(),
            display: i18n!(catalog, "Visible to everyone"),
        },
        SelectOption {
            value: PostVisibility::FollowersOnly.into(),
            display: i18n!(catalog, "Visible to followers"),
        },
        SelectOption {
            value: PostVisibility::FriendsOnly.into(),
            display: i18n!(catalog, "Visible to mutuals"),
        },
        SelectOption {
            value: PostVisibility::ListedPeopleOnly.into(),
            display: i18n!(catalog, "Only visible to mentioned users"),
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