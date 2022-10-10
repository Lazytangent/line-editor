use line_editor::{model::Model, utils};

fn main() {
    let mut app = Model::new();

    let contents = match utils::open_file(&app.filename) {
        Ok(c) => c,
        Err(_) => unreachable!(),
    };

    app.contents = contents;

    rustea::run(app).unwrap();
}
