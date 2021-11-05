use std::path::PathBuf;

use dirs_next;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
pub struct Config {
    pub local: Local,
    pub google_drive: Option<GoogleDrive>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Local {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleDrive {}

impl Default for Config {
    fn default() -> Self {
        let mut path = dirs_next::home_dir().unwrap();
        path.push(".rpswd");
        Config {
            local: Local { path },
            google_drive: None,
        }
    }
}
