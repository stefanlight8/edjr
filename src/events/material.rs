use {
    crate::elite::material::{Material, TraderType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialCollectedEvent {
    #[serde(flatten)]
    material: Material,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    #[serde(flatten)]
    material: Material,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    paid: Material,
    received: Material,
    trader_type: TraderType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialsEvent {
    encoded: Vec<Material>,
    manufactured: Vec<Material>,
    raw: Vec<Material>,
}
