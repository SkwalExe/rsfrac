use std::fmt::Debug;

use futures::executor::block_on;
use wgpu::{Adapter, Backends, Device, ShaderModuleDescriptor};

use crate::helpers::markup::esc;

#[derive(Default)]
pub(crate) struct WgpuState {
    /// Whether or not to use the GPU for computations.
    pub(crate) use_gpu: bool,
    /// Instance of WGPU, used for all other contexts.
    pub(crate) instance: wgpu::Instance,
    /// Represents an open connection to a graphics device
    pub(crate) device: Option<wgpu::Device>,
    pub(crate) queue: Option<wgpu::Queue>,
    pub(crate) cs_module: Option<wgpu::ShaderModule>,
    pub(crate) compute_pipeline: Option<wgpu::ComputePipeline>,
    /// A list of detected adapters, which are handles to physical devices.
    /// The list is created when the WgpuState is initialized, and aren't updated thereafter.
    detected_adapters: Vec<Adapter>,
    /// Index in detected_adapters of the preferred adapter. Guaranteed to be always valid.
    preferred_adapter: usize,
    /// The shader name of the desired fractal.
    frac_name: String,
}

impl WgpuState {
    /// Initialize all GPU components.
    pub(crate) fn initialize(&mut self, frac_name: impl Into<String>) -> Result<(), String> {
        // Perform adapter detection.
        self.detect_adapters()?;

        self.frac_name = frac_name.into();

        // try to select the first available adapter (the preferred_adapter should be set to 0 by
        // default), or the selected one. It will automatically update all the dependant
        // components.
        self.set_preferred_adapter(self.preferred_adapter)?;
        Ok(())
    }

    pub(crate) fn detected_adapters(&self) -> &Vec<Adapter> {
        &self.detected_adapters
    }

    /// List available WGPU adapters and put them in .detected_adapters.
    /// Is ran once on initialization. Returns a fixed error message if no adapter is detected
    fn detect_adapters(&mut self) -> Result<(), String> {
        self.detected_adapters = self.instance.enumerate_adapters(Backends::all());
        if self.detected_adapters.is_empty() {
            return Err("No GPU adapter has been detected on your system.".to_string());
        }
        Ok(())
    }

    /// Set the preferred adapter index, and make sure the index is valid.
    pub(crate) fn set_preferred_adapter(&mut self, index: usize) -> Result<(), String> {
        if index >= self.detected_adapters.len() {
            return Err(
                "Cannot update the preferred adapter because the provided index is invalid."
                    .to_string(),
            );
        }

        self.preferred_adapter = index;
        block_on(self.update_device_and_queue())?;
        Ok(())
    }

    /// Return a CS descriptor associated with the current fractal name.
    fn get_cs_descriptor(&self) -> Result<ShaderModuleDescriptor<'static>, String> {
        // Loads the shader from WGSL

        Ok(match self.frac_name.to_lowercase().as_ref() {
            // TODO: implement other fractal shaders
            "mandelbrot" => wgpu::include_wgsl!("../fractals/shaders/mandelbrot.wgsl"),
            "burningship" => wgpu::include_wgsl!("../fractals/shaders/burning_ship.wgsl"),
            "julia" => wgpu::include_wgsl!("../fractals/shaders/julia.wgsl"),
            _ => {
                return Err(format!(
                    "Fractal shader not yet implemented for: {}",
                    self.frac_name
                ));
            }
        })
    }

    /// Set the wanted shader module descriptor.
    pub(crate) fn set_cs(&mut self, frac_name: impl Into<String>) -> Result<(), String> {
        self.frac_name = frac_name.into();
        self.update_cs_module()
    }

    /// After changing the selected adapter, update the device and the queue.
    async fn update_device_and_queue(&mut self) -> Result<(), String> {
        // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
        //  `features` being the available features.
        let (device, queue) = self
            .get_adapter()?
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .map_err(|err| format!("Could not request device: {}", esc(err)))?;

        (self.device, self.queue) = (Some(device), Some(queue));

        self.update_cs_module()
    }

    /// After selecting an adapter and updating the device and queue, update the cs module
    fn update_cs_module(&mut self) -> Result<(), String> {
        self.cs_module = Some(
            self
            .device
            .as_ref()
            .ok_or("ERROR: Tried to update the CS module while the GPU device object was not initialized.")?
            .create_shader_module(
                self.get_cs_descriptor()?
            ),
        );
        self.update_pipeline()?;
        Ok(())
    }

    /// The latest step of GPU initialization is the pipeline.
    fn update_pipeline(&mut self) -> Result<(), String> {
        self.compute_pipeline = Some(
            self.get_device()?
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: self
                        .cs_module
                        .as_ref()
                        .ok_or("Cannot update the pipeline if no CS module is defined.")?,
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                }),
        );
        Ok(())
    }

    /// Returns a reference to the preferred adapter. Returns a fixed error message when no adapter
    /// has been detected.
    pub(crate) fn get_adapter(&self) -> Result<&Adapter, String> {
        self.detected_adapters
            .get(self.preferred_adapter)
            .ok_or("Cannot access the selected adapter.".to_string())
    }

    /// Returns a reference to the currently initialized GPU device. Returns a fixed error message
    /// when the GPU device has not been initialized yet.
    fn get_device(&self) -> Result<&Device, String> {
        self.device.as_ref().ok_or(
            "ERROR: Tried to access the GPU device while it was not initialized.".to_string(),
        )
    }
}

/// Do no keep WGPU data when copying the AppState.
impl Clone for WgpuState {
    fn clone(&self) -> Self {
        Self {
            use_gpu: self.use_gpu,
            preferred_adapter: self.preferred_adapter,
            ..Default::default()
        }
    }
}

// This implementation is needed to make RenderSettings displayable.
impl Debug for WgpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{WgpuState}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgpu_state_clone() {
        let mut st = WgpuState::default();
        assert!(!st.use_gpu);

        st.use_gpu = true;
        // Cloning the WgpuState should keep the use_gpu property.
        assert!(st.clone().use_gpu);
    }
}
