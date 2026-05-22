use {
    crate::elite::{
        body::BodyType,
        faction::{Faction, FactionConflict},
        powerplay::Powerplay,
        station::Station,
        system::System,
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    conflicts: Option<Vec<FactionConflict>>,
    controlling_power: Option<String>,
    dist_from_star_ls: Option<f64>,
    factions: Option<Vec<Faction>>,
    #[serde(default, alias = "InSRV")]
    in_srv: bool,
    latitude: Option<f64>,
    longitude: Option<f64>,
    #[serde(default)]
    docked: bool,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    on_foot: bool,
    population: u64,
    #[serde(flatten)]
    powerplay: Option<Powerplay>,
    #[serde(flatten)]
    system: Option<System>,
    system_address: u64,
    #[serde(flatten)]
    station: Option<Station>,
    star_pos: [f64; 3],
    star_system: String,
    #[serde(default)]
    taxi: bool,
    #[serde(default)]
    wanted: bool,
    // TODO: thargoid war
}
