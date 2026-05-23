use {crate::elite::ship::Ship, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    star_system: String,
    station_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardBuyEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    ship_price: u64,
    #[serde(alias = "ShipType")]
    ship: String,
    #[serde(alias = "ShipType_Localised")]
    ship_display: Option<String>,
    store_old_ship: String,
    #[serde(alias = "StoreShipID")]
    store_ship_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardNewEvent {
    #[serde(alias = "ShipType")]
    ship: String,
    #[serde(alias = "ShipType_Localised")]
    ship_display: Option<String>,
    #[serde(alias = "NewShipID")]
    ship_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSellEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    ship_price: u64,
    #[serde(alias = "ShipType")]
    ship: String,
    #[serde(alias = "ShipType_Localised")]
    ship_display: Option<String>,
    // seems like if fields below are some
    // it means that its remote sell
    #[serde(alias = "ShipMarketID")]
    ship_market_id: Option<u64>,
    system: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardSwapEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    #[serde(flatten)]
    ship: Ship,
    store_old_ship: String,
    #[serde(alias = "StoreShipID")]
    store_ship_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardTransferEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    distance: f64,
    transfer_price: u64,
    transfer_time: u64,
    #[serde(flatten)]
    ship: Ship,
    #[serde(alias = "ShipMarketID")]
    ship_market_id: Option<u64>,
    system: String,
}
