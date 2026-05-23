use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShieldStateEvent {
    shields_up: bool,
}
