use {
    crate::elite::genus::Genus,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaaSignal {
    pub count: u64,
    #[serde(alias = "Type")]
    pub signal_type: SaaSignalType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenusSignal {
    pub genus: Genus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaaScanCompleteEvent {
    pub body_name: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub efficiency_target: u64,
    pub probes_used: u64,
    pub system_address: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SaaSignalsFoundEvent {
    pub body_name: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub genuses: Vec<GenusSignal>,
    pub signals: Vec<SaaSignal>,
    pub system_address: u64,
}
