use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResurrectOption {
    Free,
    Handin,
    Rebuy,
    Recover,
    Rejoin,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResurrectEvent {
    pub cost: u64,
    pub option: ResurrectOption,
    #[serde(default)]
    pub bankrupt: bool,
}
