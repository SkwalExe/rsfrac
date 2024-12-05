use std::{
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
};

use chrono::{Local, Utc};
use image::ImageBuffer;
use ratatui::style::Color;

use crate::{
    commands::save::execute_save,
    frac_logic::{DivergMatrix, RenderSettings},
    helpers::Vec2,
    AppState,
};

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
    pub(crate) frac_name: &'static str,
    pub(crate) finished: bool,
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
}

/// The struct representing the screenshot job state
/// for the main process.
impl ScreenshotMaster {
    pub(crate) fn new(
        size: Vec2<i32>,
        receiver: Receiver<SlaveMessage>,
        handle: JoinHandle<Result<DivergMatrix, String>>,
        frac_name: &'static str,
    ) -> Self {
        Self {
            finished: false,
            receiver,
            size,
            rendered_lines: 0,
            handle: Some(handle),
            id: Utc::now().timestamp_micros(),
            frac_name,
        }
    }
    /// Handles the output of the screenshot child process:
    /// Save the render to a png file, and print a log message.
    pub(crate) fn finished(&self, state: &mut AppState, result: Result<DivergMatrix, String>) {
        match result {
            Ok(result) => {
                let height = self.size.y as usize;
                let buf =
                    ImageBuffer::from_par_fn(self.size.x as u32, self.size.y as u32, |x, y| {
                        let color = state
                            .render_settings
                            .color_from_div(&result[height - y as usize - 1][x as usize]);
                        if let Color::Rgb(r, g, b) = color {
                            image::Rgb([r, g, b])
                        } else {
                            image::Rgb([0, 0, 0])
                        }
                    });

                let filename_base =
                    format!("{} {}", self.frac_name, Local::now().format("%F %H-%M-%S"));

                let filename = format!(
                    "{}.{}",
                    filename_base,
                    state.render_settings.image_format.extensions_str()[0]
                );

                if let Err(err) =
                    buf.save_with_format(&filename, state.render_settings.image_format)
                {
                    state.log_error(format!("Could not save screenshot: {err}"));
                } else {
                    state.log_success(format!(
                        "Screenshot ({}x{}) saved to <acc {}>",
                        self.size.x, self.size.y, filename
                    ));

                    if let Err(err) = execute_save(state, vec![&filename_base]) {
                        state.log_error(err);
                    }
                }
            }
            Err(err) => state.log_error(format!("Could not finish screenshot, reason: {err}")),
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
    pub(crate) fn new(size: Vec2<i32>, sender: Sender<SlaveMessage>, rs: &RenderSettings) -> Self {
        Self {
            size,
            sender,
            rs_copy: rs.clone(),
        }
    }
}
impl ScreenshotSlave {
    /// Creates a new process, running the screenshot rendering.
    pub(crate) fn start(mut screenshot: Self) -> JoinHandle<Result<DivergMatrix, String>> {
        thread::spawn(move || screenshot.run())
    }
    pub(crate) fn run(&mut self) -> Result<DivergMatrix, String> {
        if self.rs_copy.use_gpu {
            self.rs_copy.initialize_gpu_sync(Some(&self.sender))?;
            let result = self
                .rs_copy
                .get_gpu_diverg_matrix_sync(&self.size, Some(&self.sender));
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
