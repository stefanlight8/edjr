use {
    crate::elite::material::{Material, TraderType},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialCollectedEvent {
    #[serde(flatten)]
    pub material: Material,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    #[serde(flatten)]
    pub material: Material,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub paid: Material,
    pub received: Material,
    pub trader_type: TraderType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialsEvent {
    pub encoded: Vec<Material>,
    pub manufactured: Vec<Material>,
    pub raw: Vec<Material>,
}
