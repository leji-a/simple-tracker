use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct WatchedEpisodes(pub HashMap<String, bool>);
