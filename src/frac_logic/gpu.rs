use std::{
    sync::mpsc::Sender,
    time::{Duration, Instant},
};

use crate::{app::SlaveMessage, helpers::Vec2};

use super::{DivergMatrix, RenderSettings};
use futures::executor;
use humantime::format_duration;
use wgpu::{util::DeviceExt, AdapterInfo};

#[derive(bytemuck::NoUninit, Clone, Copy)]
#[repr(C)]
pub(crate) struct ParamsBinding {
    max_iter: i32,
    size_x: i32,
    size_y: i32,
    pos_real: f32,
    pos_imag: f32,
    cell_size: f32,
    y_offset: i32,
}

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

pub(crate) struct GpuRenderingTracker<'a> {
    current_pass: u32,
    sender: Option<&'a Sender<SlaveMessage>>,
    size: Vec2<i32>,
    max_buf_size: u64,
    begin_time: Instant,
    adapter: AdapterInfo,
}

impl<'a> GpuRenderingTracker<'a> {
    pub(crate) fn new(
        sender: Option<&'a Sender<SlaveMessage>>,
        size: &Vec2<i32>,
        max_buf_size: u64,
        adapter: AdapterInfo,
    ) -> Self {
        Self {
            sender,
            adapter,
            size: size.clone(),
            current_pass: 0,
            max_buf_size,
            begin_time: Instant::now(),
        }
    }

    fn scroll_logs(&self) {
        if let Some(sender) = self.sender {
            let _ = sender.send(SlaveMessage::ScrollLogs);
        }
    }

    fn max_lines_per_pass(&self) -> i32 {
        (self.max_buf_size / self.output_buffer_line_size()) as i32
    }

    fn pass_count(&self) -> u32 {
        1.max((self.size.y as f32 / self.max_lines_per_pass() as f32).ceil() as u32)
    }

    fn pass_duration(&self) -> Option<Duration> {
        let finished_passes = self.current_pass - 1;
        if finished_passes == 0 {
            None
        } else {
            Some(self.begin_time.elapsed() / finished_passes)
        }
    }

    fn estimated_duration_tot(&self) -> Option<Duration> {
        Some(self.pass_duration()? * self.pass_count())
    }

    fn estimated_duration_left(&self) -> Option<Duration> {
        Some(self.estimated_duration_tot()? - self.begin_time.elapsed())
    }
    fn output_buffer_line_size(&self) -> u64 {
        // The byte size of one divergence line in the output buffer.
        self.size.x as u64 * size_of::<u32>() as u64
    }

    fn output_buffer_chunk_size(&self) -> u64 {
        // The size of the output buffer when the input lines are limited, see above.
        self.output_buffer_line_size() * self.size.y.min(self.max_lines_per_pass()) as u64
    }

    fn render_finished(&self) -> bool {
        self.current_pass >= self.pass_count()
    }

    fn pass_line_count(&self) -> i32 {
        self.pass_last_line() - self.pass_first_line()
    }

    fn pass_first_line(&self) -> i32 {
        (self.current_pass as i32 - 1) * self.max_lines_per_pass()
    }

    fn pass_last_line(&self) -> i32 {
        self.size
            .y
            .min(self.pass_first_line() + self.max_lines_per_pass())
    }

    pub(crate) fn send(&self, msg: impl Into<String>) -> Result<(), String> {
        msg_send(
            self.sender,
            format!(
                "GPU: <acc {}>\nPass <acc {}/{}>\nLeft: <acc {}>\n<green {}>",
                self.adapter.name,
                self.current_pass,
                self.pass_count(),
                // Prevent showing ms and ns... should improve this shit
                match self.estimated_duration_left() {
                    None => "Estimating...".to_string(),
                    Some(dur) => format_duration(Duration::from_secs(dur.as_secs())).to_string(),
                },
                msg.into(),
            ),
        )
    }

