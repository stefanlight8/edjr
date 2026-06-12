use {
    crate::elite::rank::{EmpireRank, FederationRank, Rank},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressEvent {
    #[serde(alias = "CQC")]
    pub cqc: u64,
    pub combat: u64,
    pub empire: u64,
    pub exobiologist: u64,
    pub explore: u64,
    pub federation: u64,
    pub soldier: u64,
    pub trade: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionEvent {
    pub combat: Option<Rank>,
    pub soldier: Option<Rank>,
    pub empire: Option<EmpireRank>,
    pub explore: Option<Rank>,
    pub exobiologist: Option<Rank>,
    pub federation: Option<FederationRank>,
    pub trade: Option<Rank>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RankEvent {
    #[serde(alias = "CQC")]
    pub cqc: Rank,
    pub combat: Rank,
    pub empire: Rank,
    pub exobiologist: Rank,
    pub explore: Rank,
    pub federation: Rank,
    pub soldier: Rank,
    pub trade: Rank,
}
