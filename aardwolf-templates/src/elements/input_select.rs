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
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption<'a> {
    pub value: &'a str,
    pub display: String,
}

impl<'a> InputSelect<'a> {
    pub fn with_follow_policy_options(catalog: &'a Catalog) -> Self {
        Self {
            name: "follow_policy",
            label: i18n!(catalog, "Follow policy"),
            selected_value: FollowPolicy::AutoAccept.into(),
            options: follow_policy_options(catalog),
            error: None,
        }
    }

    pub fn with_visibility_options(catalog: &'a Catalog) -> Self {
        Self {
            name: "visibility",
            label: i18n!(catalog, "Post visibility"),
            selected_value: PostVisibility::Public.into(),
            options: visibility_options(catalog),
            error: None,
        }
    }
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

