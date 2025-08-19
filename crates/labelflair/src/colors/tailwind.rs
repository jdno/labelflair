//! Color generator based on the Tailwind CSS color palettes
//!
//! This module provides a color generator that uses the Tailwind CSS color palettes to generate
//! colors. It uses a subset of each color palette to avoid very bright and dark variants.
//!
//! See https://tailwindcss.com/docs/colors for more information on the Tailwind CSS color palettes.

use serde::Deserialize;

use crate::Color;
use crate::colors::Generate;

/// Color generator based on the Tailwind CSS color palettes
///
/// The `Tailwind` enum represents the different colors available in the Tailwind CSS color palette.
/// It implements the `Generate` trait to produce a list of colors based on the selected color and
/// the count of colors to generate. The colors are generated in an alternating pattern to provide
/// a balanced distribution of shades.
///
/// See https://tailwindcss.com/docs/colors for more information on the Tailwind CSS color palettes.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::missing_docs_in_private_items)] // Enum variants are self-explanatory
pub enum Tailwind {
    Red,
    Orange,
    Amber,
    Yellow,
    Lime,
    Green,
    Emerald,
    Teal,
    Cyan,
    Sky,
    Blue,
    Indigo,
    Violet,
    Purple,
    Fuchsia,
    Pink,
    Rose,
    Slate,
    Gray,
    Zinc,
    Natural,
    Stone,
}

impl Tailwind {
    /// Generate a list of alternating indices for the Tailwind color shades
    ///
    /// This method generates a list of indices that alternate between the shades of the Tailwind
    /// color. It starts from the center shade and alternates outward to provide a balanced
    /// distribution of colors.
    ///
    /// For example, if there are 9 shades and a list of 3 is requested, the indices will be:
    /// `[4, 3, 5]`, where `4` is the shade at the center of the palette. This provides strong and
    /// balanced colors for the labels and avoids shades that are more challenging to differentiate
    /// from other colors due to their brightness or darkness.
    ///
    /// If more colors are requested than available shades, the indices will wrap around to
    /// provide a repeating pattern.
    fn alternating_indices(&self, count: usize) -> Vec<usize> {
        // If the count is zero, exit early and return an empty vector
        if count == 0 {
            return Vec::new();
        }

        // Establish the number of shades for each color
        let shades = Tailwind::Red.colors().len();

        // Assign an alternating index to each shade
        let center: usize = shades / 2;
        let mut base: Vec<usize> = Vec::with_capacity(shades);
        base.push(center);
        for d in 1..=center {
            base.push(center - d);
            base.push(center + d);
        }

        // Generate a list of indices based on the count
        (0..count).map(|i| base[i % base.len()]).collect()
    }

    /// Get the colors for the Tailwind color palette
    ///
    /// This method returns a list of colors for each Tailwind color variant. A subset of colors has
    /// been picked from Tailwind's palette, ignoring the brightest and darkest variants because
    /// they are too challenging to differentiate from other colors.
    fn colors(&self) -> [Color; 9] {
        match self {
            Tailwind::Red => [
                "#fee2e2", "#fecaca", "#fca5a5", "#f87171", "#ef4444", "#dc2626", "#b91c1c",
                "#991b1b", "#7f1d1d",
            ],
            Tailwind::Orange => [
                "#ffedd5", "#fed7aa", "#fdba74", "#fb923c", "#f97316", "#ea580c", "#c2410c",
                "#9a3412", "#7c2d12",
            ],
            Tailwind::Amber => [
                "#fef3c7", "#fde68a", "#fcd34d", "#fbbf24", "#f59e0b", "#d97706", "#b45309",
                "#92400e", "#78350f",
            ],
            Tailwind::Yellow => [
                "#fef9c3", "#fef08a", "#fde047", "#facc15", "#eab308", "#ca8a04", "#a16207",
                "#854d0e", "#713f12",
            ],
            Tailwind::Lime => [
                "#ecfccb", "#d9f99d", "#bef264", "#a3e635", "#84cc16", "#65a30d", "#4d7c0f",
                "#3f6212", "#365314",
            ],
            Tailwind::Green => [
                "#dcfce7", "#bbf7d0", "#86efac", "#4ade80", "#22c55e", "#16a34a", "#15803d",
                "#166534", "#14532d",
            ],
            Tailwind::Emerald => [
                "#d1fae5", "#a7f3d0", "#6ee7b7", "#34d399", "#10b981", "#059669", "#047857",
                "#065f46", "#064e3b",
            ],
            Tailwind::Teal => [
                "#ccfbf1", "#99f6e4", "#5eead4", "#2dd4bf", "#14b8a6", "#0d9488", "#0f766e",
                "#115e59", "#134e4a",
            ],
            Tailwind::Cyan => [
                "#cffafe", "#a5f3fc", "#67e8f9", "#22d3ee", "#06b6d4", "#0891b2", "#0e7490",
                "#155e75", "#164e63",
            ],
            Tailwind::Sky => [
                "#e0f2fe", "#bae6fd", "#7dd3fc", "#38bdf8", "#0ea5e9", "#0284c7", "#0369a1",
                "#075985", "#0c4a6e",
            ],
            Tailwind::Blue => [
                "#dbeafe", "#bfdbfe", "#93c5fd", "#60a5fa", "#3b82f6", "#2563eb", "#1d4ed8",
                "#1e40af", "#1e3a8a",
            ],
            Tailwind::Indigo => [
                "#e0e7ff", "#c7d2fe", "#a5b4fc", "#818cf8", "#6366f1", "#4f46e5", "#4338ca",
                "#3730a3", "#312e81",
            ],
            Tailwind::Violet => [
                "#ede9fe", "#ddd6fe", "#c4b5fd", "#a78bfa", "#8b5cf6", "#7c3aed", "#6d28d9",
                "#5b21b6", "#4c1d95",
            ],
            Tailwind::Purple => [
                "#f3e8ff", "#e9d5ff", "#d8b4fe", "#c084fc", "#a855f7", "#9333ea", "#7e22ce",
                "#6b21a8", "#581c87",
            ],
            Tailwind::Fuchsia => [
                "#fae8ff", "#f5d0fe", "#f0abfc", "#e879f9", "#d946ef", "#c026d3", "#a21caf",
                "#86198f", "#701a75",
            ],
            Tailwind::Pink => [
                "#fce7f3", "#fbcfe8", "#f9a8d4", "#f472b6", "#ec4899", "#db2777", "#be185d",
                "#9d174d", "#831843",
            ],
            Tailwind::Rose => [
                "#ffe4e6", "#fecdd3", "#fda4af", "#fb7185", "#f43f5e", "#e11d48", "#be123c",
                "#9f1239", "#881337",
            ],
            Tailwind::Slate => [
                "#f1f5f9", "#e2e8f0", "#cbd5e1", "#94a3b8", "#64748b", "#475569", "#334155",
                "#1e293b", "#0f172a",
            ],
            Tailwind::Gray => [
                "#f3f4f6", "#e5e7eb", "#d1d5db", "#9ca3af", "#6b7280", "#4b5563", "#374151",
                "#1f2937", "#111827",
            ],
            Tailwind::Zinc => [
                "#f4f4f5", "#e4e4e7", "#d4d4d8", "#a1a1aa", "#71717a", "#52525b", "#3f3f46",
                "#27272a", "#18181b",
            ],
            Tailwind::Natural => [
                "#f5f5f5", "#e5e5e5", "#d4d4d4", "#a3a3a3", "#737373", "#525252", "#404040",
                "#262626", "#171717",
            ],
            Tailwind::Stone => [
                "#f5f5f4", "#e7e5e4", "#d6d3d1", "#a8a29e", "#78716c", "#57534e", "#44403c",
                "#292524", "#1c1917",
            ],
        }
        .map(Into::into)
    }
}

