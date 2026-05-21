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
    name: String,
    faction_state: Option<FactionState>,
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
    name: String,
    stake: String,
    won_days: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionConflict {
    faction_1: FactionWarState,
    faction_2: FactionWarState,
    status: FactionWarStatus,
    war_type: FactionWarType,
}
