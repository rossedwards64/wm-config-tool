use std::{
    collections::HashMap,
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

pub fn get_key_value_from_line(
    line: &mut str,
    str_start: &String,
    str_split: &String,
    map: &mut HashMap<String, String>,
) {
    let (_, mapping) = line.split_at_mut(str_start.len());
    println!("{mapping}");
    match mapping.split_once(str_split) {
        Some((key, value)) => map.insert(key.to_string(), value.to_string()),
        None => panic!("Could not get variable from line."),
    };
}
