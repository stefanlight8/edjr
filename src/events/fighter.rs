use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FighterDestroyedEvent {
    #[serde(alias = "ID")]
    id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    #[serde(alias = "ID")]
    id: u64,
    loadout: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockFighterEvent {
    #[serde(alias = "ID")]
    id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    #[serde(alias = "ID")]
    id: u64,
    loadout: String,
    player_controlled: bool,
}
