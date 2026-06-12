use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HullDamageEvent {
    #[serde(default)]
    pub fighter: bool,
    pub health: f64,
    pub player_pilot: bool,
}
