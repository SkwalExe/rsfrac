use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    style::Style,
    DefaultTerminal,
};
use std::{
    io,
    thread::sleep,
    time::{Duration, Instant},
};

use crate::{
    app::SlaveMessage, commands::gpu::execute_gpu, frac_logic::CanvasCoords, helpers::Chunks, App,
};

/// The delay listening for key events before each terminal redraw.
#[cfg(feature = "web-runner")]
const FRAME_DELAY: i32 = 800;
#[cfg(not(feature = "web-runner"))]
const FRAME_DELAY: i32 = 80;
const WAITING_JOBS_MESSAGE_ID: i64 = 0x1;

/// The minimum height or width necessary to render the app.
const MIN_SCREEN_SIZE: u16 = 10;

impl App {
    /// Run the main application loop, perform rendering and event passing
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.app_state.initial_message();
        if let Err(err) = execute_gpu(&mut self.app_state, Default::default()) {
            self.app_state.log_error(err);
            self.app_state.cpu_defaults();
        }
        while !self.app_state.quit {
            let start = Instant::now();

            // Will be set to true if the screen was to small to render.
            let mut too_small = false;

            term.draw(|frame| {
                self.chunks = Chunks::new(frame.area(), self);
                // DO NOT try to render anything if the available size is less
                if self.chunks.canvas_inner().width < MIN_SCREEN_SIZE
                    || self.chunks.canvas_inner().height < MIN_SCREEN_SIZE
                {
                    too_small = true;
                    frame
                        .buffer_mut()
                        .set_string(0, 0, "Not enough space", Style::default());
                    return;
                }

                self.app_state.render_settings.canvas_size = CanvasCoords::new(
                    self.chunks.canvas_inner().width,
                    self.chunks.canvas_inner().height * 2,
                );

                // TODO: Do this before starting the main loop
                // We need to already know the canvas size to set the correct initial cell size
                // and render the canvas for the first time
                if self.app_state.render_settings.cell_size == 0 {
                    self.app_state.render_settings.reset_cell_size();
                }

                if self.app_state.redraw_canvas || self.app_state.repaint_canvas {
                    self.render_canvas();
                }

                self.render_frame(frame);
            })?;

            // If the screen was too small to render, don't try to do anything at all.
            if too_small {
                sleep(Duration::from_millis(100));
                continue;
            }

            // 1 - Remove all jobs if asked so
            if self.app_state.remove_jobs {
                self.app_state.remove_jobs = false;
                self.app_state.requested_jobs = Vec::new();
                while !self.parallel_jobs.is_empty() {
                    let job = self.parallel_jobs.remove(0);
                    // If the job thread is still running, it should
                    // exit by itself when detecting that the message pipe is closed.
                    self.app_state.prioritized_log_messages.remove(&job.id);
                }
            }

            // 2 - Cycle through all the running jobs, non-blockingly handle messages
            // and remove finished ones.
            self.parallel_jobs.retain_mut(|job| {
                if job.finished {
                    // remove the priorotized progression message when the screenshot if
                    // finished
                    self.app_state.prioritized_log_messages.remove(&job.id);
                    let result = job.handle.take().unwrap().join().unwrap();
                    job.finished(&mut self.app_state, result);
                    return false;
                }
                for message in job.receiver.try_iter() {
                    match message {
                        SlaveMessage::LineRender => {
                            job.rendered_lines += 1;
                            // Display the current progression as a prioritized log message.
                            self.app_state.prioritized_log_messages.insert(
                                job.id,
                                format!(
                                    "Screenshot progression:\nline {}/{} (<command {:?}%>)",
                                    job.rendered_lines,
                                    job.size.y,
                                    job.rendered_lines * 100 / job.size.y
                                ),
                            );
                        }
                        SlaveMessage::JobFinished => {
                            job.finished = true;
                        }
                        SlaveMessage::Warning(warn) => self.app_state.log_warn(warn),
                        SlaveMessage::SetMessage(message) => {
                            self.app_state
                                .prioritized_log_messages
                                .insert(job.id, message);
                        }
                        SlaveMessage::ScrollLogs => {
                            self.app_state
                                .log_panel_scroll_state
                                .lock()
                                .unwrap()
                                .scroll_to_bottom();
                        }
                        SlaveMessage::LimitGPUChunkSize(size) => {
                            self.app_state.render_settings.chunk_size_limit = Some(size)
                        }
                    }
                }
                true
            });

            // 3 - Start a new job if possible
            if !self.app_state.pause_jobs
                && self.parallel_jobs.is_empty()
                && !self.app_state.requested_jobs.is_empty()
            {
                self.parallel_jobs
                    .push(self.app_state.requested_jobs.remove(0).start());
            }

            // 4 - Report waiting jobs
            if !self.app_state.requested_jobs.is_empty() || self.app_state.pause_jobs {
                self.app_state.prioritized_log_messages.insert(
                    WAITING_JOBS_MESSAGE_ID,
                    format!(
                        "<yellow {}> job(s) waiting in queue. {}",
                        self.app_state.requested_jobs.len(),
                        self.app_state.pause_jobs.then_some(
                            "Job execution is paused, you can resume with the <command pause> command.").unwrap_or_default()
                    ),
                );
            } else {
                self.app_state
                    .prioritized_log_messages
                    .remove(&WAITING_JOBS_MESSAGE_ID);
            }

            // Try not to sleep if the previous operations took some time.
            let delay = 0.max(FRAME_DELAY - start.elapsed().as_millis() as i32) as u64;

            // Wait 100ms for event and handle it
            if event::poll(Duration::from_millis(delay)).unwrap() {
                // Catch the event
                match event::read().unwrap() {
                    Event::Key(key) => {
                        // Only handle key PRESSES
                        if key.kind == KeyEventKind::Press {
                            // If the global handler did not catch the key,
                            // send it to the focused component
                            if !self.handle_event(key) {
                                self.dispatch_event(key)
                            }
                        }
                    }
                    Event::Paste(text) => self.handle_paste(text),
                    Event::Resize(_, _) => self.app_state.request_redraw(),
                    Event::Mouse(mouse_ev) => self.handle_mouse_event(mouse_ev),
                    _ => {}
                }
                // Clear all other events
                while event::poll(Duration::ZERO).unwrap() {
                    let _ = event::read();
                }
            }
        }

        Ok(())
    }
}
