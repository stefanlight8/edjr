use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReputationEvent {
    alliance: Option<f64>,
    empire: Option<f64>,
    federation: Option<f64>,
    independent: Option<f64>,
}
