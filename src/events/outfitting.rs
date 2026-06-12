use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub station_name: String,
}
