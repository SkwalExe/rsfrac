use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind},
    layout::Position,
};

use crate::{
    app::App,
    components::{canvas::Canvas, Input, LogPanel},
    helpers::Focus,
};

impl App {
    /// Send a key event to the focused component
    pub(crate) fn dispatch_event(&mut self, key: KeyEvent) {
        match self.app_state.focused {
            Focus::Canvas => Canvas::handle_key_code(self, key.code),
            Focus::Input => Input::handle_event(&mut self.app_state, key),
            Focus::LogPanel => LogPanel::handle_event(&mut self.app_state, key.code),
        }
    }

    pub(crate) fn handle_mouse_event(&mut self, event: MouseEvent) {
        // Only handle key PRESSES
        if let MouseEventKind::Down(_) = event.kind {
            let component = self.get_component_at_pos(Position::new(event.column, event.row));
            if component.is_none() {
                return;
            }

            // We cheked if None just before so we can unwrap
            match component.unwrap() {
                Focus::Canvas => Canvas::handle_mouse_event(self, event),
                Focus::Input => Input::handle_mouse_event(&mut self.app_state, event),
                Focus::LogPanel => LogPanel::handle_mouse_event(&mut self.app_state, event),
            }
        }
    }

    /// Return the component at the given position as a Focus variant
    pub(crate) fn get_component_at_pos(&self, pos: Position) -> Option<Focus> {
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
    pub(crate) fn handle_event(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                self.app_state.quit = true
            }
            KeyCode::Tab => {
                self.app_state.focused = match self.app_state.focused {
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

    pub(crate) fn handle_paste(&mut self, text: String) {
        // Todo: there must be a better way to do this
        for char in text.chars().map(|c| if c == '\n' { ' ' } else { c }) {
            self.app_state
                .command_input
                .0
                .handle(tui_input::InputRequest::InsertChar(char));
        }
    }
}
