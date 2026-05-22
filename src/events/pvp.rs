use {crate::elite::rank::EmpireRank, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PvpKillEvent {
    combat_rank: EmpireRank,
    victim: String,
}
