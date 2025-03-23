use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

use chrono::{Local, Utc};
use futures::executor::block_on;
use image::ImageBuffer;
use ratatui::style::Color;

use crate::{
    commands::save::SAVE_EXTENSION,
    frac_logic::{DivergMatrix, RenderSettings},
    helpers::{markup::esc, Vec2},
    AppState,
};

pub(crate) struct WaitingScreenshot {
    pub(crate) size: Vec2<i32>,
    pub(crate) rs: RenderSettings,
    pub(crate) name: Option<String>,
}

impl WaitingScreenshot {
    pub(crate) fn start(self) -> ScreenshotMaster {
        let (tx, rx) = mpsc::channel();

        let screenshot = ScreenshotSlave::new(self.size.clone(), tx, self.rs.clone());
        let handle = ScreenshotSlave::start(screenshot);
        ScreenshotMaster::new(self.size.clone(), rx, handle, self.rs, self.name)
    }
}

pub(crate) struct ScreenshotMaster {
    /// We want to know the size of the screenshot in order
    /// to compute the current progression percentage.
    pub(crate) size: Vec2<i32>,
    /// Messages received through this channel are sent
    /// by the corresponding screenshot job child process.
    pub(crate) receiver: Receiver<SlaveMessage>,
    /// Used to keep track of the progression and display
    /// a percentage in the main process.
    pub(crate) rendered_lines: i32,
    pub(crate) handle: Option<JoinHandle<Result<DivergMatrix, String>>>,
    pub(crate) id: i64,
    pub(crate) rs_copy: RenderSettings,
    pub(crate) finished: bool,
    pub(crate) name: Option<String>,
}

/// Represents a message sent from a child process
/// (only screenshot use child processes for now)
/// to the main process
#[derive(Debug)]
pub(crate) enum SlaveMessage {
    LineRender,
    JobFinished,
    SetMessage(String),
    ScrollLogs,
    Warning(String),
    LimitGPUChunkSize(i32),
}

/// The struct representing the screenshot job state
/// for the main process.
impl ScreenshotMaster {
    pub(crate) fn new(
        size: Vec2<i32>,
        receiver: Receiver<SlaveMessage>,
        handle: JoinHandle<Result<DivergMatrix, String>>,
        rs: RenderSettings,
        name: Option<String>,
    ) -> Self {
        Self {
            finished: false,
            receiver,
            size,
            rendered_lines: 0,
            handle: Some(handle),
            id: Utc::now().timestamp_micros(),
            rs_copy: rs,
            name,
        }
    }
    /// Handles the output of the screenshot child process:
    /// Save the render to a png file, and print a log message.
    pub(crate) fn finished(&self, state: &mut AppState, result: Result<DivergMatrix, String>) {
        match result {
            Err(err) => state.log_error(format!("Could not finish screenshot, reason: {err}")),
            Ok(result) => {
                let height = self.size.y as usize;
                let rs_copy = &self.rs_copy;
                let buf =
                    ImageBuffer::from_par_fn(self.size.x as u32, self.size.y as u32, |x, y| {
                        let div = &result[height - y as usize - 1][x as usize];
                        let color = rs_copy.color_from_div(div);
                        if let Color::Rgb(r, g, b) = color {
                            image::Rgb([r, g, b])
                        } else {
                            image::Rgb([0, 0, 0])
                        }
                    });

                let filename_base = self.name.clone().unwrap_or(format!(
                    "{} {}",
                    self.rs_copy.get_frac_obj().name,
                    Local::now().format("%F %H-%M-%S%.f")
                ));

                let filename_save = format!("{}{}", filename_base, SAVE_EXTENSION);
                let filename_cap = format!(
                    "{}.{}",
                    filename_base,
                    self.rs_copy.image_format.extensions_str()[0]
                );

                match self.rs_copy.save(&filename_save) {
                    Err(err) => state.log_error(err),
                    Ok(_) => state.log_info(format!(
                        "State file containing capture parameters saved to <acc {}> in case of failure.",
                        filename_save
                    )),
                }

                if let Err(err) = buf.save_with_format(&filename_cap, self.rs_copy.image_format) {
                    state.log_error(format!("Could not save screenshot: {}", esc(err)));
                } else {
                    state.log_success(format!(
                        "Screenshot ({}x{}) saved to <acc {}>",
                        self.size.x,
                        self.size.y,
                        esc(filename_cap)
                    ));
                }
            }
        }
    }
}

/// The struct representing the screenshot job state
/// in the context of the child process.
pub(crate) struct ScreenshotSlave {
    size: Vec2<i32>,
    /// Messages sent through this channel will be
    /// received by the corresponding `ScreenshotMaster`.
    sender: Sender<SlaveMessage>,
    /// Copy of the render settings at the moment of the request
    rs_copy: RenderSettings,
}

impl ScreenshotSlave {
    pub(crate) fn new(size: Vec2<i32>, sender: Sender<SlaveMessage>, rs: RenderSettings) -> Self {
        Self {
            size,
            sender,
            rs_copy: rs,
        }
    }
}
impl ScreenshotSlave {
    /// Creates a new process, running the screenshot rendering.
    pub(crate) fn start(mut screenshot: Self) -> JoinHandle<Result<DivergMatrix, String>> {
        thread::spawn(move || block_on(screenshot.run()))
    }
    pub(crate) async fn run(&mut self) -> Result<DivergMatrix, String> {
        if self.rs_copy.wgpu_state.use_gpu {
            self.rs_copy.initialize_gpu().await?;
            let result = self
                .rs_copy
                .get_gpu_diverg_matrix_async(&self.size, Some(&self.sender))
                .await;
            self.sender
                .send(SlaveMessage::JobFinished)
                .map_err(|err| format!("Could not open message channel: {err}"))?;
            result
        } else {
            Ok(self
                .rs_copy
                .get_diverg_matrix_with_status(&self.size, &self.sender))
        }
    }
}
