use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Progress {
    Known,
    Invited,
    Unlocked,
    #[serde(other)]
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engineer {
    pub engineer: Option<String>,
    #[serde(alias = "EngineerID")]
    pub engineer_id: Option<u64>,
    pub rank: Option<u8>,
    pub rank_progress: Option<u64>,
    pub progress: Option<Progress>,
}
