use {
    crate::elite::{body::BodyType, station::Station},
    serde::Deserialize,
};

#[cfg(feature = "faction")]
use crate::elite::faction::{Faction, FactionConflict};

#[cfg(feature = "powerplay")]
use crate::elite::powerplay::Powerplay;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    #[cfg(feature = "faction")]
    pub factions: Option<Vec<Faction>>,
    #[cfg(feature = "faction")]
    pub conflicts: Option<Vec<FactionConflict>>,
    pub controlling_power: Option<String>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(flatten)]
    #[cfg(feature = "powerplay")]
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
