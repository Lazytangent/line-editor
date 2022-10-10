use rustea::{
    command,
    crossterm::event::{KeyEvent, KeyCode, KeyModifiers},
    Command,
};

use crate::model::{Model, Mode};

impl Model {
    pub fn handle_normal(&mut self, key_event: &KeyEvent) -> Option<Command> {
        if let KeyModifiers::CONTROL = key_event.modifiers {
            if let KeyCode::Char('c') = key_event.code {
                return Some(Box::new(command::quit));
            }
        }

        match key_event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.line > 1 {
                    self.line -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.line < self.contents.len() - 1 {
                    self.line += 1;
                }
            }
            KeyCode::Char(':') => {
                self.mode = Mode::Command;
            }
            _ => {}
        };

        None
    }
}
