use std::env::args;

use crate::wm::window_manager::WindowManager;

mod wm;

fn main() {
    let args = get_args();
    let mut wm = get_wm(&args);
    let vs = wm.read_variables();
    for (key, value) in vs {
        println!("{key}: {value}");
    }
    let bs = wm.read_bindings();
    for (key, value) in bs {
        println!("{key}: {value}");
    }
}

fn get_args() -> String {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        args[1].clone()
    } else {
        panic!("You must include the name of the window manager you want to check!")
    }
}

fn get_wm(wm_name: &str) -> WindowManager {
    match wm_name {
        "sway" | "Sway" => WindowManager::new("sway/config", "set ", "bindsym ", " "),
        "hypr" | "Hypr" => WindowManager::new("hypr/hyprland.conf", "$", "bind", "="),
        &_ => panic!("Unknown window manager specified"),
    }
}
