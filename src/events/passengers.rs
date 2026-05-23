use {crate::elite::passenger::PassengerType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersManifest {
    #[serde(alias = "MissionID")]
    pub mission_id: u64,
    pub count: u64,
    #[serde(alias = "Type")]
    pub passengers_type: PassengerType,
    #[serde(default, alias = "VIP")]
    pub vip: bool,
    #[serde(default)]
    pub wanted: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersEvent {
    pub manifest: Vec<PassengersManifest>,
}
