#[derive(bytemuck::NoUninit, Clone, Copy)]
#[repr(C)]
pub(crate) struct ParamsBinding {
    pub(crate) max_iter: i32,             // 4 bytes
    pub(crate) y_offset: i32,             // Move `y_offset` up to align properly (4 bytes)
    pub(crate) size: [i32; 2],            // 8 bytes (aligned to 8)
    pub(crate) pos: [f32; 2],             // 8 bytes (aligned to 8)
    pub(crate) cell_size: f32,            // 4 bytes
    pub(crate) bailout: f32,              // 4 bytes (moved next to `cell_size` for alignment)
    pub(crate) julia_constant: [f32; 2],  // 8 bytes
    pub(crate) mandel_constant: [f32; 2], // 8 bytes
}
