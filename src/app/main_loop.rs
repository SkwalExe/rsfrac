use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    DefaultTerminal,
};

use super::{fractal_logic::CanvasCoords, parallel_jobs::ParallelJob, App};
const FRAME_DELAY: i32 = 100;

impl App {
    /// Run the main application loop, perform rendering and event passing
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.app_state.initial_message();
        while !self.app_state.quit {
            let start = Instant::now();

            term.draw(|frame| {
                self.build_chunks(frame.area());

                self.app_state.render_settings.canvas_size = CanvasCoords::new(
                    self.chunks.canvas_inner.width,
                    self.chunks.canvas_inner.height * 2,
                );

                // We need to already know the canvas size to set the correct initial cell size
                // and render the canvas for the first time
                if self.app_state.render_settings.cell_size == 0 {
                    self.app_state.render_settings.reset_cell_size();
                }
                if self.app_state.redraw_canvas {
                    self.render_canvas();
                }

                self.render_frame(frame);
            })?;

            // Move all the requested jobs to the top level running jobs tracker
            for requested_job in self.app_state.requested_jobs.drain(..) {
                self.parallel_jobs.push(requested_job);
            }

            while !self.parallel_jobs.is_empty()
                && start.elapsed().as_millis() < FRAME_DELAY as u128
            {
                self.parallel_jobs
                    .retain(|job| !job.run(&mut self.app_state));
            }

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
                    Event::Resize(_, _) => self.app_state.redraw_canvas = true,
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
