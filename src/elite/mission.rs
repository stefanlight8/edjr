use {crate::elite::passenger::PassengerType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mission {
    #[serde(alias = "MissionID")]
    mission_id: u64,
    name: String,
    #[serde(alias = "LocalisedName")]
    name_display: Option<String>,
    commodity: Option<String>,
    #[serde(alias = "Commodity_Localised")]
    commodity_display: Option<String>,
    count: Option<u64>,
    destination_settlement: Option<String>,
    destination_station: Option<String>,
    destination_system: Option<String>,
    donation: Option<String>,
    expiry: Option<String>,
    faction: Option<String>,
    influence: Option<String>,
    kill_count: Option<u64>,
    passenger_count: Option<u64>,
    passenger_type: Option<PassengerType>,
    #[serde(alias = "PassengerVIPs")]
    passenger_vips: Option<bool>,
    passenger_wanted: Option<bool>,
    reputation: Option<String>,
    reward: Option<u64>,
    // target: Option<String>,
    //
    // "Target" key is sometimes duplicating in MissionAccepted event (or in every mission associated event)
    // and serde just can't parse this, so only what I can do is to ignore this field completely.
    #[serde(alias = "Target_Localised")]
    target_display: Option<String>,
    target_faction: Option<String>,
    target_type: Option<String>,
    #[serde(alias = "TargetType_Localised")]
    target_type_display: Option<String>,
    wing: Option<bool>,
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
    effect: String,
    #[serde(alias = "Effect_Localised")]
    effect_display: String,
    trend: Trend,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Influence {
    influence: String,
    system_address: u64,
    trend: Trend,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionEffect {
    faction: String,
    effects: Vec<Effect>,
    influence: Vec<Influence>,
    reputation: String,
    reputation_trend: Trend,
}
