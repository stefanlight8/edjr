use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Progress {
    Known,
    Invited,
    Unlocked,
    #[serde(other)]
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineer {
    engineer: Option<String>,
    #[serde(alias = "EngineerID")]
    engineer_id: Option<u64>,
    rank: Option<u8>,
    rank_progress: Option<u64>,
    progress: Option<Progress>,
}
