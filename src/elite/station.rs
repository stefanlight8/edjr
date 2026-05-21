use {
    crate::elite::{allegiance::Allegiance, faction::Faction},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StationEconomy {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
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
    station_economy: String,
    #[serde(alias = "SystemEconomy_Localised")]
    station_economy_display: String,
    system_second_economy: Option<String>,
    #[serde(alias = "SystemSecondEconomy_Localised")]
    system_second_economy_display: Option<String>,
    station_faction: Faction,
    station_goverment: String,
    #[serde(alias = "SystemGoverment_Localised")]
    station_goverment_localised: String,
    station_services: Vec<StationService>,
    station_state: Option<StationState>,
    station_type: Option<StationType>,
    system_security: Option<String>,
    #[serde(alias = "SystemSecurity_Localised")]
    system_security_display: Option<String>,
}
