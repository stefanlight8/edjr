use {
    crate::elite::{crew::Crew, station::StationType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmbarkEvent {
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    crew: Option<Vec<Crew>>,
    #[serde(alias = "ID")]
    id: Option<u64>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    multicrew: bool,
    on_planet: bool,
    on_station: bool,
    #[serde(alias = "SRV")]
    srv: bool,
    star_system: String,
    station_name: Option<String>,
    station_type: Option<StationType>,
    system_address: u64,
    taxi: bool,
}
