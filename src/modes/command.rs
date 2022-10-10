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

    fn run_command(&self) -> Option<Command> {
        if self.command.as_str() == ":q" {
            return Some(Box::new(command::quit));
        }

        None
    }
}
