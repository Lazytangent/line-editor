use std::fs;

use rustea::{
    command,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Command,
};

use crate::model::{Mode, Model};

impl Model {
    pub fn handle_command(&mut self, key_event: &KeyEvent) -> Option<Command> {
        if let KeyModifiers::CONTROL = key_event.modifiers {
            match key_event.code {
                KeyCode::Char('u') => self.command.clear(),
                KeyCode::Char('c') => {
                    self.command.clear();
                    self.mode = Mode::Normal;
                }
                _ => {}
            }
        }

        match key_event.code {
            KeyCode::Char(c) => self.command.push(c),
            KeyCode::Esc => {
                self.command.clear();
                self.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                self.command.pop();
                if self.command.len() == 0 {
                    self.mode = Mode::Normal;
                }
            }
            KeyCode::Enter => {
                return self.run_command();
            }
            _ => {}
        }

        None
    }

    fn run_command(&mut self) -> Option<Command> {
        match self.command.as_str() {
            ":q" | ":qu" | ":qui" | ":quit" => Some(Box::new(command::quit)),
            ":w" | ":wr" | ":wri" | ":writ" | ":write" => {
                self.save();

                self.command.clear();
                self.mode = Mode::Normal;

                None
            }
            _ => None,
        }
    }

    fn save(&self) {
        let mut contents = self.contents.join("\n");
        if !contents.ends_with("\n") {
            contents.push_str("\n");
        }
        fs::write(&self.filename, contents.as_str()).unwrap();
    }
}
