use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayConflict {
    power: String,
    conflict_progress: f64,
}

#[derive(Debug, Deserialize, Default)]
pub enum PowerplayState {
    Exploited,
    #[default]
    Unoccupied,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Powerplay {
    powerplay_conflict_progress: Vec<PowerplayConflict>,
    powerplay_state: PowerplayState,
    powerplay_state_control_progress: f64,
    powerplay_state_reinforcement: u64,
    powerplay_state_undermining: u64,
    powers: Vec<String>,
}
