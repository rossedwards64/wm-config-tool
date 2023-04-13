use std::env::args;

use crate::wm::window_manager::WindowManager;

mod wm;

fn main() {
    let args = get_args();
    let mut wm = get_wm(&args);
    let vs = wm.read_variables();
    for (key, value) in vs {
        println!("{}: {}", key, value);
    }
}

fn get_args() -> String {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("You must include the name of the window manager you want to check!")
    } else {
        args[1].clone()
    }
}

fn get_wm(wm_name: &str) -> WindowManager {
    match wm_name {
        "sway" | "Sway" => WindowManager::new("sway/config", "set ", " "),
        "hypr" | "Hypr" => WindowManager::new("hypr/hyprland.conf", "$", "="),
        &_ => panic!("Unknown window manager specified"),
    }
}
