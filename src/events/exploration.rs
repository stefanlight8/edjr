use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Discover {
    num_bodies: u64,
    system_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEvent {
    base_value: u64,
    bonus: u64,
    discovered: Vec<Discover>,
    total_earnings: u64,
}
