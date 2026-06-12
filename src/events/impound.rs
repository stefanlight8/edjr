use {
    crate::elite::ship::Ship,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClearImpoundEvent {
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(flatten)]
    pub ship: Ship,
    #[serde(alias = "ShipMarketID")]
    pub ship_market_id: Option<u64>,
    pub system: String,
}
