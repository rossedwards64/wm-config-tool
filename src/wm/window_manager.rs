use std::{collections::HashMap, io::BufRead, path::PathBuf};

use crate::wm::util;

pub struct WindowManager {
    config_path: PathBuf,
    var_stmt_start: String,
    bind_stmt_start: String,
    cmd_stmt_split: String,
    variables: HashMap<String, String>,
    keybinds: HashMap<String, String>,
}

impl WindowManager {
    pub fn new(
        config_path: &str,
        var_stmt_start: &str,
        bind_stmt_start: &str,
        cmd_stmt_split: &str,
    ) -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| panic!("Config file does not exist!"))
            .join(config_path);
        let var_stmt_start = var_stmt_start.to_string();
        let bind_stmt_start = bind_stmt_start.to_string();
        let cmd_stmt_split = cmd_stmt_split.to_string();

        Self {
            config_path,
            var_stmt_start,
            bind_stmt_start,
            cmd_stmt_split,
            variables: HashMap::new(),
            keybinds: HashMap::new(),
        }
    }

    pub fn read_variables(&mut self) -> &HashMap<String, String> {
        let reader = util::get_reader_from_path(&self.config_path);
        reader
            .lines()
            .map(util::get_line)
            .filter(|s| s.starts_with(&self.var_stmt_start))
            .for_each(|mut l| {
                util::get_key_value_from_line(
                    &mut l,
                    &self.var_stmt_start,
                    &self.cmd_stmt_split,
                    &mut self.variables,
                );
            });
        &self.variables
    }

    /* TODO: parsing hypr keybinds takes a bit more fenagling.
     *       after bind=, the modifier, key, action and command are
     *       separated by a delimiter.
     *       there are several possible characters that can be used;
     *       when parsing find out what the first delimiter is and
     *       use it for the rest of the line */
    pub fn read_bindings(&mut self) -> &HashMap<String, String> {
        let reader = util::get_reader_from_path(&self.config_path);
        reader
            .lines()
            .map(util::get_line)
            .filter(|s| s.starts_with(&self.bind_stmt_start))
            .for_each(|mut l| {
                util::get_key_value_from_line(
                    &mut l,
                    &self.bind_stmt_start,
                    &self.cmd_stmt_split,
                    &mut self.keybinds,
                );
            });
        &self.keybinds
    }
}
