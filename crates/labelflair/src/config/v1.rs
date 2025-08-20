//! Configuration for Labelflair version 1
//!
//! This module defines the configuration structure for Labelflair version 1.

use std::collections::HashMap;

use getset::Getters;
use serde::Deserialize;
use typed_builder::TypedBuilder;

pub use self::group::*;
pub use self::label_variant::*;

mod group;
mod label_variant;

/// Configuration for Labelflair version 1
///
/// This struct represents the configuration for Labelflair version 1. It contains a map of label
/// groups, where each [`Group`] is identified by a [`GroupName`]. Each group contains an optional
/// prefix, a color generator, and a list of labels.
#[derive(Clone, Eq, PartialEq, Debug, Getters, Deserialize, TypedBuilder)]
pub struct ConfigV1 {
    /// A map of label groups
    #[getset(get = "pub")]
    #[serde(flatten)]
    groups: HashMap<GroupName, Group>,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::colors::{Colors, Tailwind};

    use super::*;

    #[test]
    fn trait_deserialize() {
        let toml = indoc! {r#"
            [categories]
            prefix = "C-"
            colors = { tailwind = "red" }
            labels = ["bug", { name = "feature", description = "A new feature" }]
        "#};

        let config: ConfigV1 = toml::from_str(toml).unwrap();
        let expected = ConfigV1::builder()
            .groups(HashMap::from([(
                GroupName::new("categories"),
                Group::builder()
                    .prefix(Prefix::new("C-"))
                    .colors(Colors::Tailwind(Tailwind::Red))
                    .labels(vec![
                        LabelVariant::Name("bug".into()),
                        LabelVariant::WithDescription {
                            name: "feature".into(),
                            description: "A new feature".into(),
                        },
                    ])
                    .build(),
            )]))
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
