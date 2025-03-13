//! Contains all the logic required to render a divergence matrix with the GPU of the CPU.

mod canvas_coords;
mod fractal_logic;
mod gpu_rendering_tracker;
mod wgpu_state;
mod render_settings_methods;
mod params_binding;
pub(crate) mod gpu_init;
pub(crate) mod gpu_render;
pub(crate) mod gpu_util;
mod render_settings;

pub(crate) use canvas_coords::CanvasCoords;
pub(crate) use wgpu_state::WgpuState;
pub(crate) use params_binding::ParamsBinding;
pub(crate) use fractal_logic::DivergMatrix;
pub(crate) use render_settings::RenderSettings;
