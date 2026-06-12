use {
    crate::elite::{
        material::Material,
        mission::{FactionEffect, Mission},
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAbandonedEvent {
    #[serde(flatten)]
    pub mission: Mission,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionAcceptedEvent {
    #[serde(flatten)]
    pub mission: Mission,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionCompletedEvent {
    #[serde(flatten)]
    pub mission: Mission,
    pub donated: Option<u64>,
    pub faction_effects: Vec<FactionEffect>,
    pub materials_reward: Option<Vec<Material>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionFailedEvent {
    #[serde(flatten)]
    pub mission: Mission,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionRedirectedEvent {
    #[serde(flatten)]
    pub mission: Mission,
    pub new_destination_station: String,
    pub new_destination_system: String,
    pub old_destination_station: String,
    pub old_destination_system: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MissionsEvent {
    pub active: Option<Vec<Mission>>,
    pub complete: Option<Vec<Mission>>,
    pub failed: Option<Vec<Mission>>,
}
