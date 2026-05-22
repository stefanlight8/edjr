use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    star_system: String,
    station_name: String,
}
