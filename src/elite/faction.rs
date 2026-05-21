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
