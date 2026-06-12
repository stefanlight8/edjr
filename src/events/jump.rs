use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum JumpType {
    Hyperspace,
    Supercruise,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartJumpEvent {
    pub jump_type: JumpType,
    pub star_system: Option<String>,
    pub star_class: Option<String>,
    pub system_address: Option<u64>,
    #[serde(default)]
    pub taxi: bool,
}
