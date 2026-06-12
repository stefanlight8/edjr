use {
    crate::elite::powerplay::PowerMicroResource,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayEvent {
    pub merits: u64,
    pub power: String,
    pub rank: u64,
    pub time_pledged: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayCollectEvent {
    pub power: String,
    pub count: u64,
    #[serde(alias = "Type")]
    pub collected: String,
    #[serde(alias = "Type_Localised")]
    pub collected_display: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayMeritsEvent {
    pub merits_gained: u64,
    pub power: String,
    pub total_merits: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayRankEvent {
    pub power: String,
    pub rank: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestPowerMicroResourcesEvent {
    #[serde(alias = "MarketID")]
    pub market_id: u64,
    pub micro_resources: Vec<PowerMicroResource>,
    pub total_count: u64,
}
