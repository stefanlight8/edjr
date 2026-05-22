use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HullDamageEvent {
    #[serde(default)]
    fighter: bool,
    health: f64,
    player_pilot: bool,
}
