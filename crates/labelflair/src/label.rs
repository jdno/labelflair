//! A label for GitHub Issues
//!
//! This module defines the [`Label`] struct, which represents a label for GitHub Issues, and types
//! for its fields.

use getset::Getters;
use serde::Serialize;
use typed_builder::TypedBuilder;
use typed_fields::name;

name!(
    /// The name of a label
    LabelName
);

name!(
    /// The color of a label
    ///
    /// The `Color` type represents a color in hex format.
    Color
);

name!(
    /// The description of a label
    ///
    /// The `Description` type represents an optional description for a label.
    Description
);

/// A label for GitHub Issues
///
/// Labels for GitHub Issues are used to categorize and organize issues in a repository. They have a
/// unique name and a color represented in hex format.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Getters, Serialize, TypedBuilder)]
pub struct Label {
    /// The name of the label
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: LabelName,

    /// The color of the label
    #[builder(setter(into))]
    #[getset(get = "pub")]
    color: Color,

    /// An optional description for the label
    #[builder(default, setter(into))]
    #[getset(get = "pub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Description>,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Label>();
    }

    #[test]
    fn trait_serialize() {
        let label = Label::builder()
            .name("bug")
            .color("#FF0000")
            .description(Some("a description for the label".into()))
            .build();

        let serialized = serde_yaml_ng::to_string(&label).unwrap();
        let expected = indoc! {r#"
            name: bug
            color: '#FF0000'
            description: a description for the label
        "#};

        assert_eq!(serialized, expected);
    }

    #[test]
    fn trait_serialize_without_description() {
        let label = Label::builder().name("bug").color("#FF0000").build();

        let serialized = serde_yaml_ng::to_string(&label).unwrap();
        let expected = indoc! {r#"
            name: bug
            color: '#FF0000'
        "#};

        assert_eq!(serialized, expected);
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Label>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Label>();
    }
}
