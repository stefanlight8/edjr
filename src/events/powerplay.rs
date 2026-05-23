use {crate::elite::powerplay::PowerMicroResource, serde::Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayEvent {
    merits: u64,
    power: String,
    rank: u64,
    time_pledged: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayCollectEvent {
    power: String,
    count: u64,
    #[serde(alias = "Type")]
    collected: String,
    #[serde(alias = "Type_Localised")]
    collected_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayMeritsEvent {
    merits_gained: u64,
    power: String,
    total_merits: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayRankEvent {
    power: String,
    rank: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestPowerMicroResourcesEvent {
    #[serde(alias = "MarketID")]
    market_id: u64,
    micro_resources: Vec<PowerMicroResource>,
    total_count: u64,
}
