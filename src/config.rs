use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub folder_history: HashSet<String>,
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".config");
    path.push("tracker");
    fs::create_dir_all(&path).unwrap_or_default();
    path.push("config.json");
    path
}

pub fn load_config() -> Config {
    let path = get_config_path();
    let file = File::open(path).ok();
    file.map(|f| serde_json::from_reader(BufReader::new(f)).unwrap_or_default())
        .unwrap_or_default()
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let path = get_config_path();
    let file = File::create(path)?;
    serde_json::to_writer_pretty(BufWriter::new(file), config)?;
    Ok(())
}

pub fn add_to_history(folder: &str) -> io::Result<()> {
    let mut config = load_config();
    config.folder_history.insert(folder.to_string());
    save_config(&config)
} 