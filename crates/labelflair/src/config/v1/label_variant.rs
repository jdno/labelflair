//! Different variations to represent a label in the configuration
//!
//! This module defines the `LabelVariant` enum, which can represent a label either as a simple name
//! or with an associated description. To make it easy for users to write their configuration,
//! deserialization is smart and can detect the variant automatically.

use std::fmt::{Display, Formatter};

use serde::Deserialize;

use crate::label::{Description, LabelName};

/// Different variations to represent a label in the configuration
///
/// This enum can represent a label either as a simple name or with an associated description. To
/// make it easy for users to write their configuration, deserialization is smart and can detect the
/// variant automatically.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(untagged)]
pub enum LabelVariant {
    /// A label with just a name
    Name(LabelName),

    /// A label with a name and a description
    WithDescription {
        /// The name of the label
        name: LabelName,
        /// The description for the label
        #[serde(default)]
        description: Option<Description>,
        /// Optional aliases for the label
        #[serde(default)]
        aliases: Vec<LabelName>,
    },
}

impl LabelVariant {
    /// Returns the name of the label
    pub fn name(&self) -> &LabelName {
        match self {
            LabelVariant::Name(name) => name,
            LabelVariant::WithDescription { name, .. } => name,
        }
    }

    /// Returns the optional description of the label
    pub fn description(&self) -> Option<&Description> {
        match self {
            LabelVariant::Name(_) => None,
            LabelVariant::WithDescription { description, .. } => description.as_ref(),
        }
    }

    /// Returns the optional aliases of the label
    pub fn aliases(&self) -> Option<&Vec<LabelName>> {
        match self {
            LabelVariant::Name(_) => None,
            LabelVariant::WithDescription { aliases, .. } => Some(aliases),
        }
    }
}

impl Display for LabelVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            LabelVariant::Name(name) => name,
            LabelVariant::WithDescription { name, .. } => name,
        };

        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
    struct Container {
        labels: Vec<LabelVariant>,
    }

    #[test]
    fn trait_deserialize() {
        let toml = indoc! {r#"
            labels = ["bug", { name = "enhancement", description = "A new feature or improvement" }]
        "#};

        let container: Container = toml::from_str(toml).unwrap();
        let expected = Container {
            labels: vec![
                LabelVariant::Name("bug".into()),
                LabelVariant::WithDescription {
                    name: "enhancement".into(),
                    description: Some("A new feature or improvement".into()),
                    aliases: Vec::new(),
                },
            ],
        };

        assert_eq!(container, expected);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<LabelVariant>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<LabelVariant>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<LabelVariant>();
    }
}
