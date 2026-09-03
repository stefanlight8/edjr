use chrono::{DateTime, Utc};

#[cfg(feature = "faction")]
use crate::elite::faction::{Faction, FactionConflict};
use crate::elite::fc::{
    CarrierCrew, CarrierFinance, CarrierSpaceUsage, CarrierType, DockingAccess,
};
#[cfg(feature = "powerplay")]
use crate::elite::powerplay::Powerplay;
use {
    crate::elite::{body::BodyType, station::Station},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierDepositFuelEvent {
    pub amount: u16,
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub total: u16,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpRequestEvent {
    pub body: Option<String>,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: CarrierType,
    pub departure_time: DateTime<Utc>,
    pub system_address: u64,
    pub system_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierLocationEvent {
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: CarrierType,
    pub star_system: String,
    pub system_address: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CarrierStatsEvent {
    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
    pub carrier_type: CarrierType,
    pub callsign: String,
    pub allow_notorious: bool,
    pub crew: Vec<CarrierCrew>,
    pub docking_access: DockingAccess,
    pub finance: CarrierFinance,
    pub fuel_level: u16,
    pub jump_range_curr: u16,
    pub jump_range_max: u16,
    pub name: String,
    pub pending_decommision: bool,
    pub space_usage: CarrierSpaceUsage,
    // TODO: ModulePacks
    // TODO: ShipPacks
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FcMaterialsEvent {
    pub carrier_name: String,
    #[serde(alias = "CarrierID")]
    pub carrier_id: String,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
}
