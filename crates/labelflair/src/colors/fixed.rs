//! Color generator that applies a fixed color to all labels
//!
//! This module provides a color generator that uses the same, pre-defined color for all labels.

use getset::Getters;
use serde::Deserialize;

use crate::colors::Generate;
use crate::label::Color;

/// Color generator that applies a fixed color to all labels
///
/// The `Fixed` color generator always returns the same color for all labels. It is initialized with
/// a single `Color` value, which is then used for every label.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Getters, Deserialize)]
pub struct Fixed(Color);

impl Fixed {
    /// Create a new fixed color generator
    ///
    /// The fixed color generator always returns the same color for all labels.
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

impl Generate for Fixed {
    fn generate(&self, count: usize) -> Vec<Color> {
        vec![self.0.clone(); count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_generate_for_1() {
        let fixed = Fixed(Color::new("#ff0000"));

        let colors = fixed.generate(1);
        let expected = vec![Color::new("#ff0000")];

        assert_eq!(colors, expected);
    }

    #[test]
    fn trait_generate_for_3() {
        let fixed = Fixed(Color::new("#00ff00"));

        let colors = fixed.generate(3);
        let expected = vec![Color::new("#00ff00"); 3];

        assert_eq!(colors, expected);
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Fixed>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Fixed>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Fixed>();
    }
}
