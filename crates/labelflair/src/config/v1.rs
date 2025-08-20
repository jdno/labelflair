//! Configuration for Labelflair version 1
//!
//! This module defines the configuration structure for Labelflair version 1.

use std::collections::HashMap;

use getset::{CopyGetters, Getters};
use serde::Deserialize;
use typed_builder::TypedBuilder;
use typed_fields::name;

use crate::colors::{Colors, Generate};
use crate::label::{Label, LabelName};

name!(
    /// A name for a group of labels in Labelflair
    GroupName
);

name!(
    /// A prefix for labels in Labelflair
    Prefix
);

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

/// A group of labels in Labelflair
///
/// This struct represents a group of labels in Labelflair. Each group has an optional prefix, a
/// color generator, and a list of labels. If a prefix is provided, it will be prepended to each
/// label in the group.
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    CopyGetters,
    Getters,
    Deserialize,
    TypedBuilder,
)]
pub struct Group {
    /// An optional prefix for the labels in this group
    #[builder(setter(into))]
    #[getset(get = "pub")]
    #[serde(default)]
    prefix: Option<Prefix>,

    /// The color generator for this group
    #[getset(get_copy = "pub")]
    colors: Colors,

    /// A list of labels in this group
    #[getset(get = "pub")]
    labels: Vec<LabelName>,
}

impl Group {
    /// Expand the group into a list of labels
    ///
    /// This method generates a list of GitHub Issues labels, each with a name and a color. It does
    /// this by generating a list of colors, optionally prefixing each label name, and then mapping
    /// names and colors into [`Label`] instances.
    pub fn expand(&self) -> Vec<Label> {
        let colors = self.colors.generate(self.labels.len());
        let prefix = self.prefix.clone().unwrap_or("".into());

        self.labels
            .iter()
            .enumerate()
            .map(|(i, label)| {
                Label::builder()
                    .name(format!("{prefix}{label}"))
                    .color(colors[i].clone())
                    .build()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::colors::Tailwind;

    use super::*;

    #[test]
    fn expand() {
        let group = Group::builder()
            .prefix(Prefix::new("C-"))
            .colors(Colors::Tailwind(Tailwind::Red))
            .labels(vec![LabelName::new("bug"), LabelName::new("feature")])
            .build();

        let labels = group.expand();
        let expected = vec![
            Label::builder().name("C-bug").color("#fca5a5").build(),
            Label::builder().name("C-feature").color("#b91c1c").build(),
        ];

        assert_eq!(labels, expected);
    }

    #[test]
    fn trait_deserialize() {
        let toml = indoc! {r#"
            [categories]
            prefix = "C-"
            colors = { tailwind = "red" }
            labels = ["bug", "feature"]
        "#};

        let config: ConfigV1 = toml::from_str(toml).unwrap();
        let expected = ConfigV1::builder()
            .groups(HashMap::from([(
                GroupName::new("categories"),
                Group::builder()
                    .prefix(Prefix::new("C-"))
                    .colors(Colors::Tailwind(Tailwind::Red))
                    .labels(vec!["bug".into(), "feature".into()])
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
