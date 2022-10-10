use rustea::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Command,
};

use crate::model::{Mode, Model};

impl Model {
    pub fn handle_insert(&mut self, key_event: &KeyEvent) -> Option<Command> {
        if let KeyModifiers::CONTROL = key_event.modifiers {
            if let KeyCode::Char('c') = key_event.code {
                self.mode = Mode::Normal;
            }
        }

        if let KeyCode::Esc = key_event.code {
            self.mode = Mode::Normal;
        }

        None
    }
}
