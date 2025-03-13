use std::{sync::mpsc::Sender, time::Instant};

use futures::executor;
use wgpu::util::DeviceExt;

use crate::{app::SlaveMessage, helpers::Vec2};

use super::{
    gpu_rendering_tracker::GpuRenderingTracker, DivergMatrix, ParamsBinding, RenderSettings,
};

const GPU_JOB_TIMEOUT: u64 = 15;

impl RenderSettings {
    pub(crate) fn get_gpu_diverg_matrix_sync(
        &mut self,
        size: &Vec2<i32>,
        sender: Option<&Sender<SlaveMessage>>,
    ) -> Result<DivergMatrix, String> {
        executor::block_on(self.get_gpu_diverg_matrix_async(size, sender))
    }
    pub(crate) async fn get_gpu_diverg_matrix_async(
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
            self.chunk_size_limit,
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

        let cell_size = self.cell_size_from_height(size.y);

        while !tracker.render_finished() {
            tracker.scroll_logs()?;
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
                        size: [size.x, size.y],
                        pos: [self.pos.real().to_f32(), self.pos.imag().to_f32()],
                        cell_size: cell_size.to_f32(),
                        y_offset: tracker.pass_first_line(),
                        julia_constant: [
                            self.julia_constant.real().to_f32(),
                            self.julia_constant.imag().to_f32(),
                        ],
                        mandel_constant: [
                            self.mandel_constant.real().to_f32(),
                            self.mandel_constant.imag().to_f32(),
                        ],
                        bailout: self.bailout,
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

            let before_poll = Instant::now();
            self.wgpu_state
                .device
                .as_ref()
                .unwrap()
                .poll(wgpu::Maintain::wait())
                .panic_on_timeout();
            // .panic_on_timeout();
            // Seems to be unimplemented??????

            if before_poll.elapsed().as_secs() > GPU_JOB_TIMEOUT {
                tracker.warn(concat!(
                    "Possible GPU timeout, cannot obtain details because ",
                    "of an issue on WGPU's end. Trying to reduce chunk size until the job completes..."
                ))?;
                tracker.limit_chunk_size()?;
                tracker.reset();
                result = Vec::new();
                // reinitialize GPU to clear queue, there must be a better way to do this.
                self.initialize_gpu_async(sender).await?;
                continue;
            }

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
        Ok(result)
    }
}
