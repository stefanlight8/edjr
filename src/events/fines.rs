use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayFinesEvent {
    pub all_fines: bool,
    pub amount: u64,
    pub broker_percentage: Option<f64>,
    pub faction: Option<String>,
    #[serde(alias = "ShipID")]
    pub ship_id: u64,
}
