use std::{collections::HashMap, io::BufRead, path::PathBuf};

use crate::wm::util;

pub struct WindowManager {
    config_path: PathBuf,
    command_stmt_start: String,
    command_stmt_split: String,
    variables: HashMap<String, String>,
    keybinds: HashMap<String, String>,
}

impl WindowManager {
    // hypr: variables start with $, binds start with bind*
    // split on '='
    // sway: variables start with set, binds start with bindsym
    // split on ' '
    pub fn new(config_path: &str, command_stmt_start: &str, command_stmt_split: &str) -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| panic!("Config file does not exist!"))
            .join(config_path);
        let command_stmt_start = command_stmt_start.to_string();
        let command_stmt_split = command_stmt_split.to_string();

        Self {
            config_path,
            command_stmt_start,
            command_stmt_split,
            variables: HashMap::new(),
            keybinds: HashMap::new(),
        }
    }

    pub fn read_variables(&mut self) -> &HashMap<String, String> {
        let reader = util::get_reader_from_path(&self.config_path);
        reader
            .lines()
            .map(util::get_line)
            .filter(|s| s.starts_with(&self.command_stmt_start))
            .for_each(|mut l| {
                let (_, variable) = l.split_at_mut(self.command_stmt_start.len());
                match variable.split_once(&self.command_stmt_split) {
                    Some((key, value)) => self.variables.insert(key.to_string(), value.to_string()),
                    None => panic!("Could not get variable from line."),
                };
            });
        &self.variables
    }
}
