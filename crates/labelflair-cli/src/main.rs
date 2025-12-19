//! Labelflair CLI
//!
//! This crate provides a command-line interface for Labelflair, a tool for generating labels for
//! GitHub Issues from a configuration file.

// We are enforcing documentation on all public items in this crate to ensure that the library is
// well-documented and easy to use.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod commands;

clawless::main!();
