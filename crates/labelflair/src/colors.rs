//! Generator for color palettes
//!
//! This module provides different generators for color palettes. Generators must implement the
//! [`Generate`] trait, which returns a list of [`Color`] based on its internal logic. All available
//! generators are listed in the [`Colors`] enum.

use serde::Deserialize;

use crate::label::Color;

pub use self::tailwind::Tailwind;

mod tailwind;

/// Color generators in Labelflair
///
/// This enum represents different color generators available in Labelflair. Each generator has its
/// own logic for generating colors, so read their documentation for more details.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Colors {
    /// Use the color palette from Tailwind CSS
    Tailwind(Tailwind),
}

/// Trait for color generators
///
/// This trait defines the interface for color generators in Labelflair. Each generator must
/// implement the `generate` method, which takes a count of colors to generate and returns a vector
/// of [`Color`]. How the colors are generated is an implementation detail of the generator.
pub trait Generate {
    /// Generate a list of colors
    ///
    /// This method takes a count of colors to generate and returns a vector of [`Color`].
    fn generate(&self, count: usize) -> Vec<Color>;
}

impl Generate for Colors {
    fn generate(&self, count: usize) -> Vec<Color> {
        let variant: Box<&dyn Generate> = match self {
            Colors::Tailwind(tailwind) => Box::new(tailwind),
        };

        variant.generate(count)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn trait_deserialize() {
        let toml = indoc! {r#"
            tailwind = "red"
        "#};

        let colors: Colors = toml::from_str(toml).unwrap();

        assert_eq!(Colors::Tailwind(Tailwind::Red), colors);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Colors>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Colors>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Colors>();
    }
}