    pub(crate) fn begin_pass(&mut self) {
        self.current_pass += 1;
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
                "Current status:\n{}",
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
        // The maximum buffer size
        let max_buf_size = self
            .wgpu_state
            .device
            .as_ref()
            .unwrap()
            .limits()
            .max_storage_buffer_binding_size as u64;

        let mut tracker = GpuRenderingTracker::new(
            sender,
            size,
            max_buf_size,
            self.wgpu_state.adapter.as_ref().unwrap().get_info(),
        );

        // The final divergence matrix, each render pass will push a chunk of lines.
        let mut result: DivergMatrix = Vec::new();

        // If a single input line can not fit in the buffer, then the render pass is too
        // complicated/impossible, so we will abort the render.
        if tracker.output_buffer_line_size() > max_buf_size {
            return Err(format!(
                "Output buffer line size ({}MB) would exceed maximum GPU buffer size ({}MB).",
                tracker.output_buffer_line_size() / 1000000,
                max_buf_size / 1000000
            ));
        }

        let cell_size = self.get_plane_wid() / size.x;

        while !tracker.render_finished() {
            tracker.begin_pass();

            tracker.send("Creating the output buffer")?;
            let output_buffer =
                self.wgpu_state
                    .device
                    .as_ref()
                    .unwrap()
                    .create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Output Buffer"),
                        size: tracker.output_buffer_chunk_size(),
                        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                        mapped_at_creation: false,
                    });

            tracker.send("Creating the staging buffer")?;
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
                        size: tracker.output_buffer_chunk_size(),
                        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: false,
                    });

            tracker.send("Creating the parameter binding buffers")?;
            let params_binding = self.wgpu_state.device.as_ref().unwrap().create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Params buffer"),
                    contents: bytemuck::bytes_of(&ParamsBinding {
                        max_iter: self.max_iter,
                        size_x: size.x,
                        size_y: size.y,
                        pos_real: self.pos.real().to_f32(),
                        pos_imag: self.pos.imag().to_f32(),
                        cell_size: cell_size.to_f32(),
                        y_offset: tracker.pass_first_line(),
                    }),
                    usage: wgpu::BufferUsages::UNIFORM,
                },
            );

            let bind_group_layout = self
                .wgpu_state
                .compute_pipeline
                .as_ref()
                .unwrap()
                .get_bind_group_layout(0);

            tracker.send("Creating bind group")?;
            let bind_group = self.wgpu_state.device.as_ref().unwrap().create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: output_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: params_binding.as_entire_binding(),
                        },
                    ],
                },
            );

            tracker.send("Creating a command encoder")?;
            // A command encoder executes one or many pipelines.
            // It is to WebGPU what a command buffer is to Vulkan.
            let mut encoder = self
                .wgpu_state
                .device
                .as_ref()
                .unwrap()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            {
                tracker.send("Starting compute pass")?;
                let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: None,
                    timestamp_writes: None,
                });
                cpass.set_pipeline(self.wgpu_state.compute_pipeline.as_ref().unwrap());
                cpass.set_bind_group(0, &bind_group, &[]);
                cpass.insert_debug_marker("compute mandelbrot iterations");
                cpass.dispatch_workgroups(size.x as u32, tracker.pass_line_count() as u32, 1);
            }
            // Sets adds copy operation to command encoder.
            // Will copy data from storage buffer on GPU to staging buffer on CPU.
            tracker.send("Copying output buffer to staging buffer")?;
            encoder.copy_buffer_to_buffer(
                &output_buffer,
                0,
                &staging_buffer,
                0,
                tracker.output_buffer_chunk_size(),
            );

            // Submits command encoder for processing
            tracker.send("Sending the command encoder")?;
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

            tracker.send("Waiting for the GPU job to finish")?;
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
            tracker.send("Receiving output buffer data")?;
            receiver
                .recv_async()
                .await
                .map_err(|err| format!("Staging buffer could not be read from: {err}"))?
                .map_err(|err| format!("Staging buffer could not be read from: {err}"))?;

            // Gets contents of buff
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to i32
            tracker.send("Parsing output data")?;
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

        tracker.send(
            "Saving image to file... The application will be blocked during the time of writing.",
        )?;
        tracker.scroll_logs();
        Ok(result)
    }
}
