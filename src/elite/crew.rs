use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CrewRole {
    FighterCon,
    FireCon,
    Helm,
    Idle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Crew {
    pub name: String,
    pub role: CrewRole,
}
