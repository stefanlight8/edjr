use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResurrectOption {
    Free,
    Handin,
    Rebuy,
    Recover,
    Rejoin,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResurrectEvent {
    cost: u64,
    option: ResurrectOption,
    #[serde(default)]
    bankrupt: bool,
}
