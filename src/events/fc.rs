use {
    crate::elite::{
        body::BodyType,
        faction::{Faction, FactionConflict},
        powerplay::Powerplay,
        station::Station,
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    pub conflicts: Option<Vec<FactionConflict>>,
    pub controlling_power: Option<String>,
    pub factions: Option<Vec<Faction>>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(flatten)]
    pub powerplay: Option<Powerplay>,
    pub star_pos: [f64; 3],
    pub star_system: String,
    #[serde(flatten)]
    pub station: Option<Station>,
    pub system_address: u64,
    #[serde(default)]
    pub docked: bool,
    #[serde(default)]
    pub taxi: bool,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub on_foot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FcMaterialsEvent {
    pub carrier_name: String,
    #[serde(alias = "CarrierID")]
    pub carrier_id: String,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
}
