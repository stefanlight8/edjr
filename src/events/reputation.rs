use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReputationEvent {
    pub alliance: Option<f64>,
    pub empire: Option<f64>,
    pub federation: Option<f64>,
    pub independent: Option<f64>,
}
