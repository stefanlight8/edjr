use {crate::elite::rank::Rank, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PvpKillEvent {
    pub combat_rank: Rank,
    pub victim: String,
}
