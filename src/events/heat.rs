use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeatDamageEvent;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HeatWarningEvent;
