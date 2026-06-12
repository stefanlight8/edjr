use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Vessel {
    Ship,
    #[serde(alias = "SRV")]
    Srv,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(alias = "Name", alias = "Type")]
    pub name: String,
    #[serde(alias = "Name_Localised", alias = "Type_Localised")]
    pub name_display: Option<String>,
    pub count: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEvent {
    pub count: u64,
    pub vessel: Vessel,
    pub inventory: Option<Vec<Cargo>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoDepotEvent {
    pub cargo_type: String,
    pub count: u64,
    pub progress: f64,
    pub items_collected: u64,
    pub items_delivered: u64,
    pub total_items_to_deliver: u64,
    #[serde(alias = "MissionID")]
    pub mission_id: u64,
    #[serde(alias = "StartMarketID")]
    pub start_market_id: u64,
    #[serde(alias = "EndMarketID")]
    pub end_market_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEvent {
    pub transfers: Vec<Cargo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CollectCargoEvent {
    pub stolen: bool,
    #[serde(flatten)]
    pub cargo: Cargo,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EjectCargoEvent {
    pub abandoned: bool,
    #[serde(flatten)]
    pub cargo: Cargo,
}
