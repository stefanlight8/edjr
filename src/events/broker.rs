use {
    crate::elite::{broker::BrokerType, material::Material},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commodity {
    count: u64,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Technology {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEvent {
    broker_type: BrokerType,
    commodities: Vec<Commodity>,
    materials: Vec<Material>,
    items_unlocked: Vec<Technology>,
    #[serde(alias = "MarketID")]
    market_id: u64,
}
