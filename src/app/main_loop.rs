use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    DefaultTerminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::{
    app::SlaveMessage, commands::gpu::execute_gpu, frac_logic::CanvasCoords, helpers::Chunks, App,
};

/// The delay listening for key events before each terminal redraw.
const FRAME_DELAY: i32 = 80;

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

            term.draw(|frame| {
                self.chunks = Chunks::from(frame.area());

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

            // Move all the requested jobs to the top level running jobs tracker
            for requested_job in self.app_state.requested_jobs.drain(..) {
                self.parallel_jobs.push(requested_job);
            }

            // Cycle through all the running jobs, non-blockingly handle messages
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
                            *self
                                .app_state
                                .prioritized_log_messages
                                .get_mut(&job.id)
                                .expect("There was no entry in the prioritized log messages corresponding to the current job.") = format!(
                                    "Screenshot progression:\nline {}/{} (<command {:?}%>)",
                                    job.rendered_lines,
                                    job.size.y,
                                    job.rendered_lines * 100 / job.size.y
                                );
                        }
                        SlaveMessage::JobFinished => {
                            job.finished = true;
                        }
                        SlaveMessage::Warning(warn) => self.app_state.log_warn(warn),
                        SlaveMessage::SetMessage(message) => {

                            *self
                                .app_state
                                .prioritized_log_messages
                                .get_mut(&job.id)
                                .expect("There was no entry in the prioritized log messages corresponding to the current job.") = message;
                        },
                        SlaveMessage::ScrollLogs => {
                            self.app_state.log_panel_scroll_state.lock().unwrap().scroll_to_bottom();
                        }
                    }
                }
                true
            });

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
