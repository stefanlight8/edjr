use {
    crate::elite::station::Station,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBodyEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub star_system: String,
    pub system_address: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachSettlementEvent {
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: Option<String>,
    #[serde(flatten)]
    pub station: Option<Station>,
    pub system_address: u64,
}
