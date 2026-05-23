use {
    crate::elite::signal::{Signal, SignalType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssAllBodiesFoundEvent {
    pub count: u64,
    pub system_address: u64,
    pub system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssBodySignalsEvent {
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_name: String,
    pub system_address: u64,
    pub signals: Vec<Signal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssDiscoveryScanEvent {
    pub body_count: u64,
    pub non_body_count: u64,
    pub progress: f64,
    pub system_address: u64,
    pub system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssSignalDiscoveredEvent {
    #[serde(default)]
    pub is_station: bool,
    pub opposing_power: Option<String>,
    pub signal_name: String,
    #[serde(alias = "SignalName_Localised")]
    pub signal_name_display: Option<String>,
    pub signal_type: Option<SignalType>,
    pub spawning_faction: Option<String>,
    #[serde(alias = "SpawningFaction_Localised")]
    pub spawning_faction_display: Option<String>,
    pub spawning_power: Option<String>,
    pub spawning_state: Option<String>,
    #[serde(alias = "SpawningState_Localised")]
    pub spawning_state_display: Option<String>,
    pub system_address: u64,
    pub threat_level: Option<u8>,
    pub time_remaining: Option<f64>,
    #[serde(alias = "USSType")]
    pub uss_type: Option<String>,
    #[serde(alias = "USSType_Localised")]
    pub uss_type_display: Option<String>,
}
