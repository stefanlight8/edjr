use {crate::elite::body::BodyType, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseDestinationDropEvent {
    threat: u8,
    #[serde(alias = "Type")]
    destination: String,
    #[serde(alias = "Type_Localised")]
    destination_display: Option<String>,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseEntryEvent {
    star_system: String,
    system_address: u64,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    taxi: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseExitEvent {
    body: String,
    #[serde(alias = "BodyID")]
    body_id: u64,
    body_type: BodyType,
    star_system: String,
    system_address: u64,
    #[serde(default)]
    multicrew: bool,
    #[serde(default)]
    taxi: bool,
}
