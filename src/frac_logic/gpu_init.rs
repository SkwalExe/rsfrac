use std::sync::mpsc::Sender;

use futures::executor;

use crate::app::SlaveMessage;

use super::{gpu_util::msg_send, RenderSettings};

impl RenderSettings {

    /// Will initialize the global wgpu state synchronously, while sending status messages
    pub(crate) fn initialize_gpu_sync(
        &mut self,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<(), String> {
        executor::block_on(self.initialize_gpu_async(sender))
    }

    /// Will initialize the global wgpu state asynchronously, while sending status messages
    pub(crate) async fn initialize_gpu_async(
        &mut self,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<(), String> {
        msg_send(sender, "Requesting WGPU Instance")?;
        // Instantiates instance of WebGPU
        self.wgpu_state.instance = Some(wgpu::Instance::default());

        msg_send(sender, "Instantiating a connection to the GPU")?;
        self.wgpu_state.adapter = Some(
            self.wgpu_state
                .instance
                .as_ref()
                .unwrap()
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .ok_or("Could not get WGPU adapter.")?,
        );

        msg_send(
            sender,
            "Requesting Device handle and job queue from the GPU.",
        )?;
        // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
        //  `features` being the available features.
        let (device, queue) = self
            .wgpu_state
            .adapter
            .as_ref()
            .unwrap()
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
            .map_err(|err| format!("Could not request device: {err}"))?;

        (self.wgpu_state.device, self.wgpu_state.queue) = (Some(device), Some(queue));

        self.update_fractal_shader_async(sender).await?;
        msg_send(sender, "GPU initialization finished")?;
        Ok(())
    }

    /// will initlize the computing pipeline corresponding to the (newly) selected fractal
    /// synchronously.
    pub(crate) fn update_fractal_shader_sync(
        &mut self,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<(), String> {
        executor::block_on(self.update_fractal_shader_async(sender))
    }

    /// will initlize the computing pipeline corresponding to the (newly) selected fractal
    /// asynchronously.
    pub(crate) async fn update_fractal_shader_async(
        &mut self,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<(), String> {
        msg_send(sender, "Loading the fractal shader")?;
        // Loads the shader from WGSL
        let cs_descriptor = match self.get_frac_obj().name.to_lowercase().as_ref() {
            // TODO: implement other fractal shaders
            "mandelbrot" => wgpu::include_wgsl!("shaders/mandelbrot.wgsl"),
            "burningship" => wgpu::include_wgsl!("shaders/burning_ship.wgsl"),
            "julia" => wgpu::include_wgsl!("shaders/julia.wgsl"),
            _ => {
                if let Some(sender) = sender {
                    sender
                        .send(SlaveMessage::JobFinished)
                        .map_err(|err| format!("Could not open message channel: {err}"))?;
                }
                return Err(format!(
                    "Fractal shader not yet implemented for: {}",
                    self.get_frac_obj().name,
                ));
            }
        };
        self.wgpu_state.cs_module = Some(
            self.wgpu_state
                .device
                .as_ref()
                .unwrap()
                .create_shader_module(cs_descriptor),
        );
        msg_send(sender, "Creating a GPU compute pipeline")?;
        // Instantiates the pipeline.
        self.wgpu_state.compute_pipeline = Some(
            self.wgpu_state
                .device
                .as_ref()
                .unwrap()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: self.wgpu_state.cs_module.as_ref().unwrap(),
                    entry_point: Some("main"),
                    compilation_options: Default::default(),
                    cache: None,
                }),
        );
        Ok(())
    }
}
