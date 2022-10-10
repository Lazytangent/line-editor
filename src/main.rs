use line_editor::{model::{Model, Mode}, utils};

fn main() {
    let mut app = Model {
        line: 1,
        column: 1,
        mode: Mode::Normal,
        filename: "src/main.rs".to_string(),
        contents: vec![],
        command: String::new(),
    };

    let contents = match utils::open_file(&app.filename) {
        Ok(c) => c,
        Err(_) => unreachable!(),
    };

    app.contents = contents;

    rustea::run(app).unwrap();
}
