use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum JumpType {
    Hyperspace,
    Supercruise,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartJumpEvent {
    jump_type: JumpType,
    star_system: Option<String>,
    star_class: Option<String>,
    system_address: Option<u64>,
    #[serde(default)]
    taxi: bool,
}
