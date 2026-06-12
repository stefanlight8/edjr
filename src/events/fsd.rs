#[cfg(feature = "faction")]
use crate::elite::faction::{Faction, FactionConflict};
#[cfg(feature = "powerplay")]
use crate::elite::powerplay::Powerplay;
use {
    crate::elite::{body::BodyType, system::System},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJumpEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    pub boost_used: Option<u8>,
    #[cfg(feature = "faction")]
    pub conflicts: Option<Vec<FactionConflict>>,
    pub controlling_power: Option<String>,
    #[cfg(feature = "faction")]
    pub factions: Option<Vec<Faction>>, // TODO: add active states, pending states, my reputation, happinness and etc. to faction or make a separate faction object for fsd jump
    pub fuel_level: f64,
    pub jump_dist: f64,
    pub population: u64,
    #[serde(flatten)]
    #[cfg(feature = "powerplay")]
    pub powerplay: Option<Powerplay>,
    pub star_pos: [f64; 3],
    pub star_system: String,
    #[serde(flatten)]
    pub system: Option<System>,
    pub system_address: u64,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub taxi: bool,
    // TODO: ThargoidWar
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FsdTargetEvent {
    pub name: String,
    pub star_class: String,
    pub system_address: u64,
    pub remaining_jumps_in_route: Option<u64>,
}
