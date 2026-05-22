use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClearImpoundEvent {
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    #[serde(alias = "ShipID")]
    ship_id: Option<u64>,
    #[serde(alias = "ShipMarketID")]
    ship_market_id: Option<u64>,
    ship_type: String,
    system: String,
}
