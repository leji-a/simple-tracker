use crate::model::WatchedEpisodes;
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

pub fn load_watched(path: &PathBuf) -> WatchedEpisodes {
    let file = File::open(path.join("watched.json")).ok();
    file.map(|f| serde_json::from_reader(BufReader::new(f)).unwrap_or_default())
        .unwrap_or_default()
}

pub fn save_watched(path: &PathBuf, watched: &WatchedEpisodes) {
    let file = File::create(path.join("watched.json")).unwrap();
    serde_json::to_writer_pretty(BufWriter::new(file), watched).unwrap();
}

pub fn get_episodes(path: &PathBuf) -> io::Result<Vec<String>> {
    let mut entries = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename != "watched.json" {
                entries.push(filename);
            }
        }
    }
    entries.sort();
    Ok(entries)
}
