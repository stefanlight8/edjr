use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShieldStateEvent {
    pub shields_up: bool,
}
