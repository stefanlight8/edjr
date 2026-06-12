use {
    crate::elite::ship::Ship,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub station_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub ship_price: u64,
    #[serde(alias = "ShipType")]
    pub ship: String,
    #[serde(alias = "ShipType_Localised")]
    pub ship_display: Option<String>,
    pub store_old_ship: String,
    #[serde(alias = "StoreShipID")]
    pub store_ship_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardNewEvent {
    #[serde(alias = "ShipType")]
    pub ship: String,
    #[serde(alias = "ShipType_Localised")]
    pub ship_display: Option<String>,
    #[serde(alias = "NewShipID")]
    pub ship_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSellEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub ship_price: u64,
    #[serde(alias = "ShipType")]
    pub ship: String,
    #[serde(alias = "ShipType_Localised")]
    pub ship_display: Option<String>,
    // seems like if fields below are some
    // it means that its remote sell
    #[serde(alias = "ShipMarketID")]
    pub ship_market_id: Option<u64>,
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSwapEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    #[serde(flatten)]
    pub ship: Ship,
    pub store_old_ship: String,
    #[serde(alias = "StoreShipID")]
    pub store_ship_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardTransferEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub distance: f64,
    pub transfer_price: u64,
    pub transfer_time: u64,
    #[serde(flatten)]
    pub ship: Ship,
    #[serde(alias = "ShipMarketID")]
    pub ship_market_id: Option<u64>,
    pub system: String,
}
