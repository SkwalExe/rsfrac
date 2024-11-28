use std::{cell::RefCell, sync::mpsc::Sender};

use crate::{app::SlaveMessage, frac_logic::CanvasCoords, helpers::Vec2};

use super::{DivergMatrix, RenderSettings};
use futures::executor;
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
    message: impl Into<String> + Clone,
) -> Result<(), String> {
    // eprintln!("{:?}", message.clone().into());
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

    pub(crate) async fn get_gpu_diverg_matrix(
        &mut self,
        size: &Vec2<i32>,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<DivergMatrix, String> {
        // The byte size of one divergence line in the output buffer.
        let output_buffer_line_size = size.x as u64 * size_of::<u32>() as u64;

        // The size of an input line of coordinates.
        let input_line_size = 2 * size.x as u64 * size_of::<f32>() as u64;

        // The maximum buffer size
        let max_buf_size = self
            .wgpu_state
            .device
            .as_ref()
            .unwrap()
            .limits()
            .max_storage_buffer_binding_size as u64;

        // Maximum number of input lines that can be stored in a buffer.
        let max_lines_per_pass = (max_buf_size / input_line_size) as i32;

        // The size of the output buffer when the input lines are limited, see above.
        let output_buffer_chunk_size =
            output_buffer_line_size * size.y.min(max_lines_per_pass) as u64;

        // The final divergence matrix, each render pass will push a chunk of lines.
        let mut result: DivergMatrix = Vec::new();

        // If a single input line can not fit in the buffer, then the render pass is too
        // complicated/impossible, so we will abort the render.
        if input_line_size > max_buf_size {
            return Err(format!(
                "Input buffer line size ({}MB) would exceed maximum GPU buffer size ({}MB).",
                input_line_size / 1000000,
                max_buf_size / 1000000
            ));
        }

        // The number of the current pass.
        // Is there a way to avoid using RefCell?
        let current_pass = RefCell::new(0);

        // The total number of passes required to finish the render.
        let total_pass_count = 1.max((size.y as f32 / max_lines_per_pass as f32).ceil() as i32);

        // Will add the render pass progress before sending the status message.
        let msg_send_progress = |msg: String| {
            msg_send(
                sender,
                format!(
                    "Render pass <acc {}/{}>: {}",
                    current_pass.borrow(),
                    total_pass_count,
                    msg
                ),
            )
        };

        let half_y = size.y / 2;
        let half_x = size.x / 2;
        let cell_size = self.get_plane_wid() / size.x;

        // The first line of the next render pass.
        let mut current_line = 0;

        while current_line < size.y {
            // The first line of this render pass.
            let first_line = -half_y + current_line;
            // The last line (not included) of this render pass.
            let last_line = size.y.min(first_line + max_lines_per_pass);

            *current_pass.borrow_mut() += 1;

            msg_send_progress("Generating GPU input data".to_string())?;
            let points: Vec<f32> = (first_line..last_line)
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

            current_line += max_lines_per_pass;

            msg_send_progress("Creating the input buffer".to_string())?;
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

            msg_send_progress("Creating the output buffer".to_string())?;
            let output_buffer =
                self.wgpu_state
                    .device
                    .as_ref()
                    .unwrap()
                    .create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Output Buffer"),
                        size: output_buffer_chunk_size,
                        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        mapped_at_creation: false,
                    });

            msg_send_progress("Creating the staging buffer".to_string())?;
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
                        size: output_buffer_chunk_size,
                        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });

            msg_send_progress("Creating the parameter binding buffer".to_string())?;
            let params_binding = self.wgpu_state.device.as_ref().unwrap().create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Params buffer"),
                    contents: bytemuck::cast_slice(&[self.max_iter, size.x]),
                    usage: wgpu::BufferUsages::UNIFORM,
                },
            );

            let bind_group_layout = self
                .wgpu_state
                .compute_pipeline
                .as_ref()
                .unwrap()
                .get_bind_group_layout(0);

            msg_send_progress("Creating bind group".to_string())?;
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

            msg_send_progress("Creating a command encoder".to_string())?;
            // A command encoder executes one or many pipelines.
            // It is to WebGPU what a command buffer is to Vulkan.
            let mut encoder = self
                .wgpu_state
                .device
                .as_ref()
                .unwrap()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            {
                msg_send_progress("Starting compute pass".to_string())?;
                let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: None,
                    timestamp_writes: None,
                });
                cpass.set_pipeline(self.wgpu_state.compute_pipeline.as_ref().unwrap());
                cpass.set_bind_group(0, &bind_group, &[]);
                cpass.insert_debug_marker("compute mandelbrot iterations");
                cpass.dispatch_workgroups(size.x as u32, (last_line - first_line) as u32, 1);
            }
            // Sets adds copy operation to command encoder.
            // Will copy data from storage buffer on GPU to staging buffer on CPU.
            msg_send_progress("Copying output buffer to staging buffer".to_string())?;
            encoder.copy_buffer_to_buffer(
                &output_buffer,
                0,
                &staging_buffer,
                0,
                output_buffer_chunk_size,
            );

            // Submits command encoder for processing
            msg_send_progress("Sending the command encoder to the job queue".to_string())?;
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

            msg_send_progress("Waiting for the GPU job to finish".to_string())?;
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
            msg_send_progress("Receiving output buffer data".to_string())?;
            receiver
                .recv_async()
                .await
                .map_err(|err| format!("Staging buffer could not be read from: {err}"))?
                .map_err(|err| format!("Staging buffer could not be read from: {err}"))?;

            // Gets contents of buff
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to i32
            msg_send_progress("Parsing output data".to_string())?;
            let lines_flat = bytemuck::cast_slice(&data).to_vec();
            let mut lines = lines_flat
                .chunks(size.x as usize)
                .map(|chunk| chunk.to_vec())
                .collect();
            result.append(&mut lines);

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            staging_buffer.unmap();
        }

        msg_send_progress("Finishing capture".to_string())?;
        Ok(result)
    }
}
