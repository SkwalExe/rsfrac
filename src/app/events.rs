use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
    layout::Position,
};

use crate::{
    app::App,
    components::{canvas::Canvas, input::Input, log_panel::LogPanel},
    helpers::Focus,
};

impl App {
    /// Send a key event to the focused component
    pub fn dispatch_event(&mut self, key: KeyEvent) {
        match self.focused {
            Focus::Canvas => Canvas::handle_key_code(self, key.code),
            Focus::Input => Input::handle_event(self, key),
            Focus::LogPanel => LogPanel::handle_event(self, key.code),
        }
    }

    pub fn handle_mouse_event(&mut self, event: MouseEvent) {
        // Only handle key PRESSES
        if let MouseEventKind::Down(_) = event.kind {
            let component = self.get_component_at_pos(Position::new(event.column, event.row));
            if component.is_none() {
                return;
            }

            // We cheked if None just before so we can unwrap
            match component.unwrap() {
                Focus::Canvas => Canvas::handle_mouse_event(self, event),
                Focus::Input => Input::handle_mouse_event(self, event),
                Focus::LogPanel => LogPanel::handle_mouse_event(self, event),
            }
        }
    }

    /// Return the component at the given position as a Focus variant
    pub fn get_component_at_pos(&self, pos: Position) -> Option<Focus> {
        Some(if self.chunks.canvas.contains(pos) {
            Focus::Canvas
        } else if self.chunks.log_panel.contains(pos) {
            Focus::LogPanel
        } else if self.chunks.input.contains(pos) {
            Focus::Input
        } else {
            return None;
        })
    }

    /// Handle global key events, return false if the key was not catched
    pub fn handle_event(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => self.quit = true,
            KeyCode::Tab => {
                self.focused = match self.focused {
                    Focus::Input => Focus::Canvas,
                    Focus::Canvas => Focus::LogPanel,
                    Focus::LogPanel => Focus::Input,
                }
            }
            _ => {
                // return false if the key was not match
                // to let the selected component handle it
                return false;
            }
        }
        true
    }
}
