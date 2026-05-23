use {
    crate::elite::{
        body::BodyType,
        faction::{Faction, FactionConflict},
        powerplay::Powerplay,
        system::System,
    },
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FsdJumpEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    pub boost_used: Option<u8>,
    pub conflicts: Option<Vec<FactionConflict>>,
    pub controlling_power: Option<String>,
    pub factions: Option<Vec<Faction>>, // TODO: add active states, pending states, my reputation, happinness and etc. to faction or make a separate faction object for fsd jump
    pub fuel_level: f64,
    pub jump_dist: f64,
    pub population: u64,
    #[serde(flatten)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FsdTargetEvent {
    pub name: String,
    pub star_class: String,
    pub system_address: u64,
    pub remaining_jumps_in_route: Option<u64>,
}
