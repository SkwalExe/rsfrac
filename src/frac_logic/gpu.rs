use std::sync::mpsc::Sender;

use crate::{app::SlaveMessage, frac_logic::CanvasCoords, helpers::Vec2};

use super::{DivergMatrix, RenderSettings};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use wgpu::util::DeviceExt;

#[derive(Default)]
pub(crate) struct WgpuState {
    instance: Option<wgpu::Instance>,
    adapter: Option<wgpu::Adapter>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    cs_module: Option<wgpu::ShaderModule>,
    compute_pipeline: Option<wgpu::ComputePipeline>,
}

impl Clone for WgpuState {
    fn clone(&self) -> Self {
        Default::default()
    }
}

fn msg_send(
    sender: Option<&Sender<SlaveMessage>>,
    message: impl Into<String>,
) -> Result<(), String> {
    if let Some(sender) = sender {
        sender
            .send(SlaveMessage::SetMessage(format!(
                "Current status: {}...",
                message.into()
            )))
            .map_err(|err| format!("Message channel could not be opened: {err}"))?;
    }
    Ok(())
}

impl RenderSettings {
    pub(crate) async fn initialize_gpu(
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

        self.update_fractal_shader(sender).await?;
        msg_send(sender, "GPU initialization finished")?;
        Ok(())
    }

    pub(crate) async fn update_fractal_shader(
        &mut self,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<(), String> {
        msg_send(sender, "Loading the fractal shader")?;
        // Loads the shader from WGSL
        let cs_descriptor = match self.get_frac_obj().name.to_lowercase().as_ref() {
            // TODO: implement other fractal shaders
            "mandelbrot" => wgpu::include_wgsl!("shaders/mandelbrot.wgsl"),
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

    pub(crate) async fn get_gpu_diverg_matrix(
        &mut self,
        size: &Vec2<i32>,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<DivergMatrix, String> {
        // Gets the size in bytes of the buffer.
        let output_buffer_size =
            (size.y as u64 * size.x as u64 * size_of::<u32>() as u64) as wgpu::BufferAddress;
        let input_buffer_size = 2 * size.y as u64 * size.x as u64 * size_of::<f32>() as u64;

        let max_buf_size = self
            .wgpu_state
            .device
            .as_ref()
            .unwrap()
            .limits()
            .max_storage_buffer_binding_size as u64;

        if input_buffer_size > max_buf_size {
            return Err(format!(
                "Input buffer size ({}MB) would exceed maximum GPU buffer size ({}MB).",
                input_buffer_size / 1000000,
                max_buf_size / 1000000
            ));
        }

        msg_send(sender, "Generating GPU input data")?;
        let half_y = size.y / 2;
        let half_x = size.x / 2;
        let cell_size = self.get_plane_wid() / size.x;
        let points: Vec<f32> = (-half_y..-half_y + size.y)
            .into_par_iter()
            .flat_map(|y| -> Vec<f32> {
                (-half_x..-half_x + size.x)
                    .into_par_iter()
                    .flat_map(|x| {
                        let point =
                            self.coord_to_c_with_cell_size(CanvasCoords::new(x, y), &cell_size);
                        vec![point.real().to_f32(), point.imag().to_f32()]
                    })
                    .collect()
            })
            .collect();

        msg_send(sender, "Creating the input buffer")?;
        // Instantiates buffer with data (`numbers`).
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let input_buffer = self.wgpu_state.device.as_ref().unwrap().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Input Buffer"),
                contents: bytemuck::cast_slice(&points),
                usage: wgpu::BufferUsages::STORAGE,
            },
        );

        msg_send(sender, "Creating the output buffer")?;
        let output_buffer =
            self.wgpu_state
                .device
                .as_ref()
                .unwrap()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Output Buffer"),
                    size: output_buffer_size,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });

        msg_send(sender, "Creating the staging buffer")?;
        // Instantiates buffer without data.
        // `usage` of buffer specifies how it can be used:
        //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
        //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
        let staging_buffer =
            self.wgpu_state
                .device
                .as_ref()
                .unwrap()
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Staging buffer"),
                    size: output_buffer_size,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

        msg_send(sender, "Creating the parameter binding buffer")?;
        let params_binding = self.wgpu_state.device.as_ref().unwrap().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Params buffer"),
                contents: bytemuck::cast_slice(&[self.max_iter, size.x]),
                usage: wgpu::BufferUsages::UNIFORM,
            },
        );

        // A bind group defines how buffers are accessed by shaders.
        // It is to WebGPU what a descriptor set is to Vulkan.
        // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

        // A pipeline specifies the operation of a shader

        // Instantiates the bind group, once again specifying the binding of buffers.
        let bind_group_layout = self
            .wgpu_state
            .compute_pipeline
            .as_ref()
            .unwrap()
            .get_bind_group_layout(0);

        msg_send(sender, "Creating bind group")?;
        let bind_group = self.wgpu_state.device.as_ref().unwrap().create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: input_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: output_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: params_binding.as_entire_binding(),
                    },
                ],
            },
        );

        msg_send(sender, "Creating a command encoder")?;
        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = self
            .wgpu_state
            .device
            .as_ref()
            .unwrap()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            msg_send(sender, "Starting compute pass")?;
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(self.wgpu_state.compute_pipeline.as_ref().unwrap());
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("compute mandelbrot iterations");
            cpass.dispatch_workgroups(size.x as u32, size.y as u32, 1);
        }
        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        msg_send(sender, "Copying output buffer to staging buffer")?;
        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_buffer_size);

        // Submits command encoder for processing
        msg_send(sender, "Sending the command encoder to the job queue")?;
        self.wgpu_state
            .queue
            .as_ref()
            .unwrap()
            .submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = staging_buffer.slice(..);
        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender_, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender_.send(v).unwrap());

        msg_send(sender, "Waiting for the GPU job to finish")?;
        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        self.wgpu_state
            .device
            .as_ref()
            .unwrap()
            .poll(wgpu::Maintain::wait())
            .panic_on_timeout();

        // Awaits until `buffer_future` can be read from
        msg_send(sender, "Receiving output buffer data")?;
        receiver
            .recv_async()
            .await
            .map_err(|err| format!("Staging buffer could not be read from: {err}"))?
            .map_err(|err| format!("Staging buffer could not be read from: {err}"))?;

        // Gets contents of buff
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to i32
        msg_send(sender, "Parsing output data")?;
        let result: Vec<i32> = bytemuck::cast_slice(&data).to_vec();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        staging_buffer.unmap();

        msg_send(sender, "Finishing capture")?;
        Ok(result
            .chunks(size.x as usize)
            .map(|chunk| chunk.into())
            .collect())
    }
}
