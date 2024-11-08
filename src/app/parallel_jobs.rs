use std::{
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
};

use chrono::Utc;
use image::ImageBuffer;
use ratatui::style::Color;

use crate::{
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
    pub(crate) handle: Option<JoinHandle<DivergMatrix>>,
    pub(crate) id: i64,
}

/// Represents a message sent from a child process
/// (only screenshot use child processes for now)
/// to the main process
#[derive(Debug)]
pub(crate) enum SlaveMessage {
    LineRender,
    JobFinished,
}

/// The struct representing the screenshot job state
/// for the main process.
impl ScreenshotMaster {
    pub(crate) fn new(
        size: Vec2<i32>,
        receiver: Receiver<SlaveMessage>,
        handle: JoinHandle<DivergMatrix>,
    ) -> Self {
        Self {
            receiver,
            size,
            rendered_lines: 0,
            handle: Some(handle),
            id: Utc::now().timestamp(),
        }
    }
    /// Handles the output of the screenshot child process:
    /// Save the render to a png file, and print a log message.
    pub(crate) fn finished(&self, state: &mut AppState, result: DivergMatrix) {
        let buf = ImageBuffer::from_fn(self.size.x as u32, self.size.y as u32, |x, y| {
            let color = state
                .render_settings
                .color_from_div(&result[y as usize][x as usize]);
            if let Color::Rgb(r, g, b) = color {
                image::Rgb([r, g, b])
            } else {
                image::Rgb([0, 0, 0])
            }
        });

        let filename = format!(
            "{}{}.png",
            state.render_settings.get_frac_obj().name,
            Utc::now().timestamp()
        );

        let _ = buf.save_with_format(&filename, image::ImageFormat::Png);

        state.log_success(format!(
            "Screenshot ({}x{}) saved to <acc {}>",
            self.size.x, self.size.y, filename
        ));
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
    pub(crate) fn start(mut screenshot: Self) -> JoinHandle<DivergMatrix> {
        thread::spawn(move || screenshot.run())
    }
    pub(crate) fn run(&mut self) -> DivergMatrix {
        self.rs_copy
            .get_diverg_matrix_with_status(&self.size, &self.sender)
    }
}
