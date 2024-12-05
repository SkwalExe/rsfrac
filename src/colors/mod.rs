//! Contains the logic for coloring the sets using a divergence matrix

use ratatui::style::Color;

mod palettes;
pub(crate) use palettes::*;

/// Returns the color assiciated to the given divergence in the provided palette.
pub(crate) fn palette_color(i: i32, offset: i32, pal: &Palette, smoothing: i32) -> Color {
    let d = (offset + i) as f32 / smoothing as f32;
    let min = d.floor() as i32;
    let max = d.ceil() as i32;
    interpolate(
        palette_color_at(min, pal),
        palette_color_at(max, pal),
        d % 1.0,
    )
}

pub(crate) fn palette_color_at(i: i32, pal: &Palette) -> Color {
    pal.colors[i as usize % pal.colors.len()]
}

/// Returns the palette matching the provided name, and `None` if nothing matched.
pub(crate) fn get_palette_index_by_name(name: &str) -> Option<usize> {
    COLORS
        .iter()
        .position(|pal| pal.name.to_lowercase().starts_with(&name.to_lowercase()))
}

pub(crate) fn interpolate_byte(b1: u8, b2: u8, p: f32) -> u8 {
    let d = b2 as f32 - b1 as f32;
    let incr = d * p;
    (b1 as f32 + incr) as u8
}
pub(crate) fn interpolate(c1: Color, c2: Color, p: f32) -> Color {
    if let (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) = (c1, c2) {
        Color::Rgb(
            interpolate_byte(r1, r2, p),
            interpolate_byte(g1, g2, p),
            interpolate_byte(b1, b2, p),
        )
    } else {
        panic!("Invalid color passed to interpolate()");
    }
}

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

    #[test]
    fn test_interpolate_byte() {
        assert_eq!(interpolate_byte(10, 20, 0.5), 15);
        assert_eq!(interpolate_byte(20, 10, 0.5), 15);
        assert_eq!(interpolate_byte(10, 20, 0.1), 11);
        assert_eq!(interpolate_byte(20, 10, 0.1), 19);
    }

    #[test]
    fn test_interpolate_color() {
        assert_eq!(
            interpolate(Color::Rgb(10, 20, 100), Color::Rgb(20, 10, 0), 0.5),
            Color::Rgb(15, 15, 50)
        );
        assert_eq!(
            interpolate(Color::Rgb(10, 20, 100), Color::Rgb(20, 10, 0), 0.1),
            Color::Rgb(11, 19, 90)
        );
    }
}
