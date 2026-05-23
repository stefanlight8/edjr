use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelAllEvent {
    pub amount: f64,
    pub cost: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelPartialEvent {
    pub amount: f64,
    pub cost: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelScoopEvent {
    pub scooped: f64,
    pub total: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReservoirReplenishedEvent {
    pub fuel_main: f64,
    pub fuel_reservoir: f64,
}
