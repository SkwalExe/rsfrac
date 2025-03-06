pub(crate) const MAX_HSL_VALUE: i32 = 100;

#[derive(Debug, Clone)]
pub(crate) struct HSLSettings {
    pub(crate) enabled: bool,
    pub(crate) saturation: i32,
    pub(crate) lum: i32,
    pub(crate) hue_offset: i32,
    pub(crate) smoothness: i32,
}

impl Default for HSLSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            saturation: 64,
            lum: 48,
            hue_offset: 69,
            smoothness: 5,
        }
    }
}
