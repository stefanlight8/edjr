use serde::Deserialize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pilot {
    pilot_name: String,
    pilot_name_display: String,
    pilot_rank: String,
    power: String,
    #[serde(alias = "Squadron_ID")]
    squadron_id: Option<String>,
    legal_status: LegalStatus,
    bounty: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Subsystem {
    subsystem: String,
    subsystem_health: f64,
    #[serde(alias = "Subsystem_Localised")]
    subsystem_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipTargetedEvent {
    target_locked: bool,
    scan_stage: Option<u8>,
    #[serde(flatten)]
    pilot: Option<Pilot>,
    ship: Option<String>,
    #[serde(alias = "Ship_Localised")]
    ship_display: Option<String>,
    shield_health: Option<f64>,
    hull_health: Option<f64>,
    #[serde(flatten)]
    subsystem: Option<Subsystem>,
}
