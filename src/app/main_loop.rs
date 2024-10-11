use std::{io, time::Duration};

use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    DefaultTerminal,
};

use super::{fractal_logic::CanvasCoords, App};

impl App {
    /// Run the main application loop, perform rendering and event passing
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.initial_message();
        while !self.quit {
            term.draw(|frame| {
                self.build_chunks(frame.area());

                self.render_settings.canvas_size = CanvasCoords::new(
                    self.chunks.canvas_inner.width,
                    self.chunks.canvas_inner.height * 2,
                );

                // We need to already know the canvas size to set the correct initial cell size
                // and render the canvas for the first time
                if self.render_settings.cell_size == 0 {
                    self.reset_cell_size();
                }
                if self.redraw_canvas {
                    self.render_canvas();
                }

                self.render_frame(frame);
            })?;

            // Wait 100ms for event and handle it
            if event::poll(Duration::from_millis(100)).unwrap() {
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
                    Event::Resize(_, _) => self.redraw_canvas = true,
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
