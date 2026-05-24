#[cfg(feature = "passengers")]
use crate::elite::passengers::PassengerType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mission {
    #[serde(alias = "MissionID")]
    pub mission_id: u64,
    pub name: String,
    #[serde(alias = "LocalisedName")]
    pub name_display: Option<String>,
    pub commodity: Option<String>,
    #[serde(alias = "Commodity_Localised")]
    pub commodity_display: Option<String>,
    pub count: Option<u64>,
    pub destination_settlement: Option<String>,
    pub destination_station: Option<String>,
    pub destination_system: Option<String>,
    pub donation: Option<String>,
    pub expiry: Option<String>,
    pub faction: Option<String>,
    pub influence: Option<String>,
    pub kill_count: Option<u64>,
    pub passenger_count: Option<u64>,
    #[cfg(feature = "passengers")]
    pub passenger_type: Option<PassengerType>,
    #[serde(alias = "PassengerVIPs")]
    pub passenger_vips: Option<bool>,
    pub passenger_wanted: Option<bool>,
    pub reputation: Option<String>,
    pub reward: Option<u64>,
    // target: Option<String>,
    //
    // "Target" key is sometimes duplicating in MissionAccepted event (or in every mission associated event)
    // and serde just can't parse this, so only what I can do is to ignore this field completely.
    #[serde(alias = "Target_Localised")]
    pub target_display: Option<String>,
    pub target_faction: Option<String>,
    pub target_type: Option<String>,
    #[serde(alias = "TargetType_Localised")]
    pub target_type_display: Option<String>,
    pub wing: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub enum Trend {
    DownBad,
    DownGood,
    UpGood,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Effect {
    pub effect: String,
    #[serde(alias = "Effect_Localised")]
    pub effect_display: String,
    pub trend: Trend,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Influence {
    pub influence: String,
    pub system_address: u64,
    pub trend: Trend,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionEffect {
    pub faction: String,
    pub effects: Vec<Effect>,
    pub influence: Vec<Influence>,
    pub reputation: String,
    pub reputation_trend: Trend,
}
