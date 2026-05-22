use {
    crate::elite::rank::{EmpireRank, FederationRank, Rank},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressEvent {
    #[serde(alias = "CQC")]
    cqc: u64,
    combat: u64,
    empire: u64,
    exobiologist: u64,
    explore: u64,
    federation: u64,
    soldier: u64,
    trade: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionEvent {
    combat: Option<Rank>,
    soldier: Option<Rank>,
    empire: Option<EmpireRank>,
    explore: Option<Rank>,
    exobiologist: Option<Rank>,
    federation: Option<FederationRank>,
    trade: Option<Rank>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RankEvent {
    #[serde(alias = "CQC")]
    cqc: Rank,
    combat: Rank,
    empire: Rank,
    exobiologist: Rank,
    explore: Rank,
    federation: Rank,
    soldier: Rank,
    trade: Rank,
}
