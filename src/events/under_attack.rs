use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Target {
    You,
    Mothership,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnderAttackEvent {
    target: Target,
}
