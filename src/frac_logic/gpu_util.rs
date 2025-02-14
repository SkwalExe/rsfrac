use std::{
    fmt::Debug,
    sync::mpsc::Sender,
    time::{Duration, Instant},
};

use humantime::format_duration;
use wgpu::AdapterInfo;

use crate::{app::SlaveMessage, helpers::Vec2};

pub(crate) trait SendSlaveMessage {
    fn send(&self, msg: SlaveMessage) -> Result<(), String>;
}

impl SendSlaveMessage for Option<&Sender<SlaveMessage>> {
    fn send(&self, msg: SlaveMessage) -> Result<(), String> {
        if let Some(sender) = self {
            sender
                .send(msg)
                .map_err(|err| format!("Could not open message channel: {err}"))?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct WgpuState {
    pub(crate) instance: Option<wgpu::Instance>,
    pub(crate) adapter: Option<wgpu::Adapter>,
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

impl Debug for WgpuState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{WgpuState}")
    }
}

pub(crate) struct GpuRenderingTracker<'a> {
    current_pass: u32,
    sender: Option<&'a Sender<SlaveMessage>>,
    size: Vec2<i32>,
    max_buf_size: u64,
    begin_time: Instant,
    adapter: AdapterInfo,
    lines_per_chunk_limit: i32,
    restart_count: i32,
}

impl<'a> GpuRenderingTracker<'a> {
    pub(crate) fn new(
        sender: Option<&'a Sender<SlaveMessage>>,
        size: &Vec2<i32>,
        max_buf_size: u64,
        adapter: AdapterInfo,
        chunk_size_limit: Option<i32>,
    ) -> Self {
        Self {
            sender,
            adapter,
            size: size.clone(),
            current_pass: 0,
            max_buf_size,
            begin_time: Instant::now(),
            restart_count: 0,
            lines_per_chunk_limit: chunk_size_limit.unwrap_or(size.y),
        }
    }

    /// Send a message to scroll the logs panel to the bottom.
    pub(crate) fn scroll_logs(&self) -> Result<(), String> {
        self.sender.send(SlaveMessage::ScrollLogs)
    }

    /// Calculate the maximum number of lines that can be rendered per pass.
    pub(crate) fn max_lines_per_pass(&self) -> i32 {
        ((self.max_buf_size / self.output_buffer_line_size()) as i32)
            .min(self.size.y)
            .min(self.lines_per_chunk_limit)
    }

    /// Calculate the number of passes required to fininsh the render.
    pub(crate) fn pass_count(&self) -> u32 {
        1.max((self.size.y as f32 / self.max_lines_per_pass() as f32).ceil() as u32)
    }

    /// Calculate the average duration of a render pass.
    pub(crate) fn pass_duration(&self) -> Option<Duration> {
        let finished_passes = self.current_pass - 1;
        if finished_passes == 0 {
            None
        } else {
            Some(self.begin_time.elapsed() / finished_passes)
        }
    }

    /// Calculate the estimated duration of all the render passes.
    pub(crate) fn estimated_duration_tot(&self) -> Option<Duration> {
        Some(self.pass_duration()? * self.pass_count())
    }

    /// Calculate the estimated time before finishing all the passes.
    pub(crate) fn estimated_duration_left(&self) -> Option<Duration> {
        Some(self.estimated_duration_tot()? - self.begin_time.elapsed())
    }

    /// Calculate the size of one divergence line in the output buffer.
    pub(crate) fn output_buffer_line_size(&self) -> u64 {
        // The byte size of one divergence line in the output buffer.
        self.size.x as u64 * size_of::<u32>() as u64
    }

    /// Calculate the necessary size for the output buffer of the current pass.
    pub(crate) fn output_buffer_chunk_size(&self) -> u64 {
        // The size of the output buffer when the input lines are limited, see above.
        self.output_buffer_line_size() * self.size.y.min(self.pass_line_count()) as u64
    }

    /// Renturns true if no render passes are left.
    pub(crate) fn render_finished(&self) -> bool {
        self.current_pass >= self.pass_count()
    }

    /// Return the number of divergence lines that will be generated during the current pass.
    pub(crate) fn pass_line_count(&self) -> i32 {
        self.pass_last_line() - self.pass_first_line()
    }

    /// Returns the y coordinate (0->size.y) of the first line of the current pass.
    pub(crate) fn pass_first_line(&self) -> i32 {
        (self.current_pass as i32 - 1) * self.max_lines_per_pass()
    }

    /// Returns the y coordinate (0->size.y) of the last line of the current pass.
    pub(crate) fn pass_last_line(&self) -> i32 {
        self.size
            .y
            .min(self.pass_first_line() + self.max_lines_per_pass())
    }

    /// Report the capture progress to the main thread.
    pub(crate) fn warn(&self, msg: impl Into<String>) -> Result<(), String> {
        self.sender.send(SlaveMessage::Warning(msg.into()))
    }
    /// Report the capture progress to the main thread.
    pub(crate) fn send(&self, msg: impl Into<String>) -> Result<(), String> {
        msg_send(
            self.sender,
            format!(
                "GPU: <acc {}>\nChunk Size: <acc {}>\nPass: <acc {}/{}>\n{}Left: <acc {}>\n<green {}>",
                self.adapter.name,
                self.max_lines_per_pass(),
                self.current_pass,
                self.pass_count(),
                if self.restart_count == 0 {
                    "".to_string()
                } else {
                    format!("Restarted: <acc {}>\n", self.restart_count)
                },
                // Prevent showing ms and ns... should improve this shit
                match self.estimated_duration_left() {
                    None => "Estimating...".to_string(),
                    Some(dur) => format_duration(Duration::from_secs(dur.as_secs())).to_string(),
                },
                msg.into(),
            ),
        )
    }

    /// Decrease the maximum number of lines to render per pass after a GPU timeout.
    pub(crate) fn limit_chunk_size(&mut self) -> Result<(), String> {
        self.lines_per_chunk_limit = self.max_lines_per_pass() / 4;
        self.sender
            .send(SlaveMessage::LimitGPUChunkSize(self.lines_per_chunk_limit))
    }

    /// When a render is repeated from the beginning because of a GPU timeout.
    pub(crate) fn reset(&mut self) {
        self.restart_count += 1;
        self.current_pass = 0;
        self.begin_time = Instant::now();
    }

    /// Inform the tracker that a new render pass was just begun.
    pub(crate) fn begin_pass(&mut self) {
        self.current_pass += 1;
        // eprintln!("Begin pass: {}", self.current_pass);
    }
}

pub(crate) fn msg_send(
    sender: Option<&Sender<SlaveMessage>>,
    message: impl Into<String> + Clone,
) -> Result<(), String> {
    // eprintln!("{}\n\n", message.clone().into());
    sender.send(SlaveMessage::SetMessage(format!(
        "Current status:\n{}",
        message.into()
    )))
}
