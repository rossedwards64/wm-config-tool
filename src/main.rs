use std::env::args;
use wm::{sway::SwayWm, hypr::HyprWm};

use crate::wm::window_manager::WindowManager;

mod wm;

fn main() {
    let args = get_args();
    let wm = get_wm(&args);

    let variables = wm.get_variables();
    for (name, value) in variables {
        println!("{name}: {value}");
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

fn get_wm(wm_name: &str) -> Box<dyn WindowManager> {
    match wm_name {
        "sway" | "Sway" => Box::new(SwayWm::new()),
        "hypr" | "Hypr" => Box::new(HyprWm::new()),
        &_ => panic!("Unknown window manager specified")
    }
}
