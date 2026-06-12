use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SrvType {
    #[serde(alias = "testbuggy")]
    Scarab,
    #[serde(alias = "combat_multicrew_srv_01")]
    Scorpio,
}
