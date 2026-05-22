use {crate::elite::passenger::PassengerType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersManifest {
    #[serde(alias = "MissionID")]
    mission_id: u64,
    count: u64,
    #[serde(alias = "Type")]
    passengers_type: PassengerType,
    #[serde(default, alias = "VIP")]
    vip: bool,
    #[serde(default)]
    wanted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersEvent {
    manifest: Vec<PassengersManifest>,
}
