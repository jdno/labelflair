//! Generate a colorful palette of labels for your GitHub Issues
//!
//! Labelflair is a library and command-line application that generates colorful labels for GitHub
//! Issues. It accepts a list of labels, generates a palette of colors for them, and outputs them in
//! a format that can be used to create or update labels in a GitHub repository.
//!
//! The library of Labelflair does the heavy lifting of generating the colors and can be used in
//! other applications or scripts. The command-line application provides a simple interface to use
//! the library and generate labels directly from a configuration file.

// We are enforcing documentation on all public items in this crate to ensure that the library is
// well-documented and easy to use.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use crate::config::v1::ConfigV1;
use crate::label::Label;

pub mod colors;
pub mod config;
pub mod label;

/// Generate a colorful palette of labels for your GitHub Issues
///
/// This struct provides the high-level interface to convert a configuration into a list of labels
/// for GitHub Issues. Its `generate` method takes a configuration and expands it into a list of
/// labels that can be serialized for use with GitHub's API.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Labelflair {}

impl Labelflair {
    /// Generate a list of GitHub Issues labels from the configuration
    ///
    /// This function takes a configuration and generates a list of labels based on the groups
    /// defined in the configuration.
    ///
    /// Given this configuration:
    ///
    /// ```toml
    /// [[group]]
    /// prefix = "C-"
    /// colors = { tailwind = "red" }
    /// labels = ["bug", "feature"]
    /// ```
    ///
    /// It will generate the following labels:
    ///
    /// ```yaml
    /// - name: C-bug
    ///   color: '#fca5a5'
    /// - name: C-feature
    ///   color: '#b91c1c'
    /// ```
    pub fn generate(config: &ConfigV1) -> Vec<Label> {
        let mut labels = config.labels().clone();
        let mut labels_from_groups = config
            .groups()
            .iter()
            .flat_map(|group| group.expand())
            .collect();

        labels.append(&mut labels_from_groups);

        labels
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn generate() {
        let toml = indoc! {r##"
            [[label]]
            name = "good-first-issue"
            color = "#4ade80"

            [[group]]
            prefix = "C-"
            colors = { tailwind = "red" }
            labels = ["bug", "feature"]

            [[group]]
            prefix = "P-"
            colors = { tailwind = "blue" }
            labels = ["merge", "block"]
        "##};
        let config: ConfigV1 = toml::from_str(toml).unwrap();

        let mut labels = Labelflair::generate(&config);
        let mut expected = vec![
            Label::builder()
                .name("good-first-issue")
                .color("#4ade80")
                .build(),
            Label::builder().name("C-bug").color("#fca5a5").build(),
            Label::builder().name("C-feature").color("#b91c1c").build(),
            Label::builder().name("P-block").color("#93c5fd").build(),
            Label::builder().name("P-merge").color("#1d4ed8").build(),
        ];

        labels.sort();
        expected.sort();

        assert_eq!(labels, expected);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Labelflair>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Labelflair>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Labelflair>();
    }
}
