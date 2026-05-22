use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuyAmmoEvent {
    cost: u64,
}
