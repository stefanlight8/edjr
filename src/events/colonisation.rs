use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resource {
    name: String,
    #[serde(alias = "Name_Localised")]
    name_display: Option<String>,
    payment: u64,
    required_amount: u64,
    provided_amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ColonisationConstructionDepotEvent {
    construction_complete: bool,
    construction_failed: bool,
    construction_progress: f64,
    #[serde(alias = "MarketID")]
    market_id: Option<u64>,
    resources_required: Vec<Resource>,
}
