use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Vessel {
    Ship,
    #[serde(alias = "SRV")]
    Srv,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cargo {
    #[serde(alias = "Name", alias = "Type")]
    name: String,
    #[serde(alias = "Name_Localised", alias = "Type_Localised")]
    name_display: Option<String>,
    count: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEvent {
    count: u64,
    vessel: Vessel,
    inventory: Option<Vec<Cargo>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoDepotEvent {
    cargo_type: String,
    count: u64,
    progress: f64,
    items_collected: u64,
    items_delivered: u64,
    total_items_to_deliver: u64,
    #[serde(alias = "MissionID")]
    mission_id: u64,
    #[serde(alias = "StartMarketID")]
    start_market_id: u64,
    #[serde(alias = "EndMarketID")]
    end_market_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEvent {
    transfers: Vec<Cargo>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CollectCargoEvent {
    stolen: bool,
    #[serde(flatten)]
    cargo: Cargo,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EjectCargoEvent {
    abandoned: bool,
    #[serde(flatten)]
    cargo: Cargo,
}
