use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use rustea::{
    command,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    App, Command, Message,
};

struct Model {
    line: usize,
    column: usize,
    mode: Mode,
    filename: String,
    contents: Vec<String>,
    command: String,
}

#[derive(Eq, PartialEq, Debug)]
enum Mode {
    Normal,
    Insert,
    Command,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if self.mode == Mode::Normal {
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
                }
            }

            if self.mode == Mode::Insert {
                if let KeyModifiers::CONTROL = key_event.modifiers {
                    if let KeyCode::Char('c') = key_event.code {
                        self.mode = Mode::Normal;
                    }
                }

                if let KeyCode::Esc = key_event.code {
                    self.mode = Mode::Normal;
                }
            }

            if self.mode == Mode::Command {
                if let KeyModifiers::CONTROL = key_event.modifiers {
                    match key_event.code {
                        KeyCode::Char('u') => self.command.clear(),
                        KeyCode::Char('c') => {
                            self.command.clear();
                            self.mode = Mode::Normal;
                        }
                        KeyCode::Backspace => {
                            self.command.pop();
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
                    _ => {}
                }
            }
        }

        None
    }

    fn view(&self) -> String {
        let line = &self.contents[self.line - 1];

        let mut s = line.to_string();

        s.push_str("\n");
        self.generate_modeline(&mut s);

        s.push_str(&format!("Line/Col: {}/{}", self.line, self.column));

        s
    }
}

impl Model {
    fn generate_modeline(&self, s: &mut String) {
        if self.mode == Mode::Insert {
            s.push_str("--INSERT--");
        } else if self.mode == Mode::Command {
            s.push_str(&format!(":{}", self.command));
        } else {
            s.push_str("          ");
        }
    }
}

fn open_file(path: &str) -> io::Result<Vec<String>> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let contents: Vec<String> = reader.lines().map(|res| res.unwrap()).collect();

    Ok(contents)
}

fn main() {
    let mut app = Model {
        line: 1,
        column: 0,
        mode: Mode::Normal,
        filename: "src/main.rs".to_string(),
        contents: vec![],
        command: String::new(),
    };

    let contents = match open_file(&app.filename) {
        Ok(c) => c,
        Err(_) => unreachable!(),
    };

    app.contents = contents;

    rustea::run(app).unwrap();
}
