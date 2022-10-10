use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

pub fn open_file(path: &str) -> io::Result<Vec<String>> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            let mut file = File::create(path).unwrap();
            file.write_all(b"\n").unwrap();
            file
        }
    };
    let reader = BufReader::new(f);

    let mut contents: Vec<String> = reader.lines().map(|res| res.unwrap()).collect();
    if contents.len() == 0 {
        contents.push(String::new());
    }

    Ok(contents)
}
