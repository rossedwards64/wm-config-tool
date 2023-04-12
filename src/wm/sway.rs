use std::{collections::HashMap, io::BufRead, path::PathBuf};

use crate::wm::{util, window_manager::WindowManager};

pub struct SwayWm {
    config_path: PathBuf,
    variables: HashMap<String, String>,
    keybinds: HashMap<String, String>,
}

impl WindowManager for SwayWm {
    fn new() -> Self
    where
        Self: Sized,
    {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| panic!("Config file does not exist!"))
            .join("sway/config");
        let mut wm = Self {
            config_path,
            variables: HashMap::new(),
            keybinds: HashMap::new(),
        };
        let reader = util::get_reader_from_path(&wm.config_path);
        reader
            .lines()
            .map(util::get_line)
            .filter(|s| s.starts_with("set"))
            .for_each(|l| wm.add_variable(l));
        wm
    }

    fn get_variables(&self) -> &HashMap<String, String> {
        &self.keybinds
    }

    fn add_variable(&mut self, mut line: String) {
        let (_, variable) = line.split_at_mut(5);
        match variable.split_once(' ') {
            Some((key, value)) => self.variables.insert(key.to_string(), value.to_string()),
            None => panic!("Could not get variable from line."),
        };
    }

    fn get_keybinds(&self) -> &HashMap<String, String> {
        todo!()
    }
}
