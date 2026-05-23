use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelAllEvent {
    amount: f64,
    cost: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RefuelPartialEvent {
    amount: f64,
    cost: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FuelScoopEvent {
    scooped: f64,
    total: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReservoirReplenishedEvent {
    fuel_main: f64,
    fuel_reservoir: f64,
}
