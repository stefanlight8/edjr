use {crate::elite::station::StationType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisembarkEvent {
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    #[serde(alias = "ID")]
    id: Option<u64>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    star_system: String,
    station_name: Option<String>,
    station_type: Option<StationType>,
    system_address: u64,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    on_planet: bool,
    #[serde(default)]
    on_station: bool,
    #[serde(default, alias = "SRV")]
    srv: bool,
    #[serde(default)]
    taxi: bool,
}
