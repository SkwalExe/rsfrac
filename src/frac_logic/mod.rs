//! Contains all the logic required to render a divergence matrix with the GPU of the CPU.

mod canvas_coords;
mod fractal_logic;
pub(crate) mod gpu_init;
pub(crate) mod gpu_render;
mod gpu_rendering_tracker;
pub(crate) mod gpu_util;
mod params_binding;
mod render_settings;
mod render_settings_methods;
mod wgpu_state;

pub(crate) use canvas_coords::CanvasCoords;
pub(crate) use fractal_logic::DivergMatrix;
pub(crate) use params_binding::ParamsBinding;
pub(crate) use render_settings::RenderSettings;
pub(crate) use wgpu_state::WgpuState;
