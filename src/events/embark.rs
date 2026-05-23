use {
    crate::elite::{crew::Crew, station::StationType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmbarkEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub crew: Option<Vec<Crew>>,
    #[serde(alias = "ID")]
    pub id: Option<u64>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub star_system: String,
    pub station_name: Option<String>,
    pub station_type: Option<StationType>,
    pub system_address: u64,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub on_planet: bool,
    #[serde(default)]
    pub on_station: bool,
    #[serde(default, alias = "SRV")]
    pub srv: bool,
    #[serde(default)]
    pub taxi: bool,
}
