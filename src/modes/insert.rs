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

        match key_event.code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
            }
            KeyCode::Char(c) => {
                let line = &mut self.contents[self.line - 1];
                line.insert(self.column - 1, c);
                self.column += 1;
            }
            _ => {}
        }

        None
    }
}
