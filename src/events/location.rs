#[cfg(feature = "faction")]
use crate::elite::faction::{Faction, FactionConflict};
#[cfg(feature = "powerplay")]
use crate::elite::powerplay::Powerplay;
use {
    crate::elite::{body::BodyType, station::Station, system::System},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(flatten)]
    #[cfg(feature = "powerplay")]
    pub powerplay: Option<Powerplay>,
    #[serde(flatten)]
    pub system: Option<System>,
    #[serde(flatten)]
    pub station: Option<Station>,
    #[cfg(feature = "faction")]
    pub factions: Option<Vec<Faction>>,
    #[cfg(feature = "faction")]
    pub conflicts: Option<Vec<FactionConflict>>,
    pub controlling_power: Option<String>,
    pub dist_from_star_ls: Option<f64>,
    pub population: u64,
    pub star_pos: [f64; 3],
    pub star_system: String,
    pub system_address: u64,
    #[serde(default, alias = "InSRV")]
    pub in_srv: bool,
    #[serde(default)]
    pub docked: bool,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub on_foot: bool,
    #[serde(default)]
    pub taxi: bool,
    #[serde(default)]
    pub wanted: bool,
    // TODO: thargoid war
}
