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
// well-documented and easy to use. Documentation on private items is not required but encouraged.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use typed_fields::name;

pub mod colors;
pub mod config;

name!(
    /// A color in Labelflair
    Color
);
