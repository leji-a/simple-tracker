mod app;
mod config;
mod files;
mod model;

use crate::app::{run_app, run_history_menu};
use crate::config::{add_to_history, load_config};
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

fn launch_tracker(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let folder = PathBuf::from(folder);
    if let Err(e) = add_to_history(folder.to_str().unwrap_or_default()) {
        eprintln!("Warning: Could not save folder to history: {}", e);
    }

    let mut watched = load_watched(&folder);
    let mut episodes = get_episodes(&folder)?;

    run_app(&mut episodes, &mut watched)?;

    save_watched(&folder, &watched);
    clear_screen();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    
    match env::args().nth(1) {
        Some(folder_path) => {
            // Direct folder tracking mode
            clear_screen();
            launch_tracker(&folder_path)?;
        }
        None => {
            // History menu mode
            let config = load_config();
            if let Some(folder) = run_history_menu(&config.folder_history)? {
                clear_screen();
                launch_tracker(&folder)?;
            }
        }
    }

    Ok(())
}

