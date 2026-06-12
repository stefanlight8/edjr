use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LegalStatus {
    Clean,
    Enemy,
    Lawless,
    Wanted,
    WantedEnemy,
    Warrant,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pilot {
    pub pilot_name: String,
    pub pilot_name_display: String,
    pub pilot_rank: String,
    pub power: Option<String>,
    #[serde(alias = "Squadron_ID")]
    pub squadron_id: Option<String>,
    pub legal_status: LegalStatus,
    pub bounty: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Subsystem {
    pub subsystem: String,
    pub subsystem_health: f64,
    #[serde(alias = "Subsystem_Localised")]
    pub subsystem_display: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEvent {
    pub target_locked: bool,
    pub scan_stage: Option<u8>,
    #[serde(flatten)]
    pub pilot: Option<Pilot>,
    pub ship: Option<String>,
    #[serde(alias = "Ship_Localised")]
    pub ship_display: Option<String>,
    pub shield_health: Option<f64>,
    pub hull_health: Option<f64>,
    #[serde(flatten)]
    pub subsystem: Option<Subsystem>,
}
