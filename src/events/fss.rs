use {
    crate::elite::signal::{Signal, SignalType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssAllBodiesFoundEvent {
    count: u64,
    system_address: u64,
    system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssBodySignalsEvent {
    #[serde(alias = "BodyID")]
    body_id: u64,
    body_name: String,
    system_address: u64,
    signals: Vec<Signal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssDiscoveryScanEvent {
    body_count: u64,
    non_body_count: u64,
    progress: f64,
    system_address: u64,
    system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FssSignalDiscoveredEvent {
    #[serde(default)]
    is_station: bool,
    opposing_power: Option<String>,
    signal_name: String,
    #[serde(alias = "SignalName_Localised")]
    signal_name_display: Option<String>,
    signal_type: Option<SignalType>,
    spawning_faction: Option<String>,
    #[serde(alias = "SpawningFaction_Localised")]
    spawning_faction_display: Option<String>,
    spawning_power: Option<String>,
    spawning_state: Option<String>,
    #[serde(alias = "SpawningState_Localised")]
    spawning_state_display: Option<String>,
    system_address: u64,
    threat_level: Option<u8>,
    time_remaining: Option<f64>,
    #[serde(alias = "USSType")]
    uss_type: Option<String>,
    #[serde(alias = "USSType_Localised")]
    uss_type_display: Option<String>,
}
