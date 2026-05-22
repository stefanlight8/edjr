use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayFinesEvent {
    all_fines: bool,
    amount: u64,
    broker_percentage: Option<f64>,
    faction: Option<String>,
    #[serde(alias = "ShipID")]
    ship_id: u64,
}
