use {crate::elite::station::Station, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBodyEvent {
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    star_system: String,
    system_address: u64,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachSettlementEvent {
    #[serde(alias = "BodyID")]
    body_id: u64,
    body_name: String,
    latitude: f64,
    longitude: f64,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
    #[serde(flatten)]
    station: Option<Station>,
    system_address: u64,
}
