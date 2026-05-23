use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum FactionState {
    Boom,
    Bust,
    Election,
    Expansion,
    Investment,
    None,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub name: String,
    pub faction_state: Option<FactionState>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FactionWarStatus {
    Active,
    Pending,
    #[serde(other)]
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FactionWarType {
    War,
    CivilWar,
    Election,
    #[serde(other)]
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionWarState {
    pub name: String,
    pub stake: String,
    pub won_days: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflict {
    pub faction_1: FactionWarState,
    pub faction_2: FactionWarState,
    pub status: FactionWarStatus,
    pub war_type: FactionWarType,
}
