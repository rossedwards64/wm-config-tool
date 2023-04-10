use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// create a hashmap with the variable name as the key and the variable as the value
pub fn get_variables(wm_config: &Path) -> HashMap<String, String> {
    let reader = get_reader_from_path(wm_config);
    let mut map = HashMap::new();
    // get modifier key, then get keybinds
    reader
        .lines()
        .map(get_line)
        .filter(|s| s.starts_with("set"))
        .for_each(|l| add_variable(l, &mut map));
    map
}

fn add_variable(mut line: String, map: &mut HashMap<String, String>) {
    let (_, variable) = line.split_at_mut(5);
    match variable.split_once(' ') {
        Some((key, value)) => map.insert(key.to_string(), value.to_string()),
        None => panic!("Could not get variable from line."),
    };
}

fn get_line(line: Result<String, io::Error>) -> String {
    match line {
        Ok(line) => line,
        Err(err) => panic!("Error getting line from file. {err}"),
    }
}

fn get_reader_from_path(path: &Path) -> BufReader<File> {
    match File::open(path) {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("Error opening file for reading. {err}"),
    }
}
