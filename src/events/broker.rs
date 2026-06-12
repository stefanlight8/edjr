use {
    crate::elite::{broker::BrokerType, material::Material},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commodity {
    pub count: u64,
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_display: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Technology {
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_display: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEvent {
    pub broker_type: BrokerType,
    pub commodities: Vec<Commodity>,
    pub materials: Vec<Material>,
    pub items_unlocked: Vec<Technology>,
    #[serde(alias = "MarketID")]
    pub market_id: u64,
}
