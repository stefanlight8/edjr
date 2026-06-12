use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LaunchDroneType {
    Collection,
    Repair,
    FuelTransfer,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchDroneEvent {
    #[serde(alias = "Type")]
    pub launch_type: LaunchDroneType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuyDronesEvent {
    pub buy_price: u64,
    pub count: u64,
    pub total_cost: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SellDronesEvent {
    pub sell_price: u64,
    pub count: u64,
    pub total_sale: u64,
}