impl Generate for Tailwind {
    fn generate(&self, count: usize) -> Vec<Color> {
        let colors = self.colors();
        let indices = self.alternating_indices(count);

        indices
            .into_iter()
            .map(|index| colors[index % colors.len()].clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternating_indices_for_1() {
        let tailwind = Tailwind::Red;

        let indices = tailwind.alternating_indices(1);

        assert_eq!(indices, vec![4]);
    }

    #[test]
    fn alternating_indices_for_2() {
        let tailwind = Tailwind::Red;

        let indices = tailwind.alternating_indices(2);

        assert_eq!(indices, vec![4, 3]);
    }

    #[test]
    fn alternating_indices_for_3() {
        let tailwind = Tailwind::Red;

        let indices = tailwind.alternating_indices(3);

        assert_eq!(indices, vec![4, 3, 5]);
    }

    #[test]
    fn alternating_indices_for_9() {
        let tailwind = Tailwind::Red;

        let indices = tailwind.alternating_indices(9);

        assert_eq!(indices, vec![4, 3, 5, 2, 6, 1, 7, 0, 8]);
    }

    #[test]
    fn alternating_indices_for_10() {
        let tailwind = Tailwind::Red;

        let indices = tailwind.alternating_indices(10);

        assert_eq!(indices, vec![4, 3, 5, 2, 6, 1, 7, 0, 8, 4]);
    }

    #[test]
    fn alternating_indices_for_18() {
        let tailwind = Tailwind::Red;

        let indices = tailwind.alternating_indices(18);

        assert_eq!(
            indices,
            vec![4, 3, 5, 2, 6, 1, 7, 0, 8, 4, 3, 5, 2, 6, 1, 7, 0, 8]
        );
    }

    #[test]
    fn trait_generate_red() {
        let tailwind = Tailwind::Red;
        let colors = tailwind.generate(5);

        assert_eq!(
            colors,
            vec![
                "#ef4444".into(),
                "#f87171".into(),
                "#dc2626".into(),
                "#fca5a5".into(),
                "#b91c1c".into()
            ]
        );
    }

    #[test]
    fn trait_generate_blue() {
        let tailwind = Tailwind::Blue;
        let colors = tailwind.generate(12);

        assert_eq!(
            colors,
            vec![
                "#3b82f6".into(),
                "#60a5fa".into(),
                "#2563eb".into(),
                "#93c5fd".into(),
                "#1d4ed8".into(),
                "#bfdbfe".into(),
                "#1e40af".into(),
                "#dbeafe".into(),
                "#1e3a8a".into(),
                "#3b82f6".into(),
                "#60a5fa".into(),
                "#2563eb".into(),
            ]
        );
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Tailwind>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Tailwind>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Tailwind>();
    }
}
