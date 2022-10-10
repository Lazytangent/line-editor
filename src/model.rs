use rustea::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    App, Command, Message,
};

pub struct Model {
    pub line: usize,
    pub column: usize,
    pub mode: Mode,
    pub filename: String,
    pub contents: Vec<String>,
    pub command: String,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if self.mode == Mode::Normal {
                return self.handle_normal(key_event);
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
        self.generate_cursorline(&mut s);
        s.push_str("\n");
        self.generate_modeline(&mut s);

        s.push_str(&format!("Line/Col: {}/{}", self.line, self.column));

        s
    }
}

impl Model {
    pub fn new() -> Self {
        Model {
            line: 1,
            column: 1,
            mode: Mode::Normal,
            filename: "src/main.rs".to_string(),
            contents: vec![],
            command: String::new(),
        }
    }

    fn generate_modeline(&self, s: &mut String) {
        if self.mode == Mode::Insert {
            s.push_str("--INSERT--");
        } else if self.mode == Mode::Command {
            s.push_str(&format!(":{}", self.command));
        } else {
            s.push_str("          ");
        }
    }

    fn generate_cursorline(&self, s: &mut String) {
        let mut cursorline = String::new();

        for _ in 0..(self.column - 1) {
            cursorline.push_str(" ");
        }

        cursorline.push_str("^");
        s.push_str(&cursorline);
    }
}
