use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signal {
    #[serde(alias = "Type")]
    signal_type: String,
    #[serde(alias = "Type_Localised")]
    signal_type_display: String,
}

#[derive(Debug, Deserialize)]
pub enum SignalType {
    Codex,
    Combat,
    FleetCarrier,
    Generic,
    Installation,
    Megaship,
    NavBeacon,
    Outpost,
    ResourceExtraction,
    SquadronCarrier,
    StationAsteroid,
    StationBernalSphere,
    StationCoriolis,
    StationDodec,
    StationMegaShip,
    StationONeilCylinder,
    StationONeilOrbis,
    Titan,
    TouristBeacon,
    #[serde(alias = "USS")]
    Uss,
}
