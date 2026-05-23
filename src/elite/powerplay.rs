use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayConflict {
    pub power: String,
    pub conflict_progress: f64,
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
    pub powerplay_conflict_progress: Vec<PowerplayConflict>,
    pub powerplay_state: PowerplayState,
    pub powerplay_state_control_progress: f64,
    pub powerplay_state_reinforcement: u64,
    pub powerplay_state_undermining: u64,
    pub powers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub enum PowerMicroResouceCategory {
    Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerMicroResource {
    pub category: PowerMicroResouceCategory,
    pub count: u64,
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: String,
}
