/// Represents the data given to the compute shaders.
#[derive(bytemuck::NoUninit, Clone, Copy)]
#[repr(C)]
pub(crate) struct ParamsBinding {
    pub(crate) max_iter: i32,
    pub(crate) size: [i32; 2],
    pub(crate) pos: [f32; 2],
    pub(crate) cell_size: f32,
    pub(crate) y_offset: i32,
    pub(crate) julia_constant: [f32; 2],
    pub(crate) mandel_constant: [f32; 2],
    pub(crate) bailout: f32,
}
