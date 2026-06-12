use {
    crate::elite::rank::Rank,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PvpKillEvent {
    pub combat_rank: Rank,
    pub victim: String,
}
