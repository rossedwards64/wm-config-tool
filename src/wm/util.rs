use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

pub fn get_line(line: Result<String, io::Error>) -> String {
    match line {
        Ok(line) => line,
        Err(err) => panic!("Error getting line from file. {err}"),
    }
}

pub fn get_reader_from_path(path: &Path) -> BufReader<File> {
    match File::open(path) {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("Error opening file for reading. {err}"),
    }
}
