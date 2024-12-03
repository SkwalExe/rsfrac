//! Contains the logic for converting a iteration count to a color, as well as each color palette.

use ratatui::style::Color;

/// Returns the color assiciated to the given divergence in the provided palette.
pub(crate) fn palette_color(i: i32, pal: &Palette) -> Color {
    pal.colors[i as usize % pal.colors.len()]
}

/// Returns the palette matching the provided name, and `None` if nothing matched.
pub(crate) fn get_palette_index_by_name(name: &str) -> Option<usize> {
    COLORS
        .iter()
        .position(|pal| pal.name.to_lowercase() == name.to_lowercase())
}

/// Represents a color palette.
pub(crate) struct Palette {
    /// The list of palette colors, in fixed order.
    pub(crate) colors: &'static [Color],
    /// The name of the color palette.
    pub(crate) name: &'static str,
}

pub(crate) const COLORS: &[Palette] = &[
    Palette {
        colors: &[
            Color::Rgb(12, 4, 50),
            Color::Rgb(7, 7, 76),
            Color::Rgb(3, 10, 103),
            Color::Rgb(15, 47, 141),
            Color::Rgb(27, 85, 180),
            Color::Rgb(60, 128, 212),
            Color::Rgb(137, 184, 232),
            Color::Rgb(214, 239, 251),
            Color::Rgb(244, 236, 194),
            Color::Rgb(251, 204, 97),
            Color::Rgb(255, 173, 3),
            Color::Rgb(207, 131, 3),
            Color::Rgb(156, 90, 3),
            Color::Rgb(109, 55, 6),
            Color::Rgb(69, 33, 19),
            Color::Rgb(28, 10, 29),
        ],
        name: "Galaxy",
    },
    Palette {
        colors: &[
            Color::Rgb(25, 7, 26),
            Color::Rgb(53, 14, 37),
            Color::Rgb(88, 27, 48),
            Color::Rgb(135, 54, 72),
            Color::Rgb(186, 85, 108),
            Color::Rgb(229, 118, 142),
            Color::Rgb(252, 165, 177),
            Color::Rgb(255, 204, 187),
            Color::Rgb(255, 211, 138),
            Color::Rgb(255, 187, 90),
            Color::Rgb(252, 146, 48),
            Color::Rgb(241, 103, 31),
            Color::Rgb(208, 66, 29),
            Color::Rgb(162, 40, 33),
            Color::Rgb(111, 17, 29),
            Color::Rgb(54, 7, 20),
        ],
        name: "Sunset",
    },
    Palette {
        colors: &[
            Color::Rgb(10, 17, 5),
            Color::Rgb(14, 30, 9),
            Color::Rgb(22, 47, 12),
            Color::Rgb(33, 68, 18),
            Color::Rgb(50, 91, 25),
            Color::Rgb(72, 116, 38),
            Color::Rgb(106, 145, 55),
            Color::Rgb(139, 174, 81),
            Color::Rgb(171, 202, 114),
            Color::Rgb(194, 219, 154),
            Color::Rgb(182, 190, 129),
            Color::Rgb(158, 157, 104),
            Color::Rgb(118, 117, 77),
            Color::Rgb(85, 80, 56),
            Color::Rgb(55, 52, 36),
            Color::Rgb(25, 27, 17),
        ],
        name: "Forest",
    },
    Palette {
        colors: &[
            Color::Rgb(30, 10, 5),
            Color::Rgb(61, 17, 9),
            Color::Rgb(94, 23, 10),
            Color::Rgb(138, 33, 8),
            Color::Rgb(180, 50, 12),
            Color::Rgb(213, 84, 19),
            Color::Rgb(238, 122, 27),
            Color::Rgb(252, 157, 36),
            Color::Rgb(255, 192, 54),
            Color::Rgb(255, 218, 87),
            Color::Rgb(255, 234, 131),
            Color::Rgb(249, 213, 107),
            Color::Rgb(210, 142, 73),
            Color::Rgb(152, 71, 38),
            Color::Rgb(89, 35, 18),
            Color::Rgb(53, 18, 10),
        ],
        name: "Volcano",
    },
    Palette {
        colors: &[
            Color::Rgb(5, 5, 25),
            Color::Rgb(12, 18, 55),
            Color::Rgb(20, 32, 85),
            Color::Rgb(45, 52, 112),
            Color::Rgb(74, 78, 141),
            Color::Rgb(111, 107, 173),
            Color::Rgb(152, 145, 204),
            Color::Rgb(198, 184, 234),
            Color::Rgb(245, 226, 255),
            Color::Rgb(238, 197, 255),
            Color::Rgb(220, 153, 245),
            Color::Rgb(186, 109, 212),
            Color::Rgb(138, 63, 173),
            Color::Rgb(95, 35, 129),
            Color::Rgb(62, 18, 86),
            Color::Rgb(34, 10, 55),
        ],
        name: "Neon",
    },
    Palette {
        colors: &[
            Color::Rgb(50, 38, 18),
            Color::Rgb(73, 57, 27),
            Color::Rgb(94, 74, 36),
            Color::Rgb(118, 96, 50),
            Color::Rgb(153, 126, 68),
            Color::Rgb(186, 153, 88),
            Color::Rgb(216, 180, 109),
            Color::Rgb(236, 203, 135),
            Color::Rgb(244, 221, 164),
            Color::Rgb(251, 236, 197),
            Color::Rgb(247, 239, 206),
            Color::Rgb(216, 215, 189),
            Color::Rgb(174, 172, 148),
            Color::Rgb(127, 120, 106),
            Color::Rgb(89, 82, 71),
            Color::Rgb(59, 55, 49),
        ],
        name: "Dunes",
    },
    Palette {
        colors: &[
            Color::Rgb(70, 130, 140),
            Color::Rgb(110, 160, 175),
            Color::Rgb(170, 210, 215),
            Color::Rgb(200, 230, 235),
            Color::Rgb(230, 250, 255),
            Color::Rgb(200, 245, 255),
            Color::Rgb(150, 230, 245),
            Color::Rgb(90, 210, 230),
            Color::Rgb(50, 185, 210),
            Color::Rgb(10, 160, 190),
            Color::Rgb(0, 135, 170),
            Color::Rgb(0, 110, 150),
            Color::Rgb(0, 80, 120),
            Color::Rgb(0, 55, 90),
            Color::Rgb(0, 30, 60),
            Color::Rgb(0, 10, 20),
        ],
        name: "Iceberg",
    },
    Palette {
        colors: &[
            Color::Rgb(45, 20, 5),
            Color::Rgb(70, 30, 10),
            Color::Rgb(95, 45, 15),
            Color::Rgb(130, 65, 20),
            Color::Rgb(160, 90, 30),
            Color::Rgb(190, 120, 40),
            Color::Rgb(220, 150, 60),
            Color::Rgb(245, 180, 80),
            Color::Rgb(255, 200, 100),
            Color::Rgb(255, 220, 140),
            Color::Rgb(255, 235, 170),
            Color::Rgb(255, 245, 200),
            Color::Rgb(230, 200, 160),
            Color::Rgb(190, 150, 110),
            Color::Rgb(140, 100, 70),
            Color::Rgb(90, 60, 40),
        ],
        name: "Autumn",
    },
    Palette {
        colors: &[
            Color::Rgb(15, 10, 25),
            Color::Rgb(30, 20, 45),
            Color::Rgb(55, 30, 70),
            Color::Rgb(90, 40, 100),
            Color::Rgb(130, 60, 130),
            Color::Rgb(175, 90, 160),
            Color::Rgb(200, 115, 180),
            Color::Rgb(225, 145, 195),
            Color::Rgb(240, 180, 205),
            Color::Rgb(255, 210, 210),
            Color::Rgb(255, 190, 150),
            Color::Rgb(255, 160, 100),
            Color::Rgb(250, 130, 70),
            Color::Rgb(200, 100, 50),
            Color::Rgb(150, 70, 40),
            Color::Rgb(90, 50, 30),
        ],
        name: "Twilight",
    },
    Palette {
        colors: &[
            Color::Rgb(10, 15, 10),
            Color::Rgb(25, 30, 25),
            Color::Rgb(40, 50, 35),
            Color::Rgb(60, 70, 45),
            Color::Rgb(90, 95, 55),
            Color::Rgb(110, 120, 75),
            Color::Rgb(140, 150, 100),
            Color::Rgb(170, 180, 130),
            Color::Rgb(190, 200, 160),
            Color::Rgb(210, 220, 185),
            Color::Rgb(230, 235, 205),
            Color::Rgb(215, 220, 180),
            Color::Rgb(185, 190, 150),
            Color::Rgb(140, 150, 110),
            Color::Rgb(90, 100, 75),
            Color::Rgb(55, 65, 50),
        ],
        name: "Underground",
    },
    Palette {
        colors: &[
            Color::Rgb(30, 10, 5),
            Color::Rgb(60, 15, 10),
            Color::Rgb(90, 20, 15),
            Color::Rgb(120, 30, 20),
            Color::Rgb(150, 45, 25),
            Color::Rgb(180, 65, 30),
            Color::Rgb(210, 90, 40),
            Color::Rgb(240, 120, 50),
            Color::Rgb(255, 160, 60),
            Color::Rgb(255, 190, 80),
            Color::Rgb(255, 210, 100),
            Color::Rgb(255, 230, 130),
            Color::Rgb(240, 200, 110),
            Color::Rgb(210, 150, 90),
            Color::Rgb(170, 100, 70),
            Color::Rgb(120, 60, 50),
        ],
        name: "Lava",
    },
    Palette {
        colors: &[
            Color::Rgb(15, 20, 25),
            Color::Rgb(30, 35, 40),
            Color::Rgb(50, 55, 60),
            Color::Rgb(75, 80, 85),
            Color::Rgb(100, 110, 120),
            Color::Rgb(130, 140, 150),
            Color::Rgb(160, 170, 180),
            Color::Rgb(190, 200, 210),
            Color::Rgb(210, 220, 230),
            Color::Rgb(225, 230, 235),
            Color::Rgb(240, 240, 245),
            Color::Rgb(230, 235, 230),
            Color::Rgb(200, 210, 200),
            Color::Rgb(170, 185, 170),
            Color::Rgb(130, 140, 140),
            Color::Rgb(100, 105, 110),
        ],
        name: "Mountain",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_palette_index_by_name() {
        // Try to get the index of the `Mountain` palette by name.
        let pal = get_palette_index_by_name("MouNtAIn").unwrap();
        // Check if the name matches.
        assert_eq!(COLORS[pal].name, "Mountain");
        // Check if the first color matches.
        assert_eq!(COLORS[pal].colors[0], Color::Rgb(15, 20, 25))
    }
}
