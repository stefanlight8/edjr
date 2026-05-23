use {crate::elite::genus::Genus, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SaaSignalType {
    #[serde(alias = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(alias = "$SAA_SignalType_Geological;")]
    Geological,
    #[serde(alias = "$SAA_SignalType_Human;")]
    Human,
    #[serde(alias = "$SAA_SignalType_Guardian;")]
    Guardian, // I don't have it scheme, but I guess, TODO: check
    #[serde(alias = "$SAA_SignalType_Thargoid;")]
    Thargoid,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaaSignal {
    count: u64,
    #[serde(alias = "Type")]
    signal_type: SaaSignalType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenusSignal {
    genus: Genus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaaScanCompleteEvent {
    body_name: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    efficiency_target: u64,
    probes_used: u64,
    system_address: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaaSignalsFoundEvent {
    body_name: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    genuses: Vec<GenusSignal>,
    signals: Vec<SaaSignal>,
    system_address: u64,
}
