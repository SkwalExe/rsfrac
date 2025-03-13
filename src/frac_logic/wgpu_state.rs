use std::fmt::Debug;

#[derive(Default)]
pub(crate) struct WgpuState {
    /// Whether or not to use the GPU for computations.
    pub(crate) use_gpu: bool,
    /// Instance of WGPU, used for all other contexts
    pub(crate) instance: Option<wgpu::Instance>,
    /// Handle a physical graphics device
    pub(crate) adapter: Option<wgpu::Adapter>,
    /// Represents an open connection to a graphics device
    pub(crate) device: Option<wgpu::Device>,
    pub(crate) queue: Option<wgpu::Queue>,
    pub(crate) cs_module: Option<wgpu::ShaderModule>,
    pub(crate) compute_pipeline: Option<wgpu::ComputePipeline>,
}

/// Do no keep WGPU data when copying the AppState.
impl Clone for WgpuState {
    fn clone(&self) -> Self {
        Default::default()
    }
}

// This implementation is needed to make RenderSettings displayable.
impl Debug for WgpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{WgpuState}")
    }
}
