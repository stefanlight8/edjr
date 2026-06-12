use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileheaderEvent {
    pub odyssey: bool,
    #[serde(alias = "build")]
    pub build: String,
    #[serde(alias = "gameversion")]
    pub game_version: String,
    #[serde(alias = "language")]
    pub language: String,
    #[serde(alias = "part")]
    pub part: u64,
}
