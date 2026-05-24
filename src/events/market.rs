use {crate::elite::station::StationType, serde::Deserialize};

#[cfg(feature = "fc")]
use crate::elite::fc::DockingAccess;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub star_system: String,
    pub station_name: String,
    pub station_type: StationType,
    #[cfg(feature = "fc")]
    pub carrier_docking_access: Option<DockingAccess>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketBuyEvent {
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub buy_price: u64,
    pub count: u64,
    pub total_cost: u64,
    #[serde(alias = "Type")]
    pub commodity: String,
    #[serde(alias = "Type_Localised")]
    pub commodity_display: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarketSellEvent {
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub avg_price_paid: u64,
    pub sell_price: u64,
    pub count: u64,
    pub total_sale: u64,
    #[serde(alias = "Type")]
    pub commodity: String,
    #[serde(alias = "Type_Localised")]
    pub commodity_display: Option<String>,
}
