use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum CrewRole {
    FighterCon,
    FireCon,
    Helm,
    Idle,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Crew {
    name: String,
    role: CrewRole,
}
