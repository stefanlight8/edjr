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
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    conflicts: Option<Vec<FactionConflict>>,
    controlling_power: Option<String>,
    factions: Option<Vec<Faction>>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    #[serde(flatten)]
    powerplay: Option<Powerplay>,
    star_pos: [f64; 3],
    star_system: String,
    #[serde(flatten)]
    station: Option<Station>,
    system_address: u64,
    #[serde(default)]
    docked: bool,
    #[serde(default)]
    taxi: bool,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    on_foot: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FcMaterialsEvent {
    carrier_name: String,
    #[serde(alias = "CarrierID")]
    carrier_id: String,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
}
