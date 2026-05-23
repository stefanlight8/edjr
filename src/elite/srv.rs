use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum SrvType {
    #[serde(alias = "testbuggy")]
    Scarab,
    #[serde(alias = "combat_multicrew_srv_01")]
    Scorpio,
}
