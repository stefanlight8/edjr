#[cfg(feature = "faction")]
use crate::elite::faction::Faction;
use {
    crate::elite::{allegiance::Allegiance, economy::Economy, goverment::Goverment},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationEconomy {
    #[serde(alias = "Name")] // a bit hacky
    pub economy_type: Economy,
    pub proportion: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StationService {
    Dock,
    Autodock,
    BlackMarket,
    Commodities,
    Contacts,
    Exploration,
    Rearm,
    Missions,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub enum StationType {
    Coriolis,
    Dodec,
    Orbis,
    Ocellus,
    Outpost,
    CraterOutpost,
    CraterPort,
    SurfaceStation,
    OnFootSettlement,
    MegaShip,
    FleetCarrier,
    Bernal,
    AsteroidBase,
    PlanetaryConstructionDepot,
    SpaceConstructionDepot,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub enum StationState {
    Construction,
    UnderAttack,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    pub station_name: String,
    pub station_allegiance: Allegiance,
    pub station_economies: Vec<StationEconomy>,
    pub station_economy: Economy,
    pub system_second_economy: Option<Economy>,
    #[cfg(feature = "faction")]
    pub station_faction: Faction,
    pub station_goverment: Goverment,
    pub station_services: Vec<StationService>,
    pub station_state: Option<StationState>,
    pub station_type: Option<StationType>,
    pub system_security: Option<String>,
    #[serde(alias = "SystemSecurity_Localised")]
    pub system_security_display: Option<String>,
}
