use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LaunchDroneType {
    Collection,
    Repair,
    FuelTransfer,
}

#[derive(Debug, Deserialize)]
pub struct BuyDronesEvent {
    buy_price: u64,
    count: u64,
    total_cost: u64,
}

#[derive(Debug, Deserialize)]
pub struct LaunchDroneEvent {
    #[serde(alias = "Type")]
    launch_type: LaunchDroneType,
}
