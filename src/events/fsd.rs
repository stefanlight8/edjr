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
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    boost_used: Option<u8>,
    conflicts: Option<Vec<FactionConflict>>,
    controlling_power: Option<String>,
    factions: Option<Vec<Faction>>, // TODO: add active states, pending states, my reputation, happinness and etc. to faction or make a separate faction object for fsd jump
    fuel_level: f64,
    jump_dist: f64,
    population: u64,
    #[serde(flatten)]
    powerplay: Option<Powerplay>,
    star_pos: [f64; 3],
    star_system: String,
    #[serde(flatten)]
    system: Option<System>,
    system_address: u64,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    taxi: bool,
    // TODO: ThargoidWar
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FsdTargetEvent {
    name: String,
    star_class: String,
    system_address: u64,
    remaining_jumps_in_route: Option<u64>,
}
