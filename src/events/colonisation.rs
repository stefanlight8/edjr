use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resource {
    pub name: String,
    #[serde(alias = "Name_Localised")]
    pub name_display: Option<String>,
    pub payment: u64,
    pub required_amount: u64,
    pub provided_amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColonisationConstructionDepotEvent {
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub construction_progress: f64,
    #[serde(alias = "MarketID")]
    pub market_id: Option<u64>,
    pub resources_required: Vec<Resource>,
}
