use std::collections::HashMap;

pub trait WindowManager {
    fn new() -> Self
    where
        Self: Sized;
    fn get_variables(&self) -> &HashMap<String, String>;
    fn add_variable(&mut self, line: String);
    fn get_keybinds(&self) -> &HashMap<String, String>;
}
