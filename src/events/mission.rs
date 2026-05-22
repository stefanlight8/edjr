use {
    crate::elite::{
        material::Material,
        mission::{FactionEffect, Mission},
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    #[serde(flatten)]
    mission: Mission,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAcceptedEvent {
    #[serde(flatten)]
    mission: Mission,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEvent {
    #[serde(flatten)]
    mission: Mission,
    donated: Option<u64>,
    faction_effects: Vec<FactionEffect>,
    materials_reward: Option<Vec<Material>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionFailedEvent {
    #[serde(flatten)]
    mission: Mission,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionRedirectedEvent {
    #[serde(flatten)]
    mission: Mission,
    new_destination_station: String,
    new_destination_system: String,
    old_destination_station: String,
    old_destination_system: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionsEvent {
    active: Option<Vec<Mission>>,
    complete: Option<Vec<Mission>>,
    failed: Option<Vec<Mission>>,
}
