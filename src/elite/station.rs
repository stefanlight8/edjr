use {
    crate::elite::{
        allegiance::Allegiance, economy::Economy, faction::Faction, goverment::Goverment,
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationEconomy {
    #[serde(alias = "Name")] // a bit hacky
    economy_type: Economy,
    proportion: f64,
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
    station_allegiance: Allegiance,
    station_economies: Vec<StationEconomy>,
    station_economy: Economy,
    system_second_economy: Option<Economy>,
    station_faction: Faction,
    station_goverment: Goverment,
    station_services: Vec<StationService>,
    station_state: Option<StationState>,
    station_type: Option<StationType>,
    system_security: Option<String>,
    #[serde(alias = "SystemSecurity_Localised")]
    system_security_display: Option<String>,
}
