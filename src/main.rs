mod app;
mod files;
mod model;

use crate::app::run_app;
use crate::files::{get_episodes, load_watched, save_watched};
use std::process::Command;
use std::{env, path::PathBuf};

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    let folder = if let Some(arg) = env::args().nth(1) {
        PathBuf::from(arg)
    } else {
        eprintln!("Usage: tracker <folder_path>");
        std::process::exit(1);
    };

    let mut watched = load_watched(&folder);
    let mut episodes = get_episodes(&folder)?;

    run_app(&mut episodes, &mut watched)?;

    save_watched(&folder, &watched);
    clear_screen();
    Ok(())
}

