use rustea::{
    command,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    Command,
};

use crate::model::{Mode, Model};

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
                    let current_line_length = self.contents[self.line - 1].len();

                    if current_line_length != 0 && current_line_length < self.column {
                        self.column = current_line_length;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.line < self.contents.len() - 1 {
                    self.line += 1;
                    let current_line_length = self.contents[self.line - 1].len();

                    if current_line_length != 0 && current_line_length < self.column {
                        self.column = current_line_length;
                    }
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.column > 1 {
                    self.column -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.column < self.contents[self.line - 1].len() {
                    self.column += 1;
                }
            }
            KeyCode::Char(':') => {
                self.mode = Mode::Command;
                self.command.push(':');
            }
            _ => {}
        };

        None
    }
}
