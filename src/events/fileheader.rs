use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileheaderEvent {
    odyssey: bool,
    #[serde(alias = "build")]
    build: String,
    #[serde(alias = "gameversion")]
    game_version: String,
    #[serde(alias = "language")]
    language: String,
    #[serde(alias = "part")]
    part: u64,
}
