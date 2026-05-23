use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FighterDestroyedEvent {
    #[serde(alias = "ID")]
    pub id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    #[serde(alias = "ID")]
    pub id: u64,
    pub loadout: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockFighterEvent {
    #[serde(alias = "ID")]
    pub id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    #[serde(alias = "ID")]
    pub id: u64,
    pub loadout: String,
    pub player_controlled: bool,
}
