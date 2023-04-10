use std::{
    env::args,
    path::{Path, PathBuf},
};

use parser::config_parser;

mod parser;

// only doing sway and hypr at the moment,
// because that's what i use
pub enum WindowManager {
    Sway,
    Hypr,
}

fn main() {
    let args = get_args();
    let wm = get_wm(&args);
    let wm_config = search_for_config(&wm);
    let variables = config_parser::get_variables(&wm_config);
    for (name, value) in variables {
        println!("{name}: {value}");
    }
}

fn search_for_config(wm: &WindowManager) -> PathBuf {
    let config_path = match wm {
        WindowManager::Hypr => Path::new("/home/ross/.config/hypr/hyprland.conf").canonicalize(),
        WindowManager::Sway => Path::new("/home/ross/.config/sway/config").canonicalize(),
    };
    config_path.unwrap_or_else(|e| panic!("Config file does not exist! {e}"))
}

fn get_wm(wm: &str) -> WindowManager {
    match wm {
        "hypr" | "Hypr" => WindowManager::Hypr,
        "sway" | "Sway" => WindowManager::Sway,
        &_ => panic!("Unknown window manager supplied."),
    }
}

fn get_args() -> String {
    let args: Vec<String> = args().collect();
    if args[1].is_empty() {
        panic!("You must include the name of the window manager you want to check!")
    } else {
        args[1].clone()
    }
}
