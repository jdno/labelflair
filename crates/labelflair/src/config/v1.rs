//! Configuration for Labelflair version 1
//!
//! This module defines the configuration structure for Labelflair version 1.

use getset::Getters;
use serde::Deserialize;
use typed_builder::TypedBuilder;

use crate::label::Label;

pub use self::group::*;
pub use self::label_variant::*;

mod group;
mod label_variant;

/// Configuration for Labelflair version 1
///
/// This struct represents the configuration for Labelflair version 1. It contains a list of
/// individual labels and a map of label groups, where each [`Group`] is identified by a
/// [`GroupName`]. Each group contains an optional prefix, a color generator, and a list of labels.
#[derive(
    Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Getters, Deserialize, TypedBuilder,
)]
pub struct ConfigV1 {
    /// A list of individual labels
    #[getset(get = "pub")]
    #[serde(default, rename = "label")]
    labels: Vec<Label>,

    /// A map of label groups
    #[getset(get = "pub")]
    #[serde(default, rename = "group")]
    groups: Vec<Group>,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::colors::{Colors, Tailwind};

    use super::*;

    #[test]
    fn trait_deserialize() {
        let toml = indoc! {r##"
            [[label]]
            name = "good first issue"
            color = "#4ade80"
            description = "Good issue for newcomers"
            aliases = ["help wanted"]

            [[group]]
            prefix = "C-"
            colors = { tailwind = "red" }
            labels = ["bug", { name = "feature", description = "A new feature" }]
        "##};

        let config: ConfigV1 = toml::from_str(toml).unwrap();
        let expected = ConfigV1::builder()
            .labels(vec![
                Label::builder()
                    .name("good first issue")
                    .color("#4ade80")
                    .description(Some("Good issue for newcomers".into()))
                    .aliases(vec!["help wanted".into()])
                    .build(),
            ])
            .groups(vec![
                Group::builder()
                    .prefix(Prefix::new("C-"))
                    .colors(Colors::Tailwind(Tailwind::Red))
                    .labels(vec![
                        LabelVariant::Name("bug".into()),
                        LabelVariant::WithDescription {
                            name: "feature".into(),
                            description: Some("A new feature".into()),
                            aliases: Vec::new(),
                        },
                    ])
                    .build(),
            ])
            .build();

        assert_eq!(config, expected);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ConfigV1>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<ConfigV1>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<ConfigV1>();
    }
}
