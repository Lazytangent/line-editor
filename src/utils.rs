use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn open_file(path: &str) -> io::Result<Vec<String>> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let contents: Vec<String> = reader.lines().map(|res| res.unwrap()).collect();

    Ok(contents)
}
