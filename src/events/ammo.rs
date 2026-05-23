use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuyAmmoEvent {
    pub cost: u64,
}
