use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    local: Local,
    google_drive: Option<GoogleDrive>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Local {
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleDrive {
    name: String,
    comfy: bool,
    foo: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            local: Local {
                path: String::from("~/.passwords"),
            },
            google_drive: None,
        }
    }
}
