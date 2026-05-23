use {crate::elite::ship::Ship, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClearImpoundEvent {
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    #[serde(flatten)]
    ship: Ship,
    #[serde(alias = "ShipMarketID")]
    ship_market_id: Option<u64>,
    system: String,
}
