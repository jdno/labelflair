//! A group of labels in the configuration
//!
//! The Labelflair configuration groups labels by their purpose or category. Each group can have a
//! prefix, a color generator, and a list of labels. The prefix is prepended to each label name
//! when generating the final labels. The color generator is used to generate colors for the labels
//! in the group, ensuring a consistent color scheme across related labels.

use getset::{CopyGetters, Getters};
use serde::Deserialize;
use typed_builder::TypedBuilder;
use typed_fields::name;

use crate::colors::{Colors, Generate};
use crate::label::Label;

use super::LabelVariant;

name!(
    /// A name for a group of labels in Labelflair
    GroupName
);

name!(
    /// A prefix for labels in Labelflair
    Prefix
);

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
    labels: Vec<LabelVariant>,
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

        // Sort labels to ensure a nice color palette in GitHub's user interface
        let mut labels = self.labels.clone();
        labels.sort();

        labels
            .iter()
            .enumerate()
            .map(|(i, label)| {
                Label::builder()
                    .name(format!("{prefix}{label}"))
                    .color(colors[i].clone())
                    .description(label.description().cloned())
                    .build()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::Tailwind;

    use super::*;

    #[test]
    fn expand() {
        let group = Group::builder()
            .prefix(Prefix::new("C-"))
            .colors(Colors::Tailwind(Tailwind::Red))
            .labels(vec![
                LabelVariant::Name("bug".into()),
                LabelVariant::Name("feature".into()),
            ])
            .build();

        let labels = group.expand();
        let expected = vec![
            Label::builder().name("C-bug").color("#fca5a5").build(),
            Label::builder().name("C-feature").color("#b91c1c").build(),
        ];

        assert_eq!(labels, expected);
    }

    #[test]
    fn expand_sorts_labels() {
        let group = Group::builder()
            .prefix(Prefix::new("C-"))
            .colors(Colors::Tailwind(Tailwind::Red))
            .labels(vec![
                LabelVariant::Name("feature".into()),
                LabelVariant::Name("alpha".into()),
                LabelVariant::Name("bug".into()),
            ])
            .build();

        let labels = group.expand();
        let expected = vec![
            Label::builder().name("C-alpha").color("#fecaca").build(),
            Label::builder().name("C-bug").color("#ef4444").build(),
            Label::builder().name("C-feature").color("#991b1b").build(),
        ];

        assert_eq!(labels, expected);
    }
    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Group>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Group>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Group>();
    }
}
