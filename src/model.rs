use rustea::{crossterm::event::KeyEvent, App, Command, Message};

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
                return self.handle_insert(key_event);
            }

            if self.mode == Mode::Command {
                return self.handle_command(key_event);
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

        let location_string = format!("Line/Col: {}/{}", self.line, self.column);
        s.push_str(&format!("{:>100}", location_string));

        s
    }
}

impl Model {
    pub fn new() -> Self {
        Model {
            line: 1,
            column: 1,
            mode: Mode::Normal,
            filename: "test.txt".to_string(),
            contents: vec![],
            command: String::new(),
        }
    }

    fn generate_modeline(&self, s: &mut String) {
        if self.mode == Mode::Insert {
            s.push_str("--INSERT--");
        } else if self.mode == Mode::Command {
            s.push_str(&format!("{:<10}", self.command));
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
