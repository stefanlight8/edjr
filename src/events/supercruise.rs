use {
    crate::elite::body::BodyType,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseDestinationDropEvent {
    pub threat: u8,
    #[serde(alias = "Type")]
    pub destination: String,
    #[serde(alias = "Type_Localised")]
    pub destination_display: Option<String>,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseEntryEvent {
    pub star_system: String,
    pub system_address: u64,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub taxi: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseExitEvent {
    pub body: String,
    #[serde(alias = "BodyID")]
    pub body_id: u64,
    pub body_type: BodyType,
    pub star_system: String,
    pub system_address: u64,
    #[serde(default)]
    pub multicrew: bool,
    #[serde(default)]
    pub taxi: bool,
}
