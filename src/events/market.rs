use {
    crate::elite::{fleet_carriers::DockingAccess, station::StationType},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    star_system: String,
    station_name: String,
    station_type: StationType,
    carrier_docking_access: Option<DockingAccess>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketBuyEvent {
    buy_price: u64,
    count: u64,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    total_cost: u64,
    #[serde(alias = "Type")]
    commodity: String,
    #[serde(alias = "Type_Localised")]
    commodity_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSellEvent {
    avg_price_paid: u64,
    sell_price: u64,
    count: u64,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    total_sale: u64,
    #[serde(alias = "Type")]
    commodity: String,
    #[serde(alias = "Type_Localised")]
    commodity_display: Option<String>,
}
